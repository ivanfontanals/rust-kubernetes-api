ARG PYTHON_VERSION=3.8.11
FROM python:${PYTHON_VERSION}-slim-buster

ARG BINARY_NAME
ENV BINARY_NAME ${BINARY_NAME}

RUN apt-get update && \
    apt-get install -y \
        curl \
        git && \
    rm -rf /var/lib/apt/lists/*

# install Kustomize
ENV KUSTOMIZE_VERSION 3.8.2
ADD https://github.com/kubernetes-sigs/kustomize/releases/download/kustomize%2Fv${KUSTOMIZE_VERSION}/kustomize_v${KUSTOMIZE_VERSION}_linux_amd64.tar.gz /tmp/kustomize.tar.gz
RUN mkdir /tmp/kustomize && \
    tar xvzf /tmp/kustomize.tar.gz -C /tmp/kustomize && \
    mv /tmp/kustomize/kustomize /usr/local/bin/kustomize && \
    echo "==>" &&  /usr/local/bin/kustomize version && \
    echo 'dbbb67c23605d57b9290db0fc675607728959fec  /usr/local/bin/kustomize' | sha1sum -c -

# install mlflow cli
ENV MLFLOW_VERSION 1.19.0
RUN pip install mlflow==${MLFLOW_VERSION}

WORKDIR /opt

COPY ${BINARY_NAME} ./app
RUN chmod 755 ./app && \
    touch .env

