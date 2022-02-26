import {
  Connection,
  Keypair,
  sendAndConfirmTransaction,
  SystemProgram,
  Transaction,
  TransactionInstruction,
} from '@solana/web3.js'
import { readFileSync } from "fs"
import lo from 'buffer-layout'
import BN from 'bn.js'
import * as path from 'path';

function readKeypairFromPath (path: string): Keypair {
  const data = JSON.parse(readFileSync(path, 'utf-8'))
  return Keypair.fromSecretKey(Buffer.from(data))
}

async function main () {
  const programKeypair = readKeypairFromPath(path.join(__dirname, '../localnet/program.json'))
  const aliceKeypair = readKeypairFromPath(path.join(__dirname, '../localnet/alice.json'))
  const bobKeypair = readKeypairFromPath(path.join(__dirname, '../localnet/bob.json'))
  const connection = new Connection('http://127.0.0.1:8899', 'confirmed')

  const data = Buffer.alloc(8)
  lo.ns64('value').encode(new BN('500000000'), data)

  const ix = new TransactionInstruction({
    keys: [
      { pubkey: aliceKeypair.publicKey, isSigner: true, isWritable: true },
      { pubkey: bobKeypair.publicKey, isSigner: false, isWritable: true },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    programId: programKeypair.publicKey,
    data
  })

  const res = await sendAndConfirmTransaction(connection, new Transaction().add(ix), [aliceKeypair])
  console.log('\nResponse:\n', res)
  
}

main()
  .then(() => process.exit(0))
  .catch((err) => console.error(err))