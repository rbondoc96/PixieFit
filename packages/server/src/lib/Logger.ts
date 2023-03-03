/* eslint-disable no-console */
import chalk from 'chalk';

export default class Logger {
    static info(args: any): void {
        if (process.env.NODE_ENV !== 'test') {
            console.log(
                chalk.blue(`[${new Date().toLocaleString()}] [INFO]`),
                typeof args === 'string' ? chalk.blueBright(args) : args,
            );
        }
    }

    static warn(args: any): void {
        console.log(
            chalk.yellow(`[${new Date().toLocaleString()}] [WARN]`),
            typeof args === 'string' ? chalk.yellowBright(args) : args,
        );
    }

    static error(args: any): void {
        if (process.env.NODE_ENV !== 'test') {
            console.log(
                chalk.red(`[${new Date().toLocaleString()}] [ERROR]`),
                typeof args === 'string' ? chalk.redBright(args) : args,
            );
        }
    }

    static log(args: any): void {
        Logger.info(args);
    }
}
