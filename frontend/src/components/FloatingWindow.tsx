import { type ReactNode, useEffect, useState } from "react";
import { type DraggableData, Rnd } from "react-rnd";
import useLocalStorage from "~/lib/useLocalStorage";
import { useWindow, type WindowName, Windows } from "./WindowProvider";

export default function FloatingWindow(props: {
    name: WindowName;
    children: ReactNode;
    square?: boolean;
    toolbar?: ReactNode;
}) {
    const [isOpen, setIsOpen, z, setForeground] = useWindow(props.name);
    const [position, setPosition] = useLocalStorage<{ x: number; y: number }>(`position-${props.name}`, {
        x: 100,
        y: 100,
    });
    const [size, setSize] = useLocalStorage(`size-${props.name}`, {
        width: 400,
        height: 428,
    });
    const [isMaximized, setMaximized] = useState(false);
    const [snapPreview, setSnapPreview] = useState<
        null | "fullscreen" | { x: number; y: number; width: number; height: number }
    >(null);

    useEffect(() => {
        const handleResize = () => {
            const maxX = window.innerWidth - size.width;
            const maxY = window.innerHeight - size.height;

            const clampedX = Math.min(position.x, maxX);
            const clampedY = Math.min(position.y, maxY);

            if (clampedX !== position.x || clampedY !== position.y) {
                // Si la fenêtre dépasse maintenant l'écran, on la remet dans les limites
                setPosition({ x: clampedX, y: clampedY });
            }
        };

        window.addEventListener("resize", handleResize);
        return () => window.removeEventListener("resize", handleResize);
    }, [
        size,
        position, // Si la fenêtre dépasse maintenant l'écran, on la remet dans les limites
        setPosition,
    ]);

    useEffect(() => {
        if (!isOpen) return;
        setForeground();
    }, [isOpen, setForeground]);

    if (!isOpen) return null;

    return (
        <>
            {snapPreview && (
                <div
                    className="pointer-events-none absolute z-9999999999 rounded border-4 border-orange-400 border-dashed bg-orange-500/20"
                    style={
                        snapPreview === "fullscreen"
                            ? { inset: 0 }
                            : {
                                  left: snapPreview.x,
                                  top: snapPreview.y,
                                  width: snapPreview.width,
                                  height: snapPreview.height,
                              }
                    }
                />
            )}

            <Rnd
                size={isMaximized ? { width: "100%", height: "100%" } : size}
                position={isMaximized ? { x: 0, y: 0 } : position}
                minWidth="300px"
                minHeight="200px"
                lockAspectRatio={props.square}
                style={{ zIndex: isMaximized ? 999999999 : z }}
                onMouseDown={() => setForeground()}
                onDragStart={(e, d) => {
                    if (isMaximized) {
                        setMaximized(false);

                        const x =
                            "clientX" in e ? e.clientX - d.node.getBoundingClientRect().left - size.width / 2 : d.x;
                        setPosition({ x, y: d.y });

                        setSize({
                            width: 500,
                            height: props.square ? 500 : 300,
                        });
                    }
                }}
                onDrag={(e, d) => {
                    const snap = handleSnapPreview(e as MouseEvent, d);
                    if (snap !== snapPreview) {
                        setSnapPreview(snap);
                    }
                }}
                onDragStop={(_e, d) => {
                    if (snapPreview === "fullscreen") {
                        setMaximized(true);
                    } else if (snapPreview) {
                        setPosition({ x: snapPreview.x, y: snapPreview.y });

                        if (props.square) {
                            const size = Math.min(snapPreview.width, snapPreview.height);
                            setSize({ width: size, height: size });
                        } else {
                            setSize({ width: snapPreview.width, height: snapPreview.height });
                        }
                    } else {
                        setPosition({ x: d.x, y: Math.max(d.y, 0) });
                    }
                    setSnapPreview(null);
                }}
                onResizeStop={(_e, _dir, ref, _delta, pos) => {
                    setSize({ width: ref.offsetWidth, height: ref.offsetHeight });
                    setPosition({ x: pos.x, y: Math.max(pos.y, 0) });
                }}
                dragHandleClassName="title-bar"
                enableResizing={!isMaximized}
                className="rounded bg-orange-500 shadow-md"
            >
                <div className="flex h-full flex-col overflow-hidden">
                    <div
                        className="title-bar flex cursor-move select-none items-center p-2 pr-10"
                        onDoubleClick={() => setMaximized(true)}
                    >
                        <div className="flex w-10 gap-1">
                            <button
                                className="size-4 cursor-pointer rounded-full bg-red-700"
                                onClick={() => setIsOpen(false)}
                                onMouseDown={(e) => e.stopPropagation()}
                                type="button"
                            />
                            <button
                                className="size-4 cursor-pointer rounded-full bg-green-700"
                                onClick={() => {
                                    if (isMaximized) {
                                        setMaximized(false);
                                        setForeground();
                                    } else {
                                        setMaximized(true);
                                    }
                                }}
                                onMouseDown={(e) => e.stopPropagation()}
                                type="button"
                            />
                        </div>

                        <h1 className="grow text-center font-bold text-sm text-white uppercase">
                            {Windows[props.name]}
                        </h1>
                    </div>

                    <div className="scroll-orange m-2 mt-0 flex grow flex-col overflow-x-auto overflow-y-auto rounded-b border-2 border-orange-500 bg-black text-white">
                        {props.toolbar && <div className="mb-2 flex justify-end">{props.toolbar}</div>}
                        <div className="grid grow overflow-hidden">{props.children}</div>
                    </div>
                </div>
            </Rnd>
        </>
    );
}

export function LoadingWindow() {
    return (
        <div className="flex items-center justify-center">
            <span>Booting up...</span>
        </div>
    );
}

function handleSnapPreview(e: MouseEvent, d: DraggableData) {
    const containerRect = d.node.parentElement?.getBoundingClientRect();
    if (!containerRect) return null;

    const mousePosInContainer = {
        x: e.clientX - containerRect.x,
        y: e.clientY - containerRect.y,
    };

    const x = mousePosInContainer.x < 20 ? -1 : mousePosInContainer.x >= containerRect.width - 20 ? 1 : 0;
    const y = mousePosInContainer.y < 20 ? -1 : mousePosInContainer.y >= containerRect.height - 20 ? 1 : 0;

    if (y === -1) {
        if (mousePosInContainer.x < containerRect.width / 4) {
            return {
                x: 0,
                y: 0,
                width: containerRect.width / 2 - 8,
                height: containerRect.height / 2 - 8,
            };
        }
        if (mousePosInContainer.x > (containerRect.width * 3) / 4) {
            return {
                x: containerRect.width / 2 + 8,
                y: 0,
                width: containerRect.width / 2 - 8,
                height: containerRect.height / 2 - 8,
            };
        }

        return "fullscreen";
    }

    if (y === 1) {
        if (mousePosInContainer.x < containerRect.width / 2) {
            return {
                x: 0,
                y: containerRect.height / 2 + 8,
                width: containerRect.width / 2 - 8,
                height: containerRect.height / 2 - 8,
            };
        }

        return {
            x: containerRect.width / 2 + 8,
            y: containerRect.height / 2 + 8,
            width: containerRect.width / 2 - 8,
            height: containerRect.height / 2 - 8,
        };
    }

    if (x === -1) {
        if (mousePosInContainer.y < containerRect.height / 4) {
            return {
                x: 0,
                y: 0,
                width: containerRect.width / 2 - 8,
                height: containerRect.height / 2 - 8,
            };
        }
        if (mousePosInContainer.y > (containerRect.height * 3) / 4) {
            return {
                x: 0,
                y: containerRect.height / 2 + 8,
                width: containerRect.width / 2 - 8,
                height: containerRect.height / 2 - 8,
            };
        }

        return {
            x: 0,
            y: 0,
            width: containerRect.width / 2 - 8,
            height: containerRect.height,
        };
    }

    if (x === 1) {
        if (mousePosInContainer.y < containerRect.height / 4) {
            return {
                x: containerRect.width / 2 + 8,
                y: 0,
                width: containerRect.width / 2 - 8,
                height: containerRect.height / 2 - 8,
            };
        }
        if (mousePosInContainer.y > (containerRect.height * 3) / 4) {
            return {
                x: containerRect.width / 2 + 8,
                y: containerRect.height / 2 + 8,
                width: containerRect.width / 2 - 8,
                height: containerRect.height / 2 - 8,
            };
        }

        return {
            x: containerRect.width / 2 + 8,
            y: 0,
            width: containerRect.width / 2 - 8,
            height: containerRect.height,
        };
    }

    return null;
}
