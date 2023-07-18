const loader = document.getElementById("loader");

function hideLoader() {
    loader.style.opacity = "0";
    loader.style.top = "-100%";
    setTimeout(() => {
        loader.style.display = "none";
    }, 500);
}

function showLoader() {
    loader.style.display = "block";
    loader.style.opacity = "1";
    loader.style.top = "0%";
}

setTimeout(() => hideLoader(), 500);