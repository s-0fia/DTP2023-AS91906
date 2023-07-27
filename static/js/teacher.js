const className = document.getElementById("name");
const teacherName = document.getElementById("teachername");
const addClassScreen = document.getElementById("new-class");

function createClass() {
    let cname = className.value;
    let tname = teacherName.value;

    let response = httpQuery(user.id, `new_class${JSON.stringify([cname, tname])}`).status;
    console.log(response.toString());
    switch (response.toString()) {
        case "200":
            toggleAddClassScreen();
            refreshClasses();
            break;
        case "403":
            toggleAddClassScreen();
            alert("You do not have permissions to do this action!");
            showLoader();
            window.location.href = "/";
            break;
        default:
            alert("An error occurred, please try again!");
            break;
    }
}

function toggleAddClassScreen() {
    addClassScreen.style.display = (addClassScreen.style.display == "") ? "flex" : "";
}

function refreshClasses() {
    showLoader();
    let classes = document.getElementsByClassName("class-child");
    for (let i = classes.length - 1; i > 0; i--)
        classes[i].remove();
    loadClasses();
    hideLoader();
}