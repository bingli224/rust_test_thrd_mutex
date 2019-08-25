# Simple Pulling Game

Test Mutex+Arc

Each threads are the players that can change the racing position with
predefined role to be either -1 or 1. The player sleeps after each action.
The game stops when the game position reaches the positive-or-negative
target, and all other players will stop.

![Image of demo 1](https://github.com/bingli224/rust_test_thrd_mutex/raw/master/docs/demo.png)
![Image of demo 1](https://github.com/bingli224/rust_test_thrd_mutex/raw/master/docs/demo2.png)
