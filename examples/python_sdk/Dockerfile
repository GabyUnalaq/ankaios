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

# Setup python sdk
RUN mkdir /usr/local/lib/ank-sdk-python
RUN git clone https://github.com/GabyUnalaq/ank-sdk-python.git /usr/local/lib/ank-sdk-python
RUN python3 -m pip install /usr/local/lib/ank-sdk-python

# Prepare main script
RUN mkdir /usr/local/lib/ankaios
COPY examples/python_sdk /ankaios
ENV PYTHONPATH="${PYTHONPATH}:/usr/local/lib/ankaios"

# Specify entry point
ENTRYPOINT ["python3", "-u", "/ankaios/src/main.py"]
