import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TestRay } from "../target/types/test_ray";
import { key } from "../key/config"
import { ComputeBudgetProgram, Connection, Keypair, PublicKey, sendAndConfirmTransaction, Transaction } from "@solana/web3.js";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { BN } from "bn.js";
import { ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddress, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { ASSOCIATED_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";

describe("test-ray", async () => {

  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.TestRay as Program<TestRay>;
  const user = Keypair.fromSecretKey(bs58.decode(key));
  const connection = new Connection("https://api.devnet.solana.com");

  const [pool] = PublicKey.findProgramAddressSync([Buffer.from("pool"), user.publicKey.toBytes()], program.programId)

  // const lp_token_kp = Keypair.generate()
  // const lp_token = lp_token_kp.publicKey;
  const lp_token = new PublicKey("C8PXxeuLjEhU8RcC7Hr1LFs2L7J7XcLwvKUVwYGVhryf");
  const lp_token_pool = await getAssociatedTokenAddress(lp_token, pool, true, TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID);
  const lp_token_Ata = await getAssociatedTokenAddress(lp_token, user.publicKey)
  const metadata = {
    name: 'Solana Gold',
    symbol: 'GOLDSOL',
    uri: 'https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/spl-token.json',
  };

  const amount_base: anchor.BN = new BN(10 ** 7);
  const lp_token_amount: anchor.BN = new BN(10 ** 9);

  const base_token = new PublicKey("G71rYtZFY1cumZVh6HRpV4Q1EbXyNfk6dYmmFUYHzoiW");
  const baseUserAta = await getAssociatedTokenAddress(base_token, user.publicKey)
  const base_token_pool = await getAssociatedTokenAddress(base_token, pool, true, TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID)

  const [sol_pool] = PublicKey.findProgramAddressSync([Buffer.from("sol_pool"), user.publicKey.toBytes(), pool.toBytes()], program.programId)
  const amount_sol: anchor.BN = new BN(10);

  // it("Is initialized!", async () => {
  //   try {
  //     const tx = new Transaction()
  //     tx.add(
  //       ComputeBudgetProgram.setComputeUnitLimit({ units: 200_000 }),
  //       ComputeBudgetProgram.setComputeUnitPrice({ microLamports: 1_200_000 }),
  //       await program.methods
  //         .initialize()
  //         .accounts({
  //           owner: user.publicKey,
  //           baseToken: base_token,
  //         })
  //         .instruction()
  //     )

  //     tx.feePayer = user.publicKey;
  //     tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash

  //     console.log(await connection.simulateTransaction(tx))
  //     const sig = await sendAndConfirmTransaction(connection, tx, [user], { skipPreflight: true })
  //     console.log("length", tx.serialize().length)
  //     console.log("Successfully initialized : ", `https://solscan.io/tx/${sig}?cluster=devnet`)
  //   } catch (error) {
  //     console.log("error", error)
  //   }
  // });


  // it("Is Mint Lptoken!", async () => {
  //   try {
  //     const tx = new Transaction()
  //     tx.add(
  //       ComputeBudgetProgram.setComputeUnitLimit({ units: 200_000 }),
  //       ComputeBudgetProgram.setComputeUnitPrice({ microLamports: 1_200_000 }),
  //       await program.methods
  //         .mintLptoken(metadata.name, metadata.symbol, metadata.uri, lp_token_amount)
  //         .accounts({
  //           pool: pool,
  //           owner: user.publicKey,
  //           lpToken: lp_token
  //         })
  //         .instruction()
  //     )

  //     tx.feePayer = user.publicKey;
  //     tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash

  //     console.log(await connection.simulateTransaction(tx))
  //     const sig = await sendAndConfirmTransaction(connection, tx, [user, lp_token_kp], { skipPreflight: true })
  //     console.log("length", tx.serialize().length)
  //     console.log("Successfully mint lp token : ", `https://solscan.io/tx/${sig}?cluster=devnet`)
  //   } catch (error) {
  //     console.log("error", error)
  //   }
  // })


  it("Is Add Liquidity!", async () => {
    try {
      const tx = new Transaction()
      tx.add(
        ComputeBudgetProgram.setComputeUnitLimit({ units: 200_000 }),
        ComputeBudgetProgram.setComputeUnitPrice({ microLamports: 1_200_000 }),
        await program.methods
          .addLiquidity(amount_base, amount_sol)
          .accounts({
            pool: pool,
            lpToken: lp_token,
            lpTokenPool: lp_token_pool,
            owner: user.publicKey,
            baseUserAta: baseUserAta,
            baseTokenPool: base_token_pool,
            solPool: sol_pool
          })
          .instruction()
      )

      tx.feePayer = user.publicKey;
      tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash

      console.log(await connection.simulateTransaction(tx))
      const sig = await sendAndConfirmTransaction(connection, tx, [user], { skipPreflight: true })
      console.log("length", tx.serialize().length)
      console.log("Successfully add liquidity : ", `https://solscan.io/tx/${sig}?cluster=devnet`)
    } catch (error) {
      console.log("error", error)
    }
  })


  // it("Is Remove Liquidity!", async () => {
  //   try {
  //     const tx = new Transaction()
  //     tx.add(
  //       ComputeBudgetProgram.setComputeUnitLimit({ units: 200_000 }),
  //       ComputeBudgetProgram.setComputeUnitPrice({ microLamports: 1_200_000 }),
  //       await program.methods
  //         .removeLiquidity(new BN(10 ** 7 + 10))
  //         .accounts({
  //           pool: pool,
  //           lpToken: lp_token,
  //           lpTokenPool: lp_token_pool,
  //           lpTokenUserAta: lp_token_Ata,
  //           owner: user.publicKey,
  //           baseUserAta: baseUserAta,
  //           baseTokenPool: base_token_pool,
  //           solPool: sol_pool,
  //         })
  //         .instruction()
  //     )

  //     tx.feePayer = user.publicKey;
  //     tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash

  //     console.log(await connection.simulateTransaction(tx))
  //     const sig = await sendAndConfirmTransaction(connection, tx, [user], { skipPreflight: true })
  //     console.log("length", tx.serialize().length)
  //     console.log("Successfully remove liquidity : ", `https://solscan.io/tx/${sig}?cluster=devnet`)
  //   } catch (error) {
  //     console.log("error", error)
  //   }
  // })
});
