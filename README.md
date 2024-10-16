# templated

## wrap-file

To wrap a file or bundle into Anti-Raid compatible format, use the ``wrap-file`` operation:

```bash
./out/templated --input abc/main.luau --output abc/test.luau wrap-file
```

## bundle-file

To bundle a set of files making up a Luau module script, you can use the ``bundle-file`` operation:

Assume we have the below files:

**double.luau**

```luau
function double(a)
    return a * 2
end

return {
    double = double
}
```

**main.luau**

```luau
function main(args)
    local double = require("./double.luau")

    local a = 1
    print(double.double(a))
end

main()
```

You can bundle this with the following command:

```bash
./out/templated --input examples/bundle/main.luau --output abc bundle-file
```

Note that the bundled files are *not* wrapped in the required entrypoint capsule, so run the following command to wrap the bundled file:

```bash
out/templated --input abc/main.luau --output abc/wrapped.luau wrap-file
```

You can then use ``abc/wrapped.luau`` as an Anti-Raid template in whatever setting you desire.