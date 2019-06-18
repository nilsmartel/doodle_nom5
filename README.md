# Nom5 doodles

Since nom 4.0 _a lot_ has changed and adapting these changes really was worth it.
Having working autocompletion and error messages at hand really makes the learning curve bearable, I'm quite satisfied.

The actual project here is rather simple, really.
It just parses Math expressions into some rather unconventional intermediate representation/AST I wanted to experiment with.
(Instead of a Linked Tree I'm using nested Vectors to keep track of the meaning)

the program was written in ~10 Minutes and works rather lovely.

here's the output:

```
12+7:
    Ok(("", [[[[12.0]]], [[[7.0]]]]))
3-14:
    Ok(("", [[[[3.0]], [[14.0]]]]))
2*10+7:
    Ok(("", [[[[2.0], [10.0]]], [[[7.0]]]]))
1+2*7+4:
    Ok(("", [[[[1.0]]], [[[2.0], [7.0]]], [[[4.0]]]]))
```

wich is exacly what I was hoping for, whew. Always glad, when small experiments work! :)
