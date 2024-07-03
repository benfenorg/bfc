# Use Docker to Run zk prover

## Prerequisites

 * [Install Docker](https://docs.docker.com/get-docker/) 
 * [Install Docker Compose](https://docs.docker.com/compose/install/)
 * [Download the Groth16 proving key zkey file](#download-the-groth16-proving-key-zkey-file)
 * Download the zk prover [docker-compose.yaml](https://github.com/hellokittyboy-code/obc/blob/develop_v1.2.0/docker/zk-prover/docker-compose.yaml) file.

## Download the Groth16 proving key zkey file
* if your machine does not have [git-lfs](https://git-lfs.com/) installed, you must install it first
* download zkey file
```shell
sudo mkdir -p /opt/software/
cd /opt/software/
sudo wget -O - https://raw.githubusercontent.com/sui-foundation/zklogin-ceremony-contributions/main/download-main-zkey.sh | bash
```

## Start your zk prover

Run the following command to start the zk prover in Docker:

```shell
docker compose up -d
```

**Important:** The commands in this document assume you use Docker Compose V2. The `docker compose` command uses a dash (`docker-compose`) in Docker Compose V1. If you use Docker Compose V1, replace the space in each `docker compose` command with a dash (`docker-compose`). For more information, see [Docker Compose V2](https://docs.docker.com/compose/#compose-v2-and-the-new-docker-compose-command).

### Stop the zk prover

Run the following command to stop the zk prover when you finish using it:
```shell
docker compose stop
```

