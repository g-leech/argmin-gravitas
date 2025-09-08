# Argmin-gravitas: a simulacrum standing in place of Gavin Leech

Hosted at [www.gleech.org](https://www.gleech.org)

# Commands

## Run local dev server

```bash
./scripts/dev-server.sh
```

## Update Gemfile.lock

```bash
./scripts/bundle-lock.sh
```

## Build site

```bash
./scripts/build.sh
```

# Github actions

Github actions are included for netlify and cloudflare deploys.  These use the build scripts to ensure version compatibility and reproducibility.

A Github action for checking pull requests is included.

See `.github/workflows`

## Build site

