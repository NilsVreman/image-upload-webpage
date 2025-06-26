<template>
  <main class="login-page">
    <section
      class="login-card"
      role="form"
    >
      <h1 class="title">Login</h1>
      <form
        novalidate
        @submit.prevent="handleLogin"
      >
        <label class="field">
          <span class="field-label">Password</span>
          <input
            id="password"
            v-model="password"
            type="password"
            placeholder="Password"
            required
            autocomplete="current-password"
          />
        </label>

        <button
          type="submit"
          class="btn"
        >
          Sign In
        </button>
      </form>
      <p
        v-if="errorMessage"
        class="error"
        aria-live="assertive"
      >
        {{ errorMessage }}
      </p>
    </section>
  </main>
</template>

<script setup>
import { ref } from "vue";
import { useRouter } from "vue-router";
import api from "@/services/api";

const router = useRouter();

const password = ref("");
const errorMessage = ref("");

const checkSession = () =>
  api
    .get("/check-session")
    .then(response => {
      if (response.data.valid) {
        router.push("/");
      }
    })
    .catch(err => {
      console.log("Session check error:", err);
    });

const handleLogin = () =>
  api
    .post("/login", {
      password: password.value,
    })
    .then(() => {
      // server sets the HTTPOnly cookie with the jwt token
      // recheck the session to verify cookie validity
      checkSession();
    })
    .catch(error => {
      if (error.response && error.response.status === 401) {
        errorMessage.value = "Password is incorrect.";
      } else {
        errorMessage.value = "Login failed. Please try again.";
      }
    });
</script>

<style scoped>
/* centres any full-screen form page */
.login-page {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 80vh;
  padding: 1rem;
  background: var(--body-bg);
}

/* glassmorphism card */
.login-card {
  width: 100%;
  max-width: 420px;
  background: var(--card-bg);
  border: var(--card-border);
  border-radius: 1rem;
  backdrop-filter: blur(16px) saturate(180%);
  -webkit-backdrop-filter: blur(16px) saturate(180%);
  padding-block: 2.5rem;
  padding-inline: 2rem;
  box-shadow: 0 8px 24px rgba(0 0 0 / 0.12);
}

/* typography */
.login-card .title {
  margin: 0 0 1.5rem;
  font-weight: 600;
  text-align: center;
}

/* field block */
.field {
  display: flex;
  flex-direction: column;
  margin-bottom: 1.25rem;
}
.field-label {
  margin-bottom: 0.5rem;
  font-size: 0.95rem;
  font-weight: 500;
  color: var(--text-color);
}
.field-input {
  padding: 0.65rem 0.8rem;
  border: 1px solid var(--border-color);
  border-radius: 0.5rem;
  background: transparent;
  color: inherit;
  font-size: 1rem;
  transition:
    border-color 0.2s,
    box-shadow 0.2s;
}
.field-input:focus {
  border-color: var(--button-color);
  box-shadow: 0 0 0 3px var(--button-overlay);
  outline: none;
}

/* green action button */
.btn {
  width: 100%;
  padding: 0.75rem 1rem;
  background: var(--button-color);
  border: none;
  border-radius: 0.5rem;
  color: #fff;
  font-weight: 600;
  font-size: 1rem;
  cursor: pointer;
  transition:
    background 0.15s,
    transform 0.15s;
}
.btn:hover {
  background: var(--button-hover-color);
}
.btn:active {
  transform: scale(0.97);
}

/* error feedback */
.error {
  margin-top: 1rem;
  color: hsl(0 75% 60%);
  font-size: 0.875rem;
  text-align: center;
}
</style>
