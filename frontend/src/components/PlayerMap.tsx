import { useEffect, useRef } from "react";
import { getTeamColor } from "~/lib/teamColors";
import { drawGround } from "./canvas/drawingUtils";
import { useServerData } from "./WebSocketProvider";

export default function PlayerMap() {
    const containerRef = useRef<HTMLDivElement>(null);
    const canvasRef = useRef<HTMLCanvasElement>(null);
    const { gameState, name } = useServerData();
    const deadPlayersRef = useRef<{ id: string; x: number; y: number; start: number }[]>([]);

    const player = gameState?.players.find((p) => p.name === name);

    useEffect(() => {
        const container = containerRef.current;
        const canvas = canvasRef.current;
        const ctx = canvas?.getContext("2d");
        if (!ctx || !canvas || !container || !gameState) return;

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
            if (!gameState) return;
            if (!player) return;

            const size = Math.min(container.clientWidth, container.clientHeight);
            if (canvas.width !== size || canvas.height !== size) {
                canvas.width = size;
                canvas.height = size;
            }

            const res = canvas.clientWidth / 7; // 7x7 cases
            const offsetX = canvas.width / 2 - (player.position.x + 0.5) * res;
            const offsetY = canvas.height / 2 - (player.position.y + 0.5) * res;

            ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);

            const xMin = Math.max(0, player.position.x - 3);
            const xMax = Math.min(gameState.ground.width - 1, player.position.x + 3);
            const yMin = Math.max(0, player.position.y - 3);
            const yMax = Math.min(gameState.ground.height - 1, player.position.y + 3);

            drawGround(ctx, { ...gameState.ground, offset: { x: offsetX, y: offsetY } }, res, gameState.objects);

            for (const p of gameState.players) {
                if (p.hp <= 0 || p.name === player.name) continue;
                if (p.position.x < xMin || p.position.x > xMax || p.position.y < yMin || p.position.y > yMax) continue;

                const px = (p.position.x + 0.5) * res + offsetX;
                const py = (p.position.y + 0.5) * res + offsetY;

                const baseRadius = res * 0.4;
                const radius = Math.max(baseRadius, 4);
                const color = getTeamColor(p.name);

                ctx.save();
                ctx.shadowColor = color;
                ctx.shadowBlur = radius * 1.5;
                ctx.beginPath();
                ctx.arc(px, py, radius, 0, Math.PI * 2);
                ctx.fillStyle = color;
                ctx.fill();
                ctx.restore();
            }

            for (const proj of gameState.projectiles) {
                const { x, y } = proj.position;
                if (x < xMin || x > xMax || y < yMin || y > yMax) continue;

                const px = (x + 0.5) * res + offsetX;
                const py = (y + 0.5) * res + offsetY;

                ctx.save();
                ctx.beginPath();
                ctx.arc(px, py, res * 0.2, 0, Math.PI * 2);
                ctx.fillStyle = "#1e56a9ff";
                ctx.shadowColor = "#1e56a9ff";
                ctx.shadowBlur = 10;
                ctx.fill();
                ctx.restore();
            }

            if (player.hp > 0) {
                const px = (player.position.x + 0.5) * res + offsetX;
                const py = (player.position.y + 0.5) * res + offsetY;

                const baseRadius = res * 0.4;
                const radius = Math.max(baseRadius, 4);
                const color = getTeamColor(player.name);

                // Cercle
                ctx.save();
                ctx.shadowColor = color;
                ctx.shadowBlur = radius * 1.5;
                ctx.beginPath();
                ctx.arc(px, py, radius, 0, Math.PI * 2);
                ctx.fillStyle = color;
                ctx.fill();
                ctx.restore();

                // Halo dynamique
                const time = performance.now();
                const pulse = 0.5 + 0.5 * Math.sin(time / 300);
                const haloRadius = radius + 4 + 2 * pulse;

                ctx.beginPath();
                ctx.arc(px, py, haloRadius, 0, 2 * Math.PI);
                ctx.strokeStyle = color;
                ctx.lineWidth = 2 + pulse;
                ctx.stroke();

                // Nom
                ctx.font = `${Math.max(12, res * 0.25)}px sans-serif`;
                ctx.fillStyle = "white";
                ctx.textAlign = "center";
                ctx.textBaseline = "bottom";
                ctx.fillText(player.name, px, py - radius - 4);
            }

            animationFrameId = requestAnimationFrame(render);
        };

        render();
        return () => window.cancelAnimationFrame(animationFrameId);
    }, [gameState, player]);

    return (
        <div className="relative size-full" ref={containerRef}>
            <canvas className="mx-auto cursor-grab active:cursor-grabbing" ref={canvasRef} />

            {player && (
                <>
                    <div className="absolute top-0 left-0 z-10 flex w-full items-center justify-between whitespace-nowrap bg-black/50 px-4 py-2 font-bold text-sm text-white">
                        <div className="text-left">{player.name}</div>
                        <div className="flex gap-8 text-right">
                            <div>Score: {player.score}</div>
                        </div>
                    </div>

                    <div className="absolute bottom-0 left-0 z-10 flex w-full flex-wrap justify-center gap-4 whitespace-nowrap bg-black/50 px-4 py-2 text-center text-sm text-white">
                        <div>PV: {player.hp}</div>
                        <div>Bouclier: {player.shield}</div>
                        <div>Position: {player.position ? `(${player.position.x}, ${player.position.y})` : "?"}</div>
                        <div className="scrollbar-thin max-w-[40vw] overflow-x-auto whitespace-nowrap">
                            Inventaire:{" "}
                            {player.inventory.length > 0
                                ? player.inventory.map((item) => `${item.name} (${item.quantity ?? "∞"})`).join(", ")
                                : "vide"}
                        </div>
                    </div>
                </>
            )}
        </div>
    );
}
