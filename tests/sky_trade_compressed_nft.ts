import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SkyTradeCompressedNft } from "../target/types/sky_trade_compressed_nft";
import {
  AccountMeta,
  Connection,
  Keypair,
  PublicKey,
  Transaction,
  clusterApiUrl,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import {
  ConcurrentMerkleTreeAccount,
  SPL_ACCOUNT_COMPRESSION_PROGRAM_ID,
  SPL_NOOP_PROGRAM_ID,
  ValidDepthSizePair,
  createAllocTreeIx,
} from "@solana/spl-account-compression";
import {
  findTreeConfigPda,
  MPL_BUBBLEGUM_PROGRAM_ID,
} from "@metaplex-foundation/mpl-bubblegum";
import {
  Metaplex,
  keypairIdentity,
  CreateNftOutput,
  CreateCompressedNftOutput,
} from "@metaplex-foundation/js";

/* import {
  Metaplex,
  keypairIdentity,
  CreateNftOutput,
} from "@metaplex-foundation/js";
import { assert } from "chai"; */
//import { PROGRAM_ID as TOKEN_METADATA_PROGRAM_ID } from "@metaplex-foundation/mpl-token-metadata";
//import { extractAssetId, heliusApi } from "../utils/utils";

describe("sky_trade_compressed_nft", () => {
  // Configure the client to use the local cluster.
  //let provider = anchor.AnchorProvider.local("http://127.0.0.1:8899");
  const provider = anchor.AnchorProvider.env();
  const wallet = provider.wallet as anchor.Wallet;
  const program = anchor.workspace
    .SkyTradeCompressedNft as Program<SkyTradeCompressedNft>;
  const payer = wallet.payer;
  console.log("payer address: " + payer.publicKey.toBase58());

  //const connection = program.provider.connection;
  const connection = new Connection(clusterApiUrl("devnet"), "confirmed");

  const metaplex = Metaplex.make(connection).use(keypairIdentity(payer));

  // pda "tree creator", allows our program to update the tree
  let [pda, pdaBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [anchor.utils.bytes.utf8.encode("auth")],
    program.programId
  );
  console.log("Pda address: " + pda.toBase58());

  // keypair for tree
  const merkleTree = Keypair.generate();
  console.log("merkleTree address: " + merkleTree.publicKey.toBase58());

  const MPL_BUBBLEGUM_PROGRAM_ID_KEY = new anchor.web3.PublicKey(
    MPL_BUBBLEGUM_PROGRAM_ID
  );

  // tree authority
  const [treeAuthority] = PublicKey.findProgramAddressSync(
    [merkleTree.publicKey.toBuffer()],
    MPL_BUBBLEGUM_PROGRAM_ID_KEY
  );

  // new block
  /* const [bubblegumSigner] = PublicKey.findProgramAddressSync(
    [Buffer.from("collection_cpi", "utf8")],
    MPL_BUBBLEGUM_PROGRAM_ID_KEY
  );

  console.log("bubblegumSigner address: " + bubblegumSigner.toBase58());

  const maxDepthSizePair: ValidDepthSizePair = {
    maxDepth: 14,
    maxBufferSize: 64,
  };
  const canopyDepth = maxDepthSizePair.maxDepth - 5;

  const metadata = {
    uri: "https://arweave.net/h19GMcMz7RLDY7kAHGWeWolHTmO83mLLMNPzEkF32BQ",
    name: "NAME",
    symbol: "SYMBOL",
  };

  //let collectionNft: CreateNftOutput;
  let collectionNft: CreateCompressedNftOutput;
  //let assetId: PublicKey;
  //let assetId2: PublicKey;

  before(async () => {
    // Create collection nft
    collectionNft = await metaplex.nfts().create({
      uri: metadata.uri,
      name: metadata.name,
      sellerFeeBasisPoints: 0,
      isCollection: true,
    });

    // transfer collection nft metadata update authority to pda
    await metaplex.nfts().update({
      nftOrSft: collectionNft.nft,
      updateAuthority: wallet.payer,
      newUpdateAuthority: pda,
    });

    // instruction to create new account with required space for tree
    const allocTreeIx = await createAllocTreeIx(
      connection,
      merkleTree.publicKey,
      wallet.publicKey,
      maxDepthSizePair,
      canopyDepth
    );

    const tx = new Transaction().add(allocTreeIx);

    const txSignature = await sendAndConfirmTransaction(
      connection,
      tx,
      [wallet.payer, merkleTree],
      {
        commitment: "confirmed",
      }
    );
    //console.log(`https://explorer.solana.com/tx/${txSignature}?cluster=devnet`);
    console.log("Your transaction signature", txSignature);
  }); */
  //

  it("Is create tree!", async () => {
    let initParams = {
      maxDepth: 14,
      maxBufferSize: 64,
      public: false,
    };

    const tx = await program.methods
      .createTree(initParams)
      .accounts({
        payer: payer.publicKey,
        pda: pda,
        treeAuthority: treeAuthority,
        merkleTree: merkleTree.publicKey,
        logWrapper: SPL_NOOP_PROGRAM_ID,
        compressionProgram: SPL_ACCOUNT_COMPRESSION_PROGRAM_ID,
        bubblegumProgram: MPL_BUBBLEGUM_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([payer])
      .rpc();
    console.log("Your transaction signature", tx);

    //let result = await program.account.rider.fetch(rider);
    //console.log("rider: ", result);
  });
});
