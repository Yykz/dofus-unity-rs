# protocol decode

This crate use [dofus_protocol](../protocol/) to decodes dofus messages and print it.

## Online version

[Online wasm version](https://yykz.github.io/dofus-unity-rs/decode-wasm/)

## Usage
```
Usage: decode <PROTOCOL> <HEX|--stdin>

Arguments:
  <PROTOCOL>  Protocol you want to decode, either game or connection
  [HEX]       The hex stream you want to decode

Options:
  -s, --stdin  Read input hex stream from stdin instead of from arguments
  -h, --help   Print help
```

## Usage example

```
$ ./decode game 571a550a530a51747970652e616e6b616d612e636f6d2f636f6d2e616e6b616d612e646f6675732e7365727665722e67616d652e70726f746f636f6c2e636f6e6e656374696f6e2e51756575655374617475734576656e74
Event { Ok(QueueStatusEvent { position: 0, total: 0 }) }

$ ./decode g 5d0a5b08ffffffffffffffffff01124e0a4c747970652e616e6b616d612e636f6d2f636f6d2e616e6b616d612e646f6675732e7365727665722e67616d652e70726f746f636f6c2e62616b2e42616b417069546f6b656e52657175657374
Request { uid: -1, Ok(BakApiTokenRequest) }
```
