<template>
  <div class="login-container">
    <h1>Login</h1>
    <form @submit.prevent="handleLogin">
      <div>
        <label for="username">Username: </label>
        <input
          id="username"
          v-model="username"
          type="text"
          placeholder="Enter username"
        />
      </div>

      <div>
        <label for="password">Password: </label>
        <input
          id="password"
          v-model="password"
          type="password"
          placeholder="Enter password"
        />
      </div>

      <button type="submit">Sign In</button>
    </form>
    <p
      v-if="errorMessage"
      class="error"
    >
      {{ errorMessage }}
    </p>
  </div>
</template>

<script setup>
import { ref } from "vue";
import { useRouter } from "vue-router";
import api from "@/services/api";

const router = useRouter();

const username = ref("");
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
      username: username.value,
      password: password.value,
    })
    .then(() => {
      // server sets the HTTPOnly cookie with the jwt token
      // recheck the session to verify cookie validity
      checkSession();
    })
    .catch(error => {
      if (error.response && error.response.status === 401) {
        errorMessage.value = "Username or password is incorrect.";
      } else {
        errorMessage.value = "Login failed. Please try again.";
      }
    });
</script>

<style scoped>
.login-container {
  max-width: 400px;
  margin: 0 auto;
  padding: 2rem;
  border: 1px solid #ccc;
  border-radius: 6px;
}

form div {
  margin-bottom: 1rem;
}

button {
  margin-top: 1rem;
}

.error {
  color: red;
  margin-top: 1rem;
}
</style>
