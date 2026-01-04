import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Valut } from "../target/types/valut";
import { TokenProgram } from "../target/types/token_program";

import { assert } from "chai";
import {
  getAssociatedTokenAddressSync,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import {
  program,
  SYSTEM_PROGRAM_ID,
} from "@coral-xyz/anchor/dist/cjs/native/system";

describe("vault", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const programVault = anchor.workspace.valut as Program<Valut>;
  const programToken = anchor.workspace.token_program as Program<TokenProgram>;
  const user = (provider.wallet as anchor.Wallet).payer;

  console.log(user.publicKey);

  const mint = anchor.web3.Keypair.generate();

  let vaultPdaAddress: anchor.web3.PublicKey;

  let valutPdaATA: anchor.web3.PublicKey;
  //user token ata
  const userTokenAta = getAssociatedTokenAddressSync(
    mint.publicKey,
    user.publicKey
  );
  before(async () => {
    //Airtdrop 5 sol to the user
    //
    // const sig = await provider.connection.requestAirdrop(
    //   user.publicKey,
    //   1 * anchor.web3.LAMPORTS_PER_SOL
    // );
    //
    // const latest = await provider.connection.getLatestBlockhash();
    // await provider.connection.confirmTransaction({
    //   signature: sig,
    //   ...latest,
    // });

    [vaultPdaAddress] = anchor.web3.PublicKey.findProgramAddressSync(
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

    const userAtaBalance = await provider.connection.getTokenAccountBalance(
      userTokenAta
    );

    const userBalance = await provider.connection.getBalance(user.publicKey);

    assert.equal(userAtaBalance.value.amount, "0");
  });

  //Mint Token To user ATA
  it("Mint 100 Token to user Ata", async () => {
    const amount = new anchor.BN(100);

    const txn = await programToken.methods
      .mintToken(amount)
      .accounts({
        signer: user.publicKey,
        mint: mint.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([user])
      .rpc();

    const signature = await provider.connection.confirmTransaction(txn);

    if (!signature)
      throw new Error("The mint 100 token to user is not successfull");

    const userAta = await getAssociatedTokenAddressSync(
      mint.publicKey,
      user.publicKey
    );

    const userAtaBalance = await provider.connection.getTokenAccountBalance(
      userAta
    );

    assert.equal(userAtaBalance.value.amount, "100");
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

    const vaultData = await provider.connection.getAccountInfo(vaultPdaAddress);
    assert.equal(vaultData.owner.toString(), programVault.programId.toString());
  });

  it("Create vaultPda Token ATA Transfer", async () => {
    const [vaultPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), user.publicKey.toBuffer()],
      programVault.programId
    );

    const vaultTokenAccount = getAssociatedTokenAddressSync(
      mint.publicKey, // mint
      vaultPda, // owner (PDA)
      true // allowOwnerOffCurve = true, because PDA
    );

    //create the ata and transferToken
    const txn = await programVault.methods
      .transferToken()
      .accountsPartial({
        signer: user.publicKey,
        vaultPda,
        userTokenAccount: userTokenAta,
        vaultTokenAccount,
        mintAccount: mint.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID, // spl-token program
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    const signature = await provider.connection.confirmTransaction(txn);

    if (!signature) throw new Error("Transacton not successful");

    const vaultAta = getAssociatedTokenAddressSync(
      mint.publicKey,
      vaultPda,
      true
    );
    // const bal = await provider.connection.getTokenAccountBalance(vaultAta);
    // console.log(bal.value.amount);

    //assert.equal(userTokenBalance, bal);
    const valuAtaInfo = await provider.connection.getAccountInfo(vaultAta);
    assert.equal(valuAtaInfo.owner, vaultPda);
  });

  it("withdraw", async () => {
    const [vaultPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), user.publicKey.toBuffer()],
      programVault.programId
    );
    const amount = new anchor.BN(100);

    const tx = await programVault.methods
      .withdraw(amount)
      .accounts({
        signer: user.publicKey,
        vaultTokenAta: valutPdaATA,
        mintAccount: mint.publicKey,
        userTokenAta: userTokenAta,
      })
      .signers([user])
      .rpc();

    const signature = await provider.connection.confirmTransaction(tx);

    const userTokenBalance = await provider.connection.getTokenAccountBalance(
      userTokenAta
    );

    assert.equal(userTokenBalance.value.amount, "100");

    const vaultTokenAccountBalane =
      await provider.connection.getTokenAccountBalance(valutPdaATA);
    assert.equal(vaultTokenAccountBalane.value.amount, "O");
  });
});
