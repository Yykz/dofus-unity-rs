# proto-gen

This crate generates [.proto](../protos) files for the Dofus Unity protocol based on the `dump.cs` file from [Il2CppDumper](https://github.com/Perfare/Il2CppDumper).

It should also work with minor adjustments for other Unity games using the IL2CPP backend that utilize protobuf.

## Usage
```
Usage: proto-gen [OPTIONS] <DUMP_FILE_PATH> <NAMESPACE_FILTER>

Arguments:
  <DUMP_FILE_PATH>    Path to dump.cs file
  <NAMESPACE_FILTER>  Regex to match namespace(s) to filter

Options:
  -o, --output <OUTPUT>  Output directory for .proto files [default: ./protos/]
  -h, --help             Print help
```


### Example Usage

```./proto-gen dump.cs "^Com\.Ankama\.Dofus\.Server\.(\w+)\.Protocol"```



## Known Limitations
<details>
<summary><b>Not all protobuf <a href="https://protobuf.dev/programming-guides/proto3/#scalar">scalar value types</a> are retrieved</b></summary>

Multiple protobuf types are coerced into a single C# type, leading to a loss of type-specific information. For example:

`int32`, `sint32`, and `sfixed32` are all coerced into the `int` type in C#.
</details>

<details>
<summary><b>Incorrectly retrieved composite types due to ambiguous type references</b></summary>
The code in `dump.cs` contains ambiguities or missing information regarding type namespaces.

For example:

```csharp
// Namespace: Com.Common
public class Foo { /* ... */ }

// Namespace: Com.Baz
public class Foo { /* ... */ }

// Namespace: Com.Baz
public class Bar {
    private Foo foo; // Which Foo?
}
```
By default, `proto-gen` prioritizes types in the same namespace, then external well-known types, and finally types with common hierarchical namespaces, but this might not be exact.
</details>
