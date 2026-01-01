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