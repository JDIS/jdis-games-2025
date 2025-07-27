import { useEffect } from "react";
import { useServerData } from "../WebSocketProvider";

interface Props {
    isVisible: boolean;
    onNewMessage: () => void;
}

export default function UnreadNotifier({ isVisible, onNewMessage }: Props) {
    const { broadcast } = useServerData();

    useEffect(() => {
        if (!broadcast) return;
        if (!isVisible) {
            onNewMessage();
        }
    }, [broadcast, isVisible, onNewMessage]);

    return null;
}
