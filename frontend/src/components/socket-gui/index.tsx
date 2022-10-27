import React, { useEffect, useState } from "react";

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

    setSocket(newSocket);
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
