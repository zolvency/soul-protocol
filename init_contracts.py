import subprocess

RELAYER = "GAK35OYQKEHPETRCH2JW64OYYJH6WMSBDVRG2SFZ4XJLQ4OHOM45GV75"
SOUL_CONTRACT = "CDRE2FJYMKGD5D6AT6QTICFPBPFDBXBLKBIDBV7EEEBHDPTOL6IESI2Q"
GITHUB_CONTRACT = "CCCFOJUS3ZEOSOAK3MLWBEBDCXRUYRZUCA5IMGISBVUP3ZBED2JNCQFB"
BINANCE_CONTRACT = "CAUK3AMZOMD33Q6D4VFEXI7QYUF36VHFOGW55DIOKFEPRWGLJCOD3ELW"

for contract in [GITHUB_CONTRACT, BINANCE_CONTRACT]:
    cmd = [
        "stellar", "contract", "invoke",
        "--id", contract,
        "--source", "relayer",
        "--network", "testnet",
        "--", "initialize",
        "--admin", RELAYER,
        "--registry", RELAYER,
        "--soul_contract", SOUL_CONTRACT,
        "--fee_token", RELAYER,
        "--access_control", RELAYER,
        "--treasury", RELAYER,
        "--mint_fee", "0"
    ]
    print(f"Initializing {contract}...")
    subprocess.run(cmd, check=True)
