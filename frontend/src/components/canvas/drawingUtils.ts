import type { GameState } from "~/lib/types";

const FIRE_TEXTURE_SIZE = 32;

const createFireTexture = (): HTMLCanvasElement => {
    const canvas = document.createElement("canvas");
    canvas.width = FIRE_TEXTURE_SIZE;
    canvas.height = FIRE_TEXTURE_SIZE;
    const ctx = canvas.getContext("2d")!;

    const gradient = ctx.createRadialGradient(
        FIRE_TEXTURE_SIZE / 2,
        FIRE_TEXTURE_SIZE / 2,
        0,
        FIRE_TEXTURE_SIZE / 2,
        FIRE_TEXTURE_SIZE / 2,
        FIRE_TEXTURE_SIZE / 2,
    );

    gradient.addColorStop(0, "rgb(255, 80, 30)");
    gradient.addColorStop(1, "rgb(100, 20, 10)");

    ctx.fillStyle = gradient;
    ctx.fillRect(0, 0, FIRE_TEXTURE_SIZE, FIRE_TEXTURE_SIZE);

    return canvas;
};

// Lazy load the fire texture only when in the browser
let fireTexture: HTMLCanvasElement | null = null;

function getFireTexture(): HTMLCanvasElement | null {
    if (typeof document === "undefined") return null;
    if (!fireTexture) {
        fireTexture = createFireTexture();
    }
    return fireTexture;
}

export const drawGround = (
    ctx: CanvasRenderingContext2D,
    ground: GameState["ground"],
    res: number,
    objects: GameState["objects"],
) => {
    const { width, height, data, offset } = ground;
    const cellSize = res;

    // Ground cells
    for (let y = 0; y < height; y++) {
        for (let x = 0; x < width; x++) {
            const cell = data[y * width + x];

            switch (cell) {
                case "via":
                    ctx.fillStyle = "#101010";
                    break;
                case "pcb":
                    ctx.fillStyle = "#21864F";
                    break;
                default:
                    continue;
            }

            ctx.fillRect(x * cellSize + offset.x, y * cellSize + offset.y, cellSize, cellSize);
        }
    }

    // Objects
    for (const obj of objects) {
        const x = obj.position.x * res + offset.x;
        const y = obj.position.y * res + offset.y;

        switch (obj.type) {
            case "chest":
                ctx.fillStyle = "#964B00";
                break;
            case "resistance": {
                ctx.fillStyle = "#FFD700";
                ctx.fillRect(x, y, res, res);

                // Barre vie des resistances
                if (obj.hp < 100) {
                    const maxHp = 100;
                    const ratio = Math.max(0, Math.min(1, obj.hp / maxHp));
                    const barWidth = res * 0.8;
                    const barHeight = res * 0.1;
                    const barX = x + (res - barWidth) / 2;
                    const barY = y - barHeight - 2;

                    ctx.fillStyle = "#333";
                    ctx.fillRect(barX, barY, barWidth, barHeight);

                    const lightness = 20 + ratio * 30;
                    const hpColor = `hsl(50, 100%, ${lightness}%)`;
                    ctx.fillStyle = hpColor;
                    ctx.fillRect(barX, barY, barWidth * ratio, barHeight);

                    ctx.strokeStyle = "#000";
                    ctx.lineWidth = 1;
                    ctx.strokeRect(barX, barY, barWidth, barHeight);
                }

                break;
            }
            case "trap":
                ctx.fillStyle = "#8B0000";
                break;
            default:
                continue;
        }

        ctx.fillRect(x, y, res, res);
    }

    // Animated firewall
    const texture = getFireTexture();
    if (texture) {
        const time = performance.now();

        for (let y = 0; y < height; y++) {
            for (let x = 0; x < width; x++) {
                if (data[y * width + x] !== "firewall") continue;

                const dx = x * cellSize + offset.x;
                const dy = y * cellSize + offset.y;

                const alpha = 0.8 + 0.1 * Math.sin((time + (x + y) * 50) / 1000);
                ctx.globalAlpha = alpha;
                ctx.drawImage(texture, dx, dy, cellSize, cellSize);
            }
        }
        ctx.globalAlpha = 1;
    }

    // Grid overlay
    ctx.strokeStyle = "rgba(255,255,255,0.05)";
    ctx.lineWidth = 1;

    for (let x = 0; x <= width; x++) {
        const px = x * cellSize + offset.x;
        ctx.beginPath();
        ctx.moveTo(px, offset.y);
        ctx.lineTo(px, offset.y + height * cellSize);
        ctx.stroke();
    }

    for (let y = 0; y <= height; y++) {
        const py = y * cellSize + offset.y;
        ctx.beginPath();
        ctx.moveTo(offset.x, py);
        ctx.lineTo(offset.x + width * cellSize, py);
        ctx.stroke();
    }
};
