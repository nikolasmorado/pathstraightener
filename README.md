# PathStraightener
Straightens out svg paths into jsx/tsx files.

It's a transpiler, since SVGs are just HTML, we convert that HTML strcuture to
[HAST](https://github.com/syntax-tree/hast). From there we squish/straighten out
out any weird svgisms and spit out a jsx/tsx component 

