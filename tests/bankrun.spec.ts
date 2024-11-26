import { ProgramTestContext, startAnchor } from "solana-bankrun";
import ProgramIdl from "../target/idl/smart_vestiny.json";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";

describe("Vesting Smart Contract Tests", () => {
  let beneficiary: Keypair;
  let bankrunContext: ProgramTestContext;
  beforeEach(async () => {
    beneficiary = new anchor.web3.KeyPair();
    bankrunContext = await startAnchor(
      "",
      [{ name: "smart_vestiny", programId: new PublicKey(ProgramIdl.address) }],
      [
        {
          address: beneficiary.publicKey,
          info: {
            lamports: 1 * LAMPORTS_PER_SOL,
            data: Buffer.alloc(0),
            owner: SYSTEM_PROGRAM_ID,
            executable: false,
          },
        },
      ]
    );
  });
});
