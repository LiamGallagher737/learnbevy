const fs = require('fs').promises;
const path = require('path');

async function traverseDirectory(directoryPath, basePath) {
    let list = [];

    try {
        const files = await fs.readdir(directoryPath);

        for (const file of files) {
            const filePath = path.join(directoryPath, file);
            const stats = await fs.stat(filePath);

            // If the current item is a directory, recursively traverse it
            if (stats.isDirectory()) {
                list = [...list, ...(await traverseDirectory(filePath, basePath))];
            } else if (path.extname(file) === '.rs') {
                const relativePath = path.relative(basePath, filePath);
                list.push(createEntry(relativePath));
            }
        }
    } catch (err) {
        console.error('Error reading directory:', err);
    }

    return list;
}

function createEntry(filePath) {
    let label = filePath.split('/').map(part => part.split('_').map(str => str.charAt(0).toUpperCase() + str.slice(1)).join(' ')).join(' / ').replace('.rs', '');
    return { value: filePath, label };
}

async function main() {
    const directoryPath = process.argv[2];
    const basePath = path.resolve(directoryPath);
    const list = await traverseDirectory(directoryPath, basePath);
    const code = "// Generated by generate.cjs, changes will be overridden\n" + "export const examples = " + JSON.stringify(list) + ';';
    process.stdout.write(code);
}

main();
