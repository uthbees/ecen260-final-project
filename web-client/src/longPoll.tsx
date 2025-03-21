import React, { ReactNode } from 'react';

const MAX_CONSECUTIVE_FAILURES = 3;

export default async function longPoll(
    url: string,
    handleDataUpdate: (newData: unknown) => void,
    setError: (error: ReactNode | null) => void,
) {
    let consecutiveFailures = 0;

    async function handleFailedRequest(baseMessage: string) {
        consecutiveFailures++;

        let secondLine;
        if (consecutiveFailures < MAX_CONSECUTIVE_FAILURES) {
            secondLine =
                'Retrying in a few seconds..' + '.'.repeat(consecutiveFailures);
        } else {
            // The while loop will stop looping once we return
            secondLine =
                'Maximum consecutive failures reached, reload the page to try again.';
        }

        setError(
            <>
                <div>{baseMessage}</div>
                <div>{secondLine}</div>
            </>,
        );

        // between 0 and 3 seconds
        const jitter = Math.random() * 3;
        await delay((jitter + 3) * 1000);

        // continue with the while loop
    }

    while (consecutiveFailures < MAX_CONSECUTIVE_FAILURES) {
        await fetch(url).then(async (response) => {
            try {
                if (response.ok) {
                    handleDataUpdate(await response.json());
                    consecutiveFailures = 0;
                    // Clear the error, in case the previous request failed.
                    setError(null);
                    // Wait for a little bit before polling again.
                    await delay(2000);
                } else {
                    await handleFailedRequest(
                        `Request failed with status code ${response.status}.`,
                    );
                }
            } catch (error) {
                await handleFailedRequest(
                    `Encountered error when handling request: ${JSON.stringify(error)}`,
                );
            }
        });
    }
}

async function delay(milliseconds: number) {
    await new Promise((resolve) => setTimeout(resolve, milliseconds));
}
