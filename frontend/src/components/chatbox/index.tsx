import React, { useEffect, useRef } from "react";
import { ChatMessage, IChatMessage } from "../chat-message";
import classNames from "classnames";
import * as styles from "./index.module.scss";

export function ChatBox({
  chatLog,
  className,
}: {
  chatLog: IChatMessage[];
  className?: string;
}) {
  const bottomRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    bottomRef.current?.scrollIntoView();
  }, [bottomRef, chatLog]);

  return (
    <div className={classNames(styles.chatList, className)}>
      {chatLog.map((chat, index) => (
        <ChatMessage key={`chat-${index}`} message={chat} />
      ))}
      <div ref={bottomRef} />
    </div>
  );
}
