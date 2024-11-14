const { invoke } = window.__TAURI__.core;

window.addEventListener("DOMContentLoaded", async () => {
    document.querySelector("#new-window").addEventListener("click", () => {
        invoke("new_window");
    });

    document
        .querySelector("#new-window-or-focus")
        .addEventListener("click", () => {
            invoke("new_window_or_focus");
        });


    document
        .querySelector("#options")
        .addEventListener("click", () => {
            invoke("options");
        });
});
