module Chess::game {
    // Part 1: imports
    //use std::ascii::string;
    use sui::transfer;
    use sui::object::{Self, UID};
    use sui::tx_context::{Self, TxContext};
    use std::vector;
    use std::string;
    use sui::event;
    //use std::debug;

    //use sui::test_scenario;

    const ChessBoardsize: u64 = 20;

    struct Board has store{
       board: vector<vector<Node>>,
    }
    struct Mark has key, store {
        id: UID,
        player: address,
        row: u64,
        col: u64,
    }


    struct Game has key, store{
        id: UID,
        admin:address,
        chessBoard: Board,
        stepsCount: u64,
        player1 : address,
        player2 : address,
    }
    struct Node has store {
        value: u8
    }

    struct ChessEvent has copy, drop {
        name: string::String,
    }

    entry public fun testBool(value: bool): bool {
        let result = value;

        result
    }

    // Part 3: transfer the counter object to the sender
    entry public fun createChess(player1:address,  player2:address, ctx: &mut TxContext ) {

        // sender address
        let sender = tx_context::sender(ctx);
        let board = Board{
            board : vector::empty<vector<Node>>(),
        };
        let game_obj = Game {
            id: object::new(ctx),
            admin:sender,
            chessBoard: board,
            stepsCount : 0,
            player1 : player1,
            player2 : player2,
        };

        let i:u64 = 0;
        while(i < ChessBoardsize){
            let row = vector::empty<Node>();
            let j:u64 = 0;
            while(j < ChessBoardsize){
                let value:Node = Node{
                    value: 0
                };
                vector::push_back(&mut row, value);
                j = j + 1;
            };
            vector::push_back(&mut game_obj.chessBoard.board, row);
            i = i + 1;
        };
        transfer::transfer(game_obj, sender);

    }

    entry public fun setChessBoardValue(game: &mut Game,
                                        x:u64, y:u64,
                                        ctx: &mut TxContext
    ) {
        // sender address
        let sender = tx_context::sender(ctx);
        assert!(sender==game.admin, 0);

        assert!(x < ChessBoardsize, 0);
        assert!(y < ChessBoardsize, 0);
        let row = vector::borrow_mut(&mut game.chessBoard.board, x);
        let node = vector::borrow_mut(row, y);
        node.value = 1

    }

    entry public fun getChessBoardValue(game: &mut Game,
                                        x:u64, y:u64,
                                        ctx: &mut TxContext): u8{

        // sender address
        let sender = tx_context::sender(ctx);
        assert!(sender==game.admin, 0);

        assert!(x < ChessBoardsize, 0);
        assert!(y < ChessBoardsize, 0);
        let row = vector::borrow_mut(&mut game.chessBoard.board, x);
        let node = vector::borrow_mut(row, y);
        node.value

    }

    entry public fun play(game:&mut Game, x:u64, y:u64, ctx: &mut TxContext) {

        let sender = tx_context::sender(ctx);
        let currentSenderValue:u8;
        if (game.stepsCount % 2 == 0) {
            assert!(sender==game.player1, 0);
            currentSenderValue = 1;
        } else{
            assert!(sender==game.player2, 0);
            currentSenderValue = 2
        };

        assert!(x < ChessBoardsize, 0);
        assert!(y < ChessBoardsize, 0);
        let row = vector::borrow_mut(&mut game.chessBoard.board, x);
        let node = vector::borrow_mut(row, y);
        node.value = currentSenderValue;

        game.stepsCount = game.stepsCount + 1;
        if (game.stepsCount == ChessBoardsize * ChessBoardsize) {
            //send event, game board is full.
            let eventname= b"the chessBoard is full";
            event::emit(ChessEvent{
                name: string::utf8(eventname),
            });

        };
        //todo: add check win logic,
        {
            checkBoard(game, x, y)
        }

    }

    fun Horizontal(a:u64, b:u64,  game:&mut Game) :bool {
        let row = vector::borrow_mut(&mut game.chessBoard.board, a);
        let node = vector::borrow_mut(row, b);
        let qz = node.value;
        if (qz == 0) {
            return false
        };

        let length : u64 = 1;
        let i : u64 = 1;
        while (i <= 4) {
            if (b + i < ChessBoardsize) {
                let row = vector::borrow_mut(&mut game.chessBoard.board, a);
                let node = vector::borrow_mut(row, b + i);
                if (node.value != qz) {
                    break
                };
                i = i + 1;
                length = length + 1;
            } else {
                break
            }
        };

        i = 1;
        while (i <= 4) {
            if (b  >=  i) {
                let row = vector::borrow_mut(&mut game.chessBoard.board, a);
                let node = vector::borrow_mut(row, b - i);

                if (node.value != qz) {
                    break
                };
                i = i + 1;
                length = length + 1;
            } else {
                break
            }
        };

        if (length >= 5) {
            return true
        };
        false
    }

    fun Vertical(a:u64, b:u64, game:&mut Game) :bool {
        let row = vector::borrow_mut(&mut game.chessBoard.board, a);
        let node = vector::borrow_mut(row, b);
        let qz = node.value;
        if (qz == 0 ) {
            return false
        };

        let length : u64 = 1;
        let i : u64 = 1;
        while (i <= 4) {
            if  (a + i < ChessBoardsize) {
                let row = vector::borrow_mut(&mut game.chessBoard.board, a + i);
                let node = vector::borrow_mut(row, b);
                if (node.value != qz) {
                    break
                };
                i = i + 1;
                length = length + 1;
            } else {
                return false
            }
        };

        i = 1;
        while (i <= 4) {
            if  (a  >=  i) {
                let row = vector::borrow_mut(&mut game.chessBoard.board, a - i);
                let node = vector::borrow_mut(row, b);
                if (node.value != qz) {
                    break
                };
                i = i + 1;
                length = length + 1;
            } else {
                return false
            }
        };

        if (length >= 5) {
            return true
        };
        false
    }

    fun RightTwill(a:u64, b:u64, game:&mut Game) : bool {
        let row = vector::borrow_mut(&mut game.chessBoard.board, a);
        let node = vector::borrow_mut(row, b);
        let qz = node.value;
        if (qz == 0 ) {
            return false
        };

        let i : u64 = 1;
        let length : u64 = 1;
        while (i <= 4) {
            if (a  + i < ChessBoardsize && b+ i < ChessBoardsize) {
                let row = vector::borrow_mut(&mut game.chessBoard.board, a + i);
                let node = vector::borrow_mut(row, b + i);
                if (node.value != qz) {
                    break
                };
                i = i + 1;
                length = length + 1;
            } else {
                break
            }
        };

        i = 1;
        while (i <= 4) {
            if (a  >= i && b > i) {
                let row = vector::borrow_mut(&mut game.chessBoard.board, a - i);
                let node = vector::borrow_mut(row, b - i);
                if (node.value != qz) {
                    break
                };
                i = i + 1;
                length = length + 1;
            } else {
                break
            }
        };

        if (length >= 5) {
            return true
        };
        false
    }

    fun LeftTwill(a:u64, b:u64, game:&mut Game) : bool {
        let row = vector::borrow_mut(&mut game.chessBoard.board, a);
        let node = vector::borrow_mut(row, b);
        let qz = node.value;
        if (qz == 0 ) {
            return false
        };

        let i : u64 = 1;
        let length : u64 = 1;
        while (i <= 4) {
            if (b >= i && a + i < ChessBoardsize) {
                let row = vector::borrow_mut(&mut game.chessBoard.board, a + i);
                let node = vector::borrow_mut(row, b - i);
                if (node.value != qz) {
                    break
                };
                i = i + 1;
                length = length + 1;
            } else {
                break
            }
        };

        i = 1;
        while (i <= 4) {
            if (b + i < ChessBoardsize && a >= i) {
                let row = vector::borrow_mut(&mut game.chessBoard.board, a - i);
                let node = vector::borrow_mut(row, b + i);
                if (node.value != qz) {
                    break
                };
                i = i + 1;
                length = length + 1;
            } else {
                break
            }
        };

        if (length >= 5) {
            return true
        };
        false
    }

    fun checkBoard(game:&mut Game, i : u64, j: u64) {
        if (Horizontal(i,j, game) || Vertical(i,j, game) || LeftTwill(i, j, game) || RightTwill(i, j, game)) {
            let row = vector::borrow_mut(&mut game.chessBoard.board, i);
            let node = vector::borrow_mut(row, j);
            if (node.value == 1) {
                let eventname= b"White player win";
                event::emit(ChessEvent{
                    name: string::utf8(eventname),
                });
                //let whiteWin = 200;
                //debug::print(&string(eventname));
            } else {
                let eventname= b"Black player win";
                event::emit(ChessEvent{
                    name: string::utf8(eventname),
                });

                //let blackWin = 300;
                //debug::print(&blackWin);
                //debug::print(&string(eventname));
            }
        };
    }

    entry public fun resetGame(game: &mut Game,
                               ctx: &mut TxContext) {
        // sender address
        let sender = tx_context::sender(ctx);
        assert!(sender==game.admin, 0);

        let i:u64 = 0;
        while(i < ChessBoardsize){
            let row = vector::borrow_mut(&mut game.chessBoard.board, i);

            let j:u64 = 0;
            while(j < ChessBoardsize){

                let node = vector::borrow_mut(row, j);
                node.value = 0;

                j = j + 1;
            };
            i = i + 1;
        };
        game.stepsCount = 0;

    }



    #[test]
    public fun test_chess_init(){
        use sui::test_scenario;
        use std::debug;

        let owner = @0xC0FFEE;
        let user1 = @0xA1;
        let user2 = @0xB1;

        let scenario_val = test_scenario::begin(owner);

        test_scenario::next_tx(&mut scenario_val, owner);
        {
            createChess(user1, user2, test_scenario::ctx(&mut scenario_val));
        };

        test_scenario::next_tx(&mut scenario_val, owner);
        {

            let game = test_scenario::take_from_sender<Game>(&mut scenario_val);

            resetGame(&mut game, test_scenario::ctx(&mut scenario_val));

            let x:u64  = 1;
            let y:u64 = 1;
            getChessBoardValue(&mut game, x, y, test_scenario::ctx(&mut scenario_val));
            //assert!(value==0, 0);
            test_scenario::return_to_sender(&mut scenario_val, game);
        };

        // 1 X X X X X
        // X X X X X X
        // X X X X X X

        //player1 send mark to admin
        test_scenario::next_tx(&mut scenario_val, user1);
        {
            let row = 0;
            let col = 0;
            let ctx = test_scenario::ctx(&mut scenario_val);
            let mark =  Mark {
                id: object::new(ctx),
                player: tx_context::sender(ctx),
                row,
                col,
            };

            transfer::public_transfer(mark,  owner);

        };
        
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let game = test_scenario::take_from_sender<Game>(&mut scenario_val);
            let mark = test_scenario::take_from_sender<Mark>(&mut scenario_val);
            
             debug::print(&mark.row);
             debug::print(&mark.col);
             
            game.player1 = owner;
            play(&mut game, mark.row, mark.col, test_scenario::ctx(&mut scenario_val));
            test_scenario::return_to_sender(&mut scenario_val, game);
            test_scenario::return_to_sender(&mut scenario_val, mark);
            
        };

        // 1 X X X X X
        // 2 X X X X X
        // X X X X X X
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let game = test_scenario::take_from_sender<Game>(&mut scenario_val);
            game.player2 = owner;
            play(&mut game, 1, 0, test_scenario::ctx(&mut scenario_val));
          //  let value =
            getChessBoardValue(&mut game, 0, 0, test_scenario::ctx(&mut scenario_val));
            //debug::print(&value);
            test_scenario::return_to_sender(&mut scenario_val, game);
        };

        // 1 1 X X X X
        // 2 X X X X X
        // X X X X X X
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let game = test_scenario::take_from_sender<Game>(&mut scenario_val);
            game.player1 = owner;
            play(&mut game, 0, 1, test_scenario::ctx(&mut scenario_val));
            test_scenario::return_to_sender(&mut scenario_val, game);
        };

        // 1 1 X X X X
        // 2 2 X X X X
        // X X X X X X
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let game = test_scenario::take_from_sender<Game>(&mut scenario_val);
            game.player2 = owner;
            play(&mut game, 1, 1, test_scenario::ctx(&mut scenario_val));
            test_scenario::return_to_sender(&mut scenario_val, game);
        };

        // 1 1 1 X X X
        // 2 2 X X X X
        // X X X X X X
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let game = test_scenario::take_from_sender<Game>(&mut scenario_val);
            game.player1 = owner;
            play(&mut game, 0, 2, test_scenario::ctx(&mut scenario_val));
            test_scenario::return_to_sender(&mut scenario_val, game);
        };

        // 1 1 1 X X X
        // 2 2 2 X X X
        // X X X X X X
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let game = test_scenario::take_from_sender<Game>(&mut scenario_val);
            game.player2 = owner;
            play(&mut game, 1, 2, test_scenario::ctx(&mut scenario_val));
            test_scenario::return_to_sender(&mut scenario_val, game);
        };

        // 1 1 1 1 X X
        // 2 2 2 X X X
        // X X X X X X
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let game = test_scenario::take_from_sender<Game>(&mut scenario_val);
            game.player1 = owner;
            play(&mut game, 0, 3, test_scenario::ctx(&mut scenario_val));
            test_scenario::return_to_sender(&mut scenario_val, game);
        };

        // 1 1 1 1 X X
        // 2 2 2 2 X X
        // X X X X X X
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let game = test_scenario::take_from_sender<Game>(&mut scenario_val);
            game.player2 = owner;
            play(&mut game, 1, 3, test_scenario::ctx(&mut scenario_val));
            test_scenario::return_to_sender(&mut scenario_val, game);
        };

        // 1 1 1 1 1 X
        // 2 2 2 2 X X
        // X X X X X X
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let game = test_scenario::take_from_sender<Game>(&mut scenario_val);
            game.player1 = owner;
            play(&mut game, 0, 4, test_scenario::ctx(&mut scenario_val));
            test_scenario::return_to_sender(&mut scenario_val, game);
        };
        test_scenario::end(scenario_val);
    }
}




//https://suiexplorer.com/object/0xe9db03e16cc663e1a15e1f9caf462e4c19e6f9d20982482a2e9c1ae999367247?network=devnet