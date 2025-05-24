// app/src/services/nft-generator.ts
import { Connection, PublicKey, Keypair } from '@solana/web3.js';
import { Program, AnchorProvider } from '@project-serum/anchor';
import axios from 'axios';
import dotenv from 'dotenv';
dotenv.config();

export class NFTGeneratorService {
    private connection: Connection;
    private program: Program;
    private imageServiceUrl: string;
    private imageServiceApiKey: string;

    constructor(
        connection: Connection,
        program: Program,
    ) {
        this.connection = connection;
        this.program = program;
        
        this.imageServiceUrl = process.env.IMAGE_SERVICE_URL;
        this.imageServiceApiKey = process.env.IMAGE_SERVICE_API_KEY;

        if (!this.imageServiceUrl || !this.imageServiceApiKey) {
            throw new Error('Missing IMAGE_SERVICE configuration');
        }
    }

    async monitorEvents() {
        console.log('Starting event monitoring...');
        
        // Subscribe to program account changes
        this.connection.onProgramAccountChange(
            this.program.programId,
            async (accountInfo) => {
                try {
                    const decodedAccount = this.program.coder.accounts.decode(
                        'state',
                        accountInfo.accountInfo.data
                    );

                    if (decodedAccount.pendingNfts.length <= decodedAccount.minBufferThreshold) {
                        await this.replenishNFTBuffer(
                            decodedAccount.bufferSize - decodedAccount.pendingNfts.length
                        );
                    }
                } catch (error) {
                    console.error('Error processing account change:', error);
                }
            }
        );
    }

    async replenishNFTBuffer(count: number) {
        console.log(`Requesting ${count} NFTs from approved pool...`);

        try {
            // Request pre-approved NFTs from the Image Service. We're creating a completely custom NFT art system never seen before. You thought we would just share it with everyone? 
            // Moved that portion of code to a private service for now, we might share it later.
            const response = await axios.get(
                `${this.imageServiceUrl}/api/events/current/nfts/random/${count}`,
                {
                    headers: {
                        'Authorization': `Bearer ${this.imageServiceApiKey}`,
                    }
                }
            );

            const approvedNFTs = response.data;

            for (const nft of approvedNFTs) {
                try {
                    // Add to buffer
                    await this.program.methods
                        .addToBuffer(
                            nft.metadataUri,
                            nft.name,
                            'NEWS',
                            nft.price || 500
                        )
                        .accounts({
                            state: this.program.state.address,
                            authority: this.program.provider.wallet.publicKey,
                        })
                        .rpc();

                    // Notify Image Service that NFT is now in buffer
                    await axios.post(
                        `${this.imageServiceUrl}/api/nfts/${nft.id}/status`,
                        { status: 'in_buffer' },
                        {
                            headers: {
                                'Authorization': `Bearer ${this.imageServiceApiKey}`,
                            }
                        }
                    );

                } catch (error) {
                    console.error('Error adding NFT to buffer:', error);
                }
            }
        } catch (error) {
            console.error('Error fetching approved NFTs:', error);
        }
    }
}

