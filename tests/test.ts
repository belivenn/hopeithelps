import { AnchorProvider, BN, Program, Wallet, web3 } from "@coral-xyz/anchor"
import { SecuritySeries, IDL} from "../target/types/security_series"
const kpFile = "/home/beliv/wba/anchor/wba_vault/akBRJuA7rAPm4ZTqAbcQyVS1iki44w8c7hukP5wmRGs.json"
const fs = require("fs")
const kp : web3.Keypair = web3.Keypair.fromSecretKey(
  new Uint8Array(JSON.parse(fs.readFileSync(kpFile).toString())),
);
const wallet = new Wallet(kp);
const c = new web3.Connection("https://api.devnet.solana.com/ ")
const provider = new AnchorProvider(c, wallet, {});
const programId = new web3.PublicKey("BaRtR9LkkzMWysBndwLf1wwWyHTHCYEGiPPa2ooXmodW")
const program = new Program<SecuritySeries>(IDL, programId, provider)

async function initPlayer() {
  const playerAccKP = web3.Keypair.generate();
  const [pda, _bump] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("player"), kp.publicKey.toBytes()], programId);
  const tx = await program.methods.doNothing()
  .accounts({
      player: kp.publicKey,
      // playerAccount: playerAccKP.publicKey,// pda,
      systemProgram: web3.SystemProgram.programId
  })
  // .signers([playerAccKP])
  .transaction();

  const sx = await c.sendTransaction(tx, [kp]);
  console.log(sx)

}
initPlayer();