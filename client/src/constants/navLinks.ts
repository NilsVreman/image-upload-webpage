export interface NavigationLink {
  label: string;
  path: string;
  page?: string;
}

// NOTE: Page-path must be relative to the src directory according to
// [these rules](https://github.com/rollup/plugins/tree/master/packages/dynamic-import-vars#limitations)
export const menuLinks: NavigationLink[] = [
  // prettier-ignore
  {
    label: "Home",
    path: "/",
    page: "LandingPage.vue"
  },
  {
    label: "Gallery",
    path: "/gallery",
    page: "GalleryPage.vue",
  },
  {
    label: "Program",
    path: "/static/program.pdf",
    page: undefined,
  },
];
