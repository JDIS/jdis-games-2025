import { useServerData } from "~/components/WebSocketProvider";
import { getTeamColor } from "~/lib/teamColors";
import { LoadingWindow } from "../FloatingWindow";
import { useGameMap } from "../GameMap";

export default function PlayerList() {
    const { gameState } = useServerData();
    const { focusOnTeam } = useGameMap();

    if (!gameState) return <LoadingWindow />;

    const players = [...gameState.players].sort((a, b) => {
        const aDead = a.hp <= 0;
        const bDead = b.hp <= 0;

        if (aDead && !bDead) return 1;
        if (!aDead && bDead) return -1;

        const aTotal = a.hp + a.shield;
        const bTotal = b.hp + b.shield;

        return bTotal - aTotal;
    });

    return (
        <div className="scroll-orange flex h-full flex-col gap-3 overflow-y-auto p-2">
            {players.map((player) => {
                const isDead = player.hp <= 0;
                const color = getTeamColor(player.name);
                const nameClass = player.name.length > 16 ? "text-sm" : "text-base";

                return (
                    <div
                        key={player.name}
                        className={`w-full cursor-pointer rounded border border-white/10 px-4 py-3 transition hover:shadow-lg ${
                            isDead ? "opacity-60 grayscale" : ""
                        }`}
                        style={{
                            background: isDead ? "#111" : `linear-gradient(to right, ${color}, ${color})`,
                        }}
                    >
                        <button
                            onClick={() => focusOnTeam(player.name)}
                            className={`block w-full font-bold ${nameClass} cursor-pointer truncate text-left transition hover:underline ${
                                isDead ? "text-gray-400 line-through" : "text-black"
                            }`}
                            type="button"
                        >
                            {player.name}
                        </button>

                        <div className="relative mt-2 h-2 w-full overflow-hidden rounded bg-gray-900">
                            {/* PV (blanc) */}
                            <div
                                className="absolute top-0 left-0 z-0 h-2 rounded transition-all"
                                style={{
                                    width: `${Math.min(100, player.hp)}%`,
                                    backgroundColor: isDead ? "#444" : "#fff",
                                }}
                            />

                            {/* Shield (bleu) */}
                            {player.shield > 0 && (
                                <div
                                    className="player.hp)}%)] absolute top-0 left-[calc(${Math.min(100, z-10 h-2 rounded transition-all"
                                    style={{
                                        width: `${Math.min(100, player.shield)}%`,
                                        backgroundColor: isDead ? "#666" : "#354a65",
                                    }}
                                />
                            )}
                        </div>
                    </div>
                );
            })}
        </div>
    );
}
