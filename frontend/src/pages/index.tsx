import * as React from "react";
import type { HeadFC } from "gatsby";
import { SocketGui } from "../components/socket-gui";
import * as styles from "../styles/index.module.scss";

const IndexPage = () => {
  return (
    <main className={styles.container}>
      <header>
        <h1 className={styles.siteTitle}>Encrypted Chat</h1>
      </header>
      <SocketGui />
    </main>
  );
};

export default IndexPage;

export const Head: HeadFC = () => <title>Encrypted Chat</title>;
