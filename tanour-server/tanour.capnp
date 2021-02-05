@0xd767186f03554834;

struct Account {
  sequence @0: UInt64;
  balance @1: UInt64;
  code @2: Data;
}

struct Transaction {
  sender @0: Data;
  value @1: UInt64;
  gas @2: UInt64;
  gasPrice @3: UInt64;
  action: union{
    create: group {
      code @4: Data;
      salt @5: Data;
    }
    call: group {
      address @6: Data;
    }
  }
  args @7: Data;
}

struct LogEntry {
  address @0: Data;
  topics @1: List(Data);
  data @2: List(Int8);
}

struct ResultData {
  gasLeft @0: UInt64;
  data @1: Data;
  contract @2: Data;
  logs @3: List(LogEntry);
}

interface Executor {
  execute @0 (provider: Provider, transaction: Transaction) -> (resultData: ResultData);
}

interface Provider {
  exists @0         ( address: Data                                     ) -> (exist: Bool);
  account @1        ( address: Data                                     ) -> (account: Account);
  updateAccount @2  ( address: Data, balance: UInt64, sequence: UInt64  ) -> ();
  createContract @3 ( address: Data, code: Data                         ) -> ();
  getStorage @4     ( address: Data, key: Data                          ) -> (storage: Data);
  setStorage @5     ( address: Data, key: Data, value: Data             ) -> ();
  timestamp @6      (                                                   ) -> (timestamp: UInt64);
  blockNumber @7    (                                                   ) -> (number: UInt64);
  blockHash @8      ( blockNo: UInt64                                   ) -> (hash: Data);
  gasLimit @9       (                                                   ) -> (gasLimit: UInt64);
}
