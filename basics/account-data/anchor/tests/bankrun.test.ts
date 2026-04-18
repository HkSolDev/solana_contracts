import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Keypair, PublicKey } from "@solana/web3.js";
import { startAnchor } from "anchor-bankrun";
import { BankrunProvider } from "anchor-bankrun";
import { assert } from "chai";
import { Anchor } from "../target/types/anchor";
import IDL from "../target/idl/anchor.json" with { type: 'json' };

describe("anchor-bankrun test", () => {
  let provider: BankrunProvider;
  let program: Program<Anchor>;
  let payer: Keypair;
  let accountInfoAccount: Keypair;

  const addressInfo = {
    name: "Joe C",
    houseNumber: 136,
    address: "Mile High Dr.",
    street: "Mile High Dr.",
    city: "Solana Beach",
  };

  before(async () => {
    // Start the bankrun context with our program
    // You can pass the path to your workspace directory if needed,
    // but startAnchor by default loads programs from the workspace.
    const context = await startAnchor(
      "", 
      [{ name: "anchor", programId: new PublicKey(IDL.address) }], 
      []
    );
    provider = new BankrunProvider(context);
    anchor.setProvider(provider);
    payer = provider.wallet.payer;
    
    // Instantiate the program
    program = new Program<Anchor>(IDL as any, provider);
    accountInfoAccount = Keypair.generate();
  });

  it("Is initialized!", async () => {
    const tx = await program.methods.createAddressInfo(
        addressInfo.name, 
        addressInfo.address,
        addressInfo.street, 
        addressInfo.city, 
        addressInfo.houseNumber
      )
      .accounts({
        user: payer.publicKey,
        addressInfo: accountInfoAccount.publicKey,
      })
      .signers([accountInfoAccount])
      .rpc();
      
    console.log("Your transaction signature", tx);
    assert.ok(true);
  });

  it("Read the value", async () => {
    const account = await program.account.addressInfo.fetch(accountInfoAccount.publicKey);
    assert.equal(account.name, addressInfo.name);
    assert.equal(account.address, addressInfo.address);
    assert.equal(account.street, addressInfo.street);
    assert.equal(account.city, addressInfo.city);
    assert.equal(account.houseNo, addressInfo.houseNumber);
    assert.equal(account.owner.toBase58(), payer.publicKey.toBase58());
    console.log("Account", account);
    assert.ok(true);
  });
});
