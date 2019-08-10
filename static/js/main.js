var free_memory = document.getElementById("fmem");
var used_memory = document.getElementById("umem");
var total_memory = document.getElementById("tmem");
var cpu = document.getElementById("cpu");


setInterval(function () {
    var name = document.getElementById("context").innerHTML;

    
    var ourRequest = new XMLHttpRequest();
    ourRequest.open('GET', 'http://localhost:8000/stats/' + name);
    ourRequest.onload = function () {
        var ourData = JSON.parse(ourRequest.responseText);
        renderHTML(ourData);
    };
    ourRequest.send();

}, 5000);

function renderHTML(data) {
    free_memory.innerHTML = data.freemem;
    used_memory.innerHTML = data.usedmem;
    total_memory.innerHTML = data.totalmem;
    cpu.innerHTML = data.cpuu;

    updateConfigByMutating(myRadialGauge1, data.cpuu);
    updateConfigByMutating(myRadialGauge2, data.usedmem);
}

function updateConfigByMutating(chart, value) {
    chart.config.data.datasets[0].data = [value];
    chart.update();
}

/* https://www.w3schools.com/howto/howto_js_tabs.asp */
function openTab(evt, tabName) {
    // Declare all variables
    var i, tabcontent, tablinks;

    // Get all elements with class="tabcontent" and hide them
    tabcontent = document.getElementsByClassName("tabcontent");
    for (i = 0; i < tabcontent.length; i++) {
        tabcontent[i].style.display = "none";
    }

    // Get all elements with class="tablinks" and remove the class "active"
    tablinks = document.getElementsByClassName("tablinks");
    for (i = 0; i < tablinks.length; i++) {
        tablinks[i].className = tablinks[i].className.replace(" active", "");
    }

    // Show the current tab, and add an "active" class to the button that opened the tab
    document.getElementById(tabName).style.display = "block";
    evt.currentTarget.className += " active";
}