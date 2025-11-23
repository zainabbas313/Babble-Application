# Babble [RUST Programming Language]


| Concept | Python | Rust | Notes / Differences |
|---------|--------|------|-------------------|
| **Dictionary** | `dict` | `HashMap<K, V>` | Keys must be `Hash + Eq`. Use enums or `serde_json::Value` for mixed types. |
| **Set** | `set` | `HashSet<T>` | No duplicates. Elements must implement `Hash + Eq`. |
| **Tuple** | `tuple` | `(T1, T2, ...)` | Fixed-length, heterogeneous, can destructure. |
| **Enum** | `enum` | `enum` | Rust enums are **algebraic**, can store data per variant. |
| **List** | `list` | `Vec<T>` | Dynamic array, homogeneous, grows/shrinks at runtime. |
| **String** | `str` / `str` objects | `String` / `&str` | `String` = owned, growable; `&str` = borrowed slice. |
| **Integer** | `int` | `i8, i16, i32, i64, i128, isize` | Signed integers. Choose size. |
| **Unsigned Integer** | - | `u8, u16, u32, u64, u128, usize` | Only positive numbers. `usize` depends on machine architecture. |
| **Float** | `float` | `f32, f64` | Default `f64`. |
| **Boolean** | `bool` | `bool` | `True/False` ↔ `true/false` |
| **None / Null** | `None` | `Option<T>` | Rust has `Option::Some(value)` and `Option::None`. |
| **Complex** | `complex` | `num::Complex<T>` | Requires `num` crate; not built-in. |
| **Range** | `range()` | `a..b` / `a..=b` | Exclusive (`..`) or inclusive (`..=`). |
| **Array (fixed-size)** | `list` / `array` | `[T; N]` | Fixed-size, stored on stack, elements must be same type. |
| **Slice** | list slice | `&[T]` | Borrowed view of an array or Vec. |
| **Queue** | `collections.deque` | `VecDeque<T>` | Double-ended queue. |
| **Stack** | `list.append()/pop()` | `Vec<T>` | Push/pop from the end. |
| **Optional / Maybe** | None / value | `Option<T>` | Enforces null safety. |
| **Result / Exception Handling** | `try/except` | `Result<T, E>` | Rust uses explicit error handling (`Ok(value)` / `Err(error)`). |
| **Iterator / Generator** | `iter()` / `yield` | `Iterator` trait / `for` loops | Lazy evaluation; combinators like `.map()`, `.filter()`. |
| **Frozen / Immutable Set** | `frozenset` | `BTreeSet<T>` or `HashSet<T>` with immutability | Can use `const` or `&` to enforce immutability. |
| **Bytes** | `bytes` | `Vec<u8>` / `&[u8]` | Binary data; strings are UTF-8 in Rust. |
| **NamedTuple / Struct** | `collections.namedtuple` | `struct` | Fixed fields, type-safe, can implement traits. |
| **Object / Class** | `class` | `struct` + `impl` | Methods in `impl` blocks; no inheritance (use traits). |
| **Interface / Abstract Class** | `ABC` | `trait` | Defines behavior that structs can implement. |

---

### 🔹 Notes

1. Rust is **strongly typed**, so most Python dynamic types (like dict with mixed types) require enums or traits.  
2. Rust’s **Option** and **Result** replace `None` and exceptions.  
3. Iterators in Rust are **lazy and chainable**, more powerful than Python in some cases.  
4. Rust separates **stack vs heap types** explicitly (`[T; N]` on stack, `Vec<T>` on heap).  
