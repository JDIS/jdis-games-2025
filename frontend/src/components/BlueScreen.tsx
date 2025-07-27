import { useEffect, useState } from "react";

export default function BlueScreen() {
    const [enabled, setEnabled] = useState(false);

    useEffect(() => {
        const controller = new AbortController();
        let timer = 0;
        window.addEventListener(
            "bluescreen",
            () => {
                setEnabled(true);
                clearTimeout(timer);
                timer = setTimeout(() => setEnabled(false), 2000) as unknown as number;
            },
            {
                signal: controller.signal,
            },
        );

        return () => controller.abort();
    }, []);

    if (!enabled) return null;

    return (
        <div className="pointer-events-auto fixed inset-0 z-[9999] flex items-center justify-start bg-[#0874d4] text-left font-sans text-white">
            <div className="max-w-[600px] select-none p-8 pl-32 text-[1.1rem] leading-7">
                <div className="mb-4 text-[4rem] leading-none">:(</div>
                <div className="mb-4">
                    Your PC ran into a problem and needs to restart. We're just collecting some error info, and then
                    we'll restart for you.
                </div>
                <div className="mb-4 text-[1rem]">20% complete</div>
                <div className="mt-4 flex items-center gap-4">
                    <div className="grid h-[84px] w-[84px] grid-cols-21 grid-rows-21 border-2 border-white bg-white">
                        {[...Array(441)]
                            .map((_, i) => i)
                            .map((i) => {
                                const row = Math.floor(i / 21);
                                const col = i % 21;

                                const isCornerTopLeft = row <= 6 && col <= 6;
                                const isCornerTopRight = row <= 6 && col >= 14;
                                const isCornerBottomLeft = row >= 14 && col <= 6;

                                const isCornerOutline =
                                    (isCornerTopLeft || isCornerTopRight || isCornerBottomLeft) &&
                                    (row % 6 === 0 ||
                                        col % 6 === 0 ||
                                        row === 6 ||
                                        col === 6 ||
                                        row === 0 ||
                                        col === 0);
                                const isCornerInner =
                                    (isCornerTopLeft && row >= 2 && row <= 4 && col >= 2 && col <= 4) ||
                                    (isCornerTopRight && row >= 2 && row <= 4 && col >= 16 && col <= 18) ||
                                    (isCornerBottomLeft && row >= 16 && row <= 18 && col >= 2 && col <= 4);

                                const isRandom =
                                    !isCornerTopLeft && !isCornerTopRight && !isCornerBottomLeft && Math.random() > 0.7;

                                return (
                                    <div
                                        key={i}
                                        className={`h-full w-full ${
                                            isCornerOutline || isCornerInner || isRandom ? "bg-[#0874d4]" : "bg-white"
                                        }`}
                                    />
                                );
                            })}
                    </div>

                    <div className="text-[0.75rem] leading-5">
                        For more information about this issue and possible fixes, visit <br />
                        <a
                            href="https://docs.google.com/document/d/1z9da2ppF_QKXt7Akz34RwxGXGMTAKbiBzoqFQSzM7kc/edit?usp=sharing"
                            target="_blank"
                            className="text-white underline"
                            rel="noopener"
                        >
                            https://www.windows.com/stopcode
                        </a>
                        <br />
                        <br />
                        If you call a support person, give them this info: <br />
                        Stop code: <strong>CRITICAL_PROCESS_DIED</strong>
                    </div>
                </div>
            </div>
        </div>
    );
}
