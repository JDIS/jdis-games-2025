import {
    createContext,
    type Dispatch,
    type ReactNode,
    type SetStateAction,
    use,
    useEffect,
    useMemo,
    useState,
} from "react";
import useLocalStorage from "~/lib/useLocalStorage";

const WindowContext = createContext<{
    windows: Record<WindowName, boolean>;
    setWindows: Dispatch<SetStateAction<Record<WindowName, boolean>>>;
    z: WindowName[];
    setZ: Dispatch<SetStateAction<WindowName[]>>;
} | null>(null);

export default function WindowProvider(props: { children: ReactNode }) {
    const [loaded, setLoaded] = useState(false);
    const [windows, setWindows] = useLocalStorage<Record<WindowName, boolean>>("windows", {
        "kill-feed": false,
        "player-list": false,
        map: false,
        leaderboard: false,
        "change-team-name": false,
        connection: false,
        "user-map": false,
        documentation: true,
    });
    const [z, setZ] = useLocalStorage<WindowName[]>("windows-z", [
        "kill-feed",
        "player-list",
        "map",
        "leaderboard",
        "change-team-name",
        "connection",
        "user-map",
        "documentation",
    ]);

    useEffect(() => {
        if (typeof windows !== "undefined") setLoaded(true);
    });

    if (!loaded) return props.children;

    return <WindowContext.Provider value={{ windows, setWindows, z, setZ }}>{props.children}</WindowContext.Provider>;
}

export function useWindow(name: WindowName): [boolean, (open: boolean) => void, number, () => void] {
    const ctx = use(WindowContext);
    const setZ = useMemo(
        () => () => {
            ctx?.setZ((prev) => [...prev.filter((w) => w !== name), name]);
        },
        [ctx?.setZ, name],
    );

    if (!ctx) {
        return [false, () => {}, 0, () => {}];
    }

    return [
        ctx.windows[name],
        (open: boolean) => {
            ctx.setWindows((prev) => ({ ...prev, [name]: open }));
        },
        ctx.z.indexOf(name),
        setZ,
    ];
}

export const Windows = {
    "kill-feed": "Kill Feed",
    "player-list": "Liste des joueurs",
    map: "Carte",
    leaderboard: "Leaderboard",
    "change-team-name": "Nom d'équipe",
    connection: "Connexion",
    "user-map": "Carte du joueur",
    documentation: "Documentation",
};

export type WindowName = keyof typeof Windows;
