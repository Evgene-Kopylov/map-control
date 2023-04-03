setTimeout(function(){
    var el = $('#loading_screen.ring');
    if (parseFloat(el.css("opacity")) > 0) {
        setInterval(function () {
            if (parseFloat(el.css("opacity")) > 0) {
                el.css("opacity", el.css("opacity") - 0.1);
            };
            if (parseFloat(el.css("opacity")) <= 0) {
                el.remove();
                return
            }
        }, 70);
    }
},5000);