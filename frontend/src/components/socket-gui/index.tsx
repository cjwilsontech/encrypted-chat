import React, { useEffect, useState } from "react";

interface IChatMessageRecievedDto {
  text: string;
  timestamp: Date;
}

export function SocketGui() {
  const [socket, setSocket] = useState<WebSocket | null>(null);
  const [message, setMessage] = useState("");

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
          handleChatMessageRecieved({ text: e.data, timestamp: new Date() });
          break;
        default:
          console.error(`Unknown message type: ${e.type}`);
      }
    };

    setSocket(newSocket);
  };

  const handleChatMessageRecieved = (chat: IChatMessageRecievedDto) => {
    console.log(`[${chat.timestamp.toLocaleTimeString()}] ${chat.text}`);
  };

  useEffect(() => {
    if (typeof window === "object") {
      connectSocket();
    }
  }, []);

  return (
    <div>
      <h3>Socket</h3>
      <p>
        <span>Status: {socket?.OPEN ? "Connected" : "Disconnected"}</span>&nbsp;
        <button onClick={connectSocket} disabled={!!socket}>
          Reconnect
        </button>
      </p>
      <div>
        <input
          type="text"
          onChange={(e) => setMessage(e.currentTarget.value)}
        />
        <button
          onClick={() => {
            socket?.send(message);
          }}
          disabled={!socket}
        >
          Send
        </button>
      </div>
    </div>
  );
}
