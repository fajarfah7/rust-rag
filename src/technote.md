# this is technote
this file is used for me to write important syntax or rule in Rust<br>
for now, actually i am still learning for this programming language which have roller coaster learning curve

# pattern one instance have lifetime dependecy to another instance
lifetime will take shortest age as default lifetime.

for example:
if age of X is one month<br> 
then age of Y is two months<br> 
rust will take shortest age(X -> one month)<br> 
```fn take_largest_str<'a>(x: &'a str, y: &'a str) -> &'str```
if we implement above fn:<br> 
```
let str_y: &str = "qwerty";
let result &str;
{
    let x: &str = "hello world"
    result = take_largest_str(x, y);
    println{"{}", result} // still valid here but if you move this to outside will invalid
}

println{"{}", result} // will invalid here

```
## these are several example one instance have dependency to another lifetime instance
```
struct Owner {
    resource: Resource,
}

impl Owner {
    fn borrow<'a>(&'a self) -> Borrowed<'a> {
        Borrowed {
            resource: &self.resource,
        }
    }
}

struct Borrowed<'a> {
    resource: &'a Resource,
}
```
```
struct Context {
    inner: *mut NativeCtx,
}

struct Handle<'a> {
    ctx: &'a Context,
}
```



| Jenis | Contoh                    | Akses                |
| ----- | ------------------------- | -------------------- |
| Named | `struct User { id: u64 }` | `u.id`               |
| Tuple | `struct UserId(u64)`      | `id.0` / destructure |
| Unit  | `struct Admin;`           | tidak ada            |

Type::<Generic>::function()

🚀 NEXT LEVEL (kalau mau lanjut)
Kalau lu mau, gue bisa:
bedah Result::<T, E> sampai tulang
jelasin where T: Trait + generic bound
jelasin impl<T> Type<T> vs impl Type<u32>
bandingin ini sama Go generic (biar klik)

tuple struct UserId(u64)
tuple struct UserId(String)
atau struct UserId<T>(T)

-- set
let id = UserId(10) atau let UserId(inner) = id;
-- call
id.0

-- bisa digunakan untuk "newtype pattern"
struct UserId(u64)
struct OrderId(u64)
pub fn find_user(UserId)
pub fn find_order(OrderId)
sama-sama u64 tapi tidak mungkin ketuker, atau mungkin bisa diset
struct UserId(u64)
struct OrderId(u32)
pub fn find_user(UserId)
pub fn find_order(OrderId) <-- u32

pemanggilan lewat destructing
let User { id, name } = user;

let q: Query(T) = query; <-- diambil valuenya dengan cara q.0 untuk mengambil T/value
atau dengan destructing
let Query(T) = query;
T <- return T

let (a, b) = (1, 2);

-- pengembangan
contoh 1
struct A;
impl A {
    fn hello() {}
}
pemanggilan A::hello()

contoh 2
struct A<T>(T); <- intinya adalah A adalah wrapper/pembungkus T, jadi T adalah isi, bungkusnya A
impl<T> A<T> {
    fn new(val: T) -> Self {
        A(val)
    }
}

let A(T) = A::T::new(10 as u32);

T akan menjadi u32
pemanggilan let a = A::<u32>::new(10); <- 10 auto u32
let a = A::new(10) <- masih bisa ditebak tapi T = i32
akan susah di bagian
let a = A::new(Vec::new()); <- rust bingung itu vector tipe apaan jeng;
makanya kudu didefinisikan
let a = A::<Vec<u32>>::new([1,2,3]) <- rust paham sekarang
*note 
dipakai path/namespace -> "::<> "
dipakai buat type -> "<>" -> ex: A<T>

-- kalau ada:
struct SomeStruct<T>(T);
impl<T> SomeStruct<T> {
    fn some_function()
}
dipanggil dengan cara SomeStruct::<T>::some_function()
ex:
let v = Vec::<i32>::new();
let o = Option::<i32>::None;
let r = Result::<i32, _>::Ok(10);

finally -> Type::<Generic>::function()