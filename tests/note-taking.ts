import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { NoteTaking } from "../target/types/note_taking";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { assert } from "chai";


describe("note-taking", () => {
  const title = "Text Note";
  const message = "message dropp";

  const titlee = "updated title";
  const messagee = "updated message";
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.noteTaking as Program<NoteTaking>;
  const provider = anchor.AnchorProvider.env();

  let dataPda: PublicKey;
  let bump: number;

  it("Is initialized data!", async () => {

    [dataPda, bump] = PublicKey.findProgramAddressSync(
      [Buffer.from("data"), provider.wallet.publicKey.toBuffer()],
      program.programId
    );
    // Add your test here.
    const tx = await program.methods.initializeData(title, message)
      .accounts({
        data: dataPda,
        owner: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      }).rpc();

    console.log("Your transaction signature", tx);

    const account = await program.account.data.fetch(dataPda);

    assert.equal(account.title, title);
    assert.equal(account.message, message);
    assert.equal(
      account.owner.toBase58(),
      provider.wallet.publicKey.toBase58()
    );
    console.log("account.title", account.title);
    console.log("initialized title", title);
    console.log("account.message", account.message);
    console.log("initialized message", message);

  });

  it("updating data", async () => {

    [dataPda, bump] = PublicKey.findProgramAddressSync(
      [Buffer.from("data"), provider.wallet.publicKey.toBuffer()],
      program.programId
    );

    const tx = await program.methods.updateData(titlee, messagee)
      .accounts({
        data: dataPda,
        owner: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      }).rpc();

    console.log("updated tx signature", tx);

    const account = await program.account.data.fetch(dataPda);

    assert.equal(account.title, titlee);
    assert.equal(account.message, messagee);

    assert.equal(
      account.owner.toBase58(),
      provider.wallet.publicKey.toBase58()
    );

    console.log("messagee for update", messagee);
    console.log("titlee for update", titlee);
    console.log("updated title", account.title);
    console.log("updated message", account.message);
  });
});
