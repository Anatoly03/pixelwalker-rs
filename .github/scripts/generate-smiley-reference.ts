/**
 * @brief Generates a list of smileys from PixelWalker client assets.
 *
 * @note This script is designed to be run in a GitHub Actions workflow. If
 * you run this script locally, it will generate files which are not meant
 * to be committed.
 */

import { PNG } from 'pngjs';
import { mkdir, writeFile } from 'node:fs/promises';
import { join } from 'node:path';

/**
 * @brief The generated atlas image of all sprites - including smileys, auras
 * and other items.
 */
const SPRITES_PNG = 'https://client.pixelwalker.net/atlases/sprites.png';

/**
 * @brief The generated atlas json of all sprites - including smileys, auras
 * and other items.
 */
const SPRITES_JSON = 'https://client.pixelwalker.net/atlases/sprites.json';

/**
 * @brief The relative path to the media directory, used by sprites within
 * MDBOOK.
 */
const GENERATE_MEDIA_PATH_RELATIVE = './smileys/';

/**
 * @brief The list of localizations: MDBook directories which should be filled
 * with generated content.
 */
const LOCALIZATIONS = ['en'];

/**
 * @brief Element in the array type of the atlas `frames` field.
 */
type AtlasFrame = {
    filename: string;
    frame: {
        x: number;
        y: number;
        w: number;
        h: number;
    };
    rotated: false;
    trimmed: false;
    spriteSourceSize: {
        x: number;
        y: number;
        w: number;
        h: number;
    };
    sourceSize: {
        w: number;
        h: number;
    };
    pivot: {
        x: 0.5;
        y: 0.5;
    };
};

/**
 * @brief The type of the atlas `meta` field.
 */
type AtlasMeta = {
    app: 'https://github.com/odrick/gulp-free-tex-packer';
    version: string;
    image: 'sprites.png';
    format: 'RGBA8888';
    size: { w: number; h: number };
    scale: number;
};

/**
 * @brief The type of the generated atlas json of all sprites.
 */
type Atlas = {
    frames: AtlasFrame[];
    meta: AtlasMeta;
};

/**
 * @brief The type of the generated atlas json of all sprites.
 */
type Frame = {
    filename: string;
    frame: AtlasFrame;
    sprite: PNG;
};

/**
 * @brief Generate the smiley reference for a specific locale.
 */
function generatePreambelForLocale(locale: string): string {
    switch (locale) {
        case 'en':
            return `
# Smiley List

> Note: For reference purposes only. This list may be out of date
> or inconsistant with the game. Generated on ${new Date().toDateString()}.
> This list is sorted alphabetically.

| | Name |\n| :---: | --- |\n`;
        default:
            throw new Error(`Unsupported locale: ${locale}`);
    }
}

/**
 * @brief Generate the smiley reference for a specific locale.
 */
function generateForLocale(locale: string, frames: Frame[]): string {
    const preambel = generatePreambelForLocale(locale);
    const tableContent = frames
        .map((frame) => {
            const frame_path = join(GENERATE_MEDIA_PATH_RELATIVE, frame.filename);
            const name = frame.frame.filename;
            return `| ![Image](${frame_path}) | \`${name}\` |\n`;
        })
        .join('');
    return preambel + tableContent;
}

/**
 * @brief Copies an image segment from a larger image atlas.
 */
function createFrame(source: PNG, frame: AtlasFrame): Frame {
    const width = frame.sourceSize.w;
    const height = frame.sourceSize.h;
    const srcX = frame.frame.x;
    const srcY = frame.frame.y;
    const sprite = new PNG({ width, height });
    const filename = frame.filename.replace('/', '_') + '.png';

    for (let y = 0; y < height; y += 1) {
        for (let x = 0; x < width; x += 1) {
            const srcIdx = ((srcY + y) * source.width + (srcX + x)) * 4;
            const dstIdx = (y * sprite.width + x) * 4;
            sprite.data[dstIdx] = source.data[srcIdx];
            sprite.data[dstIdx + 1] = source.data[srcIdx + 1];
            sprite.data[dstIdx + 2] = source.data[srcIdx + 2];
            sprite.data[dstIdx + 3] = source.data[srcIdx + 3];
        }
    }

    return { frame, sprite, filename };
}

/**
 * @brief Fetches the smiley frames and metadata.
 */
async function fetchSmileys(): Promise<Frame[]> {
    const response_image = await fetch(SPRITES_PNG);
    const response_data = await fetch(SPRITES_JSON);
    const imageBytes = Buffer.from(await response_image.arrayBuffer());
    const json: Atlas = await response_data.json();
    const image = PNG.sync.read(imageBytes);
    const smileys = json.frames.filter((f) => f.filename.startsWith('smileys/'));
    return smileys.map((f) => createFrame(image, f));
}

/**
 * @brief Writes smileys to a shared media directory.
 */
async function printSmileys(locale: string, frames: Frame[]) {
    const GENERATE_MEDIA_PATH = `./pixelwalker-docs/${locale}/src/generated/smileys`;
    await mkdir(GENERATE_MEDIA_PATH, { recursive: true });

    for (const entry of frames) {
        const frame_path = join(GENERATE_MEDIA_PATH, entry.filename);
        writeFile(frame_path, PNG.sync.write(entry.sprite));
    }
}

/**
 * @brief Generate smiley references for every locale.
 */
async function generate() {
    // Fetch the smiley list.
    const frames = await fetchSmileys();
    console.debug(`Fetched ${frames.length} frames.`);
    
    LOCALIZATIONS.forEach(async (locale) => {
        await printSmileys(locale, frames);

        const GENERATE_PATH = `./pixelwalker-docs/${locale}/src/generated/smileys.md`;
        let content = generateForLocale(locale, frames);
        writeFile(GENERATE_PATH, content);
    });
}

// Run the script.
generate();
