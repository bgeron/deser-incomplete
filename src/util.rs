/// Make a FnOnce that doesn't take any arguments.
///
/// This actually doesn't change the value, but the returned value
/// can definitely only be called once.
pub(crate) fn make_fnonce<T>(f: impl FnOnce() -> T) -> impl FnOnce() -> T {
    f
}

/// Erase `&Result` to a dyn error.
pub(crate) fn erase_error_ref<T, E: std::error::Error>(
    result: &Result<T, E>,
) -> Option<&(dyn std::error::Error)> {
    result
        .as_ref()
        .err()
        .map(|x| -> &(dyn std::error::Error) { x })
}

/// Correspond to methods and arguments of [`serde::Deserializer`].
///
/// Not public interface in the foreseeable future.
#[derive(Clone, Copy, Debug)]
pub(crate) enum DeserializeKind {
    Any,
    Bool,
    I8,
    I16,
    I32,
    I64,
    I128,
    U8,
    U16,
    U32,
    U64,
    U128,
    F32,
    F64,
    Char,
    Str,
    String,
    Bytes,
    ByteBuf,
    Option,
    Unit,
    UnitStruct {
        name: &'static str,
    },
    NewtypeStruct {
        name: &'static str,
    },
    Seq,
    Tuple {
        len: usize,
    },
    TupleStruct {
        name: &'static str,
        len: usize,
    },
    Map,
    Struct {
        name: &'static str,
        fields: &'static [&'static str],
    },
    Enum {
        name: &'static str,
        variants: &'static [&'static str],
    },
    Identifier,
    IgnoredAny,
}
