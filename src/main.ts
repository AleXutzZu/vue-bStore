import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import {createRouter, createWebHistory} from "vue-router";

const routes = [
    {
        path: "/",
        name: "Home",
        component: Home,
    },
    {
        path: "/add",
        name: "Add",
        component: Add,
    },
    {
        path: "/search",
        name: "Search",
        component: Search,
    }
];

const router = createRouter({ history: createWebHistory(), routes })

createApp(App).use(router).mount("#app");
