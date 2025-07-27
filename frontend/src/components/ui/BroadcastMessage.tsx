import { useEffect, useState } from "react";
import { useServerData } from "../WebSocketProvider";

export default function BroadcastMessage() {
    const { broadcast } = useServerData();
    const [message, setMessage] = useState<string | null>(null);
    const [visible, setVisible] = useState(false);

    useEffect(() => {
        if (!broadcast) return;

        setMessage(broadcast.toUpperCase());
        setVisible(false);

        const showTimeout = setTimeout(() => setVisible(true), 10);
        const hideTimeout = setTimeout(() => setVisible(false), 15000);

        return () => {
            clearTimeout(showTimeout);
            clearTimeout(hideTimeout);
        };
    }, [broadcast]);

    if (!message) return null;

    return (
        <div
            className={`-translate-x-1/2 fixed top-4 left-1/2 z-40 transition-opacity duration-1000 ${
                visible ? "pointer-events-auto opacity-100" : "pointer-events-none opacity-0"
            }`}
        >
            <div
                className="relative rounded border-4 px-14 py-4 text-center font-bold shadow-xl"
                style={{
                    backgroundColor: "#EB661B",
                    borderColor: "#F8EFE4",
                    color: "#F8EFE4",
                    fontFamily: "Chilada, sans-serif",
                    maxWidth: "600px",
                    wordWrap: "break-word",
                    whiteSpace: "normal",
                }}
            >
                <img
                    src="/broadcast/fire.png"
                    alt="Flamme"
                    className="-left-8 -translate-y-1/2 absolute top-1/2 h-20 w-auto"
                />
                <img
                    src="/broadcast/virus.png"
                    alt="Virus"
                    className="-right-8 -translate-y-1/2 absolute top-1/2 h-20 w-auto"
                />
                <span className="block break-words px-4">{message}</span>
            </div>
        </div>
    );
}
