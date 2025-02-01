import { createRouter, createWebHistory } from "vue-router";
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
  },
];

const pageRouter = createRouter({
  history: createWebHistory(),
  routes,
});

// Global Navigation Guard
pageRouter.beforeEach((to, from, next) => {
  const token = localStorage.getItem("token");

  if (!token && to.name !== "login") {
    // If there's no token and user isn't already on '/login', redirect to '/login'
    next({ name: "login" });
  } else if (token && to.name === "login") {
    // else if there exists a token redirect to "/"
    next({ name: "home" });
  } else {
    next();
  }
});

export default pageRouter;
