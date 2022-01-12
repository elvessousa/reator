<h1 align="center">
    <img alt="Reator Icon" title="Reator" src="./.github/reator.png" width="150px" /><br>
    Reator
</h1>

**Reator** _[pronunced: "hae-a-tor"]_ is a simple CLI for React projects made in Rust.
If you catch yourself typing to much boilerplate, you should give Reator a chance!

## Usage

```
$ reator command template ComponentName [--style-module-option]
```

### Commands

| Commands               | Description        |
| ---------------------- | ------------------ |
| `n` / `new` / `create` | Creates a new file |
| `h` / `help`           | Shows help screen  |

### Templates

| Abbr  | Template             | Description                      |
| ----- | -------------------- | -------------------------------- |
| `rc`  | `component`          | React Component                  |
| `cc`  | `compound-component` | React Compound Component         |
| `rn`  | `native`             | React Native Component           |
| `nsc` | `native-screen`      | React Native Screen              |
| `cn`  | `compound-native`    | React Native Compound Component  |
| `ct`  | `context`            | React Context API file           |
| `np`  | `next-page`          | Next.js Page                     |
| `ns`  | `next-ssg`           | Next.js Static Page              |
| `nss` | `next-ssr`           | Next.js SSR Page                 |
| `nd`  | `next-doc`           | Custom Next.js '\_document' file |
| `s`   | `style`              | CSS Module                       |
| `sc`  | `styled`             | Styled Component                 |

### Options (WIP)

| Abbr    | Option                | Description                                                   |
| ------- | --------------------- | ------------------------------------------------------------- |
| `-rns`  | `--reactnative-style` | Creates a React Native Component in a folder, with a style.js |
| `-css`  | `--css-module`        | Creates a component in a folder, with a CSS Module            |
| `-sass` | `--sass-module`       | Creates a component in a folder, with a Sass Module           |

## Todo

- Add configuration options via JSON
