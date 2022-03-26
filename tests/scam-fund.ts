import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { ScamFund } from "../target/types/scam_fund";

describe("scam-fund", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.ScamFund as Program<ScamFund>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
