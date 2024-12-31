import { PagePane } from "./types"
import { useWritable } from "./use-shared-store"

export const useErrorPopup = () =>
  useWritable("errorPopup", {
    state: false,
    message: null,
    data: null
  })

export const usePagePaneState = () => useWritable("pagePane", PagePane.None)
