// app/src/services/ipfs-service.ts
import { create } from 'ipfs-http-client';

const ipfs = create({ url: process.env.IPFS_NODE_URL || 'http://localhost:5001' });

export async function uploadToIPFS(data: Buffer | string): Promise<string> {
    const result = await ipfs.add(data);
    return `ipfs://${result.cid.toString()}`;
}
