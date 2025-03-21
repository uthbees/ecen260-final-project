import { useEffect, useRef } from 'react';

// Runs the callback exactly once. (This is possible to do with just an effect,
// but it's confusing and bad practice since that's not what the dependency
// array is intended for.)
export function useInitialSetup(callback: () => void) {
    const run = useRef(false);

    useEffect(() => {
        if (!run.current) {
            run.current = true;
            callback();
        }
    }, [callback]);
}
