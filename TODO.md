# TODO

There's a lot to do, the goals might change a lot along the line.

## Streams

- [] Add grouping capability's for streams, for example: ```cmdA (1,2)> example.txt```. This will redirect stdout and stderr to example.txt. This shall work for all operators that use streams.
- [] Range's for streams, for example: ```cmdA (0..3)> example.txt```. This would redirect all streams (stdout, stderr and stdin) of cmdA to the file example.txt. This range syntax for could be used for for loops, later on.

### Pipe
- [] Allow for pipes to pipe a specefic stream, for example: ```cmdA 2|0 cmdB```. This will pipe ```cmdA```'s stderr to ```cmdB```'s stdin.

### Redirect
- [] Add ```<<``` operator, with the above speceficied stream syntax.
- [] Add ```<<<``` operator, with the above speceficied stream syntax.
