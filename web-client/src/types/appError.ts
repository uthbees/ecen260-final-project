export interface AppError {
    message: string | string[]; // A line or array of lines
    reason: AppErrorReason;
}

export enum AppErrorReason {
    LONG_POLLING_REQUEST,
    MISC,
}
