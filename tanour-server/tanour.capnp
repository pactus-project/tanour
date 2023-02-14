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
  address @4: Data;
  action: union{
    instantiate: group {
      code @5: Data;
      storageSize @6: UInt32;
      validUntil @7: UInt32;
      owner @8: Data;
    }
    process @9: Void;
    query @10: Void;
  }
  args @11: Data;
}

struct ResultData {
  gasLeft @0: UInt64;
  data @1: Data;
  contract @2: Data;
}

interface Executor {
  execute @0 (provider: Provider, transaction: Transaction) -> (resultData: ResultData);
}

interface Provider {
  exists @0         ( address: Data                                     ) -> (exist: Bool);
  account @1        ( address: Data                                     ) -> (account: Account);
}
