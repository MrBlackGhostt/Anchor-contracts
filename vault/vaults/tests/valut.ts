import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Valut } from "../target/types/valut";
import { assert } from "chai";

describe("vault", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.valut as Program<Valut>;
  const user = anchor.web3.Keypair.generate();

  let vaultAddress: anchor.web3.PublicKey;

  before(async () => {
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(
        user.publicKey,
        5 * anchor.web3.LAMPORTS_PER_SOL
      )
    );

    [vaultAddress] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), user.publicKey.toBuffer()],
      program.programId
    );
  });

  it("Is deposit!", async () => {
    const amount = new anchor.BN(10_000_000);

    await program.methods
      .deposit(amount)
      .accounts({
        user: user.publicKey,
        vault: vaultAddress,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    const vaultBal = await provider.connection.getBalance(vaultAddress);

    assert.equal(vaultBal, amount.toNumber());
  });

  it("Withdraw", async () => {
    const amount = new anchor.BN(10_000_000);
    const seeds = [Buffer.from("vault"), user.publicKey.toBuffer()];
    const ixn = await program.methods
      .withdraw(amount)
      .accounts({
        user: user.publicKey,
        vault: vaultAddress,
      })
      .signers([user])
      .rpc();

    const signature = await provider.connection.confirmTransaction(ixn);
    if (!signature) throw new Error("Withdraw transaction is failes");

    const vaultBalance = await provider.connection.getBalance(vaultAddress);

    const userBalance = await provider.connection.getBalance(user.publicKey);

    assert.equal(vaultBalance, 0);
    assert.equal(userBalance, 5 * anchor.web3.LAMPORTS_PER_SOL);
  });
});
