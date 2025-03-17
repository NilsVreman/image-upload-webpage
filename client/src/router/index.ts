import { createRouter, createWebHistory } from "vue-router";
import api from "@/services/api";
import LoginPage from "@/components/auth/LoginPage.vue";
import ImagePage from "@/components/image/ImagePage.vue";

const routes = [
  {
    path: "/login",
    name: "login",
    component: LoginPage,
  },
  {
    path: "/",
    name: "home",
    component: ImagePage,
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
