import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BanksClient, ProgramTestContext, startAnchor } from "solana-bankrun";
import ProgramIdl from "../target/idl/smart_vestiny.json";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";
import { BankrunProvider } from "anchor-bankrun";
import { SmartVestiny } from "../target/types/smart_vestiny";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { createMint } from "spl-token-bankrun";
import { expect } from "chai";

describe("Vesting Smart Contract Tests", () => {
  const companyName = "test-company-name";
  let beneficiary: Keypair;
  let bankrunContext: ProgramTestContext;
  let provider: BankrunProvider;
  let program: Program<SmartVestiny>;
  let banksclient: BanksClient;
  let employer: Keypair;
  let beneficiaryProvider: BankrunProvider;
  let programForBeneficiary: Program<SmartVestiny>;
  let vestingAccount: PublicKey;
  let treasuryTokenAccount: PublicKey;
  let employeeAccount: PublicKey;
  let mint: PublicKey;

  beforeEach(async () => {
    beneficiary = new anchor.web3.Keypair();
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

    provider = new BankrunProvider(bankrunContext);

    anchor.setProvider(provider);
    program = new Program<SmartVestiny>(ProgramIdl as SmartVestiny, provider);
    banksclient = bankrunContext.banksClient;
    employer = provider.wallet.payer;

    // @ts-ignore
    mint = await createMint(banksclient, employer, employer.publicKey, null, 2);

    beneficiaryProvider = new BankrunProvider(bankrunContext);

    beneficiaryProvider.wallet = new NodeWallet(beneficiary);
    programForBeneficiary = new Program<SmartVestiny>(
      ProgramIdl as SmartVestiny,
      beneficiaryProvider
    );

    [vestingAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from(companyName), employer.publicKey.toBuffer()],
      program.programId
    );

    [treasuryTokenAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from("treasury_token_account"), Buffer.from(companyName)],
      program.programId
    );

    [employeeAccount] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("employee_account"),
        beneficiary.publicKey.toBuffer(),
        vestingAccount.toBuffer(),
      ],
      program.programId
    );
  });

  it("It creates the vesting account", async () => {
    const tx = await program.methods
      .createVestingAccount(companyName)
      .accounts({
        owner: employer.publicKey,
        mint,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc({ commitment: "confirmed" });

    console.log("Create Vesting Account Tx::", tx);

    const vestingAccountData = await program.account.vestingAccount.fetch(
      vestingAccount,
      "confirmed"
    );

    console.log("Created Vesting Account::", vestingAccountData);
  });

});
