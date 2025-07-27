import { createFileRoute } from "@tanstack/react-router";
import BlueScreen from "~/components/BlueScreen";
import FloatingWindow from "~/components/FloatingWindow";
import { GameMapProvider } from "~/components/GameMap";
import ChangeTeamName from "~/components/windows/ChangeTeamName";
import Connection from "~/components/windows/Connection";
import Documentation from "~/components/windows/Documentation";
import KillFeed from "~/components/windows/KillFeed";
import Leaderboard from "~/components/windows/Leaderboard";
import MapWindow from "~/components/windows/MapWindow";
import PlayerList from "~/components/windows/PlayerList";
import UserMap from "~/components/windows/UserMap";

export const Route = createFileRoute("/")({
    component: Home,
});

function Home() {
    return (
        <GameMapProvider>
            <div className="grid overflow-hidden p-2">
                <BlueScreen />

                <main className="relative isolate">
                    <FloatingWindow name="kill-feed">
                        <KillFeed />
                    </FloatingWindow>

                    <FloatingWindow name="player-list">
                        <PlayerList />
                    </FloatingWindow>
                    <FloatingWindow name="map" square>
                        <MapWindow />
                    </FloatingWindow>
                    <FloatingWindow name="leaderboard">
                        <Leaderboard />
                    </FloatingWindow>
                    <FloatingWindow name="change-team-name">
                        <ChangeTeamName />
                    </FloatingWindow>
                    <FloatingWindow name="connection">
                        <Connection />
                    </FloatingWindow>
                    <FloatingWindow name="user-map" square>
                        <UserMap />
                    </FloatingWindow>
                    <FloatingWindow name="documentation">
                        <Documentation />
                    </FloatingWindow>
                </main>
            </div>
        </GameMapProvider>
    );
}
