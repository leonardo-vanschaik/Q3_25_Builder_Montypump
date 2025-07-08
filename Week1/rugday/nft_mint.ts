import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createSignerFromKeypair, signerIdentity, generateSigner, percentAmount } from "@metaplex-foundation/umi"
import { createNft, mplTokenMetadata } from "@metaplex-foundation/mpl-token-metadata";

import wallet from "./turbin3-wallet.json"
import base58 from "bs58";

const RPC_ENDPOINT = "https://api.devnet.solana.com";
const umi = createUmi(RPC_ENDPOINT);

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const myKeypairSigner = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(myKeypairSigner));
umi.use(mplTokenMetadata())

const mint = generateSigner(umi);

(async () => {
    try {
        const metadataUri = "https://gateway.irys.xyz/EQNK1abNbcezock3uxwcUcTPNK5CwfyHfz2ebs4eyBF4";
        
        let tx = createNft(umi, {
            mint,
            name: "Monty Rug",
            symbol: "MNTYRUG",
            uri: metadataUri,
            sellerFeeBasisPoints: percentAmount(5.5),
            creators: [
                {
                    address: myKeypairSigner.publicKey,
                    verified: true,
                    share: 100,
                },
            ],
        });

        let result = await tx.sendAndConfirm(umi);
        const signature = base58.encode(result.signature);
        
        console.log(`Successfully Minted! Check out your TX here:\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`);
        console.log("Mint Address: ", mint.publicKey);
    } catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();