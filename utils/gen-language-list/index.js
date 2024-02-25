import fetch from 'node-fetch';
import cheerio from 'cheerio';

const response = await fetch('https://help.acmicpc.net/language/info/all');
const text = await response.text();

const $ = cheerio.load(text);

const upperLimit = 200;
let notAvailableIndex = 0;

// TODO: generate file instead of printf it to stdout.
console.log('{');
for (let id = 0; id <= upperLimit; id++) {
    let res = $('h3', `#language-${id}`).text();
    if (res === '') {
        res = `language not available ${notAvailableIndex++}`;
    }

    const sep = id < upperLimit ? ',' : '';
    console.log(`    "${res}": ${id}${sep}`);
}
console.log('}');
