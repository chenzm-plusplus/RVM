# run

测试流程：
```shell
mkdir working_dir
cd working_dir
git clone https://gitlab.eduxiji.net/18603560353/project325618-89192.git RVM
git clone https://gitlab.eduxiji.net/18603560353/project0-rvm-tutorial-hostos.git HostOS
git clone https://gitlab.eduxiji.net/18603560353/project0-rvm-tutorial-guestos.git GuestOS
git clone https://gitlab.eduxiji.net/18603560353/project0-rvm-tutorial-riscv.git riscv
cd GuestOS
git fetch origin
git checkout -b ch2-rvm origin/ch2-rvm
cd os
make build LOG=info
cd ..
cd ..
cd HostOS
cd os
make run
```
以上测试流程将会在虚拟化层中启动一个GuestOS，GuestOS会运行一些触发异常的用户态进程。输出以下结果代表运行成功：
```
OpenSBI v0.9
   ____                    _____ ____ _____
  / __ \                  / ____|  _ \_   _|
 | |  | |_ __   ___ _ __ | (___ | |_) || |
 | |  | | '_ \ / _ \ '_ \ \___ \|  _ < | |
 | |__| | |_) |  __/ | | |____) | |_) || |_
  \____/| .__/ \___|_| |_|_____/|____/_____|
        | |
        |_|

Platform Name             : riscv-virtio,qemu
Platform Features         : timer,mfdeleg
Platform HART Count       : 1
Firmware Base             : 0x80000000
Firmware Size             : 100 KB
Runtime SBI Version       : 0.2

Domain0 Name              : root
Domain0 Boot HART         : 0
Domain0 HARTs             : 0*
Domain0 Region00          : 0x0000000080000000-0x000000008001ffff ()
Domain0 Region01          : 0x0000000000000000-0xffffffffffffffff (R,W,X)
Domain0 Next Address      : 0x0000000080200000
Domain0 Next Arg1         : 0x0000000082200000
Domain0 Next Mode         : S-mode
Domain0 SysReset          : yes

Boot HART ID              : 0
Boot HART Domain          : root
Boot HART ISA             : rv64imafdcsuh
Boot HART Features        : scounteren,mcounteren,time
Boot HART PMP Count       : 16
Boot HART PMP Granularity : 4
Boot HART PMP Address Bits: 54
Boot HART MHPM Count      : 0
Boot HART MHPM Count      : 0
Boot HART MIDELEG         : 0x0000000000000666
Boot HART MEDELEG         : 0x0000000000f0b509
Hello, world!
last 99292 Physical Frames.
.text [0x80200000, 0x8020d000)
.rodata [0x8020d000, 0x80212000)
.data [0x80212000, 0x80213000)
.bss [0x80213000, 0x80424000)
mapping .text section
mapping .rodata section
mapping .data section
mapping .bss section
mapping physical memory
mapping memory-mapped registers
memset is [
    MapArea {
        vpn_range: SimpleRange {
            l: VPN:0x80200,
            r: VPN:0x8020d,
        },
        data_frames: {},
        map_type: Identical,
        map_perm: R | X,
    },
    MapArea {
        vpn_range: SimpleRange {
            l: VPN:0x8020d,
            r: VPN:0x80212,
        },
        data_frames: {},
        map_type: Identical,
        map_perm: R,
    },
    MapArea {
        vpn_range: SimpleRange {
            l: VPN:0x80212,
            r: VPN:0x80213,
        },
        data_frames: {},
        map_type: Identical,
        map_perm: R | W,
    },
    MapArea {
        vpn_range: SimpleRange {
            l: VPN:0x80213,
            r: VPN:0x80424,
        },
        data_frames: {},
        map_type: Identical,
        map_perm: R | W,
    },
    MapArea {
        vpn_range: SimpleRange {
            l: VPN:0x80424,
            r: VPN:0x98800,
        },
        data_frames: {},
        map_type: Identical,
        map_perm: R | W,
    },
    MapArea {
        vpn_range: SimpleRange {
            l: VPN:0x10001,
            r: VPN:0x10002,
        },
        data_frames: {},
        map_type: Identical,
        map_perm: R | W,
    },
]
remap_test passed!
[INFO][GUEST] Hello, world!
[kernel] trap::init()
[kernel] batch::init()
[kernel] num_app = 8
[kernel] app_0 [0x9000a050, 0x9000b088)
[kernel] app_1 [0x9000b088, 0x9000c180)
[kernel] app_2 [0x9000c180, 0x9000d538)
[kernel] app_3 [0x9000d538, 0x9000e518)
[kernel] app_4 [0x9000e518, 0x9000f568)
[kernel] app_5 [0x9000f568, 0x90010920)
[kernel] app_6 [0x90010920, 0x90012268)
[kernel] app_7 [0x90012268, 0x90013a78)
[kernel] batch::run_next_app()
[INFO]Loading app_0
[00hello_world] Hello, world!
[kernel] IllegalInstruction in application, core dumped.
[kernel] batch::run_next_app()
[INFO]Loading app_1
[01store_fault]
Into Test store_fault, we will insert an invalid store operation...
Kernel should kill this application!
[kernel] PageFault in application, core dumped.
[kernel] batch::run_next_app()
[INFO]Loading app_2
3^10000=5079
3^20000=8202
3^30000=8824
3^40000=5750
3^50000=3824
3^60000=8516
3^70000=2510
3^80000=9379
3^90000=2621
3^100000=2749
Test power OK!
[kernel] Application exited with code 0
[kernel] batch::run_next_app()
[INFO]Loading app_3
[kernel] Application exited with code 1234
[kernel] batch::run_next_app()
[INFO]Loading app_4
Hello world from user mode program!
Test hello_world OK!
[kernel] Application exited with code 0
[kernel] batch::run_next_app()
[INFO]Loading app_5
3^10000=5079
3^20000=8202
3^30000=8824
3^40000=5750
3^50000=3824
3^60000=8516
3^70000=2510
3^80000=9379
3^90000=2621
3^100000=2749
Test power OK!
[kernel] Application exited with code 0
[kernel] batch::run_next_app()
[INFO]Loading app_6
string from data section
strinstring from stack section
strin
Test write1 OK!
[kernel] Application exited with code 0
[kernel] batch::run_next_app()
[INFO]Loading app_7
[WARN][kernel] ILLEGAL OUTPUT
[WARN][kernel] ILLEGAL OUTPUT
[WARN][kernel] ILLEGAL OUTPUT
Test write0 OK!
[kernel] Application exited with code 0
[kernel] batch::run_next_app()
hello world in kernel
```