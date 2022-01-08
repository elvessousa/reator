// Imports
pub const REACT_IMPORT: &str = "import React from 'react';\n\n";
pub const REACT_TYPED_IMPORT: &str = "import { ReactNode } from 'react';\n\n";
pub const REACT_CONTEXT_IMPORT: &str = "import { createContext } from 'react';\n";
pub const REACT_TYPED_CONTEXT_IMPORT: &str = "import { createContext, ReactNode } from 'react';\n";
pub const NEXT_TYPED_SSG_IMPORT: &str = "import { GetStaticProps } from 'next';\n\n";
pub const NEXT_TYPED_SSR_IMPORT: &str = "import { GetServerSideProps } from 'next';\n\n";
pub const REACT_NATIVE_IMPORT: &str =
    "import React from 'react';\nimport { View, Text } from 'react-native';\n\n";

pub const REACT_NATIVE_TYPED_IMPORT: &str =
    "import React, { ReactNode } from 'react';\nimport { View, Text } from 'react-native';\n\n";

// Typings
pub const REACT_COMPONENT_TYPING: &str = "type Props = {\n  children: ReactNode;\n};\n\n";
pub const REACT_CONTEXT_TYPING: &str = r#"
type [name]Data = {};

type [name]ProviderProps = {
  children: ReactNode;
};

"#;

// Text
pub const REACT_STYLE: &str = r#"import { StyleSheet } from 'react-native';

export const styles = StyleSheet.create({
  container: {},
});"#;

pub const REACT_CSS_MODULE: &str = ".container {}";

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

pub const REACT_CONTEXT_TYPED: &str = r#"export const [name]Context = createContext({} as [name]Data);

export function [name]Provider({ children }: [name]ProviderProps) {
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

pub const NEXT_DOCUMENT_TS: &str = r#"import Document, { DocumentContext, Html, Head, Main, NextScript } from 'next/document';

class MyDocument extends Document {
  static async getInitialProps(ctx: DocumentContext) {
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

// Other
pub const NEXT_STATIC_PROPS: &str = r#"
export async function getStaticProps(context) {
  return {
    props: {},
  }
}
"#;

pub const NEXT_SSR_PROPS: &str = r#"
export async function getStaticProps(context) {
  return {
    props: {},
  }
}
"#;

pub const NEXT_STATIC_PROPS_TS: &str = r#"
export const getStaticProps: GetStaticProps = async (context) => {
  return {
    props: {},
  }
}
"#;

pub const NEXT_SSR_PROPS_TS: &str = r#"
export const getServerSideProps: GetServerSideProps = async (context) => {
  return {
    props: {},
  }
}
"#;
