
使用[sysinfo](https://crates.io/crates/sysinfo)库获取设备当前信息并上报给服务端

```
Usage: feishu_table_client [OPTIONS] --server <SERVER> --token <TOKEN>

Options:
  -s, --server <SERVER>  Server to send post request
  -d, --data <DATA>      Data to post(deprecate) [default: "Hello, World!"]
  -t, --token <TOKEN>    Token for authrazation
  -h, --help             Print help
```