## webpack-tree - printing a dependency tree for a webpack module

Ever wondered why a certain file/dependency was included in your webpack build?

Running `webpack --json > build.json` and a simple

```
webpack-tree build.json <module id>
```

will tell you.

