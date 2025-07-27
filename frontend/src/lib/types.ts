export type Position = {
    x: number;
    y: number;
};

export type Team = {
    id: string;
    name: string;
    score: number;
};

export type Player = {
    team: string;
    name: string;
    score: number;
    kills: number;
    hp: number;
    shield: number;
    position: Position;
    lastPosition: Position;
    effects: string[];
    inventory: {
        name: string;
        remainingTicks: number;
        quantity: null | number;
    }[];
};

export type Projectile = {
    name: string;
    position: Position;
    direction: CardinalDirection;
    remaining_ticks: number;
    speed: number;
    damage: number;
};

export type CardinalDirection = "Up" | "Down" | "Left" | "Right";

export type Cell = "groundPlane" | "firewall" | "via" | "chest" | "resistance" | "pcb";
export type Object =
    | { type: "resistance"; position: Position; hp: number }
    | { type: "chest"; position: Position }
    | { type: "trap"; position: Position; owner: string; name: string; damage: number };

export type GameState = {
    timestamp: number;
    players: Player[];
    stats: {
        alive_count: number;
        dead_count: number;
    };
    ground: {
        width: number;
        height: number;
        data: Cell[];
        offset: Position;
    };
    objects: Object[];
    projectiles: Projectile[];
};

export type Event =
    | { type: "kill"; killer: string; victim: string; weapon?: string }
    | { type: "nuke"; player: string }
    | { type: "gameEnd"; winner: string };
