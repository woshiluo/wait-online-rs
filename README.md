# wait-online-rs

对 <https://github.com/lilydjwg/wait-online> 的拙劣模仿。

我自己机器在跑 wait-online 的时候老是挂掉但是找不到原因，所以就自己写了一个。

## 使用

项目目录下提供了 PKGBUILD ，请自行编译安装。

### 服务

运行 `systemctl enable --now wait-online-rs.service` 即可。

该 Service 会在尝试启动 `network-online.target` 时先执行 `wait-online-rs`。

### 命令行

`echo 'test' > /run/wait-online-rs/204.cond`

在启动了 `wait-online-rs` 后，命令会一直堵塞直到确认连网。
