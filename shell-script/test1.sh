#!/bin/sh

# set -e：命令执行失败 (exit≠0)，脚本直接退出
# set -u：使用未定义变量，直接退出（防止变量为空引发诡异 bug）
# -x：打印每一条执行的命令，便于定位在哪一步挂掉。
set -eux


function1() {
    sleep 3 &
}

function1 &

echo "start"

sleep 10



