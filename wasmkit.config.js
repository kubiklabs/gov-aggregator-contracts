const gaia_localnet_accounts = [
  {
    name: 'account_0',
    address: 'cosmos10h9stc5v6ntgeygf5xf945njqq5h32r53uquvw',
    mnemonic: 'veteran try aware erosion drink dance decade comic dawn museum release episode original list ability owner size tuition surface ceiling depth seminar capable only'
  },
];

const neutron_testnet_accounts = [
  {
    name: 'account_0',
    address: 'neutron1jtdje5vq42sknl22r4wu9sahryu5wcrdqsccjh',
    mnemonic: 'category fine rapid trumpet dune early wish under nothing dance property wreck'
  },
];

const neutron_localnet_accounts = [
  {
    name: 'account_0',
    address: 'neutron1m9l358xunhhwds0568za49mzhvuxx9ux8xafx2',
    mnemonic: 'banner spread envelope side kite person disagree path silver will brother under couch edit food venture squirrel civil budget number acquire point work mass'
  },
  {
    name: 'account_1',
    address: 'neutron10h9stc5v6ntgeygf5xf945njqq5h32r54rf7kf',
    mnemonic: 'veteran try aware erosion drink dance decade comic dawn museum release episode original list ability owner size tuition surface ceiling depth seminar capable only'
  },
  {
    name: 'account_2',
    address: 'neutron14xcrdjwwxtf9zr7dvaa97wy056se6r5erln9pf',
    mnemonic: 'obscure canal because tomorrow tribe sibling describe satoshi kiwi upgrade bless empty math trend erosion oblige donate label birth chronic hazard ensure wreck shine'
  }
];

const neutron_mainnet_accounts = [
];

const networks = {
  neutron_localnet: {
    endpoint: 'http://localhost:26657/',
    chainId: 'test-1',
    accounts: neutron_localnet_accounts,
    fees: {
      upload: {
        amount: [{ amount: "750000", denom: "untrn" }],
        gas: "3000000",
      },
      init: {
        amount: [{ amount: "250000", denom: "untrn" }],
        gas: "1000000",
      },
      exec: {
        amount: [{ amount: "250000", denom: "untrn" }],
        gas: "1000000",
      }
    },
  },
  gaia_localnet: {
    endpoint: 'http://localhost:16657/',
    chainId: 'test-2',
    accounts: gaia_localnet_accounts,
    fees: {
      upload: {
        amount: [{ amount: "750000", denom: "uatom" }],
        gas: "3000000",
      },
      init: {
        amount: [{ amount: "250000", denom: "uatom" }],
        gas: "1000000",
      },
      exec: {
        amount: [{ amount: "250000", denom: "uatom" }],
        gas: "1000000",
      }
    },
  },
  neutron_testnet: {
    endpoint: 'https://neutron-testnet-rpc.polkachu.com/',
    chainId: 'pion-1',
    accounts: neutron_testnet_accounts,
    fees: {
      upload: {
        amount: [{ amount: "750000", denom: "untrn" }],
        gas: "3000000",
      },
      init: {
        amount: [{ amount: "250000", denom: "untrn" }],
        gas: "1000000",
      },
      exec: {
        amount: [{ amount: "250000", denom: "untrn" }],
        gas: "1000000",
      }
    },
  },
  neutron_mainnet: {
    endpoint: 'https://rpc-kralum.neutron-1.neutron.org',
    chainId: 'neutron-1',
    accounts: neutron_mainnet_accounts,
    fees: {
      upload: {
        amount: [{ amount: "750000", denom: "untrn" }],
        gas: "3000000",
      },
      init: {
        amount: [{ amount: "250000", denom: "untrn" }],
        gas: "1000000",
      },
      exec: {
        amount: [{ amount: "250000", denom: "untrn" }],
        gas: "1000000",
      }
    },
  }
};

module.exports = {
  networks: {
    default: networks.neutron_localnet,
    testnet: networks.neutron_testnet,
    localnet: networks.neutron_localnet,
    localgaia: networks.gaia_localnet,
    mainnet: networks.neutron_mainnet,
  },
  localnetworks: {
    neutron: {
      docker_image: "uditgulati0/neutron-node",
      rpc_port: 26657,
      rest_port: 1317,
      flags: ["RUN_BACKGROUND=0"],
    },
    osmosis: {
      docker_image: "uditgulati0/osmosis-node",
      rpc_port: 26657,
      rest_port: 1317,
      flags: [],
      docker_command: "/osmosis/setup.sh",
    },
  },
  mocha: {
    timeout: 60000
  },
  rust: {
    version: "1.68.2",
  },
  commands: {
    compile: "RUSTFLAGS='-C link-arg=-s' cargo build --release --target wasm32-unknown-unknown",
    schema: "cargo run --example schema",
  }
};