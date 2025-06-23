import type { UserRegister } from "@/composables/auth";
import router from "@/router";
import { defineStore } from "pinia";

export const useAuth = defineStore("useAuth", {
    state: () => ({
        token: localStorage.getItem("token"),
        isAuthenticated: false,
        user: '',
    }),
    actions: {
        async login(username: string, password: string): Promise<boolean> {
            try {
                const response = await fetch("/api/login", {
                    method: "POST",
                    headers: {
                        "Content-Type": "application/json",
                    },
                    body: JSON.stringify({ username, password }),
                });

                if (!response.ok) {
                    throw new Error("Login failed");
                }

                const data = await response.json();
                localStorage.setItem("token", data.token);
                router.push({ name: "dashboard" });
                return true;
            } catch (error) {
                console.error("Login error:", error);
                return false;
            }
        },

        async register(registerdata: UserRegister): Promise<boolean> {
            try {
                console.log("Registering user:", registerdata);
                const response = await fetch("/api/register", {
                    method: "POST",
                    headers: {
                        "Content-Type": "application/json",
                    },
                    body: JSON.stringify(registerdata),
                });

                if (!response.ok) {
                    throw new Error("Registration failed");
                }

                const data = await response.json();
                localStorage.setItem("token", data.token);
                router.push({ name: "login" });
                return true;
            } catch (error) {
                console.error("Registration error:", error);
                return false;
            }
        },

        logout() {
            localStorage.removeItem("token");
        },

        isAuthenticated(): boolean {
            return !!localStorage.getItem("token");
        },
    },
})