import type { UserLogin, UserRegister, ForgotPassword } from "@/composables/auth";
import router from "@/router";
import { defineStore } from "pinia";


export const useAuth = defineStore("useAuth", {
    state: () => ({
        isAuthenticated: false,
        user: '',
    }),
    actions: {
        async login(userlogin: UserLogin): Promise<boolean> {
            try {
                const response = await fetch("/api/login", {
                    method: "POST",
                    headers: {
                        "Content-Type": "application/json",
                    },
                    body: JSON.stringify(userlogin),
                });

                if (!response.ok) {
                    throw new Error("Login failed");
                }

                const data = await response.json();
                console.log("Login response:", data);
                localStorage.setItem("id", data.id);
                localStorage.setItem("token", data.token);
                router.back();
                return true;
            } catch (error) {
                console.error("Login error:", error);
                return false;
            }
        },

        async forgotPassword(email: ForgotPassword): Promise<boolean> {
            try {
                const response = await fetch("/api/forgot-password", {
                    method: "POST",
                    headers: {
                        "Content-Type": "application/json",
                    },
                    body: JSON.stringify(email),
                });

                if (!response.ok) {
                    throw new Error("Forgot password request failed");
                }

                const data = await response.json();
                console.log("Forgot password response:", data);
                return true;
            } catch (error) {
                console.error("Forgot password error:", error);
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
            localStorage.removeItem("id");
            
        },

        isAuthenticated(): boolean {
            return !!localStorage.getItem("token");
        },
    },
})