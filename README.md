# embed_it
Include any directory as a struct and the entire tree will be produced into rust structures and traits

Imagine a project structure like that

- **assets/**
  - **one_txt/**
    - hello
    - world
  - hello.txt
  - one.txt
  - world.txt
- src
- Cargo.toml

You can use a macro to expand it into rust code:

```rust
use embed_it::Embed;
#[derive(Embed)]
#[embed(path = "$CARGO_MANIFEST_DIR/assets")]
pub struct Assets;

# fn main() {
    use embed_it::{Content, EntryPath, EmbeddedPath};
    assert_eq!(Assets.hello().content(), b"hello");
    assert_eq!(Assets.hello().path(), &EmbeddedPath::new("hello.txt", "hello.txt", "hello"));

    assert_eq!(Assets.one().content(), b"one");
    assert_eq!(Assets.one().path(), &EmbeddedPath::new("one.txt", "one.txt", "one"));

    assert_eq!(Assets.world().content(), b"world");
    assert_eq!(Assets.world().path(), &EmbeddedPath::new("world.txt", "world.txt", "world"));

    assert_eq!(Assets.one_txt().path(), &EmbeddedPath::new("one_txt", "one_txt", "one_txt"));

    assert_eq!(Assets.one_txt().hello().content(), b"hello");
    assert_eq!(Assets.one_txt().hello().path(), &EmbeddedPath::new("one_txt/hello", "hello", "hello"));

    assert_eq!(Assets.one_txt().world().content(), b"world");
    assert_eq!(Assets.one_txt().world().path(), &EmbeddedPath::new("one_txt/world", "world", "world"));
# }
```

## Fields

### embed
| field             | type             | multiple | required | default                | description                                                                                                                                                                                                                                                                                 |
|-------------------|------------------|----------|----------|------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `path`            | `String`         | false    | true     | -                      | The path to the directory with assets. It may contain [compile-time environment variables](https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates) (or user defined) in format `$CARGO_MANIFEST_DIR` or `${CARGO_MANIFEST_DIR}`   |
| `dir`             | `DirAttr`        | false    | false    | `DirAttr::default()`   | Change settings, how `Dir`-trait and its implementations will be generated. See more in [Dir Attr](#DirAttr) section                                                                                                                                                                        |
| `file`            | `FileAttr`       | false    | false    | `FileAttr::default()`  | Change settings, how  `File` -trait and its implementations will be generated. See more in [File Attr](#FileAttr) section                                                                                                                                                                   |
| `entry`           | `EntryAttr`      | false    | false    | `EntryAttr::default()` | Change settings, how  `Entry` -struct and its implementations will be generated. See more in [Entry Attr](#EntryAttr) section                                                                                                                                                               |
| `field`           | `Vec<FieldAttr>` | true     | false    | `vec![]`               | Add additional "fields" for dirs and files. See more in [Field Attr](#FieldAttr)                                                                                                                                                                                                            |
| `with_extensions` | `bool`           | false    | false    | `false`                | Use file extensions for method and struct name                                                                                                                                                                                                                                              |

### <a name="DirAttr"></a> DirAttr

| field                      | type            | multiple | required | default                                     | description                                                                                                                                       |
|----------------------------|-----------------|----------|----------|---------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------|
| `trait_name`               | `Option<Ident>` | false    | false    | `None`                                      | What trait name will be used for a directory                                                                                                      |
| `field_factory_trait_name` | `Option<Ident>` | false    | false    | `None`                                      | What trait name will be used for a directory field factory                                                                                        |
| `derive`                   | `Vec<DirTrait>` | true     | false    | `Path`, `Entries`, `Index`, `Meta`, `Debug` | What traits will be derived for every directory and what bounds will be set for a Dir trait. See also [EmbeddedTraits list](#EmbeddedTraits_list) |

### <a name="FileAttr"></a> FileAttr

| field                      | type            | multiple | required | default                                     | description                                                                                                                                      |
|----------------------------|-----------------|----------|----------|---------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------|
| `trait_name`               | `Option<Ident>` | false    | false    | `None`                                      | What trait name will be used for a directory                                                                                                     |
| `field_factory_trait_name` | `Option<Ident>` | false    | false    | `None`                                      | What trait name will be used for a directory field factory                                                                                       |
| `derive`                   | `Vec<DirTrait>` | true     | false    | `Path`, `Meta`, `Debug`, `Content`          | What traits will be derived for every directory and what bounds will be set for a Dir trait. See also [EmbeddedTraits list](#EmbeddedTraits_list)|

### <a name="EmbeddedTraits_list"></a> EmbeddedTraits list

| **name**    | **trait**               | **dir or file** | **method**                                                          | **purpose**                                                                                                                                                       |
|-------------|-------------------------|-----------------|---------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| **Path**    | [`crate::EntryPath`]    | any             |                                                                     | Provides full information about a path of an entry                                                                                                                |
| **Entries** | *\<auto generated\>*    | dir             | `fn entries(&self) -> &'static [Entry]`                             | Provides direct children of a dir                                                                                                                                 |
| **Index**   | *\<auto generated\>*    | dir             | `fn get(&self, path: &::std::path::Path) -> Option<&'static Entry>` | Provides fast access (`HashMap`) to all children (recursively). It constructs hash set on every level dir and might use some memory if there are a lot of entries |
| **Meta**    | [`crate::Meta`]         | any             |                                                                     | Provides metadata of an entry                                                                                                                                     |
| **Debug**   | [`std::fmt::Debug`]     | any             |                                                                     | Debugs structs                                                                                                                                                    |
| **Content** | [`crate::Content`]      | file            |                                                                     | Provides content of a file                                                                                                                                        |



### <a name="EntryAttr"></a> EntryAttr
| field                      | type            | multiple | required | default                                     | description                                                                                                                                      |
|----------------------------|-----------------|----------|----------|---------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------|
| `struct_name`              | `Option<Ident>` | false    | false    | `None`                                      | What struct name will be used for an entry                                                                                                       |

### <a name="FieldAttr"></a> FieldAttr

| field                      | type            | multiple | required | default                                     | description                                                                                                                                      |
|----------------------------|-----------------|----------|----------|---------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------|
| `name`                     | `Ident`         | false    | true     |                                             | The name of the metod, that will be used by the trait                                                                                            |
| `factory`                  | `syn::Path`     | false    | true     |                                             | The path to a factory, that will be used to create an instance of the field and to determine a field type                                        |
| `trait_name`               | `Option<Ident>` | false    | false    | "{name.to_pascal_case()}Field"              | The name of the field trait                                                                                                                      |
| `target`                   | `EntryKind`     | false    | false    | `file`                                      | Either `file` or `dir`                                                                                                                           |
| `regex`                    | `Option<String>`| false    | false    | `None`                                      | Regular expression to match a fs entry path. The trait will be implemented for a struct only if the regex matches                                |
| `pattern`                  | `Option<String>`| false    | false    | `None`                                      | Glob pattern to match a fs entry path. The trait will be implemented for a struct only if the pattern matches                                    |


## Additional fields

You can add any additional fields, which will be created in runtime (but only once) from a dir or a file. 
For each `field` defined in macros a special trait will be generated inside the module containing a root struct.

```rust
use std::str::from_utf8;
use embed_it::Embed;

#[derive(Embed)]
#[embed(
    path = "$CARGO_MANIFEST_DIR/assets",
    field(
        // it is a trait method name used to get an instance of a field.
        // you can use your own name for the trait with attribute `trait_name`.
        // By default it is `{name.to_pascal()}Field`.
        // In that case it will be `AsStrField`.
        name = "as_str", 
        
        // target is a `file` or a `dir`, `file` is by default and might be ommitted
        // for a `file` only files structures will implement the trait and for a dir vice versa
        target = "file",
        
        // factory is a path to the struct implementing either
        // a trait self::FileFieldFactory for target = "file"
        // or a trait self::DirFieldFactory for target = "dir"
        factory = AsStr,   
        
        // glob pattern
        pattern = "*.txt",
    ), 
    field(
        name = "children", 
        factory = crate::Children, 
        target = "dir", 
        regex = ".+_txt",
    ), 
    field(
        name = "root_children", 
        trait_name = "Root",
        factory = crate::Children, 
        target = "dir", 
        // this trait will be implemented only for root struct (`Assets`)
        regex = ""
    ), 
)]
pub struct Assets;


/// Our own field structure to interpret content as utf8 str
pub struct AsStr(&'static str);
impl FileFieldFactory for AsStr {
    type Field = Option<Self>;

    fn create<T: File + ?Sized>(data: &T) -> Self::Field {
        use embed_it::{ Content };
        from_utf8(data.content()).map(AsStr).ok()
    }
}

/// Our own field structure to store all children relative paths
pub struct Children(Vec<&'static str>);
impl DirFieldFactory for Children {
    type Field = Self;

    fn create<T: Dir + ?Sized>(data: &T) -> Self::Field {
        use embed_it::{ EntryPath };
        Children(data.entries().iter().map(|e| e.path().name()).collect())
    }
}

# fn main() {
use embed_it::{ Content };

// the first field `as_str`
use AsStrField;
assert_eq!(Assets.hello().content(), b"hello");
assert_eq!(Assets.one().content(), b"one");
assert_eq!(Assets.world().content(), b"world");

assert_eq!(Assets.hello().as_str().as_ref().unwrap().0, "hello");
assert_eq!(Assets.one().as_str().as_ref().unwrap().0, "one");
assert_eq!(Assets.world().as_str().as_ref().unwrap().0, "world");

// this is not compile due to `pattern` (`one_txt/hello` has no extension)
// Assets.one_txt().as_str()

// the second field `children`
use ChildrenField;
assert_eq!(Assets.one_txt().children().0, Vec::from(["hello", "world"]));

// the third field `root_children`
use Root;
assert_eq!(Assets.root_children().0, vec!["one_txt", "hello.txt", "one.txt", "world.txt"]);
# }

```

## More complex example

```rust
use embed_it::Embed;
#[derive(Embed)]
#[embed(
    path = "$CARGO_MANIFEST_DIR/assets",
    dir(
        // trait name for directories (default `Dir`)
        trait_name = AssetsDir, 
        
        // trait name for directory field's factories (default `DirFieldFactory`)
        field_factory_trait_name = AssetsDirFieldFactory, 
        
        // implement embed_it::EntryPath
        derive(Path), 
        
        // implement `Entries` trait, which stores all direct children into an array
        derive(Entries), 
        
        // implement `Index` trait, which stores (recursively) all children into a set
        derive(Index), 

        // implement `embed_it::Meta` trait, which provides metadata of the entry
        derive(Meta),
        
        // implement `std::fmt::Debug` for directory. It writes each child implementing debug
        derive(Debug),
    ),
    file(
        // trait name for files (default `File`)
        trait_name = AssetsFile, 
        
        // trait name for file field's factories (default `FileFieldFactory`)
        field_factory_trait_name = AssetsFileFieldFactory, 

        // implement embed_it::EntryPath
        derive(Path), 
        
        // implement `embed_it::Meta` trait, which provides metadata of the entry
        derive(Meta),
        
        // implement `embed_it::Content` trait, which provides content of the file as a byte array
        derive(Content),
        
        // implement `std::fmt::Debug` for a file. It writes Content len
        derive(Debug),
    ),
    // `Entry` - enum with `Dir(&'static dyn Dir)/File(&'static dyn File)` variants
    // `Entry` implements intersection of `Dir`'s and `File`'s traits
    entry(
        // struct name for `Entry` (default `Entry`).
        struct_name = AssetsEntry,
    ),
    // if true, macros will be use extension as a part of `StructName`s and `methos_name`s
    // e.g. hello.txt turns into HelloTxt/hello_txt() if with_extensions = true, and Hello/hello() if with_extensions = false
    // default is false
    with_extensions = true,
    field(
        // The name of the method of the trait
        name = as_str, 
        
        // The trait name, defaul `"{name.to_pascal()}Field"`
        trait_name = AssetsAsStrField, 
        
        // The factory to create an instance of the field
        factory = AsStr, 
        
        // The pattern to match entry's path. Default None
        pattern = "*.txt", 
        
        // The regex to match entry's path. Default None
        regex = ".+", 

        // Which entries will be processed with this trait (either `file` or `dir`). Default `file`
        target = "file"
    ),
    field(
        name = children, 
        trait_name = AssetsChildrenField, 
        factory = Children, 
        pattern = "?*", 
        regex = ".+", 
        target = "dir"
    ),
    field(
        name = root_children, 
        trait_name = AssetsRootChildrenField, 
        factory = Children, 
        // only for `Assets`
        regex = "", 
        target = "dir"
    ),
)]
pub struct Assets;

pub struct Children;

// The name of the factory as in the attribute `dir`
impl AssetsDirFieldFactory for Children {
    type Field = Vec<&'static str>;

    fn create<T: AssetsDir + ?Sized>(data: &T) -> Self::Field {
        use embed_it::EntryPath;
        data.entries().iter().map(|v| v.path().relative_path_str()).collect()
    }
}

pub struct AsStr;

// The name of the factory as in the attribute `file`
impl AssetsFileFieldFactory for AsStr {
    type Field = Option<&'static str>;

    fn create<T:AssetsFile + ?Sized>(data: &T) -> Self::Field {
        std::str::from_utf8(data.content()).ok()
    }
}

# fn main() {
    use embed_it::{Content, EntryPath, Meta};
    assert_eq!(Assets.hello_txt().as_str(), &Some("hello"));
    assert_eq!(Assets.one_txt_1().as_str(), &Some("one"));
    assert_eq!(Assets.world_txt().as_str(), &Some("world"));

    assert_eq!(Assets.one_txt().hello().content(), b"hello");
    assert_eq!(Assets.one_txt().world().content(), b"world");

    assert_eq!(Assets.one_txt().children(), &vec!["one_txt/hello", "one_txt/world"]);

    let entries: &'static [AssetsEntry] = Assets.entries();
    for entry in entries {
        // `Entry` implements intersection of `Dir`'s and `File`'s traits
        println!("relative_path: {:?}", entry.path().relative_path_str());
        println!("{:?}", entry.metadata());
        println!("{:#?}", entry);
    }

# }
```

## How does fs-entry's name turn into rust identifiers?
Each name will be processed and any unsuitable symbol will be replaced with `_`. This might cause a problem with a level uniqueness of identifiers, for example, all of the entry names below turn into `one_txt`.
- one+txt
- one-txt
- one_txt

The macros handles the problem and generates methods with a number suffix. In that case it would be 
- one+txt - `one_txt()`
- one-txt - `one_txt_1()`
- one_txt - `one_txt_2()`

Entries sorted unambiguous by entry kind (dir/file, dirs first) and then by path.

This works for struct names in the same way

- one+txt - `OneTxt`
- one-txt - `OneTxt1`
- one_txt - `OneTxt2`


## What code will be generated by macros

1. Macros generates all embedded trait definitions, like `Entries` and `Index` which depend on a context
2. Macros generates definitions for traits `Dir` and `File` where each is a compilation of previous step suitable traits
3. Macros generates enum for `Entry` (`Dir(&'static dyn Dir)`/`File(&'static dyn File)`).
4. Macros implements intersection of `Dir` and `File` traits for `Entry`
5. Macros generates traits for `FileFieldFactory` and `DirFieldFactory` with bounds to `File`/`Dir` traits for the argument of the method
6. Macros generates traits for each `field`
7. For any entry starting from the root
    * For any kind of an entry implements requested suitable embedded traits (like `Content`, `Path`, `Metadata`, `Entries`, `Index`, etc.)
    * For any kind of an entry implements traits for all suitable fields from the step 6
    * For a directory recursively generate code for an each child


**NOTICE:** All instances are static and the staticity is achieved 

* by `const` for any const things, like file content or file path
```rust
struct Hello;
#[automatically_derived]
impl ::embed_it::Content for Hello {
    fn content(&self) -> &'static [u8] {
        const VALUE: &[u8] = b"hello"; // in real world it will be `include_bytes!(...)`
        VALUE
    }
}
```

* by `static` `LazyLock` for non-const things, which can be created without a context
```rust
pub struct Assets;

pub trait Dir: Send + Sync + Index {}
pub trait File: Send + Sync + embed_it::Content {}

pub enum Entry {
    Dir(&'static dyn Dir),
    File(&'static dyn File),
}

trait Index {
    fn get(&self, path: &::std::path::Path) -> Option<&'static Entry>;
}

#[automatically_derived]
impl Index for Assets {
    fn get(&self, path: &::std::path::Path) -> Option<&'static Entry> {
        static VALUE: ::std::sync::LazyLock<
            ::std::collections::HashMap<
                &'static ::std::path::Path,
                Entry,
            >,
        > = ::std::sync::LazyLock::new(|| {
            let mut map = ::std::collections::HashMap::with_capacity(2usize);
            // inserts
            map
        });
        VALUE.get(path)
    }

}

```

* by `static` `OnceLock` for non-const things, wich requires a context (like additional `field`s)

```rust

// user-defined struct and implementation
pub struct AsStr;
impl FileFieldFactory for AsStr {
    type Field = Option<&'static str>;
    fn create<T: File + ?Sized>(data: &T) -> Self::Field {
        std::str::from_utf8(data.content())
    }
}

pub struct Assets;

// auto-generetad
pub trait Dir: Send + Sync {}
pub trait File: Send + Sync + ::embed_it::Content {}

pub struct Hello;
impl ::embed_it::Content for Hello {
    fn content(&self) -> &'static [u8] {
        // Some implementation
        unimplemented!();
    }
}

impl File for Hello {};

pub enum Entry {
    Dir(&'static dyn Dir),
    File(&'static dyn File),
}

pub trait FileFieldFactory {
    type Field;
    fn create<T: File + ?Sized>(data: &T) -> Self::Field;
}

pub trait AsStrField {
    fn as_str(
        &self,
    ) -> &'static <AsStr as FileFieldFactory>::Field;
}

#[automatically_derived]
impl AsStrField for Hello {
    fn as_str(
        &self,
    ) -> &'static <AsStr as FileFieldFactory>::Field {
        static VALUE: ::std::sync::OnceLock<
            <AsStr as FileFieldFactory>::Field,
        > = ::std::sync::OnceLock::new();
        VALUE.get_or_init(|| {
            <AsStr as FileFieldFactory>::create(self)
        })
    }

}

```