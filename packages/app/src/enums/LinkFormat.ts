const LinkFormat = {
    GIF: 'gif',
    JPEG: 'jpeg',
    MOV: 'mov',
    MP3: 'mp3',
    MP4: 'mp4',
    PNG: 'png',
    SVG: 'svg',
} as const;

type LinkFormat = typeof LinkFormat[keyof typeof LinkFormat];

export default LinkFormat;
