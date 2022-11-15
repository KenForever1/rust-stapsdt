### 启动

```
cargo run --example example-udst-file
```

```
ps -aux | grep udst

sudo /sbin/trace-bpfcc -p 132022  'u::PROBE_NAME "i=%d j=%d", arg1, arg2'
```