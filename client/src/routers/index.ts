import {
  createRouter,
  createWebHistory,
  type RouteRecordRaw,
} from "vue-router";
import api from "@/services/api";

const routes: RouteRecordRaw[] = [
  {
    path: "/login",
    name: "login",
    component: () => import("@/components/pages/LoginPage.vue"),
  },
  {
    path: "/",
    name: "home",
    component: () => import("@/components/pages/LandingPage.vue"),
    meta: { requiresAuth: true },
  },
  {
    path: "/gallery",
    name: "gallery",
    component: () => import("@/components/pages/GalleryPage.vue"),
    meta: { requiresAuth: true },
  },
  {
    path: "/entertainment",
    name: "entertainment",
    component: () => import("@/components/pages/EntertainmentPage.vue"),
    meta: { requiresAuth: true },
  },
];

const pageRouter = createRouter({
  history: createWebHistory(),
  routes,
});

// Global Navigation Guard
pageRouter.beforeEach(async to => {
  if (to.meta.requiresAuth) {
    try {
      const response = await api.get("/check-session");
      if (response.data.valid) {
        return true;
      }
    } catch (error) {
      console.error("Session check failed:", error);
    }

    // Redirect to login if session is invalid
    return { name: "login" };
  }
  return true;
});

export default pageRouter;
