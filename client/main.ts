import {
  Connection,
  Keypair,
  sendAndConfirmTransaction,
  SystemProgram,
  Transaction,
  TransactionInstruction,
  PublicKey
} from '@solana/web3.js'
import Wallet from '@project-serum/sol-wallet-adapter'
import { readFileSync } from "fs"
import lo from 'buffer-layout'
import BN from 'bn.js'
import * as path from 'path';

const connection = new Connection('http://localhost:8899')

declare global {
  interface Window {
    solana: any
  }
}

//////////////////// Sollet wallet functions ////////////////////
let solletWallet = new Wallet('https://www.sollet.io')
solletWallet.on("connect", (publicKey) => console.log("sollet connected", publicKey.toBase58()))

export async function connectSolletWallet() {
  return solletWallet.connect()
}

async function prepareTransaction(userPubkey: PublicKey): Promise<Transaction> {
  const bobPubkey = new PublicKey("9C8ARBpAqcmoDfqZTDtvB1JgZC7gjvcq48xRJoR7Wpeq")
  const programId = new PublicKey("Cf2FH5TEV6T511C4nJDyuyuaVc34vDA66rmmkwquyWeM")

  // encode 0.5 SOL as an input_data
  const data = Buffer.alloc(64)
  lo.ns64("value").encode(new BN("500000000"), data)

  const ix = new TransactionInstruction({
    keys: [
      { pubkey: userPubkey, isSigner: true, isWritable: true },
      { pubkey: bobPubkey, isSigner: false, isWritable: true },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    programId: programId,
    data: data,
  })
  let tx = new Transaction()
  tx.add(ix)
  tx.feePayer = userPubkey
  tx.recentBlockhash = (await connection.getRecentBlockhash()).blockhash

  return tx
}

async function broadcastSignedTransaction(signed) {
  let signature = await connection.sendRawTransaction(signed.serialize())
  console.log("Submitted transaction " + signature + ", awaiting confirmation")
  await connection.confirmTransaction(signature)
  console.log("Transaction " + signature + " confirmed")
}

export async function sendViaSollet() {
  console.log("sendViaSollet called")
  const tx = await prepareTransaction(solletWallet.publicKey)
  let signed = await solletWallet.signTransaction(tx)
  await broadcastSignedTransaction(signed)
}
/////////////////////////////////////////////////////////////////

function readKeypairFromPath (path: string): Keypair {
  const data = JSON.parse(readFileSync(path, 'utf-8'))
  return Keypair.fromSecretKey(Buffer.from(data))
}

async function main () {
  //////////////////////// Browser version ////////////////////////
  
  
  //////////////////////// CLI version ////////////////////////
  // const programKeypair = readKeypairFromPath(path.join(__dirname, '../localnet/program.json'))
  // const altruistKeypair = readKeypairFromPath(path.join(__dirname, '../localnet/altruist.json'))
  // const fundKeypair = readKeypairFromPath(path.join(__dirname, '../localnet/fund.json'))
  // const connection = new Connection('http://127.0.0.1:8899', 'confirmed')

  // console.log('\n\nprogramKeypair: ', programKeypair);
  // console.log('altruistKeypair: ', altruistKeypair);
  // console.log('fundKeypair: ', fundKeypair);

  // const data = Buffer.alloc(8)
  // lo.ns64('value').encode(new BN('500000000'), data)

  // const ix = new TransactionInstruction({
  //   keys: [
  //     { pubkey: altruistKeypair.publicKey, isSigner: true, isWritable: true },
  //     { pubkey: fundKeypair.publicKey, isSigner: false, isWritable: true },
  //     { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
  //   ],
  //   programId: programKeypair.publicKey,
  //   data
  // })

  // const res = await sendAndConfirmTransaction(connection, new Transaction().add(ix), [altruistKeypair])
  // console.log('\nTransaction hash: ', res)
  
}

main()
  .then(() => process.exit(0))
  .catch((err) => console.error(err))