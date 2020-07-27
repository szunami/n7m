# n7m (neuronym)

`n7m` is a library for working with [neuronyms](http://www.i18nguy.com/origini18n.html) like `i18n`, `k8s`, or `w9k`.

If you've ever thought "infrastructure engineers sure like to make their work sound important by introducing stupid jargon," this library might be for you.

More than anything, this is a simple project for me to improve my Rust.

## Usage

```rust
        let kubernetes = String::from("kubernetes");
        from_full_word(&kubernetes);
        # k8s
        
        to_full_word(&String::from("k8s"), dictionary_of_words);
        # HashSet of many words, including kubernetes
```
