export const API_URL = import.meta.env.DEV
    ? import.meta.env.VITE_DEV_SERVER_URL
    : import.meta.env.VITE_PROD_SERVER_URL;
