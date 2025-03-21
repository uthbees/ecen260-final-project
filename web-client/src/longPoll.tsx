import { SetStateAction } from 'react';
import { AppError, AppErrorReason } from './types.ts';

const MAX_CONSECUTIVE_FAILURES = 3;
// The amount of time to wait between one (non-errored) request ending and the next starting.
const POLL_DELAY_MS = 2000;

export default async function longPoll(
    url: string,
    handleDataUpdate: HandleDataUpdateCallback,
    setError: SetErrorCallback,
) {
    let state: LongPollState = {
        consecutiveFailures: 0,
        lastKnownUpdateTimestamp: 0,
    };

    while (state.consecutiveFailures < MAX_CONSECUTIVE_FAILURES) {
        try {
            await fetch(
                `${url}?last_known_update_timestamp=${state.lastKnownUpdateTimestamp}`,
            ).then(async (response) => {
                state = await handlePollResponse(
                    response,
                    handleDataUpdate,
                    setError,
                    state,
                );
            });
        } catch (error) {
            await handleFailedRequest(
                `Encountered error handling request: ${(error as Error).message}.`,
                setError,
                state,
            );
        }
    }
}

async function handlePollResponse(
    response: Response,
    handleDataUpdate: HandleDataUpdateCallback,
    setError: SetErrorCallback,
    state: LongPollState,
): Promise<LongPollState> {
    if (!response.ok) {
        return await handleFailedRequest(
            `Request failed with status code ${response.status}.`,
            setError,
            state,
        );
    }

    try {
        const responseBody = await response.text();
        // If the response body is empty, we want to just poll again - that's expected if there hasn't been an update.
        if (responseBody.length !== 0) {
            const responseJSON = JSON.parse(responseBody);

            if (
                typeof responseJSON !== 'object' ||
                responseJSON === null ||
                !('update_timestamp' in responseJSON) ||
                typeof responseJSON.update_timestamp !== 'number'
            ) {
                return await handleFailedRequest(
                    "Received invalid response - failed to access 'update_timestamp' property.",
                    setError,
                    state,
                );
            }

            if (
                responseJSON.update_timestamp === state.lastKnownUpdateTimestamp
            ) {
                return await handleFailedRequest(
                    "Error - 'update_timestamp' property of response is the same as the sent timestamp.",
                    setError,
                    state,
                );
            }

            state.lastKnownUpdateTimestamp = responseJSON.update_timestamp;
            handleDataUpdate(responseJSON);
        }

        state.consecutiveFailures = 0;
        // Clear the error if it's from the previous request failing.
        setError((prevState) => {
            if (prevState?.reason === AppErrorReason.LONG_POLLING_REQUEST) {
                return null;
            }
            return prevState;
        });
        // Wait for a little bit before polling again.
        await delay(POLL_DELAY_MS);
    } catch (error) {
        return await handleFailedRequest(
            `Encountered error when handling request: ${(error as Error).message}`,
            setError,
            state,
        );
    }

    return state;
}

async function handleFailedRequest(
    baseMessage: string,
    setError: SetErrorCallback,
    state: LongPollState,
): Promise<LongPollState> {
    state.consecutiveFailures++;

    let secondLine;
    if (state.consecutiveFailures < MAX_CONSECUTIVE_FAILURES) {
        secondLine =
            'Retrying in a few seconds...' +
            ' again...'.repeat(state.consecutiveFailures - 1);
    } else {
        // The while loop will stop looping once we return
        secondLine =
            'Maximum consecutive failures reached, reload the page to try again.';
    }

    setError({
        message: [baseMessage, secondLine],
        reason: AppErrorReason.LONG_POLLING_REQUEST,
    });

    // wait for between 3 and 6 seconds
    const jitter = Math.random() * 3;
    await delay((jitter + 3) * 1000);

    return state;
}

async function delay(milliseconds: number) {
    await new Promise((resolve) => setTimeout(resolve, milliseconds));
}

type HandleDataUpdateCallback = (newData: Record<string, unknown>) => void;
type SetErrorCallback = (error: SetStateAction<AppError | null>) => void;

interface LongPollState {
    consecutiveFailures: number;
    lastKnownUpdateTimestamp: number;
}
