import {
  Keypair,
  PublicKey,
  Connection,
  Commitment
} from "@solana/web3.js";
import {
  getOrCreateAssociatedTokenAccount,
  mintTo
} from "@solana/spl-token";
import wallet from "./turbin3-wallet.json";

const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

const token_decimals = 1_000_000n; // 1 token
const mint = new PublicKey("HGzLKZNpadhhEm3Yd4WRoNgozr4ZdZspscA17zcthfza");

(async () => {
  try {
    const ata = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      mint,
      keypair.publicKey
    );

    const mintTx = await mintTo(
      connection,
      keypair,
      mint,
      ata.address,
      keypair.publicKey,
      token_decimals
    );
    
    console.log(`Minted 1 token: ${mintTx}`);
    
  } catch (error) {
    console.log(`Error: ${error}`);
  }
})();