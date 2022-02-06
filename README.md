# Backend API

This is a Proof of concept to create an internal API ready to be used from Backstage.

It's following Hexagonal Architecture and made with Rust (edition 2021).

**Pre-requisites**: We use a gitops approach, so that means, that all the changes applied to a cluster, will be through a Pull request and once approved, synchronized by Argo-Cd.

**Features**:
- Endpoint to list AWS instances types: The list is updated periodically using a Datasource port, that can be a File Datasource or a URL. Here, tokio has been used to create a new thread.
- Endpoint to list/create a CRD from kubernetes (Nodegroup) and secrets: Here, I've used the kube-code library and I've created a reflector. The reflector, basically, keeps an internal storage (like a cache) that is automatically synchronized by the library. All the internal calls to retrieve secrets or the CRD, will go directly to the internal store.  The reflector is really usefull in this case because you don't need to manage the received events.
- Creation of Pull Requests: The API follows the GitOps approach, so in order to create a new Nodegroup, we need to create a pull request.
- Creation of a Sealed Secret
- Template service: Used to crate the manifests files required for the pull requests.

## Endpoints
- List and create nodegroups: [http://localhost:8000/api/nodegroups](http://localhost:8000/api/nodegroups)
- List your secrets: [http://localhost:8000/api/secrets](http://localhost:8000/api/secrets)
- List all the AWS instance types: [http://localhost:8000/api/instance_types](http://localhost:8000/api/instance_types)
- Health checks:
    - [http://localhost:8000/api/liveness](http://localhost:8000/api/liveness)
    - [http://localhost:8000/api/readiness](http://localhost:8000/api/readiness)

## Internal services
- Template service to create YAML dynamically.
- Version Control service to clone, create pull requests and auto-commits.
- Self updater for instances types.
## Architecture


For unit testing it uses [mock-it] for building mocks. As Rust doesn't have a JIT like the JVM, mocking requires a bit more boilerplate. There are [plenty of options](https://asomers.github.io/mock_shootout/), but the interfaces that had to be mocked were pretty advanced, and `mock-it` provided the basic functionality to build the mocks.

Missing tests yest. This was just a PoC.

## Development

### Rust version

Travis uses the 1.52.1 version, so, be sure to use the same Rust version locally. To upgrade Rust, you can run the following commands:

```
make rust-toolchain-setup
```

### Environment setup
We use the .env file to setup the environment. 


For now, the environment variables are:

```
APP_NAME=cre-backend-api
APP_VERSION=0.0.1
RUST_LOG=info
NAMESPACE=[CLUSTER_NAMESPACE]

#Instance types
INSTANCE_TYPES_FILE_SOURCE=./pricing-list.json

#GITOPS config
GITPOS_ORGANIZATION=[ORGANIZATION]
GITPOS_REPO=unicron
GITPOS_DESTINATION_FOLDER=/tmp/gitops
GITPOS_BRANCH=dev
```

### Laptop setup

To install Rust you can run:

```
make rust-toolchain-setup
```

If anything fails, please follow the [instructions here](https://www.rust-lang.org/tools/install).

To run the integration tests you will need to have [kind](https://kind.sigs.k8s.io/) installed in your computer.

Finally you'll need to update the `.env` file with the values that fit with your development needs (see the *Environment setup* section).

### Running all the tasks

The following command will:
- format the code 
- run static analysis
- build a binary
- run the unit tests

```bash
make
```

### Build the technical documentation


```bash
make doc
```

### Running the application locally

Running the server in your laptop requires to setup different things, please follow the previous instructions to setup an `.env` file and download the `pricing-list.json` file.

```bash
make run
```

### Running the application using Docker

```bash
make docker-run
```

you can cleanup the binary from the root folder with:

```bash
make docker-clean
```
