import {
  Keypair,
  Connection,
  PublicKey,
  Transaction,
  TransactionInstruction,
  sendAndConfirmTransaction,
  SystemProgram,
  AccountInfo
} from "@solana/web3.js"
import { serialize, deserialize, deserializeUnchecked } from 'borsh';
import { Buffer } from 'buffer';
import { readFileSync } from "fs"
import lo from "buffer-layout"
import BN from "bn.js"

class Assignable {
  constructor(properties) {
      Object.keys(properties).map((key) => {
          return (this[key] = properties[key]);
      });
  }
}

export class AccoundData extends Assignable { }

const dataSchema = new Map([
  [
      AccoundData,
      {
          kind: "struct",
          fields: [
              ["initialized", "u8"],
              ["tree_length", "u32"],
              ["t", "u8"],
              ["t1", "u8"],
              ["t2", "u8"],
              ["t3", "u8"]
          ]
      }
  ]
]);

async function getAccountData(connection: Connection, account: PublicKey): Promise<AccoundData> {
  let nameAccount: any = await connection.getAccountInfo(
      account,
      'processed'
  );

  console.log('\nnameAccount\n', nameAccount, '\n\n');
  return deserializeUnchecked(dataSchema, AccoundData, nameAccount.data)
}




function readKeypairFromPath(path: string): Keypair {
  const data = JSON.parse(readFileSync(path, "utf-8"))
  return Keypair.fromSecretKey(Buffer.from(data))
}

async function main() {
  const programKeypair = readKeypairFromPath(__dirname + "/../localnet/program.json")
  const aliceKeypair = readKeypairFromPath(__dirname + "/../localnet/alice.json")
  const idKeypair = readKeypairFromPath("/home/vzhukov/.config/solana/id.json")
  const bobKeypair = readKeypairFromPath(__dirname + "/../localnet/bob.json")
  const connection = new Connection("http://localhost:8899", "confirmed")
  
  console.log('\n\nALICE');
  console.log(aliceKeypair.publicKey.toBase58());
  console.log(idKeypair.publicKey.toBase58());
  console.log('\n');
  

  // encode 0.5 SOL as an input_data
  const data = Buffer.alloc(8)
  lo.ns64("value").encode(new BN("500000000"), data)

  const ix = new TransactionInstruction({
    keys: [
      { pubkey: aliceKeypair.publicKey, isSigner: true, isWritable: true },
      { pubkey: bobKeypair.publicKey, isSigner: false, isWritable: true },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    programId: programKeypair.publicKey,
    data: data,
  })
  const res = await sendAndConfirmTransaction(connection, new Transaction().add(ix), [aliceKeypair])
  console.log(res)

  const programPublicKey = new PublicKey(programKeypair.publicKey.toString())
  const programAccount = await getAccountData(connection, programPublicKey)

  console.log('Program Account\n', programAccount, '\n\n');
}

main()
  .then(() => process.exit(0))
  .catch((err) => console.error(err))