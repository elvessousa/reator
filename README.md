# Reator

**Reator** _[pronunced: "hay-a-tor"]_ is a simple CLI for React projects made in Rust.
If you catch yourself typing to much boilerplate, you should give Reator a chance!

## Usage

```
$ reator command template ComponentName [--option]
```

### Commands

| Commands         | Description        |
| ---------------- | ------------------ |
| n / new / create | Creates a new file |
| h / help         | Shows help screen  |

### Templates

| Template       | Description                  |
| -------------- | ---------------------------- |
| rc / component | React component              |
| rnc / native   | React Native component       |
| ct / context   | React Context API file       |
| np / next-page | Next.js page                 |
| nd / next-doc  | Next.js "\_document.js" file |
| s / style      | React StyleSheet file        |
| sc / styled    | Styled-Components file       |

### Options (WIP)

| Option               | Description                                         |
| -------------------- | --------------------------------------------------- |
| --ws / --with-jstyle | Creates a component in a folder, with a style.js    |
| --css                | Creates a component in a folder, with a CSS Module  |
| --sass               | Creates a component in a folder, with a Sass Module |
| --i / --iprops       | Creates a component with a "Props" interface        |
| --t / --tprops       | Creates a component with a "Props" type             |

## Todos

- Check for package.json on component creation
- [x] Add scss/sass/css modules option
- Add component props option
- Deal with filename cases (PascalCase, [nextpage])
- Add configuration options via JSON
