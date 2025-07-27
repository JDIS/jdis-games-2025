import {
    BookIcon,
    LogInIcon,
    MapIcon,
    MapPinnedIcon,
    PencilLineIcon,
    SkullIcon,
    TrophyIcon,
    UsersIcon,
} from "lucide-react";
import LogoPowerButton from "~/assets/logo.svg";
import { Button } from "~/components/ui/button";
import { useWebSocket } from "./WebSocketProvider";
import { useWindow, type WindowName, Windows } from "./WindowProvider";

export default function NavBar() {
    const { switchServer, url } = useWebSocket();

    return (
        <nav className="scroll-orange flex items-center justify-evenly gap-4 px-2 sm:w-auto sm:flex-col sm:justify-start sm:gap-8 sm:pt-4">
            <div className="flex flex-wrap items-center justify-center gap-2 sm:flex-col">
                <li className="flex gap-4 rounded-xl border-2 border-gray-500 bg-neutral-700 p-3 sm:flex-col">
                    <NavButton name="map" />
                    <NavButton name="user-map" />
                    <NavButton name="player-list" />
                    <NavButton name="kill-feed" />
                </li>
                <li className="flex gap-4 rounded-xl border-2 border-gray-500 bg-neutral-700 p-3 sm:flex-col">
                    <NavButton name="connection" />
                    <NavButton name="change-team-name" />
                    <NavButton name="leaderboard" />
                    <NavButton name="documentation" />
                </li>
            </div>

            <div className="group relative">
                <Button
                    size="icon"
                    className="mr-2 size-8 rounded-full bg-center bg-contain bg-no-repeat shadow-xl ring-4 ring-neutral-700 sm:mr-0 md:size-12"
                    onClick={switchServer}
                >
                    <img src={LogoPowerButton} alt="Power Button" className="size-6 sm:size-7 md:size-10" />
                </Button>
                <div className="-translate-x-1/2 md:-translate-y-1/2 md:-translate-x-0 pointer-events-none absolute bottom-full left-1/2 z-50 mb-2 whitespace-nowrap rounded bg-black px-2 py-1 text-white text-xs opacity-0 transition-opacity group-hover:opacity-100 md:top-1/2 md:bottom-auto md:left-full md:mb-0 md:ml-2 ">
                    {url.includes("playground") ? "Aller en mode ranked" : "Aller en mode playground"}
                </div>
            </div>
        </nav>
    );
}

function NavButton(props: { name: WindowName }) {
    const [open, setOpen] = useWindow(props.name);
    const IconComponent = getIcon(props.name);

    return (
        <div className="group relative">
            <Button
                size="icon"
                onClick={() => setOpen(!open)}
                data-open={open}
                className="size-6 rounded-full ring-blue-400 transition-shadow data-open:text-blue-400 data-open:shadow-blue-400/50 data-open:shadow-lg data-open:ring-2 sm:size-8 md:size-12"
            >
                <IconComponent className="size-4 sm:size-5 md:size-8" />
            </Button>

            <div className="-translate-x-1/2 md:-translate-y-1/2 md:-translate-x-0 pointer-events-none absolute bottom-full left-1/2 z-50 mb-2 whitespace-nowrap rounded bg-black px-2 py-1 text-white text-xs opacity-0 transition-opacity group-hover:opacity-100 md:top-1/2 md:bottom-auto md:left-full md:mb-0 md:ml-2 ">
                {Windows[props.name]}
            </div>
        </div>
    );
}

function getIcon(name: WindowName) {
    switch (name) {
        case "map":
            return MapIcon;
        case "user-map":
            return MapPinnedIcon;
        case "player-list":
            return UsersIcon;
        case "kill-feed":
            return SkullIcon;
        case "connection":
            return LogInIcon;
        case "change-team-name":
            return PencilLineIcon;
        case "leaderboard":
            return TrophyIcon;
        case "documentation":
            return BookIcon;
        default:
            return MapIcon;
    }
}
