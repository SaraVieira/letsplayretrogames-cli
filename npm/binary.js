const { Binary } = require("binary-install");
const os = require("os");

const windows = "x86_64-pc-windows-msvc";

const getPlatform = () => {
  const type = os.type();
  const arch = os.arch();

  if (type === "Windows_NT" && arch === "x64") {
    return windows;
  }
  if (type === "Linux" && arch === "x64") {
    return "x86_64-unknown-linux-musl";
  }
  if (type === "Linux" && arch === "arm64") {
    return "aarch64-unknown-linux-musl";
  }
  if (type === "Darwin" && (arch === "x64" || arch === "arm64")) {
    return "x86_64-apple-darwin";
  }

  throw new Error(`Unsupported platform: ${type} ${arch}`);
};

const getBinary = () => {
  const platform = getPlatform();
  const version = require("./package.json").version;
  const author = "saravieira";
  const repo = "letsplayretrogames-cli";
  const name = "letsplayretrogames";
  const url = `https://github.com/${author}/${repo}/releases/download/v${version}/${name}-v${version}-${platform}.tar.gz`;
  return new Binary(
    platform === windows ? "letsplayretrogames.exe" : "letsplayretrogames",
    url
  );
};

const install = () => {
  const binary = getBinary();
  binary.install();
};

const run = () => {
  const binary = getBinary();
  binary.run();
};

module.exports = {
  install,
  run,
};
