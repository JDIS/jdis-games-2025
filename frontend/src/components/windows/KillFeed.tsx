import { getTeamColor } from "~/lib/teamColors";
import type { Event } from "~/lib/types";
import { useServerData } from "../WebSocketProvider";

export default function KillFeed() {
    const { events } = useServerData();

    const formatKill = (event: Extract<Event, { type: "kill" }>) => {
        const { killer, victim, weapon } = event;

        if (killer === victim) {
            return (
                <span>
                    <strong style={{ color: getTeamColor(victim) }}>{victim}</strong> s'est <i>auto-éliminé</i>
                </span>
            );
        }
        if (killer === "FireWall") {
            return (
                <span>
                    <strong style={{ color: getTeamColor(victim) }}>{victim}</strong> a été éliminé par le{" "}
                    <span style={{ color: "#ff6a00", fontWeight: "bold" }}>FireWall</span>
                </span>
            );
        }
        return (
            <span>
                <strong style={{ color: getTeamColor(killer) }}>{killer}</strong> a éliminé{" "}
                <strong style={{ color: getTeamColor(victim) }}>{victim}</strong>
                {weapon && (
                    <>
                        {" "}
                        avec <strong>{weapon}</strong>
                    </>
                )}
            </span>
        );
    };

    return (
        <div className="scroll-orange flex flex-col gap-1 overflow-y-auto p-4 text-sm">
            {events.map((event) => {
                const key = JSON.stringify(event);
                switch (event.type) {
                    case "nuke":
                        return (
                            <div
                                key={key}
                                className="animate-pulse rounded bg-blue-700 p-2 text-center font-bold text-lg text-white shadow-lg"
                            >
                                BLUESCREEN :(
                            </div>
                        );
                    case "kill":
                        return (
                            <div key={key} className="rounded bg-black/50 p-1 text-white">
                                {formatKill(event)}
                            </div>
                        );
                    case "gameEnd":
                        return (
                            <div key={key}>
                                <hr className="my-2 border-orange-500 border-t" />
                                <div className="rounded bg-yellow-500 p-1 text-center font-bold text-black">
                                    {event.winner ? (
                                        <>
                                            <div>
                                                <strong
                                                    style={{
                                                        color: getTeamColor(event.winner),
                                                        textShadow: "0 0 2px black",
                                                    }}
                                                >
                                                    {event.winner}
                                                </strong>
                                            </div>
                                            <div>couronné vainqueur de la partie!</div>
                                        </>
                                    ) : (
                                        <div>Partie terminée sans vainqueur!</div>
                                    )}
                                </div>
                            </div>
                        );
                }
            })}
        </div>
    );
}
