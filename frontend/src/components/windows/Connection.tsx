import { useState } from "react";
import { useServerData, useWebSocket } from "~/components/WebSocketProvider";
import { LoadingWindow } from "../FloatingWindow";
import { Button } from "../ui/button";
import { Input } from "../ui/input";

export default function Connection() {
    const { name } = useServerData();
    const { connect, token, linkFailed } = useWebSocket();
    const [inputToken, setInputToken] = useState(token ?? "");

    if (!connect) return <LoadingWindow />;

    return (
        <div className="scroll-orange flex flex-col items-center justify-center overflow-y-scroll p-10">
            {token && !name && !linkFailed && <p className="mb-4 text-gray-500">Connexion en cours...</p>}
            {linkFailed && <p className="mb-4 text-red-500">Échec de la connexion</p>}

            {name ? (
                <>
                    <h1 className="mb-4 font-bold text-xl">{name}</h1>
                    <Button
                        onClick={() => connect(null)}
                        className="cursor-pointer rounded bg-gray-500 px-4 py-2 text-white hover:bg-gray-600"
                    >
                        Déconnexion
                    </Button>
                </>
            ) : (
                <form className="flex flex-col items-center justify-center" action={() => connect(inputToken.trim())}>
                    <h1 className="mb-4 font-bold text-3xl">Connexion</h1>

                    <Input
                        type="text"
                        placeholder="Token d'équipe"
                        value={inputToken}
                        onChange={(e) => setInputToken(e.target.value)}
                        className="mb-4 rounded border p-2"
                    />

                    <Button
                        className="cursor-pointer rounded bg-orange-600 px-4 py-2 text-white hover:bg-orange-700"
                        type="submit"
                    >
                        Se connecter
                    </Button>
                </form>
            )}
        </div>
    );
}
