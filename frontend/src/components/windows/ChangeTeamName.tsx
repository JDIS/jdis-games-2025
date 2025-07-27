import { useState } from "react";
import { Button } from "~/components/ui/button";
import { Input } from "~/components/ui/input";
import { useServerData } from "~/components/WebSocketProvider";

export default function ChangeTeamName() {
    const { name, changeName } = useServerData();
    const [newName, setNewName] = useState("");
    const [confirmation, setConfirmation] = useState<string>();

    const handleSubmit = () => {
        if (newName.trim().length < 3 || newName.trim().length > 24 || !changeName) return;

        changeName(newName.trim());
        setConfirmation("Nom d'équipe mis à jour !");
        setTimeout(() => setConfirmation(undefined), 3000);
        setNewName("");
    };

    if (!name) {
        return (
            <div className="p-10 text-center">
                <h1 className="mb-4 font-bold text-3xl">Changer le nom de l’équipe</h1>
                <p className="text-red-600">Vous devez être connecté pour changer le nom de votre équipe.</p>
            </div>
        );
    }

    return (
        <form
            className="scroll-orange flex flex-col items-center justify-center overflow-y-auto p-2"
            action={handleSubmit}
        >
            <h1 className="mb-4 font-bold text-3xl">Changer le nom de l’équipe</h1>
            <p className="mb-4 text-gray-500">
                <span data-secret-url="https://docs.google.com/document/d/1oMRxfZvE1fmfdv8Hfb5T7bhim7PAdK_gOJTCxY0l9xA/edit?usp=sharing" />
                Nom actuel : <strong>{name ?? "Équipe inconnue"}</strong>
            </p>

            <Input
                placeholder="Nouveau nom d’équipe"
                value={newName}
                onChange={(e) => setNewName(e.target.value)}
                className="mb-4 rounded border p-2"
            />

            <Button
                className="cursor-pointer rounded bg-orange-600 px-4 py-2 text-white hover:bg-orange-700"
                disabled={newName.trim().length < 3 || newName.trim().length > 24}
                type="submit"
            >
                Changer le nom
            </Button>

            {confirmation && <p className="mt-4 text-green-600">{confirmation}</p>}
        </form>
    );
}
