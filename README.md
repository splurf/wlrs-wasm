# wlrs-wasm

## Build
```bash
trunk build --release
wasm-opt --strip-debug dist/*.wasm -o dist/*.wasm
wasm-opt dist/*.wasm -Oz -o dist/*.wasm
```

## Production
```bash
cp -r dist/* /var/www/wlrs
```


## NGINX Configuration
```nginx
# /etc/nginx/sites-enabled/default

# serve the home page
location = / {
    rewrite ^ /index.html last;
}

# overwrite default type for WASM files
location ~* .*\.wasm$ {
    default_type application/wasm;
}
```
