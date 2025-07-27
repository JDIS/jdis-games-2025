import { useServerData } from "~/components/WebSocketProvider";
import { LoadingWindow } from "../FloatingWindow";
import PlayerMap from "../PlayerMap";

export default function UserMap() {
    const { name, gameState } = useServerData();
    if (!gameState) return <LoadingWindow />;
    if (!name) {
        return (
            <div className="flex items-center justify-center">
                <span>Connectez vous pour suivre votre bot.</span>
            </div>
        );
    }

    return <PlayerMap />;
}
