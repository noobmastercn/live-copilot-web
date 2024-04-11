# live-copilot 推广介绍页 

- leptos ssr
- seo为第一优先级

## 相关命令

```shell
rustup target add wasm32-unknown-unknown
cargo install cargo-leptos
cargo leptos watch
cargo leptos build --release
```
- release模式下，可以设置环境变量修改端口
```shell
export LEPTOS_SITE_ADDR="0.0.0.0:3000"
```

linux下使用systemd管理服务
```shell
nano /etc/systemd/system/live-copilot-web.service
```
```shell
[Unit]
Description=live-copilot-web
After=network.target

[Service]
Environment=PATH=/home/appuser:$PATH
AmbientCapabilities=CAP_NET_BIND_SERVICE
User=appuser
Group=appuser
Environment="LEPTOS_SITE_ADDR=0.0.0.0:3000"
WorkingDirectory=/home/appuser
ExecStart=/home/appuser/live-copilot-web
Restart=always

[Install]
WantedBy=multi-user.target
```
```shell
systemctl daemon-reload
systemctl enable live-copilot-web
systemctl start live-copilot-web
journalctl -f -u live-copilot-web
```
