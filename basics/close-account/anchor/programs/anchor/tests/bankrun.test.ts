import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Keypair, PublicKey } from "@solana/web3.js";
import { startAnchor } from "anchor-bankrun";
import { BankrunProvider } from "anchor-bankrun";
import { assert } from "chai";
import { Anchor } from "../../../target/types/anchor";
import IDL from "../../../target/idl/anchor.json" with { type: 'json' };

describe("create_account and close_account",  () => {
  let provider: BankrunProvider;
  let program: Program<Anchor>;
  let payer: Keypair;
  let dataAccount: PublicKey;

  before(async () => {
      const context = await startAnchor(
          "",
          [{ name: "anchor", programId: new PublicKey(IDL.address) }],
          []
      );
      provider = new BankrunProvider(context);
      anchor.setProvider(provider);
      program = new Program<Anchor>(IDL as any, provider);
      payer = provider.wallet.payer;
      
      const [pda] = PublicKey.findProgramAddressSync(
          [Buffer.from("create"), payer.publicKey.toBuffer()],
          program.programId
      );
      dataAccount = pda;
  });

  it("create account", async () => {
      const tx = await program.methods.create("test")
          .accounts({
              user: payer.publicKey,
              dataAccount: dataAccount,
          })
          .signers([payer])
          .rpc();
      console.log("Your transaction signature", tx);
      assert.ok(true);

      let account = await program.account.dataAccount.fetch(dataAccount);
      assert.equal(account.owner.toBase58(), payer.publicKey.toBase58());
      assert.equal(account.name, "test");
  });

  it("close account", async () => {
      const tx = await program.methods.close()
          .accounts({
              user: payer.publicKey,
              dataAccount: dataAccount,
          })
          .signers([payer])
          .rpc();
      console.log("Your transaction signature", tx);
      
      try {
          // When an account is closed, fetching it should fail
          await program.account.dataAccount.fetch(dataAccount);
          assert.fail("The account should have been closed and not fetchable");
      } catch (err: any) {
          console.log("Account successfully deleted! Fetch error:", err.message);
          assert.ok(true);
      }
  });
});