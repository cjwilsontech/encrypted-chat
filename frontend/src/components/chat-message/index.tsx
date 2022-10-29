import React from "react";
import * as styles from "./index.module.scss";

export interface IChatMessage {
  text: string;
  timestamp: Date;
}

export function ChatMessage({ message }: { message: IChatMessage }) {
  return (
    <div className={styles.chat}>
      <span>[{message.timestamp.toLocaleTimeString()}]</span>{" "}
      <span>{message.text}</span>
    </div>
  );
}
