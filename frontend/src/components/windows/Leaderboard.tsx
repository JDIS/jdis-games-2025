import { useServerData, useWebSocket } from "~/components/WebSocketProvider";
import { getTeamColor } from "~/lib/teamColors";
import { LoadingWindow } from "../FloatingWindow";
import { useGameMap } from "../GameMap";

export default function Leaderboard() {
    const { url } = useWebSocket();
    const { scoreboard } = useServerData();
    const { focusOnTeam } = useGameMap();

    if (url === "/ws-playground") {
        return (
            <div className="flex items-center justify-center">
                <span>Le classement n'est pas disponible sur le serveur de test.</span>
            </div>
        );
    }

    if (!scoreboard) return <LoadingWindow />;

    return (
        <div className="scroll-orange space-y-2 overflow-y-auto p-2">
            {scoreboard.map((team, rank) => {
                const isTop3 = rank < 3;
                const color = getTeamColor(team.name);
                const containerStyle = isTop3 ? getTop3Style(rank) : "border border-white/10 px-4 py-2";
                const nameClass = team.name.length > 16 ? "text-sm" : "text-lg";

                return (
                    <div
                        key={team.name}
                        className={`flex w-full items-center justify-between px-4 py-2 font-mono ${containerStyle}`}
                    >
                        <span className="font-bold text-lg" style={{ color: isTop3 ? "black" : color }}>
                            {rank + 1}
                        </span>
                        <button
                            onClick={() => focusOnTeam(team.name)}
                            className={`font-bold ${nameClass} max-w-[60%] cursor-pointer truncate text-left transition hover:scale-105 hover:underline active:scale-95`}
                            style={{ color: isTop3 ? "black" : color }}
                            type="button"
                        >
                            {team.name}
                        </button>
                        <span className="text-lg" style={{ color: isTop3 ? "black" : color }}>
                            {team.score}
                        </span>
                    </div>
                );
            })}
        </div>
    );
}

function getTop3Style(rank: number): string {
    switch (rank) {
        case 0:
            return "bg-[#D3AF37] text-black rounded-lg shadow-md";
        case 1:
            return "bg-[#C0C0C0] text-black rounded-lg shadow-md";
        case 2:
            return "bg-[#CE8946] text-black rounded-lg shadow-md";
        default:
            return "";
    }
}
