# pattern one instance have lifetime dependecy to another instance
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