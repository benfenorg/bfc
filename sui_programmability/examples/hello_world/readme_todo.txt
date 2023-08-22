


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


