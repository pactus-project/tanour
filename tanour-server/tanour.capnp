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
  filename @8: Text;
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
  readStorage @0    ( filename: Text, offset: UInt32, length: UInt32     ) -> (value: Data);
  writeStorage @1   ( filename: Text, offset: UInt32, value: Data        ) -> ();
}
