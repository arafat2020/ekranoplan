# Test fixtures

These PEMs are test fixtures for the loopback TLS tests. They are not a production certificate or key.

`server.crt` and `server.key` are a self-signed pair. `wrong.crt` is a different self-signed certificate. The client pins one of them. The wrong private key is not in this directory.

Generated with OpenSSL:

```text
openssl req -x509 -newkey rsa:2048 -keyout server.key -out server.crt -days 3650 -nodes -subj "/CN=localhost"
openssl req -x509 -newkey rsa:2048 -keyout /tmp/ekranoplan-wrong.key -out wrong.crt -days 3650 -nodes -subj "/CN=wrong"
```

Each file starts with a line that says it is a test fixture.
