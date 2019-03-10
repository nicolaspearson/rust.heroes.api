# Rust Heroes API

An example REST API built using Rust and PostgreSQL.

## Dependencies

This project makes use of Rocket and Diesel.

## Getting Started

**1. Clone the application**

```bash
git clone https://github.com/nicolaspearson/rust.heroes.api.git
```

**2. Start the database**
```bash
docker-compose up
```

**3. Run database migrations**
```bash
diesel migration run
```

**4. Build and run the app using cargo**

```bash
cargo build --release && cd target/release/
sudo ROCKET_ENV=prod ./hero-api
```

The app will start running at <http://localhost:80>.

Alternatively, you can run the app in development mode -

```bash
cargo run
```

## Endpoints

The following endpoints are available

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
