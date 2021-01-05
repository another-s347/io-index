# watchmedo comes from python package "watchdog"

watchmedo shell-command \
    --patterns="*.rs;*.txt" \
    --recursive \
    --command='wasm-pack build --target web --out-name wasm --out-dir ./static --dev' \