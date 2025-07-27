import type { ReactNode } from "react";
import { createContext, use, useEffect, useRef, useState } from "react";
import type { Event, GameState, Team } from "~/lib/types";
import useLocalStorage from "~/lib/useLocalStorage";

const GameContext = createContext<{
    scoreboard: Team[];
    gameState: GameState;
    events: Event[];
    broadcast?: string;
    name: string | undefined;
    changeName: (name: string) => void;
} | null>(null);

const WebsocketContext = createContext<{
    token: string | null;
    connect: (token: string | null) => void;
    url: "/ws" | "/ws-playground";
    switchServer: () => void;
    linkFailed: boolean;
} | null>(null);

export default function WebSocketProvider(props: { children: ReactNode }) {
    const websocket = useRef<WebSocket>(undefined);
    const [url, setUrl] = useLocalStorage<"/ws" | "/ws-playground">("server", "/ws");
    const [gameState, setGameState] = useState<GameState>();
    const [scoreboard, setScoreboard] = useState<Team[]>([]);
    const [events, setEvents] = useState<Event[]>([]);
    const [broadcast, setBroadcast] = useState<string>();
    const [token, setToken] = useLocalStorage<string | null>("team-token", null);
    const [name, setName] = useState<string>();
    const [linkFailed, setLinkFailed] = useState<boolean>(false);

    useEffect(() => {
        if (websocket.current && new URL(websocket.current.url).pathname === url) return;
        const ws = new WebSocket(url);
        websocket.current = ws;
        ws.addEventListener("open", () => {
            console.log("WebSocket connected!");
            websocket.current = ws;
            ws.send(
                JSON.stringify({
                    type: "link",
                    clientType: "dashboard",
                    teamId: token,
                }),
            );
        });
        ws.addEventListener("close", () => {
            console.log("WebSocket closed!");
            setGameState(undefined);
            if (websocket.current === ws) {
                websocket.current = undefined;
            }
        });
        ws.addEventListener("error", (error) => {
            console.error("WebSocket error:", error);
        });
        ws.addEventListener("message", (message) => {
            const data = JSON.parse(message.data) as WebsocketMessage;
            switch (data.type) {
                case "gameState":
                    setGameState(data.state);
                    setScoreboard(data.scoreboard?.sort((a, b) => b.score - a.score) ?? []);
                    break;
                case "events":
                    setEvents((prev) => [...data.events.reverse(), ...prev].slice(0, 100));
                    if (data.events.some((e) => e.type === "nuke")) {
                        window.dispatchEvent(new CustomEvent("bluescreen"));
                    }
                    break;
                case "broadcast":
                    setBroadcast(data.message);
                    break;
                case "nameConfirmation":
                    setName(data.name);
                    setLinkFailed(false);
                    break;
                case "linkFailed":
                    setLinkFailed(true);
                    break;
                default:
                    console.warn("Unknown WebSocket message:", data);
            }
        });

        return () => {
            if (new URL(ws.url).pathname === url) {
                ws.close();
            }
        };
    }, [url, token]);

    const connect = (token: string | null) => {
        setToken(token);
        setName(undefined);
        setLinkFailed(false);

        if (!websocket.current) return;
        websocket.current.send(
            JSON.stringify({
                type: "link",
                clientType: "dashboard",
                teamId: token,
            }),
        );
    };

    const changeName = (name: string) => {
        if (!websocket.current) return;
        websocket.current.send(
            JSON.stringify({
                type: "renameTeam",
                name,
            }),
        );
    };

    const switchServer = () => {
        setUrl(url === "/ws" ? "/ws-playground" : "/ws");
        setGameState(undefined);
        setEvents([]);
        setScoreboard([]);
        setLinkFailed(false);
        setName(undefined);
    };

    return (
        <WebsocketContext value={{ token, connect, url, switchServer, linkFailed }}>
            <GameContext
                value={
                    gameState
                        ? {
                              gameState,
                              scoreboard,
                              events,
                              broadcast,
                              name,
                              changeName,
                          }
                        : null
                }
            >
                {props.children}
            </GameContext>
        </WebsocketContext>
    );
}

export function useServerData() {
    const websocket = use(GameContext);
    return (
        websocket ?? {
            scoreboard: undefined,
            gameState: undefined,
            events: [],
            broadcast: undefined,
            name: undefined,
            changeName: undefined,
        }
    );
}

export function useWebSocket() {
    const context = use(WebsocketContext);
    if (!context) throw new Error("WebSocket context not found");
    return context;
}

type WebsocketMessage =
    | {
          type: "gameState";
          scoreboard?: Team[];
          state: GameState;
      }
    | { type: "events"; events: Event[] }
    | { type: "nameConfirmation"; name: string }
    | { type: "linkFailed" }
    | { type: "broadcast"; message: string };
