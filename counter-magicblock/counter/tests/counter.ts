import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CounterMagicblock } from "../target/types/counter_magicblock";
import { assert } from "chai";

describe("counter_magicblock", () => {
  // Configure the client to use the local cluster
  const ER_VALIDATOR = new anchor.web3.PublicKey(
    "8eBRf6JsumA9Wgu61TQURQnac7ZCsjqiSSFYhxnhTot9"
  );
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const providerEphemeralRollup = new anchor.AnchorProvider(
    new anchor.web3.Connection("https://devnet-as.magicblock.app/", {
      wsEndpoint: "wss://devnet.magicblock.app/",
    }),
    anchor.Wallet.local()
  );
  const program = anchor.workspace
    .counter_magicblock as Program<CounterMagicblock>;

  let signer = (provider.wallet as anchor.Wallet).payer;

  const [pda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("count")],
    program.programId
  );

  it("Is initialized!", async () => {
    // Add your test here.
    try {
      const tx = await program.methods
        .initialize()
        .accounts({
          signer: signer.publicKey,
        })
        .signers([signer])
        .rpc();

      const signature = await provider.connection.confirmTransaction(tx);

      if (!signature) throw new Error("Error in initialize");
      console.log("Your transaction signature", tx);

      const counterAccount = await program.account.counter.fetch(pda);

      assert.equal(counterAccount.no.toString(), "0");
    } catch (error) {
      throw Error("something went wrong");
    }
  });

  it("delegate", async () => {
    let deltx = await program.methods
      .delegate()
      .accounts({
        payer: signer.publicKey,
        validator: ER_VALIDATOR,
        pda,
      })
      .signers([signer])
      .rpc();
    const deltxHash = await provider.connection.confirmTransaction(deltx);

    if (!deltxHash) throw Error("error in deltx transaction");
  });

  it("increment", async () => {
    let inctx = await program.methods
      .incrementAndCommit()
      .accounts({
        payer: signer.publicKey,
      })
      .signers([signer])
      .rpc();
    let inctxHash = await provider.connection.confirmTransaction(inctx);

    if(!inctxHash) throw Error("Error in incrementAndCommit")

    con
  });
});
