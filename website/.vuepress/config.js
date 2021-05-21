const themeConfig = {
    logo: '/logo_drone.svg',
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
    description: "A new flight controller software built with Rust with modern abstractions",
    base: "/NimbleFlight/",
    themeConfig,
}