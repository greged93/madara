import yargs from "yargs";
import {
  RpcProvider,
  Account,
  hash,
  json,
  CompiledContract,
  RawArgs,
  BigNumberish,
  CompiledSierraCasm,
} from "starknet";
import fs from "fs";
import { CAIRO_1_ACCOUNT_CONTRACT } from "../tests/constants";

async function _main() {
  const {
    declare,
    deploy,
    invoke,
    sierra,
    casm,
    calldata,
    address,
    entrypoint,
    rpcUrl,
  } = parseCli();
  const provider = new RpcProvider({ nodeUrl: rpcUrl, retries: 3 });
  const account = new Account(provider, CAIRO_1_ACCOUNT_CONTRACT, "0x1", "1");

  if (declare || deploy) {
    const compiledSierra: CompiledContract = json.parse(
      fs.readFileSync(sierra).toString("ascii")
    );

    let classHash;
    if (declare) {
      const compiledCasm: CompiledSierraCasm = json.parse(
        fs.readFileSync(casm).toString("ascii")
      );
      classHash = await declareContract(compiledSierra, compiledCasm, account);
      console.log("contract declared at", classHash);
    }

    if (!classHash) {
      classHash = hash.computeContractClassHash(compiledSierra);
    }

    if (deploy) {
      console.log(
        "contract deployed at",
        await deployContract(classHash, calldata, account)
      );
    }
  }

  if (invoke) {
    console.log("invoking contract", address, entrypoint, calldata);
    console.log(await invokeContract(address, calldata, entrypoint, account));
  }
}

async function declareContract(
  compiledSierra: CompiledContract,
  compiledCasm: CompiledSierraCasm,
  account: Account
): Promise<BigNumberish> {
  const nonce = await account.getNonce();

  let classHash;
  try {
    const declareResponse = await account.declare(
      {
        contract: compiledSierra,
        casm: compiledCasm,
      },
      { nonce: nonce, maxFee: 0xffffffffff }
    );
    await account.waitForTransaction(declareResponse.transaction_hash);
    classHash = declareResponse.class_hash;
  } catch (err: any) {
    if (err.toString().includes("51: Class already declared")) {
      console.log("Class already declared, continuing deployment");
      classHash = hash.computeContractClassHash(compiledSierra);
    } else {
      throw err;
    }
  }

  return classHash;
}

async function deployContract(
  classHash: BigNumberish,
  contructorCalldata: RawArgs,
  account: Account
) {
  const nonce = await account.getNonce();
  const deployResponse = await account.deploy(
    {
      classHash: classHash,
      constructorCalldata: contructorCalldata,
    },
    { nonce: nonce, maxFee: 0xffffffffff }
  );
  await account.waitForTransaction(deployResponse.transaction_hash);
  return deployResponse.contract_address[0];
}

async function invokeContract(
  address: string,
  calldata: RawArgs,
  entrypoint: string,
  account: Account
) {
  const nonce = await account.getNonce();
  const invokeResponse = await account.execute(
    {
      contractAddress: address,
      entrypoint: entrypoint,
      calldata: calldata,
    },
    undefined,
    { nonce: nonce, maxFee: 0xffffffffff }
  );
  await account.waitForTransaction(invokeResponse.transaction_hash);
  return invokeResponse;
}

function parseCli() {
  const argv = yargs
    .option("deploy", {
      description: "Deploy a contract",
      type: "boolean",
    })
    .option("declare", {
      description: "Declare a contract",
      type: "boolean",
    })
    .option("invoke", {
      description: "Invoke a contract",
      type: "boolean",
    })
    .option("casm", {
      description: "Path to the casm compiled contract to deploy",
      type: "string",
    })
    .option("sierra", {
      description: "Path to the sierra compiled contract to deploy",
      type: "string",
    })
    .option("calldata", {
      alias: "cd",
      description: "Calldata for the contract constructor",
      type: "string",
    })
    .option("address", {
      alias: "a",
      description: "Address of the contract to invoke",
      type: "string",
    })
    .option("entrypoint", {
      alias: "ep",
      description: "Entrypoint to invoke",
      type: "string",
    })
    .option("rpc-url", {
      alias: "r",
      description: "URL of the Madara RPC",
      type: "string",
    })
    .help()
    .alias("help", "h").argv;

  const deploy = argv["deploy"];
  const declare = argv["declare"];
  const invoke = argv["invoke"];

  const casm = argv["casm"];
  if (!casm && declare) {
    throw new Error("Please specify a path to the casm");
  }

  const sierra = argv["sierra"];
  if (!sierra && (deploy || declare)) {
    throw new Error("Please specify a path to the sierra");
  }

  const address = argv["address"];
  const entrypoint = argv["entrypoint"];

  if (!(address && entrypoint) && invoke) {
    throw new Error("Please specify a address and a entrypoint");
  }

  const calldata = argv["calldata"]?.split(",") || [];

  let rpcUrl = argv["rpc-url"];
  if (!rpcUrl) {
    rpcUrl = "http://localhost:9944";
  }

  return {
    deploy,
    declare,
    invoke,
    sierra,
    casm,
    calldata,
    address,
    entrypoint,
    rpcUrl,
  };
}

async function main() {
  const provider = new RpcProvider({
    nodeUrl: "http://localhost:9944",
    retries: 3,
  });
  const account = new Account(provider, CAIRO_1_ACCOUNT_CONTRACT, "0x1", "1");

  let path = "../cairo-contracts/build/cairo_1/dojo/dojo_examples-";
  const compiledSierraWorld: CompiledContract = json.parse(
    fs.readFileSync(path + "world.sierra.json").toString("ascii")
  );
  const compiledCasmWorld: CompiledSierraCasm = json.parse(
    fs.readFileSync(path + "world.casm.json").toString("ascii")
  );
  const compiledSierraExecutor: CompiledContract = json.parse(
    fs.readFileSync(path + "executor.sierra.json").toString("ascii")
  );
  const compiledCasmExecutor: CompiledSierraCasm = json.parse(
    fs.readFileSync(path + "executor.casm.json").toString("ascii")
  );
  const compiledSierraMoves: CompiledContract = json.parse(
    fs.readFileSync(path + "moves.sierra.json").toString("ascii")
  );
  const compiledCasmMoves: CompiledSierraCasm = json.parse(
    fs.readFileSync(path + "moves.casm.json").toString("ascii")
  );
  const compiledSierraPosition: CompiledContract = json.parse(
    fs.readFileSync(path + "position.sierra.json").toString("ascii")
  );
  const compiledCasmPosition: CompiledSierraCasm = json.parse(
    fs.readFileSync(path + "position.casm.json").toString("ascii")
  );

  const worldClassHash = await declareContract(
    compiledSierraWorld,
    compiledCasmWorld,
    account
  );
  console.log("declared world class hash", worldClassHash);
  const executorClassHash = await declareContract(
    compiledSierraExecutor,
    compiledCasmExecutor,
    account
  );
  console.log("declared executor class hash", executorClassHash);
  const movesClassHash = await declareContract(
    compiledSierraMoves,
    compiledCasmMoves,
    account
  );
  console.log("declared moves class hash", movesClassHash);
  const positionClassHash = await declareContract(
    compiledSierraPosition,
    compiledCasmPosition,
    account
  );
  console.log("declared position class hash", positionClassHash);

  const executorAddress = await deployContract(executorClassHash, [], account);
  console.log("deployed executor at", executorAddress);

  const worldAddress = await deployContract(
    worldClassHash,
    [executorAddress],
    account
  );
  console.log("deployed world at", worldAddress);

  const registerComponentResponse = await invokeContract(
    worldAddress,
    [movesClassHash],
    "0x0321fe13e89e139339b447ce87634f69b3f7765008f93bb01a7246b66aa07ac8",
    account
  );
  console.log("register component", registerComponentResponse);
}

main();
