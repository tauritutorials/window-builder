const { invoke } = window.__TAURI__.core;

window.addEventListener("DOMContentLoaded", () => { 

    document.querySelector("#new-window").addEventListener("click", () => {
        invoke('new_window');
    });

});
