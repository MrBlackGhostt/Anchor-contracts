import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Valut } from "../target/types/valut";
import { TokenProgram } from "../target/types/token_program";

import { assert } from "chai";
import {
  getAssociatedTokenAddressSync,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";

describe("vault", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const programVault = anchor.workspace.valut as Program<Valut>;
  const programToken = anchor.workspace.token_program as Program<TokenProgram>;
  const user = anchor.web3.Keypair.generate();

  const mint = anchor.web3.Keypair.generate();

  let vaultAddress: anchor.web3.PublicKey;

  //user token ata
  const userTokenAta = getAssociatedTokenAddressSync(
    programToken.programId,
    user.publicKey
  );

  before(async () => {
    //Airtdrop 5 sol to the user
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(
        user.publicKey,
        5 * anchor.web3.LAMPORTS_PER_SOL
      )
    );

    [vaultAddress] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), user.publicKey.toBuffer()],
      programVault.programId
    );
  });

  it("CreateMint", async () => {
    const tx = await programToken.methods
      .createMint()
      .accounts({
        signer: user.publicKey,
        mint: mint.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([user, mint])
      .rpc();

    const check = await provider.connection.confirmTransaction(tx);

    if (!check) {
      throw new Error("transaction failed");
    }

    const mintAccount = await provider.connection.getAccountInfo(
      mint.publicKey
    );
    if (!mintAccount) throw new Error("The mint account not found");
    assert.equal(mintAccount.owner.toString(), TOKEN_PROGRAM_ID.toString());
  });

  //Create the Token ATA for the user
  it("Create Token Account", async () => {
    const amount = new anchor.BN(10_000_000);
    const seeds = [Buffer.from("vault"), user.publicKey.toBuffer()];
    const ixn = await programToken.methods
      .createTokenAccount()
      .accounts({
        signer: user.publicKey,
        mint: mint.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([user])
      .rpc();

    const signature = await provider.connection.confirmTransaction(ixn);
    if (!signature) throw new Error("Withdraw transaction is failes");

    const userAtaBalance = await provider.connection.getBalance(userTokenAta);

    const userBalance = await provider.connection.getBalance(user.publicKey);

    assert.equal(userAtaBalance, 0);
  });

  //Mint Token To user ATA
  it("Mint 100 Token to user Ata", async () => {
    const amount = new anchor.BN(100);

    const txn = await programToken.methods
      .mintToken(amount)
      .accounts({
        signer: user.publicKey,
        mint: mint.publicKey,
        tokenAccount: userTokenAta,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([user])
      .rpc();

    const signature = await provider.connection.confirmTransaction(txn);

    if (!signature)
      throw new Error("The mint 100 token to user is not successfull");

    const userAtaBalance = await provider.connection.getBalance(userTokenAta);

    assert.equal(userAtaBalance, 100);
  });

  //Create User Valut create_valut_pda
  it("Create User valut pda", async () => {
    const txn = await programVault.methods
      .createValutPda()
      .accounts({
        signer: user.publicKey,
      })
      .signers([user])
      .rpc();

    const signature = await provider.connection.confirmTransaction(txn);

    if (!signature)
      throw new Error("Create pda for the user in the valut failed");

    const vaultPdaAccount = await provider.connection.getAccountInfo(
      vaultAddress
    );

    assert.equal(vaultPdaAccount.owner, user.publicKey);
  });
});
