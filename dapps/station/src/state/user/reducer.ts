import { createSlice } from "@reduxjs/toolkit";

export interface UserState {
  pathTab: string;
}
export const initialState: UserState = {
  pathTab: "/",
};

const userSlice = createSlice({
  name: "user",
  initialState,
  reducers: {
    updatePathTab(state, { payload:{path} }) {
      state.pathTab = path;
    },
   
  },
});

export const {
  updatePathTab
} = userSlice.actions;

export default userSlice.reducer;
