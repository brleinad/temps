# Temps

## Backend

### Dev setup

1. Install cargo-watch
```bash
cargo install cargo-watch
```

2. Set the environment variable `OPEN_WEATHER_MAP_API_KEY` from https://openweathermap.org/

3. Run
```bash
cargo-watch -x 'run --bin backend'
```

### Deploying 

Install [flyctl](https://fly.io/docs/hands-on/installing/).
[Sign up](https://fly.io/docs/hands-on/sign-up/) or [Sign in](https://fly.io/docs/hands-on/sign-in/) to fly.io

Run:
```
cd backend
fly deploy --build-arg  OPEN_WEATHER_MAP_API_KEY="<your key>"
```


## Frontend

See the frontend [README](./frontend/README.md)

