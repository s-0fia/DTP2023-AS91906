// Gets the loader element from the DOM
const loader = document.getElementById("loader");

// Hides the loader
function hideLoader() {
    // Fades out the opacity
    loader.style.opacity = "0";
    // Slides the element up
    loader.style.top = "-100%";

    // Waits for animations to finish then completely
    // ... hides the element for functionality
    setTimeout(() => {
        loader.style.display = "none";
    }, 500);
}

// Shows the loader
function showLoader() {
    // Immediately reinstate the element
    loader.style.display = "block";
    // Fade it in
    loader.style.opacity = "1";
    // Slide it back in to position
    loader.style.top = "0%";
}

// Hides the loader after 500ms for smooth page transition
setTimeout(hideLoader, 500);