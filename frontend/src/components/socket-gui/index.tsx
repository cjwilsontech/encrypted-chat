import React, { useEffect, useState } from "react";
import { IChatMessage } from "../chat-message";
import { ChatBox } from "../chatbox";
import * as styles from "./index.module.scss";

interface IChatMessageDto {
  client_id: number;
  message: string;
}

export function SocketGui() {
  const [socket, setSocket] = useState<WebSocket | null>(null);
  const [message, setMessage] = useState("");
  const [chatLog, setChatLog] = useState<IChatMessage[]>([]);

  const connectSocket = () => {
    if (socket) {
      socket.close();
    }

    const { location } = window;
    const proto = location.protocol.startsWith("https") ? "wss" : "ws";
    const wsUrl = `${proto}://${location.host}/ws`;
    const newSocket = new WebSocket(wsUrl);

    newSocket.onopen = () => {
      console.log("connected");
    };

    newSocket.onclose = () => {
      console.log("disconnected");
      setSocket(null);
    };

    newSocket.onmessage = (e) => {
      switch (e.type) {
        case "message":
          {
            const messageDto: IChatMessageDto = JSON.parse(e.data);
            handleChatMessageRecieved({
              client_id: messageDto.client_id,
              text: messageDto.message,
              timestamp: new Date(),
            });
          }
          break;
        default:
          console.error(`Unknown message type: ${e.type}`);
      }
    };

    setSocket(newSocket);
  };

  const handleChatMessageRecieved = (chat: IChatMessage) => {
    console.log(`[${chat.timestamp.toLocaleTimeString()}] ${chat.text}`);
    setChatLog((chats) => [...chats, chat]);
  };

  const handleChatSubmitted = () => {
    if (socket) {
      socket?.send(message);
      setChatLog((chats) => [
        ...chats,
        { client_id: -1, text: message, timestamp: new Date() },
      ]);
      setMessage("");
    }
  };

  useEffect(() => {
    if (typeof window === "object") {
      connectSocket();
    }
  }, []);

  return (
    <>
      <p>
        <span>Status: {socket?.OPEN ? "Connected" : "Disconnected"}</span>&nbsp;
        <button onClick={connectSocket} disabled={!!socket}>
          Reconnect
        </button>
      </p>
      <div className={styles.chatContainer}>
        <ChatBox chatLog={chatLog} className={styles.chatLog} />
        <div className={styles.inputControls}>
          <input
            type="text"
            onChange={(e) => setMessage(e.currentTarget.value)}
            onKeyDown={({ key }) => {
              if (key === "Enter") {
                handleChatSubmitted();
              }
            }}
            value={message}
            autoFocus={true}
          />
          <button onClick={handleChatSubmitted} disabled={!socket}>
            Send
          </button>
        </div>
      </div>
    </>
  );
}
