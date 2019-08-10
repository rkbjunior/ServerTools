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

}, 2500);

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