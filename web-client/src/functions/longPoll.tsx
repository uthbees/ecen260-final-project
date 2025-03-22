import { SetStateAction } from 'react';
import { AppError, AppErrorReason } from '../types/appError.ts';

const MAX_CONSECUTIVE_FAILURES = 3;
// The amount of time to wait between one (non-errored) request ending and the next starting.
const POLL_DELAY_MS = 1000;

export default async function longPoll(
    url: string,
    handleDataUpdate: HandleDataUpdateCallback,
    setError: SetErrorCallback,
) {
    let state: LongPollState = {
        consecutiveFailures: 0,
        lastKnownRevisionNum: -1,
    };

    while (state.consecutiveFailures < MAX_CONSECUTIVE_FAILURES) {
        try {
            await fetch(
                `${url}?last_known_revision_num=${state.lastKnownRevisionNum}`,
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
        const text = await response.text();
        const secondHalf = text.length === 0 ? '' : `: ${text}`;

        return await handleFailedRequest(
            `Request failed with status code ${response.status} (${response.statusText})${secondHalf}`,
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
                !('revision_num' in responseJSON) ||
                typeof responseJSON.revision_num !== 'number'
            ) {
                return await handleFailedRequest(
                    "Received invalid response - failed to access 'revision_num' property.",
                    setError,
                    state,
                );
            }

            if (
                responseJSON.revision_num === state.lastKnownRevisionNum
            ) {
                return await handleFailedRequest(
                    "Error - 'revision_num' property of response is the same as the sent revision.",
                    setError,
                    state,
                );
            } else if (
                responseJSON.revision_num < state.lastKnownRevisionNum
            ) {
                return await handleFailedRequest(
                    "Error - 'revision_num' property of response is less than the sent revision.",
                    setError,
                    state,
                );
            }

            state.lastKnownRevisionNum = responseJSON.revision_num;
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
    lastKnownRevisionNum: number;
}
