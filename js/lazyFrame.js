function createIframe() {
          var i = document.createElement("iframe");
          i.src = "https://docs.google.com/spreadsheets/d/1EuFXFPpzRCG9Vjsb8zYPMmAPAnqM2bd_U0xfmD9Ctig/pubhtml?gid=0&amp;single=true&amp;widget=false&amp;headers=false&amp;chrome=false"
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