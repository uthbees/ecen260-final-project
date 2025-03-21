import React, { ReactNode, useState } from 'react';
import { Alert, Button, Stack, TextField, Typography } from '@mui/material';
import { API_URL } from './constants.ts';
import { useInitialSetup } from './useInitialSetup.ts';

export default function App() {
    const [temperature, setTemperature] = useState<number | null>(null);
    const [textFieldValue, setTextFieldValue] = useState<number>(0);
    const [errorMessage, setErrorMessage] = useState<ReactNode | null>(null);

    useInitialSetup(() => longPollTemperature(setTemperature, setErrorMessage));

    return (
        <>
            {(() => {
                if (errorMessage === null) {
                    return null;
                }
                return (
                    <Alert
                        severity="error"
                        style={{ position: 'absolute', width: '100%' }}
                    >
                        {errorMessage}
                    </Alert>
                );
            })()}
            <Stack
                direction="column"
                spacing={4}
                style={{
                    height: '100%',
                    alignItems: 'center',
                    justifyContent: 'center',
                }}
            >
                <Stack direction="row" spacing={2}>
                    <Button
                        variant="contained"
                        onClick={() => {
                            postTemperature(textFieldValue);
                        }}
                    >
                        Update temperature on server
                    </Button>
                    <TextField
                        type="number"
                        value={textFieldValue}
                        onChange={(
                            event: React.ChangeEvent<HTMLInputElement>,
                        ) => {
                            setTextFieldValue(
                                event.target.value as unknown as number,
                            );
                        }}
                    ></TextField>
                </Stack>
                <Typography>Temperature on server: {temperature}</Typography>
            </Stack>
        </>
    );
}

function postTemperature(temperature: number) {
    fetch(`${API_URL}/temperature`, {
        method: 'POST',
        body: JSON.stringify(temperature),
    }).then((response) => {
        if (!response.ok) {
            alert('Failed to update temperature on server');
        }
    });
}

async function longPollTemperature(
    handleUpdateTemperature: (temperature: number) => void,
    handleUpdateError: (error: ReactNode | null) => void,
) {
    let consecutiveFailures = 0;
    const maxConsecutiveFailures = 3;

    async function handleFailedRequest(baseMessage: string) {
        consecutiveFailures++;

        let secondLine = '';
        if (consecutiveFailures < maxConsecutiveFailures) {
            secondLine =
                'Retrying in a few seconds..' + '.'.repeat(consecutiveFailures);
        } else {
            secondLine =
                'Maximum consecutive failures reached, reload the page to try again.';
        }

        handleUpdateError(
            <>
                <div>{baseMessage}</div>
                <div>{secondLine}</div>
            </>,
        );

        // between 0 and 3 seconds
        const jitter = Math.random() * 3;
        await delay((jitter + 3) * 1000);

        // continue...
    }

    while (consecutiveFailures < 3) {
        await fetch(`${API_URL}/temperature`).then(async (response) => {
            try {
                if (response.ok) {
                    const rawResult = await response.json();
                    const result = parseInt(rawResult);
                    if (isNaN(result)) {
                        await handleFailedRequest(
                            `Failed to parse server response as number: ${JSON.stringify(rawResult)}`,
                        );
                    } else {
                        handleUpdateTemperature(result);
                        consecutiveFailures = 0;
                        // Clear the error, in case the previous request failed.
                        handleUpdateError(null);
                        // Wait for a little bit before polling again.
                        await delay(2000);
                    }
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
