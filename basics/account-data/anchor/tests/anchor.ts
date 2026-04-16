import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Anchor } from "../target/types/anchor";
import { assert } from "chai";


describe("anchor", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
let payer = provider.wallet as anchor.Wallet;
const accountInfoAccount = anchor.web3.Keypair.generate();
  const program = anchor.workspace.anchor as Program<Anchor>;
    // Instruction Ix data
    const addressInfo = {
      name: "Joe C",
      houseNumber: 136,
      address: "Mile High Dr.",
      street: "Mile High Dr.",
      city: "Solana Beach",
    };

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.createAddressInfo(addressInfo.name, addressInfo.address,addressInfo.street, addressInfo.city, addressInfo.houseNumber).accounts({
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
  })
});
