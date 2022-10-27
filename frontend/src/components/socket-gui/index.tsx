import React, { useEffect, useState } from "react";

export function SocketGui() {
  const [socket, setSocket] = useState<WebSocket | null>(null);
  const [message, setMessage] = useState("");

  useEffect(() => {
    if (typeof window === "object") {
      const { location } = window;
      const proto = location.protocol.startsWith("https") ? "wss" : "ws";
      const wsUrl = `${proto}://${location.host}/ws`;
      const socket = new WebSocket(wsUrl);

      socket.onopen = () => {
        console.log("connected");
      };

      socket.onclose = () => {
        console.log("disconnected");
        setSocket(null);
      };

      setSocket(socket);
    }
  }, []);

  return (
    <div>
      <h3>Socket</h3>
      <p>Status: {socket?.OPEN ? "Connected" : "Disconnected"}</p>
      <div>
        <input
          type="text"
          onChange={(e) => setMessage(e.currentTarget.value)}
        />
        <button
          onClick={() => {
            socket?.send(message);
          }}
        >
          Send
        </button>
      </div>
    </div>
  );
}
