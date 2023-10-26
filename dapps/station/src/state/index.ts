import { configureStore, combineReducers } from "@reduxjs/toolkit";
import { persistStore, persistReducer } from "redux-persist";
import storage from "redux-persist/lib/storage";
import autoMergeLevel2 from "redux-persist/lib/stateReconciler/autoMergeLevel2";
import userReducer from "./user/reducer";

import { subscribeStore } from "./subscribe";
const persistConfigs = {
  key: "root",
  storage,
  blacklist: [],
  stateReconciler: autoMergeLevel2,
};

const rootReducer: any = combineReducers({
  user: userReducer,
});

const persistedReducer: any = persistReducer(persistConfigs, rootReducer);

const store = configureStore({
  reducer: persistedReducer,
  middleware: (getDefaultMiddleware:any) =>
    getDefaultMiddleware({
      serializableCheck: false,
    }),
});
subscribeStore(store);
const persistor = persistStore(store);
export { store, persistor };

export type AppState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;
