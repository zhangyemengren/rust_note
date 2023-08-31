# ffi 学习项目
## 目标
- [ ] 在rust调用c
- [ ] 在js调用rust

## third_party
### c
c 位置: `third_party/src/c` 这是带有lib和main的c项目 
其中lib要编译成动态库和静态库`third_party/build/static` `third_party/build/dynamic`
main是测试用的编译成可执行文件到`third_party/build/bin`下
```bash
clang -c fn_a.c fn_b.c           # 编译 fn_a.c 和 fn_b.c 成目标文件
clang -c main.c                  # 编译 main.c 成目标文件

clang -o bin/my_program main.o fn_a.o fn_b.o  # 链接目标文件生成可执行文件在 bin 目录中

```

编译c动态库
```bash
clang -c -fPIC fn_a.c fn_b.c  # 编译成位置无关的目标文件（-fPIC）
clang -shared -o libmylib.so fn_a.o fn_b.o  # 在类 Unix 系统上
clang -dynamiclib -o libmylib.dylib fn_a.o fn_b.o  # 在 macOS 系统上
```

编译c静态库
```bash
clang -c fn_a.c fn_b.c
ar rcs libmylib.a fn_a.o fn_b.o
```