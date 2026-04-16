import { BorshCoder } from "@anchor-lang/core";
import { Keypair, PublicKey, SystemProgram, Transaction, TransactionInstruction } from "@solana/web3.js";
import { LiteSVM } from "litesvm";
import { assert } from "chai";

import IDL from "../target/idl/anchor.json" with { type: 'json' };

describe("anchor", () => {
    let litesvm: LiteSVM;
    let programId: PublicKey;
    let payer: Keypair;
    let accountInfoAccount: Keypair;
    let coder: BorshCoder;

    before(() => {
        litesvm = new LiteSVM();
        programId = new PublicKey(IDL.address);
        payer = Keypair.generate();
        accountInfoAccount = Keypair.generate();
        coder = new BorshCoder(IDL as any);

        // Locates the compiled program binary
        const programPath = new URL("../target/deploy/anchor.so", 
            // @ts-expect-error 
            import.meta.url
        ).pathname;
        
        litesvm.addProgramFromFile(programId, programPath);
        litesvm.airdrop(payer.publicKey, BigInt(100000000000));
    });

    it("Create the address info account", () => {
        // Instruction arguments
        const addressInfoIns = {
            name: "Joe C",
            address: "Mile High Dr.",
            street: "Mile High Dr.",
            city: "Solana Beach",
            house_no: 136,
        };

        // 1. Encode the data into binary using the Coder solana programs don't know the name of the fun the anchor give the each
        //instruction a unique id by passig the name of the fun the program go to the IDL find the fun named create_address_info and use its 
        //rules to turn my variables into a binary message 
        const data = coder.instruction.encode("create_address_info", addressInfoIns);

        // 2. Build the Instruction manually
        const ix = new TransactionInstruction({
            keys: [
                { pubkey: payer.publicKey, isSigner: true, isWritable: true },
                { pubkey: accountInfoAccount.publicKey, isSigner: true, isWritable: true },
                { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
            ],
            programId,
            data,
        });

        // 3. Wrap in a Transaction and sign
        const tx = new Transaction().add(ix);
        tx.feePayer = payer.publicKey;
        tx.recentBlockhash = litesvm.latestBlockhash();
        tx.sign(payer, accountInfoAccount);

        // 4. Send to LiteSVM
        litesvm.sendTransaction(tx);
    });

    it("Read the new account's data", () => {
        // 5. Fetch raw account data from LiteSVM
        const accountInfoAcc = litesvm.getAccount(accountInfoAccount.publicKey);
        if (!accountInfoAcc) {
            throw new Error("Failed to fetch account info");
        }

        // 6. Decode the binary data back into a JSON object
        const addressInfo = coder.accounts.decode("AddressInfo", Buffer.from(accountInfoAcc.data));

        console.log(`Name     : ${addressInfo.name}`);
        console.log(`House Num: ${addressInfo.house_no}`);
        
        assert.equal(addressInfo.name, "Joe C");
        assert.equal(addressInfo.house_no, 136);
    });
});
