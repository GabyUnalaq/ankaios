FROM docker.io/alpine:3.18.4 as base

# Install and setup python3
RUN apk update && apk add --update-cache \
    python3 \
    && rm -rf /var/cache/apk/* \
    && python3 -m ensurepip

# Install dev tools
RUN apk update && apk add --update-cache \
    git \
    && rm -rf /var/cache/apk/*

# Install Ankaios Python SDK
RUN python3 -m pip install \
    -i https://test.pypi.org/simple/ \
    --extra-index-url https://pypi.org/simple/ \
    ankaios-sdk-test

# Prepare main script
RUN mkdir /usr/local/lib/ankaios
COPY examples/python_sdk /ankaios
ENV PYTHONPATH="${PYTHONPATH}:/usr/local/lib/ankaios"

# Specify entry point
ENTRYPOINT ["python3", "-u", "/ankaios/src/main.py"]
