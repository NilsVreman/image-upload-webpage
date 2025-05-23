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
    name: "landing",
    component: () => import("@/components/pages/LandingPage.vue"),
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

// FIXME: Need to figure out a way to resend some validation request if timeout occurs
export default pageRouter;
