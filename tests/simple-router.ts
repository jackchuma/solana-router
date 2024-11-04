import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";

import { SimpleRouter } from "../target/types/simple_router";
import { CallTarget } from "../target/types/call_target";

describe("simple-router", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.SimpleRouter as Program<SimpleRouter>;
    const target = anchor.workspace.CallTarget as Program<CallTarget>;

    const caller = anchor.web3.Keypair.generate();

    let transactionAccounts: any;
    let remainingAccounts: any;
    let signers: any;
    let call: any;

    beforeEach(() => {
        transactionAccounts = [
            {
                isSigner: true,
                isWriteable: true,
                pubkey: caller.publicKey,
            },
        ];
        remainingAccounts = [
            {
                isSigner: true,
                isWriteable: true,
                pubkey: caller.publicKey,
            },
            {
                isSigner: false,
                isWritable: false,
                pubkey: new PublicKey(target.programId),
            },
        ];
        signers = [caller];
        call = {
            to: target.programId,
            data: target.coder.instruction.encode("makeCall", {
                data: Buffer.from([0]),
            }),
        };
    });

    it("Test Router", async () => {
        const tx = await program.methods
            .route(transactionAccounts, call)
            .remainingAccounts(remainingAccounts)
            .signers(signers)
            .rpc();
        console.log("Your transaction signature", tx);
    });
});
