# kifuwarabe-dynamic-programming

Programming computer shogi (Japanese chess) is difficult.  
コンピュータ将棋のプログラミングは難しいです。  

How can I improve my programming skills?  
プログラミングスキルを向上させるにはどうすればよいですか？  

A strategy of gradually stepping up from a simple game is not appropriate.  
単純なゲームから徐々にステップアップする戦略は適切ではありません。  

The reason is that the right program to solve the problem depends entirely on the game. Unless you have general artificial intelligence.  
その理由は、問題を解決するための適切なプログラムが完全にゲームに依存しているためです。汎用人工知能を持っていない限り。  

I recommend that you write computer shogi software directly, rather than bypassing it.  
迂回するのではなく、コンピューター将棋ソフトウェアを直接作成することをお勧めします。  

Of course it's best to start by modifying the strongest library. Helps to find out what is missing.  
もちろん、最強のライブラリを変更することから始めるのが最善です。 何が欠けているかを知るのに役立ちます。  

However, if you want to overcome the weaknesses of basic programming skills unrelated to computer shogi, it makes sense to start with a simple program.  
ただし、コンピュータ将棋とは関係のない基本的なプログラミングスキルの弱点を克服したい場合は、単純なプログラムから始めるのが理にかなっています。  

Dynamic programming, for example, is the basis of reinforcement learning.  
例えば動的計画法は強化学習の基礎です。  

Computer Shogi, computer Go program "Kifuwarabe" has been converted into a practice example of dynamic programming.  
コンピュータ将棋、コンピュータ囲碁プログラム「きふわらべ」を動的計画法の練習用例に変換しました。  

It was implemented with an unspecified UXI protocol that imitates the UCI / USI protocol. X has no meaning.　 
UCI / USIプロトコルを模した未指定のUXIプロトコルで実装しました。 X に意味はありません。  

Come see the repository.  
リポジトリをご覧ください。  

There is **no** GUI (Graphical user interface). Thought engine only.  
GUIは **ありません**。思考エンジンのみです。  

## Run

Terminal:  

```
cargo run
```

## How to program a dynamic-programming game?

For such small programs, it may be faster to rewrite the new program.  
このような小さなプログラムの場合、新しいプログラムを書き直す方が速い場合があります。  

During development, you may need to reproduce the behavior of your computer.  
It is difficult to compare the behavior. Instead, it is useful to get the logs and compare the logs.  
**But logger's difficult to make, so use library.**  

* [x] Step 1. Use logger library.
  * [x] Use casual_logger library at 'Cargo.toml', 'main.rs'.
  * [x] Create the 'log.rs' file.
    * [x] Extend the logger.

Let's proceed with development while testing.  

* [ ] Step 2. Create the `test.rs` file.
  * [ ] Write a 'Hello, world!!' message to log file.
  * Add little by little as you progress through the steps.  

The first thing you have to create is your motive.  
It is important to start with the appearance.  

* [ ] Step 3. Create the 'main.rs' file and 'look_and_model.rs' file.
  * [ ] Engine - Title screen.
  * [ ] Piece - "O", "X".
  * [ ] Game result - Win/Draw/Lose.
  * [ ] Position - It's the board.
  * [ ] Search - Computer player search.
  * [ ] Search info - Computer player search info.

If you want to play immediately, you have the talent of a game creator.  
Being able to control your position means being able to play.  

* [ ] Step 4. Create the 'position.rs' file.
  * [ ] do_move
  * [ ] undo_move
  * [ ] opponent

Let's enter commands into the computer. Create a command line parser.  

* [ ] Step 5. Create the 'command_line_seek.rs' file.
  * [ ] Starts with.
  * [ ] Go next to.
  * [ ] Rest.

People who are looking for something 10 minutes a day are looking for something for a week in a year.  
Before creating the game itself, let's first create the replay function. Let's get it for a week.  

* [ ] Step 6. Create the 'uxi_protocol.rs' file.
  * [ ] Do. (Before 'From XFEN') Excludes legal moves and winning/losing decisions.
  * [ ] To XFEN.
  * [ ] From XFEN.
  * [ ] Undo.

Let's make a principal command.  

* [ ] Step 7. Create the 'engine.rs' file. Command line.
  * [ ] position.
  * [ ] pos.
  * [ ] do.
  * [ ] undo.
  * [ ] uxi.
  * [ ] xfen.
* [ ] Step 8. 'src/main.rs' or 'examples/main.rs'.

Before you make a computer player, let's judge the outcome. And let's test.  

* [ ] Step 9. 'win_lose_judgment.rs'
  * [ ] Win.
  * [ ] Draw - Not win, not lose, can not play.
  * [ ] Lose. - Not win is lose.

Before creating a computer player, let's create a mechanism to measure performance.  

* [ ] Step 10. 'performance_measurement.rs'
  * [ ] Seconds. - Stopwatch.
  * [ ] Node per second.

Let's make a computer player.  

* [ ] Step 11. 'computer_player/search.rs'
  * [ ] Search.
* [ ] 'main.py'
  * [ ] Create "go" command.

Let's larning a computer thinking.

* [ ] Step 12. 'computer_player/evaluation.rs'
  * [ ] Evaluation - None.
    * [ ] ways_weight.
    * [ ] Save.
    * [ ] Load.
    * [ ] Get value.
    * [ ] Set value.
    * [ ] Print some feature.
  * [ ] Allocation functions.
    * [ ] TODO `posval` command.
    * [ ] TODO `posdiffval {file}` command.
    * [ ] TODO `posdifffeat {file}` function.
* [ ] Step 13. `computer_player/learn.rs`

Finally.

* [ ] Remeve all 'TODO' tasks. Examples: '// TODO Write a code here.'
