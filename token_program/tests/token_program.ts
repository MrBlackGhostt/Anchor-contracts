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

  before(
    //put some sol into the user by airdrop
    async () => {
      const airdrop = await provider.connection.requestAirdrop(
        user.publicKey,
        2 * anchor.web3.LAMPORTS_PER_SOL
      );
      await provider.connection.confirmTransaction(airdrop, "confirmed");
    }
  );

  const seeds = Buffer.from("ata");

  const derivePda = () => {
    return anchor.web3.PublicKey.createProgramAddressSync(
      [seeds, user.publicKey.toBuffer(), mint.publicKey.toBuffer()],
      TOKEN_PROGRAM_ID
    );
  };
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
      user.publicKey,
      false,
      TOKEN_PROGRAM_ID
    );

    console.log(`✅ ATA was created at: ${tokenAccount.toString()}`);

    // ✅ VERIFY IT EXISTS
    const accountInfo = await provider.connection.getAccountInfo(tokenAccount);
    assert.ok(accountInfo, "Token account should exist");
    assert.equal(
      accountInfo.owner.toString(),
      TOKEN_PROGRAM_ID.toString(),
      "Owner should be Token Program"
    );

    console.log(`✅ ATA verified - Owner: ${accountInfo.owner.toString()}`);
  });
});
