const LinkType = {
    Image: 'image',
    Video: 'video',
    Webpage: 'webpage',
} as const;

type LinkType = typeof LinkType[keyof typeof LinkType];

export default LinkType;
