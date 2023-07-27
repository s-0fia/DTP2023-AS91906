let user = getSignInCookie();
let welcome = document.getElementById("welcomeMessage");
welcome.innerText = `Welcome ${user.firstName}!`;

function signOutClick() {
    let expiry = "Thu, 01 Jan 1970 00:00:00 UTC";

    document.cookie = `id=; expires=${expiry}; path=/;`;
    document.cookie = `email=; expires=${expiry}; path=/;`;
    document.cookie = `firstName=; expires=${expiry}; path=/;`;
    document.cookie = `lastName=; expires=${expiry}; path=/;`;
    document.cookie = `pictureUrl=; expires=${expiry}; path=/;`;

    // redirect to the signin page
    window.location.href = "./welcome";
}

class Classroom {
    constructor(res) {
        this.UID = res.uid;
        this.name = res.name;
        this.teacherUIDs = res.teachers_uids;
        this.teacherName = res.teacher_name;
    }
}

function classroomInnerToOuterArray(resArr) {
    let outArr = Array();

    for (let i = 0; i < resArr.length; i++)
        outArr.push(new Classroom(resArr[i]));

    return outArr;
}

function classClick(classUID) {
    showLoader();
    setTimeout(() => {
        window.location.href = `/c/${classUID}`;
    }, 500);
}

function loadClasses() {
    let response = classroomInnerToOuterArray(
        JSON.parse(
            httpQuery(user.id, "get_classes").text
        )
    );

    for (let i = 0; i < response.length; i++) {
        let classroom = response[i];
        let fragment = create(
            `<div id="${classroom.UID}" class="class-child" onclick="classClick('${classroom.UID}');">` +
                `<h1><a>${classroom.name}</a></h1>` +
                `<h2>Teacher: ${classroom.teacherName}</h2>` +
            `</div>`
        );

        document.getElementById('classes')
            .appendChild(fragment);
    }
}

const create = (htmlStr) => {
    var frag = document.createDocumentFragment(),
        temp = document.createElement('div');
    temp.innerHTML = htmlStr;
    while (temp.firstChild) {
        frag.appendChild(temp.firstChild);
    }
    return frag;
}

loadClasses();