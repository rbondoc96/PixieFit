/* eslint-disable no-console */
import chalk from 'chalk';

const info = (args: any) => {
    console.log(
        chalk.blue(`[${new Date().toLocaleString()}] [INFO]`),
        typeof args === 'string' ? chalk.blueBright(args) : args,
    );
};

const warn = (args: any) => {
    console.log(
        chalk.yellow(`[${new Date().toLocaleString()}] [INFO]`),
        typeof args === 'string' ? chalk.yellowBright(args) : args,
    );
};

const error = (args: any) => {
    console.log(
        chalk.red(`[${new Date().toLocaleString()}] [INFO]`),
        typeof args === 'string' ? chalk.redBright(args) : args,
    );
};

const log = (args: any) => {
    info(args);
};

export {info, warn, error, log};
