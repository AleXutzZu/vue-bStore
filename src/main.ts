import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import Home from "./components/Home.vue"
import Add from "./components/Add.vue"
import Search from "./components/Search.vue"
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

export interface Book {
    title: string,
    author: string,
    language: string,
    status?: string,
}

const router = createRouter({ history: createWebHistory(), routes })

createApp(App).use(router).mount("#app");
