import React, { Fragment, useCallback, useState } from 'react';
import { Alert, Button, Stack, TextField, Typography } from '@mui/material';
import { API_URL } from './constants.ts';
import { useInitialSetup } from './useInitialSetup.ts';
import longPoll from './longPoll.tsx';
import { AppError, AppErrorReason } from './types.ts';

export default function App() {
    const [temperature, setTemperature] = useState<number | null>(null);
    const [textFieldValue, setTextFieldValue] = useState<number>(0);
    const [error, setError] = useState<AppError | null>(null);

    const handleUpdateTemperature = useCallback(
        (newData: Record<string, unknown>) => {
            let result;

            if (!('temperature' in newData)) {
                setError({
                    message: `Server gave invalid response for temperature: ${JSON.stringify(newData)}`,
                    reason: AppErrorReason.MISC,
                });
                return;
            }

            const newTemperature = newData.temperature;

            if (typeof newTemperature === 'number') {
                result = newTemperature;
            } else if (typeof newTemperature === 'string') {
                result = parseFloat(newTemperature);
            } else {
                setError({
                    message: `Server gave invalid response for temperature: ${JSON.stringify(newTemperature)}`,
                    reason: AppErrorReason.MISC,
                });
                return;
            }

            if (isNaN(result)) {
                setError({
                    message: `Server response for temperature evaluated to NaN: ${JSON.stringify(newTemperature)}`,
                    reason: AppErrorReason.MISC,
                });
                return;
            }

            setTemperature(result);
        },
        [],
    );

    useInitialSetup(() =>
        longPoll(`${API_URL}/temperature`, handleUpdateTemperature, setError),
    );

    return (
        <>
            {(() => {
                if (error === null) {
                    return null;
                }
                return (
                    <Alert
                        severity="error"
                        style={{ position: 'absolute', width: '100%' }}
                    >
                        {typeof error.message === 'string'
                            ? error.message
                            : error.message.map((line, index) => {
                                  if (index === 0) {
                                      return line;
                                  }
                                  return (
                                      <Fragment key={index}>
                                          <br />
                                          {line}
                                      </Fragment>
                                  );
                              })}
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
                            postTemperature(textFieldValue, setError);
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
    setError: (error: AppError | null) => void,
) {
    fetch(`${API_URL}/temperature`, {
        method: 'POST',
        body: JSON.stringify(temperature),
    }).then((response) => {
        if (!response.ok) {
            setError({
                message: `Failed to update temperature on server (error code ${response.status}).`,
                reason: AppErrorReason.MISC,
            });
        }
    });
}
