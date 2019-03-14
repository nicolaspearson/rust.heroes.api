# Rust Heroes API

An example REST API built using Rust and PostgreSQL.

## Dependencies

This project makes use of Rocket, Serde and Diesel.

## Getting Started

**1. Clone the application**

```bash
git clone https://github.com/nicolaspearson/rust.heroes.api.git
```

**2. Start the database**

```bash
docker-compose up
```

**3. Run the database migrations**

```bash
diesel migration run
```

**4. Build and run the app using cargo**

### Run the app in development mode:

```bash
cargo run
```

The app will start running at <http://localhost:8000>

### Run the app in release mode:

```bash
cargo build --release && cd target/release/
sudo ROCKET_ENV=prod ./hero-api
```

The app will start running at <http://localhost:8000>

## Endpoints

The following endpoints are available:

```
GET /heroes
```

```
POST /hero
```

```
PUT /hero/{heroId}
```

```
DELETE /hero/{heroId}
```

## Benchmarking

Run the following to benchmark request performance:

```
wrk -d1m http://localhost:8000/heroes
```
