# PathStraightener
Straightens out svg paths into jsx/tsx files.

It's a transpiler, since SVGs are just HTML, we convert that HTML strcuture to
[HAST](https://github.com/syntax-tree/hast). From there we squish/straighten out
out any weird svgisms and spit out a JSX/TSX component. 

## Usage
```bash
psx <path> <name> [options]
```

### Arguments:
- `<path>`: Relative path to an svg file. *(Required)*
- `<name>`: Custom component name. *(Optional)*

### Options:
- `-t` or `--typescript`: Exports the component with typescript annotations.
- `-r` or `--react-native`: Exports the component with React Native support. 
