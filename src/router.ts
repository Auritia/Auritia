import { createRouter, createWebHistory, RouteRecordRaw } from "vue-router";

const routes: RouteRecordRaw[] = [
  // {
  //   path: "/",
  //   redirect: "/daw",
  // },
  {
    path: "/",
    name: "DAW",
    component: () => import("~/App.vue"),
    meta: {
      title: "Auritia | DAW",
    },
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
