import React, { ReactNode, useCallback, useState } from 'react';
import { Alert, Button, Stack, TextField, Typography } from '@mui/material';
import { API_URL } from './constants.ts';
import { useInitialSetup } from './useInitialSetup.ts';
import longPoll from './longPoll.tsx';

export default function App() {
    const [temperature, setTemperature] = useState<number | null>(null);
    const [textFieldValue, setTextFieldValue] = useState<number>(0);
    const [errorMessage, setErrorMessage] = useState<ReactNode | null>(null);

    const handleUpdateTemperature = useCallback(
        (newData: Record<string, unknown>) => {
            let result;

            if (!('temperature' in newData)) {
                setErrorMessage(
                    `Server gave invalid response for temperature: ${JSON.stringify(newData)}`,
                );
                return;
            }

            const newTemperature = newData.temperature;

            if (typeof newTemperature === 'number') {
                result = newTemperature;
            } else if (typeof newTemperature === 'string') {
                result = parseFloat(newTemperature);
            } else {
                setErrorMessage(
                    `Server gave invalid response for temperature: ${JSON.stringify(newTemperature)}`,
                );
                return;
            }

            if (isNaN(result)) {
                setErrorMessage(
                    `Server response for temperature evaluated to NaN: ${JSON.stringify(newTemperature)}`,
                );
                return;
            }

            setTemperature(result);
        },
        [],
    );

    useInitialSetup(() =>
        longPoll(
            `${API_URL}/temperature`,
            handleUpdateTemperature,
            setErrorMessage,
        ),
    );

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
                            postTemperature(textFieldValue, setErrorMessage);
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
                <Typography>
                    Temperature on server:{' '}
                    {temperature === null ? 'Loading...' : `${temperature}°F`}
                </Typography>
            </Stack>
        </>
    );
}

function postTemperature(
    temperature: number,
    setErrorMessage: (errorMessage: ReactNode | null) => void,
) {
    fetch(`${API_URL}/temperature`, {
        method: 'POST',
        body: JSON.stringify(temperature),
    }).then((response) => {
        if (!response.ok) {
            setErrorMessage(
                `Failed to update temperature on server (error code ${response.status}).`,
            );
        }
    });
}
