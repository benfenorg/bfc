const watchTheme = (store: any) => {
  store.subscribe(() => {
    const {
      user: { theme },
    } = store.getState();
    // changeTheme(theme || "auto");
  });
};

export const subscribeStore = (store: any) => {
  watchTheme(store);
};
