import { useEffect, useState } from "react";
import { useServerData } from "../WebSocketProvider";

interface MessageEntry {
    text: string;
    timestamp: string;
}

export default function BroadcastHistory({ visible }: { visible: boolean }) {
    const { broadcast } = useServerData();
    const [history, setHistory] = useState<MessageEntry[]>([]);

    useEffect(() => {
        const saved = localStorage.getItem("broadcastHistory");
        if (saved) {
            try {
                setHistory(JSON.parse(saved));
            } catch {
                console.warn("Erreur de parsing localStorage");
            }
        }
    }, []);

    useEffect(() => {
        if (!broadcast) return;

        const timestamp = new Date().toLocaleTimeString();
        const newEntry = { text: broadcast.toUpperCase(), timestamp };

        setHistory((prev) => {
            const updated = [newEntry, ...prev];
            localStorage.setItem("broadcastHistory", JSON.stringify(updated));
            return updated;
        });
    }, [broadcast]);

    if (!visible) return null;

    return (
        <div className="fixed top-3 right-3 z-40 max-h-[70vh] w-[400px] overflow-hidden rounded border-4 border-orange-400 bg-black/80 p-4 text-white shadow-lg">
            <h2 className="mb-2 font-bold text-lg text-orange-300">Historique des événements</h2>
            <ul className="scroll-orange max-h-[60vh] space-y-2 overflow-y-auto pr-2">
                {history.map((item, idx) => (
                    <li key={idx} className="rounded bg-orange-800 p-2 text-sm">
                        <div className="text-orange-100 text-xs">{item.timestamp}</div>
                        <div className="break-words font-mono">{item.text}</div>
                    </li>
                ))}
            </ul>
        </div>
    );
}
