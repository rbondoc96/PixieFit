import fs from 'node:fs/promises';
import path from 'node:path';

import Seeder from '@/database/seeders/Seeder';

const directoryPath = path.join(path.resolve(), 'src', 'database', 'seeders');

const executeFilesInDirectory = async () => {
    try {
        const files = await fs.readdir(directoryPath);

        for (const file of files) {
            if (file !== 'Seeder.ts' && file.endsWith('.ts')) {
                const filePath = path.join(directoryPath, file);

                const module = await import(filePath);
                const defaultClass = new module.default();

                if (typeof defaultClass === 'object' && defaultClass instanceof Seeder) {
                    console.log('Running:', file);
                    await defaultClass.run();
                } else {
                    console.log(`Error: 'run()' function not found in ${file}.`);
                }
            }
        }
    } catch (error) {
        console.error('Error reading or executing files:', error);
        process.exit(1);
    }

    console.log('Finished.');
    process.exit(0);
};

executeFilesInDirectory().then();
