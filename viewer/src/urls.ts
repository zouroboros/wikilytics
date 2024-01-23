export function pageUrl(mainPage: string, articleName: string): URL {
    return new URL(articleName, mainPage);
}