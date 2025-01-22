## 简介
一款局域网地址冲突检测的命令行工具，能检测出局域网不同设备之间`IP`和`MAC`地址冲突
## 用法
``` bash
# 安装
cargo install --git https://github.com/lbhzy/lan-acd

# 使用说明
$ lan-acd -h
LAN address conflict detection (2025-01-22 23:48:07 +08:00)

Usage: lan-acd.exe [OPTIONS]

Options:
  -l, --list               List all interfaces and index
  -i, --iface <IFACE>      Select interface index
  -t, --timeout <TIMEOUT>  Stop if no ARP reply beyond this time (ms) [default: 300]
  -h, --help               Print help

# 查看网络接口
$ lan-acd -l
0. 192.168.150.1/24   |  VMware Virtual Ethernet Adapter for VMnet8
1. 192.168.253.1/24   |  VMware Virtual Ethernet Adapter for VMnet1
2. 10.10.10.136/24    |  Realtek PCIe GbE Family Controller
3. 10.10.10.192/24    |  Intel(R) Wi-Fi 6E AX210 160MHz
4. 0.0.0.0/0          |  Microsoft Wi-Fi Direct Virtual Adapter #2
5. 0.0.0.0/0          |  Microsoft Wi-Fi Direct Virtual Adapter
6. 172.31.48.1/20     |  Hyper-V Virtual Ethernet Adapter
7. 0.0.0.0/0          |  TAP-Windows Adapter V9

# 指定网口开始检测
$ lan-acd -i2
10.10.10.1      d4:da:21:03:12:9f
10.10.10.136    3c:2c:30:98:af:7b (local)
10.10.10.192    bc:6e:e2:dd:31:fa
10.10.10.214    86:55:f4:93:6f:67
devices count: 4
no conflict

```
## 局限
由于Windows系统不会响应以太网头中源Mac地址与自身冲突的`ARP`请求，因此，无法检测到本机与其他Windows主机之间Mac冲突的情况