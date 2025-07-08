import wallet from "./turbin3-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"

const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        const imageUri = "https://gateway.irys.xyz/BQQUL4CZ7iYTrhZUwfzgvoBwXLpqW5uea3V1LLkDJLVf";
        
        const metadata = {
            name: "Monty Rug",
            symbol: "MNTYRUG",
            description: "Rug day NFT",
            image: imageUri,
            attributes: [
                {trait_type: 'Rarity', value: 'Common'},
                {trait_type: 'Power', value: '100'}
            ],
            properties: {
                files: [
                    {
                        type: "image/png",
                        uri: imageUri
                    },
                ]
            },
            creators: [
                {
                    address: signer.publicKey,
                    verified: true,
                    share: 100
                }
            ]
        };

        const metadataFile = createGenericFile(JSON.stringify(metadata), "metadata.json", {
            contentType: "application/json"
        });

        const [myUri] = await umi.uploader.upload([metadataFile]);
        
        console.log("Your metadata URI: ", myUri);
        
        console.log("Save this URI for the minting step!");
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();