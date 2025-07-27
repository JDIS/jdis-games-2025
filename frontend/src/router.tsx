import { createRouter as createTanStackRouter } from "@tanstack/react-router";

import DefaultCatchBoundary from "~/components/ErrorPage";
import NotFound from "~/components/NotFoundPage";
import { routeTree } from "./routeTree.gen";

export function createRouter() {
    return createTanStackRouter({
        routeTree,
        defaultErrorComponent: DefaultCatchBoundary,
        defaultNotFoundComponent: NotFound,
        scrollRestoration: true,
        defaultStructuralSharing: true,
    });
}

declare module "@tanstack/react-router" {
    interface Register {
        router: ReturnType<typeof createRouter>;
    }
}
