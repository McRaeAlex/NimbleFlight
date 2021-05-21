const themeConfig = {
    nav: [
        { text: 'Home', link: '/' },
        { text: 'Docs', link: '/docs/' },
        { text: 'Blog', link: '/blog/' },
    ],
    sidebar: {
        '/blog/': [
            '',
            'day0',
            'day1',
            'day2',
            'day3'
        ],
        '/docs/': 'auto',
    }
};

module.exports = {
    title: "NimbleFlight",
    description: "Flight controller software",
    base: "/NimbleFlight/",
    themeConfig,
}