// app/src/index.ts
import { Connection, Keypair } from '@solana/web3.js';
import { Program, AnchorProvider } from '@project-serum/anchor';
import { NFTGeneratorService } from './services/nft-generator';
import dotenv from 'dotenv';

dotenv.config();

async function main() {
    const connection = new Connection(process.env.SOLANA_RPC_URL!, 'confirmed');
    const wallet = Keypair.fromSecretKey(
        Buffer.from(JSON.parse(process.env.WALLET_PRIVATE_KEY!))
    );
    
    const provider = new AnchorProvider(
        connection,
        wallet,
        { commitment: 'confirmed' }
    );

    // Initialize program (you'll need to add your program ID and IDL)
    const program = new Program(IDL, PROGRAM_ID, provider);

    const nftGenerator = new NFTGeneratorService(
        connection,
        program,
        process.env.OPENAI_API_KEY!
    );

    await nftGenerator.monitorEvents();
}

main().catch(console.error);
