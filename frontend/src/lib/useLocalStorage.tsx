import { useEffect, useState } from "react";

/**
 * A React hook that provides a stateful value synchronized with localStorage.
 * Supports Server-Side Rendering (SSR) by providing a default value until
 * the component mounts on the client.
 *
 * @param {string} key The key to use for storing the value in localStorage.
 * @param {T} initialValue The initial value to use if no value is found in localStorage
 *                          or during SSR.
 * @returns {[T, (value: T) => void]} A tuple containing the current value
 *                                                        and a setter function.
 * @template T
 */
export default function useLocalStorage<T>(key: string, initialValue: T): [T, React.Dispatch<React.SetStateAction<T>>] {
    const [state, setState] = useState<T>(() => {
        if (typeof window === "undefined") {
            // Return initialValue for SSR
            return initialValue;
        }

        try {
            const item = window.localStorage.getItem(key);
            // Parse stored json or if none return initialValue
            return item ? JSON.parse(item) : initialValue;
        } catch (error) {
            console.error("Error reading from localStorage:", error);
            return initialValue;
        }
    });

    useEffect(() => {
        if (typeof window === "undefined") return;

        try {
            window.localStorage.setItem(key, JSON.stringify(state));
        } catch (error) {
            console.error("Error writing to localStorage:", error);
        }
    }, [key, state]);

    return [state, setState];
}
