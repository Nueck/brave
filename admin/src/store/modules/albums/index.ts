import { defineStore } from 'pinia';

interface AlbumsStore {
  albumsData: Blog.ArticlesInfo[];
  albumsLoading: boolean;
  loading: boolean;
  page_total: number;
  current_page: number;
}

export const useAlbumsStore = defineStore('albums-store', {
  state: (): AlbumsStore => ({
    albumsData: [],
    albumsLoading: false,
    loading: false,
    page_total: 1,
    current_page: 1
  }),
  actions: {}
});
