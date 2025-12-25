import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TokenProgram } from "../target/types/token_program";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { assert, expect } from "chai";
import { it } from "mocha";
import { getAssociatedTokenAddressSync } from "@solana/spl-token";
//You need to install  @solana/spl-token  separately - it’s not included in Anchor by default.
//This is a standard dependency for working with SPL tokens in tests.

describe("token_program", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.tokenProgram as Program<TokenProgram>;

  const user = anchor.web3.Keypair.generate();

  const receiver = anchor.web3.Keypair.generate();

  const mint = anchor.web3.Keypair.generate();
  console.log(`USER publicKey - ${user.publicKey}`);
  console.log(`RECEIVER publicKey - ${receiver.publicKey}`);
  console.log(`MINT publicKey - ${mint.publicKey}`);
  console.log(`TOKEN_PROGRAM_ID - ${TOKEN_PROGRAM_ID}`);

  const getATAAdress = async (key) => {
    const address = getAssociatedTokenAddressSync(mint.publicKey, key);
    return address;
  };

  before(
    //put some sol into the user by airdrop
    async () => {
      const airdrop = await provider.connection.requestAirdrop(
        user.publicKey,
        5 * anchor.web3.LAMPORTS_PER_SOL
      );
      await provider.connection.confirmTransaction(airdrop, "confirmed");
      const airdropReceiver = await provider.connection.requestAirdrop(
        receiver.publicKey,
        3 * anchor.web3.LAMPORTS_PER_SOL
      );
      await provider.connection.confirmTransaction(
        airdropReceiver,
        "confirmed"
      );
    }
  );

  it("Is initialized!", async () => {
    //Creating the mint account
    const tx = await program.methods
      .createMint()
      .accounts({
        user: user.publicKey,
        mint: mint.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      //Both the user and mint account have to sign
      .signers([user, mint])
      .rpc();

    await provider.connection.confirmTransaction(tx);
    console.log("Your transaction signature", tx);

    const accountInfo = await provider.connection.getAccountInfo(
      mint.publicKey
    );
    console.log(`Mint accountInfo:- ${accountInfo.owner}`);
    assert.equal(TOKEN_PROGRAM_ID.toString(), accountInfo.owner.toString());
  });

  it("createTokenAccount", async () => {
    const tx = await program.methods
      .createTokenAccount()
      .accounts({
        user: user.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        mint: mint.publicKey,
      })
      .signers([user])
      .rpc();
    const signature = await provider.connection.confirmTransaction(tx);
    console.log(`the signature is ${signature}`);
    // ✅ Derive the ATA AFTER creation to verify
    const tokenAccount = getAssociatedTokenAddressSync(
      mint.publicKey,
      user.publicKey
      // false,
      // TOKEN_PROGRAM_ID
    );

    console.log(`✅ ATA was created at: ${tokenAccount.toString()}`);

    // ✅ VERIFY IT EISTS
    const accountInfo = await provider.connection.getAccountInfo(tokenAccount);
    assert.ok(accountInfo, "Token account should exist");
    assert.equal(
      accountInfo.owner.toString(),
      TOKEN_PROGRAM_ID.toString(),
      "Owner should be Token Program"
    );

    console.log(`✅ ATA verified - Owner: ${accountInfo.owner.toString()}`);
  });

  it("Minting the token", async () => {
    // Get the  tokenAccount (ATA)
    const tokenATA = await getAssociatedTokenAddressSync(
      mint.publicKey,
      user.publicKey
    );
    console.log(`Mint TokenATA:- ${tokenATA}`);

    const tokenAccount = await provider.connection.getAccountInfo(tokenATA);
    console.log(`The tokenAccount info  ${tokenAccount}`);
    const amount = new anchor.BN(110000);
    // why we have to do it????????
    //
    //
    const inx = await program.methods
      .mintToken(amount)
      .accounts({
        user: user.publicKey,
        mint: mint.publicKey,
        tokenAccount: tokenATA,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([user])
      .rpc();

    console.log(`the mint token ${inx}`);
    const signature = await provider.connection.confirmTransaction(inx);

    console.log(`The signature of the mint token is :${signature}`);

    const tokenAccount1 = await provider.connection.getTokenAccountBalance(
      tokenATA
    );
    console.log(`tokenAccount lamports :${tokenAccount1}}`);
    assert.equal(parseInt(tokenAccount1.value.amount), 110000);
  });

  //Test for transfer
  it("Trasfer the token ", async () => {
    const amount = new anchor.BN(80000);
    const userATA = await getATAAdress(user.publicKey);

    // The ata of receiver
    const createReceiverATA = await program.methods
      .createTokenAccount()
      .accounts({
        user: receiver.publicKey,
        mint: mint.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([receiver])
      .rpc();

    const receiverATA = await getATAAdress(receiver.publicKey);
    const inx = await program.methods
      .transferToken(amount)
      .accounts({
        user: user.publicKey,
        fromTokenAccount: userATA,
        toAccount: receiverATA,
        mint: mint.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([user])
      .rpc();

    const signature = await provider.connection.confirmTransaction(inx);

    const ata = await provider.connection.getTokenAccountBalance(userATA);

    assert.equal(parseInt(ata.value.amount), 110000 - 80000);
  });
});
