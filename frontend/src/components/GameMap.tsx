import {
    createContext,
    type Dispatch,
    type ReactNode,
    type RefObject,
    type SetStateAction,
    use,
    useEffect,
    useMemo,
    useRef,
    useState,
} from "react";
import { getTeamColor } from "~/lib/teamColors";
import { drawGround } from "./canvas/drawingUtils";
import { useServerData } from "./WebSocketProvider";

const GameMapContext = createContext<{
    canvasRef: RefObject<HTMLCanvasElement | null>;
    offset: { x: number; y: number } | string;
    zoom: number;
    setOffset: Dispatch<SetStateAction<{ x: number; y: number } | string>>;
    setZoom: Dispatch<SetStateAction<number>>;
} | null>(null);

export function GameMapProvider(props: { children: ReactNode }) {
    const canvasRef = useRef<HTMLCanvasElement>(null);
    const [offset, setOffset] = useState<{ x: number; y: number } | string>({ x: 0, y: 0 });
    const [zoom, setZoom] = useState(1);

    return (
        <GameMapContext.Provider
            value={{
                canvasRef,
                offset,
                setOffset,
                zoom,
                setZoom,
            }}
        >
            {props.children}
        </GameMapContext.Provider>
    );
}

export function useGameMap() {
    const { gameState } = useServerData();

    const ctx = use(GameMapContext);
    if (!ctx) {
        throw new Error("useGameMap must be used within a GameMapProvider");
    }

    return {
        focusOnTeam: (teamId: string) => {
            const player = gameState?.players.find((p) => p.name === teamId);
            if (!gameState || !player) return;

            ctx.setZoom(3);
            ctx.setOffset(teamId);
        },
    };
}

export default function GameMap() {
    const ctx = use(GameMapContext);
    if (!ctx) {
        throw new Error("GameMap must be used within a GameMapProvider");
    }

    const { gameState } = useServerData();
    const deadPlayersRef = useRef<{ id: string; x: number; y: number; start: number }[]>([]);
    const dragStartRef = useRef<{ x: number; y: number } | null>(null);

    const offset = useMemo(() => {
        if (typeof ctx.offset === "string") {
            const canvas = ctx.canvasRef.current;
            if (!canvas || !gameState) return { x: 0, y: 0 };

            const player = gameState.players.find((p) => p.name === ctx.offset);
            if (!player) return { x: 0, y: 0 };

            const res =
                Math.min(canvas.clientWidth, canvas.clientHeight) /
                Math.max(gameState.ground.width, gameState.ground.height);
            const px = (player.position.x + 0.5) * res;
            const py = (player.position.y + 0.5) * res;

            return {
                x: canvas.clientWidth / 2 - px,
                y: canvas.clientHeight / 2 - py,
            };
        }

        return ctx.offset;
    }, [gameState, ctx.offset, ctx.canvasRef]);

    useEffect(() => {
        const canvas = ctx.canvasRef.current;
        const container = canvas?.parentElement;
        const canvasContext = canvas?.getContext("2d");
        if (!canvasContext || !canvas || !container || !gameState) return;

        const canvasWidth = container.clientWidth;
        const canvasHeight = container.clientHeight;
        if (canvas.width !== canvasWidth || canvas.height !== canvasHeight) {
            canvas.width = canvasWidth;
            canvas.height = canvasHeight;
        }
        const res = Math.min(canvasWidth, canvasHeight) / Math.max(gameState.ground.width, gameState.ground.height);

        const isDead = (player: (typeof gameState.players)[number]) => player.hp <= 0;
        const now = performance.now();

        for (const player of gameState.players) {
            if (isDead(player) && !deadPlayersRef.current.some((d) => d.id === player.name)) {
                deadPlayersRef.current.push({
                    id: player.name,
                    x: player.position.x,
                    y: player.position.y,
                    start: now,
                });
            }
        }

        deadPlayersRef.current = deadPlayersRef.current.filter((d) => now - d.start < 1200);

        let animationFrameId: number;

        const render = () => {
            // Clear and reset transform
            canvasContext.setTransform(1, 0, 0, 1, 0, 0);
            canvasContext.clearRect(0, 0, canvasWidth, canvasHeight);

            // Apply the proper transform sequence: translate to center, scale, then translate by offset
            canvasContext.translate(canvasWidth / 2, canvasHeight / 2);
            canvasContext.scale(ctx.zoom, ctx.zoom);
            canvasContext.translate(-canvasWidth / 2 + offset.x, -canvasHeight / 2 + offset.y);

            // Draw
            drawGround(canvasContext, { ...gameState.ground, offset: { x: 0, y: 0 } }, res, gameState.objects);

            for (const player of gameState.players) {
                if (isDead(player)) continue;

                const px = (player.position.x + 0.5) * res;
                const py = (player.position.y + 0.5) * res;

                const baseRadius = res / ctx.zoom;
                const radius = Math.max(baseRadius, 3);
                const color = getTeamColor(player.name);

                canvasContext.save();
                canvasContext.shadowColor = color;
                canvasContext.shadowBlur = radius * 1.5;

                canvasContext.beginPath();
                canvasContext.arc(px, py, radius, 0, Math.PI * 2);
                canvasContext.fillStyle = color;
                canvasContext.fill();

                const highlightRadius = radius * 0.35;
                canvasContext.beginPath();
                canvasContext.arc(px - radius * 0.4, py - radius * 0.4, highlightRadius, 0, Math.PI * 2);
                canvasContext.fillStyle = "rgba(255,255,255,0.5)";
                canvasContext.arc(px - radius * 0.4, py - radius * 0.4, highlightRadius, 0, Math.PI * 2);
                canvasContext.fillStyle = "rgba(255,255,255,0.5)";
                canvasContext.fill();

                canvasContext.font = `${4 + radius * 1.4}px monospace`;
                canvasContext.textAlign = "center";
                canvasContext.textBaseline = "top";
                canvasContext.fillStyle = "white";
                canvasContext.strokeStyle = "black";
                canvasContext.lineWidth = radius * 0.5;
                canvasContext.strokeText(player.name, px, py + radius + 2);
                canvasContext.fillText(player.name, px, py + radius + 2);
                canvasContext.restore();
            }

            for (const obj of gameState.projectiles) {
                const { x, y } = obj.position;
                const px = (x + 0.5) * res;
                const py = (y + 0.5) * res;

                canvasContext.save();
                canvasContext.beginPath();
                canvasContext.arc(px, py, res * 0.2, 0, Math.PI * 2);
                canvasContext.fillStyle = "#1e56a9ff";
                canvasContext.shadowColor = "#1e56a9ff";
                canvasContext.shadowBlur = 10;
                canvasContext.fill();
                canvasContext.restore();
            }

            for (const dead of deadPlayersRef.current) {
                const px = (dead.x + 0.5) * res;
                const py = (dead.y + 0.5) * res;
                const age = now - dead.start;
                const opacity = 1 - age / 1200;

                canvasContext.save();
                canvasContext.globalAlpha = opacity;

                canvasContext.font = `${res * 0.8}px serif`;
                canvasContext.textAlign = "center";
                canvasContext.textBaseline = "middle";
                canvasContext.fillStyle = "white";
                canvasContext.strokeStyle = "black";
                canvasContext.lineWidth = 3;
                canvasContext.strokeText("☠", px, py);
                canvasContext.fillText("☠", px, py);
                canvasContext.restore();
            }

            animationFrameId = window.requestAnimationFrame(render);
        };

        render();
        return () => window.cancelAnimationFrame(animationFrameId);
    }, [gameState, offset, ctx.zoom, ctx.canvasRef]);

    // Drag + zoom
    useEffect(() => {
        const canvas = ctx.canvasRef.current;
        if (!canvas) return;

        const controller = new AbortController();
        const { signal } = controller;

        window.addEventListener(
            "mouseup",
            () => {
                dragStartRef.current = null;
            },
            { signal },
        );

        window.addEventListener(
            "mousemove",
            (e: MouseEvent) => {
                if (dragStartRef.current) {
                    ctx.setOffset({
                        x: e.clientX / ctx.zoom - dragStartRef.current.x,
                        y: e.clientY / ctx.zoom - dragStartRef.current.y,
                    });
                }
            },
            { signal },
        );

        return () => controller.abort();
    }, [ctx.zoom, ctx.canvasRef, ctx.setOffset]);

    return (
        <div className="grid size-full min-h-0 overflow-hidden">
            <canvas
                className="size-full cursor-grab active:cursor-grabbing"
                ref={ctx.canvasRef}
                onMouseDown={(e) => {
                    dragStartRef.current = {
                        x: e.clientX / ctx.zoom - offset.x,
                        y: e.clientY / ctx.zoom - offset.y,
                    };
                }}
                onWheel={(e) => {
                    e.preventDefault();
                    ctx.setZoom((zoom) => Math.max(0.5, Math.min(5, zoom - e.deltaY * 0.002)));
                }}
            />

            <div className="absolute top-2 right-2 z-10">
                <button
                    className="cursor-pointer rounded bg-orange-600 px-2 py-1 text-white text-xs hover:bg-orange-700"
                    onClick={() => {
                        ctx.setZoom(1);
                        ctx.setOffset({ x: 0, y: 0 });
                    }}
                    type="button"
                >
                    Réinitialiser la vue
                </button>
            </div>
        </div>
    );
}
