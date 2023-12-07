


//todo

//clock 输入
//sui client call --package <EXAMPLE> --module 'clock' --function 'access' --args '0x6' --gas-budget 10000

/*
1. replace global storage....
move_to , move_from, borrow_global_mut, borrow_global, exists, remove

2. replace event


3. token vote lock and return

4. token total supply gettting function
*/



todo.
1. 目前的代码使用了game admin 直接操作对象
2. 如果要使用player1, player2参与的方式， 应该用一个object 记录player的move， 然后把object发送给admin或者game，
然后由admin从这个object读取信息之后，调用合约操作game

上面是独享对象的操作方式

3. 在或者， 使用shared object的方式操作。


更多的可以参考sample
====================================

编译
bfc move build

部署
bfc client publish --gas-budget 20000000 ./

Get counter
bfc client call --function getCounter --module counter --package BFC44e41b028d1e510229576eb799b5cba2970807f1fc3094056cf30b370cbcfed81dc5 --gas-budget 100000000

Incr
bfc client call --function incr --module counter --package 0x2dd7799b671703f2470fdac80e712cb6de8807da69be0afb9d2c3aa9860a7e91 --args 0x732038fb064e8eae4d1590f142ed6308d0fe643076f52949cf88bd2135a9747f --gas-budget 100000000



