function createIframe() {
          var i = document.createElement("iframe");
          i.src = "{{ReadingFrame}}";
          i.frameborder = "0";
          i.width = "100%";
          i.height = "400px";
          document.getElementById("listFrame").appendChild(i);
};
            
// Check for browser support of event handling capability
if (window.addEventListener) {
    window.addEventListener("load", createIframe, false);
}
else if (window.attachEvent) {
    window.attachEvent("onload", createIframe);
}
else window.onload = createIframe;