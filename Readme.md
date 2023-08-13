# YH Rates Saver

# Configuration 

## .config.yaml 

Useful to setup when local development. Create a file with configuration to the root of project.

### Example 

```toml
MyServiceBusHost: "my-sb.test.svc.cluster.local:6421"
SeqUrl: "url=http://localhost:5341"
PgDb: "postgresql://sa:1234@localhost:5432/yh_test"
InstrumentsFilter: ["BTCUSDT", "ZRXUSDT", "BTCETH"]
```

## .env 

The app supports .ENV files

# Postgres ORM 

## Diesel ORM for Migration (TODO!)

https://github.com/diesel-rs/diesel
https://diesel.rs/guides/getting-started

### Install CLI

```bash
cargo install diesel_cli --no-default-features --features postgres
```

You should install postgres locally to use the CLI 

```bash
brew install postgresql
```

### Setup deisel 

You need to have `.env` file. Just add to the root of the project

```bash
echo DATABASE_URL=postgres://username:password@localhost/db_name > .env
```

Setup

```bash
diesel setup
```

Add migration 

```bash
diesel migration generate <name>
```

Run manually 

```bash
diesel migration run
```

# Development 

## Protoc 

The service use Protobuf files. Need to install protobuf compilter. 

### Compiler

```bash
brew install protobuf
```

Proto files located in the folder 
`./src/endpoints/grpc/proto`

It will be emit `*.rs` files to folder `./src/endpoints/grpc/`.
The name of the emited files is equal to the name of namespace of the packages. 

### How to test 

Install grpc cli tool 

```bash
brew install grpc
```

As the gRPC server that created for the service suppoted reflection then to test the service you could write command 

```bash
grpc_cli call localhost:50051 ratessaver.RatesSaverService.GetSavedRates "id: 'btcusdt'"
```

# Helpful 

## Release port 

```bash
lsof -i :8081 | awk '{if (NR!=1) print $2}' | xargs kill
lsof -i :8080 | awk 'NR>1 {print $2}' | xargs kill -9

(lsof -i :8080 && lsof -i :8081) | awk 'NR>1 {print $2}' | uniq | xargs kill -9

```