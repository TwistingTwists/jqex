Inspiration for this project came from making https://github.com/davydog187/jqex work with across platforms.

to install jqex on your platform, 

1. clone the repo
2. run `cargo install --locked --path .`


```bash
git clone git@github.com:TwistingTwists/jqex.git
cd jqex
cargo install --locked --path .
```

Example: 

```bash
 echo '{"name": "John", "age": 30}' | jqex 
 # %{"age" => 30, "name" => "John"}
```
