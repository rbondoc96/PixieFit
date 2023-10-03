/** @type {import('tailwindcss').Config} */

module.exports = {
    content: ['./index.html', './src/**/*.{ts,tsx}'],
    darkMode: 'class',
    theme: {
        extend: {
            colors: {
                dark: {
                    300: '#9DB3B3',
                    500: '#566666',
                    700: '#1D2828',
                },
                light: {
                    100: '#FFFFFF',
                    300: '#F9F7F3',
                    500: '#C2C2C2',
                },
                primary: {
                    DEFAULT: '#4CF0C4',
                },
            },
            screens: {
                xs: '480px',
            },
        },
    },
    plugins: [],
};
