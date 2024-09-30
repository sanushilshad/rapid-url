
# RAPID URL
 
Short URL generation Service

## Tech Stack
| Type | Technologies |
|---|---|
| Server | Rust (Actix-web), Bash |
| Database | PostgreSQL |
| API Documention | OpenAPI Swagger |


## CUSTOM COMMAND FOR DEBUG:
### FOR MIGRATION:
```
cargo run --bin ondc-retail-b2b-buyer -- migrate
```

### FOR TOKEN GENERATION:
```
cargo run --bin rapid-url -- generate_token  sanushilshad
```

## CUSTOM COMMAND FOR RELEASE:
### FOR MIGRATION:

    cargo run --release --bin  ondc-retail-b2b-buyer -- migrate

    OR 

    ./target/release/ondc-retail-b2b-buyer migrate

### FOR TOKEN GENERATION:
```
cargo run --release --bin  rapid -- generate_token  sanushilshad

OR 

./target/release/rapid-url generate_token  sanushilshad
```

## SQLX OFFLINE MODE:

```
cargo sqlx prepare
```

## ENVIRON VARIABLE 
- Set the following environ variables in `env.sh`
- `env.sh`:
```

## DATABASE VARIABLES
export DATABASE__PASSWORD=""
export DATABASE__PORT=5000
export DATABASE__HOST=""
export DATABASE__NAME=""
export DATABASE__TEST_NAME=""
export DATABASE_URL="postgres://postgres:{{password}}@{{ip}}:{{port}}/{{database_name}}"
export DATABASE__USERNAME="postgres"
export DATABASE__ACQUIRE_TIMEOUT=5
export DATABASE__MAX_CONNECTIONS=2000
export DATABASE__MIN_CONNECTIONS=10
export OTEL_SERVICE_NAME="rapid-url"
export OTEL_EXPORTER_OTLP_TRACES_ENDPOINT="http://localhost:4317"


## SECRET VARIABLE
export SECRET__JWT__SECRET=""
export SECRET__JWT__EXPIRY=876600


## APPLICATION VARIABLE
export APPLICATION__NAME=""
export APPLICATION__PORT=8001
export APPLICATION__HOST=0.0.0.0
export APPLICATION__WORKERS=16

```


- In order to verify SQL queries at compile time, set the below config in `.env` file:
```
export DATABASE_URL="postgres://postgres:{password}@{host}:{port}/{db_name}"

```

## TO RUN THE SERVER:
- For running development server:
```
bash dev_run.sh
```
- For running production server:
```
bash release.sh
```
- For killing server:
```
bash kill.sh
```

- For restarting server:
```
bash restart.sh
```


## API DOCUMENTATION:
The API Docmentation can be found at `https://{{domain}}/docs/` after running the server.
