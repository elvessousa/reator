pub const REACT_IMPORT: &str = "import React from 'react';\n\n";
pub const REACT_CONTEXT_IMPORT: &str = "import { createContext } from 'react';\n\n";

pub const REACT_NATIVE_IMPORT: &str =
    "import React from 'react';\nimport { View, Text } from 'react-native';\n\n";

pub const REACT_STYLE: &str = r#"import { StyleSheet } from 'react-native';

export const styles = StyleSheet.create({
  container: {},
});"#;

pub const REACT_STYLE_MODULE: &str = ".container {}";

pub const REACT_STYLED: &str =
    "import styled from 'styled-components';\n\nexport const StyledDiv = styled.div``;";

pub const REACT_CONTEXT: &str = r#"export const [name]Context = createContext([]);

export function [name]Provider({ children }) {
  return (
    <[name]Context.Provider value={[]}>
      {children}
    </[name]Context.Provider>
  );
}"#;

pub const REACT_STATELESS: &str = "export const [name] = () => (\n  <div>[name]</div>\n);\n\n";

pub const NEXT_DOCUMENT: &str = r#"import Document, { Html, Head, Main, NextScript } from 'next/document';

class MyDocument extends Document {
  static async getInitialProps(ctx) {
    const initialProps = await Document.getInitialProps(ctx)
    return { ...initialProps }
  }

  render() {
    return (
      <Html lang="en">
        <Head />
        <body>
          <Main />
          <NextScript />
        </body>
      </Html>
    )
  }
}

export default MyDocument;"#;
