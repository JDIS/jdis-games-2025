/// <reference types="vite/client" />
import { createRootRoute, HeadContent, Outlet, Scripts } from "@tanstack/react-router";
import { BellIcon } from "lucide-react";
import { useCallback, useEffect, useRef, useState } from "react";
import logo from "~/assets/logo.png";
import Navbar from "~/components/NavBar";
import BroadcastHistory from "~/components/ui/BroadcastHistory";
import BroadcastMessage from "~/components/ui/BroadcastMessage";
import UnreadNotifier from "~/components/ui/UnreadNotifier";
import WebSocketProvider from "~/components/WebSocketProvider";
import WindowProvider from "~/components/WindowProvider";
import appCss from "~/styles.css?url";

export const Route = createRootRoute({
    head: () => ({
        meta: [
            {
                charSet: "utf-8",
            },
            {
                name: "viewport",
                content: "width=device-width, initial-scale=1",
            },
            {
                title: "FireWall - JDIS Games 2025",
            },
            {
                name: "description",
                content: "Frontend du jeu de FireWall - JDIS Games 2025",
            },
        ],
        links: [
            { rel: "stylesheet", href: appCss },
            { rel: "icon", href: logo },
        ],
    }),
    component: RootComponent,
});

function RootComponent() {
    const [showHistory, setShowHistory] = useState(false);
    const [unreadCount, setUnreadCount] = useState(0);

    const justClosedHistory = useRef(false);

    // Reset si on ouvre l'historique
    useEffect(() => {
        if (showHistory) {
            setUnreadCount(0);
            justClosedHistory.current = false;
        }
    }, [showHistory]);

    const handleNewMessage = useCallback(() => {
        if (justClosedHistory.current) {
            justClosedHistory.current = false;
            return;
        }

        setUnreadCount((c) => c + 1);
    }, []);

    return (
        <html lang="en">
            <head>
                <HeadContent />
            </head>

            <body className="h-screen overflow-hidden bg-background text-foreground">
                <div id="tooltip-root" />

                <WebSocketProvider>
                    <WindowProvider>
                        <div
                            className="flex h-screen w-screen flex-col-reverse gap-3 overflow-hidden p-3 sm:flex-row"
                            style={{
                                background: "radial-gradient(circle at center, #D26111, #CC6011)",
                            }}
                        >
                            <Navbar />

                            <button
                                onClick={() => {
                                    setShowHistory((prev) => {
                                        const next = !prev;
                                        if (next) {
                                            setUnreadCount(0);
                                        } else {
                                            justClosedHistory.current = true;
                                        }
                                        return next;
                                    });
                                }}
                                className="fixed top-4 right-4 z-50"
                                type="button"
                            >
                                <div className="relative flex h-12 w-12 cursor-pointer items-center justify-center rounded-full border-4 border-[#F8EFE4] bg-[#EB661B] font-bold text-2xl text-[#F8EFE4] shadow-xl transition-transform duration-200 ease-out hover:scale-110 hover:bg-[#ff7a2a]">
                                    <BellIcon className="size-6 md:size-8" />
                                    {unreadCount > 0 && (
                                        <span className="-top-2 -right-2 absolute flex h-5 w-5 items-center justify-center rounded-full border-2 border-black bg-red-600 font-bold text-white text-xs shadow">
                                            {unreadCount}
                                        </span>
                                    )}
                                </div>
                            </button>

                            <div className="relative grow">
                                <div className="absolute inset-0 grid overflow-hidden rounded-md bg-background">
                                    <BroadcastMessage />
                                    <BroadcastHistory visible={showHistory} />
                                    <UnreadNotifier isVisible={showHistory} onNewMessage={handleNewMessage} />

                                    <Outlet />
                                </div>
                            </div>
                        </div>
                    </WindowProvider>
                </WebSocketProvider>

                <Scripts />
            </body>
        </html>
    );
}
