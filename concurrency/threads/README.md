# Using threads

- an executing program's code is run in a process
- program can have multiple independent pieces that run independently. Features that run these pieces are called `threads`

### Threads

- Splitting computation of program into multiple thread can improve performance.
- threads can run simultaneously, hence no guarantee about the order of execution of different threads


### Problems

- **Race conditions**: where threads access data and resources in an inconsistent order
- **Deadlocks**:  where two threads are awaiting resources the other is consuming, hence 
preventing each of them from continuing 
