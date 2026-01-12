import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Counter } from "../target/types/counter";

describe("anchor-1", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.counter as Program<Counter>;

  // initialize counter startsfrom 0
  it("initialize >> increment >> fetch", async () => {
    const provider = program.provider as anchor.AnchorProvider;
    const payer = provider.wallet as anchor.Wallet;

    const counterKeypair = anchor.web3.Keypair.generate();

    await program.methods.initialize()
      .accounts({
        counter: counterKeypair.publicKey,
        payer: payer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([counterKeypair])
      .rpc();

    //1st increment 0 >> 1
    await program.methods.increment()
      .accounts({
        counter: counterKeypair.publicKey,
      })
      .rpc();

    let counterAcc = await program.account.counter.fetch(
      counterKeypair.publicKey
    );

    console.log("after 1st increment count = ", counterAcc.count.toString());

    // Second increment 1 >> 2
    await program.methods.increment()
      .accounts({
        counter: counterKeypair.publicKey
      }).rpc();

    counterAcc = await program.account.counter.fetch(
      counterKeypair.publicKey
    );

    console.log("after 2ns increment count is", counterAcc.count.toString());

    // 3rd increment
    await program.methods.increment()
      .accounts({
        counter: counterKeypair.publicKey
      }).rpc();

    counterAcc = await program.account.counter.fetch(
      counterKeypair.publicKey
    );

    console.log("3rd increment counter is", counterAcc.count.toString());

  });
});
