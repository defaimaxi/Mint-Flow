import { 
    Connection, 
    Keypair, 
    PublicKey, 
    Transaction,
    clusterApiUrl
} from '@solana/web3.js';
import { Program, AnchorProvider, Wallet } from '@project-serum/anchor';
import { IDL } from '../idl/nft_minter'; // TODO We need to generate this
import dotenv from 'dotenv';

dotenv.config();

export class SolanaService {
    private connection: Connection;
    private program: Program;
    private wallet: Wallet;

    constructor() {
        // Use environment variables for network selection
        const network = process.env.SOLANA_NETWORK || 'devnet';
        const programId = new PublicKey(process.env.PROGRAM_ID || '');
        
        this.connection = new Connection(
            network === 'mainnet-beta' 
                ? process.env.MAINNET_RPC_URL! 
                : clusterApiUrl('devnet'),
            'confirmed'
        );

        // Initialize wallet TODO (We might want to inject this)
        this.wallet = new Wallet(Keypair.generate()); // TODO Replace with actual wallet logic

        // Initialize provider
        const provider = new AnchorProvider(
            this.connection,
            this.wallet,
            { commitment: 'confirmed' }
        );

        // Initialize program
        this.program = new Program(IDL, programId, provider);
    }

    async initializeNFTMinter(
        bufferSize: number,
        minBufferThreshold: number,
        collectionMint: PublicKey
    ) {
        try {
            const tx = await this.program.methods
                .initialize(bufferSize, minBufferThreshold, collectionMint)
                .rpc();
            return tx;
        } catch (error) {
            console.error('Error initializing NFT minter:', error);
            throw error;
        }
    }

    async addToBuffer(
        metadataUri: string,
        name: string,
        symbol: string,
        sellerFeeBasisPoints: number
    ) {
        try {
            const tx = await this.program.methods
                .addToBuffer(metadataUri, name, symbol, sellerFeeBasisPoints)
                .rpc();
            return tx;
        } catch (error) {
            console.error('Error adding to buffer:', error);
            throw error;
        }
    }

    async mintNFT(
        creatorBump: number,
        metadataTitle: string,
        metadataSymbol: string,
        metadataUri: string
    ) {
        try {
            const tx = await this.program.methods
                .mintNft(creatorBump, metadataTitle, metadataSymbol, metadataUri)
                .rpc();
            return tx;
        } catch (error) {
            console.error('Error minting NFT:', error);
            throw error;
        }
    }

    // Helper methods
    getConnection() {
        return this.connection;
    }

    getProgram() {
        return this.program;
    }
}
