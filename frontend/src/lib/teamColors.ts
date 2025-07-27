const teamColors = new Map<string, string>();

export function getTeamColor(teamId: string): string {
    const value = teamColors.get(teamId);
    if (value) return value;

    const hue = (teamColors.size * 137) % 360;
    const color = `hsl(${hue}, 65%, 55%)`;
    teamColors.set(teamId, color);
    return color;
}
