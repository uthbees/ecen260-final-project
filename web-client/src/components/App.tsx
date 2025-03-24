import React, { Fragment, useCallback, useState } from 'react';
import { Alert, Button, Stack, TextField, Typography } from '@mui/material';
import { API_URL } from '../config/apiUrl.ts';
import { useInitialSetup } from '../hooks/useInitialSetup.ts';
import longPoll from '../functions/longPoll.tsx';
import { AppError, AppErrorReason } from '../types/appError.ts';

export default function App() {
    const [temperature, setTemperature] = useState<number | null>(null);
    const [textFieldValue, setTextFieldValue] = useState<number>(0);
    const [error, setError] = useState<AppError | null>(null);

    const handleUpdateTemperature = useCallback(
        (newData: Record<string, unknown>) => {
            if (!validateGetSensorDataResponse(newData)) {
                setError({
                    message: `Server gave invalid response for temperature: ${JSON.stringify(newData)}`,
                    reason: AppErrorReason.MISC,
                });
                return;
            }

            const newTemperature = newData.value;

            if (isNaN(newTemperature)) {
                setError({
                    message: `Server response for temperature evaluated to NaN: ${JSON.stringify(newTemperature)}`,
                    reason: AppErrorReason.MISC,
                });
                return;
            }

            setTemperature(newTemperature);
        },
        [],
    );

    useInitialSetup(() =>
        longPoll(`${API_URL}/sensor_data`, handleUpdateTemperature, setError),
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
                                parseInt(event.target.value),
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

interface GetSensorDataResponse {
    value: number;
    revision_num: number;
}

function validateGetSensorDataResponse(
    response: unknown,
): response is GetSensorDataResponse {
    return (
        typeof response === 'object' &&
        response !== null &&
        'value' in response &&
        typeof response.value === 'number' &&
        'revision_num' in response &&
        typeof response.revision_num === 'number'
    );
}

function postTemperature(
    temperature: number,
    setError: (error: AppError | null) => void,
) {
    fetch(`${API_URL}/sensor_data`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({ temperature }),
    }).then((response) => {
        if (!response.ok) {
            setError({
                message: `Failed to update temperature on server (error code ${response.status}).`,
                reason: AppErrorReason.MISC,
            });
        }
    });
}
