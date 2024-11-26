import anchor, { Program } from "@coral-xyz/anchor";
import { BanksClient, ProgramTestContext, startAnchor } from "solana-bankrun";
import ProgramIdl from "../target/idl/smart_vestiny.json";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";
import { BankrunProvider } from "anchor-bankrun";
import { SmartVestiny } from "../target/types/smart_vestiny";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";

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
  let vesting_account: PublicKey;
  let treasury_token_account: PublicKey;
  let employee_account: PublicKey;

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
    employer = provider.wallet.payer;

    banksclient = bankrunContext.banksClient;

    beneficiaryProvider = new BankrunProvider(bankrunContext);

    beneficiaryProvider.wallet = new NodeWallet(beneficiary);
    programForBeneficiary = new Program<SmartVestiny>(
      ProgramIdl as SmartVestiny,
      beneficiaryProvider
    );

    [vesting_account] = PublicKey.findProgramAddressSync(
      [Buffer.from(companyName), employer.publicKey.toBuffer()],
      program.programId
    );

    [treasury_token_account] = PublicKey.findProgramAddressSync(
      [Buffer.from("treasury_token_account"), Buffer.from(companyName)],
      program.programId
    );

    [employee_account] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("employee_account"),
        beneficiary.publicKey.toBuffer(),
        vesting_account.toBuffer(),
      ],
      program.programId
    );
  });
});
