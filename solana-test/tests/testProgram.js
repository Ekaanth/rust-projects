const anchor = require("@project-serum/anchor");

const main = async () => {
  console.log("Starting test ...");

  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.TestProgram;
  const tx = await program.rpc.testFunction();

  console.log("Your transaction signature", tx);
};

const runMain = async () => {
  try {
    await main();
    process.exit(0);
  } catch (error) {
    console.log(error);
    process.exit(0);
  }
};

runMain();
