# Quick!

A rust library meant for maximum efficiency and the best time management in regards to programming.

# Guide

How do you use this library?

## Declarations

### Enum

```rs
qenum![[DeriveMacro1, ...] /pub EnumName; EnumField1, EnumField2, ...];
//     ^^^^^^^^^^^^^^^^^^^ ^^^^
//     |                   |
//     optional            optional
```

### Variable

```rs
qvar!(VariableName[Type], Value);
//                ^^^^^^
//                |
//                optional
```

### Struct

```rs
qstruct!([DeriveMacro1, ...] /pub StructName; StructField1:Type1, ...)
//       ^^^^^^^^^^^^^^^^^^^ ^^^^
//       |                   |
//       optional            optional
```
