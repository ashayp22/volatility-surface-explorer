// When the user clicks on the button, open the modal
document.getElementById("howToButton").onclick = function () {
    document.getElementById("howToModal").style.display = "block";
}

// When the user clicks on <span> (x), close the modal
document.getElementsByClassName("close")[0].onclick = function () {
    document.getElementById("howToModal").style.display = "none";
}

// When the user clicks anywhere outside of the modal, close it
window.onclick = function (event) {
    const modal = document.getElementById("howToModal");
    if (event.target == modal) {
        modal.style.display = "none";
    }
}