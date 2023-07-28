### enable logger

    ```
    export RUST_LOG=wordspace=debug
    ```

### setup port from environment variables

```
exprt WORDSPACE_PORT=8085
```

### make sure only owners of the directory can perfrom operations

```
chmod 700 datastore
```

### make wordspace program the owner of datastore