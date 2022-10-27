import * as React from "react";
import type { HeadFC } from "gatsby";
import { SocketGui } from "../components/socket-gui";

const IndexPage = () => {
  return (
    <main>
      <h1>Encrypted Chat</h1>
      <SocketGui />
    </main>
  );
};

export default IndexPage;

export const Head: HeadFC = () => <title>Encrypted Chat</title>;
