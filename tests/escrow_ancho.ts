import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { EscrowAncho } from "../target/types/escrow_ancho";

describe("escrow_ancho", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.EscrowAncho as Program<EscrowAncho>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
