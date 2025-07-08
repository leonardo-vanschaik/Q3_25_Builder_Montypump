import wallet from "./turbin3-wallet.json";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
  createMetadataAccountV3,
  CreateMetadataAccountV3InstructionAccounts,
  CreateMetadataAccountV3InstructionArgs,
  DataV2Args,
} from "@metaplex-foundation/mpl-token-metadata";
import {
  createSignerFromKeypair,
  signerIdentity,
  publicKey,
} from "@metaplex-foundation/umi";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";

const mint = publicKey("HGzLKZNpadhhEm3Yd4WRoNgozr4ZdZspscA17zcthfza");

const umi = createUmi("https://api.devnet.solana.com");
const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(signer));

(async () => {
  try {
    let data: DataV2Args = {
      name: "Pr0p3ll3r",
      symbol: "PROPLR",
      uri: "https://gateway.irys.xyz/9B8jybkN8PpSEr92Qv3adnzDcdg3reN4WFN4dg1rCtFt",
      sellerFeeBasisPoints: 0,
      creators: null,
      collection: null,
      uses: null,
    };

    let args: CreateMetadataAccountV3InstructionArgs = {
      data,
      isMutable: true,
      collectionDetails: null,
    };

    let accounts: CreateMetadataAccountV3InstructionAccounts = {
      mint,
      mintAuthority: signer,
      payer: signer,
      updateAuthority: signer,
    };

    let tx = createMetadataAccountV3(umi, { ...accounts, ...args });
    let result = await tx.sendAndConfirm(umi);
    
    console.log(`Metadata created: ${bs58.encode(result.signature)}`);
    
  } catch (e) {
    console.error(`Error: ${e}`);
  }
})();