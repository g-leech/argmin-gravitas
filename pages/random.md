---
layout:     page
title:      Random
nav:        Random
permalink:  /rando-calrissian/
visible:    false
---

<center>
    <a id="rando" class="post-link">a random post</a>
</center>

<script>
    function choose(l) {
        var i = Math.floor(Math.random() * l.length);
        return l[i]
    }

    var l = ["stopping", "google", "siderea", "hplr", "maniac", "heron", "experi", "med", "mogos", "bright", "spotify", "svengali", "dark-math", "pg", "stats", "clearer", "actadd", "swift", "ifp", "disco", "ladder", "hype", "association", "horses", "country", "aaronson", "sweep", "big3", "nownost", "benchmarks", "frank", "forecasters", "phil-harvey", "memoria", "logoff", "jvn", "palmer", "tools", "fermi", "codes", "masks", "gist", "vegetables", "ai-risk", "lgfo", "sceptic", "meat", "sincerity", "adventure", "graphs", "stims", "cornaro", "better-maths", "nation-sound", "culture", "acais", "ignorance", "self-help", "ai-ethics", "sites", "games-of-life", "stuff", "ilp", "strength", "watts", "psych", "uncritical", "pills", "odyssey", "dota", "insurance", "introspect", "consent", "corp", "scarcity", "meditation", "why-yoga", "ficciones", "libya", "ou", "fivebooks", "perelman", "homicide", "blindsight", "x-for-all", "browser", "aubyn", "broadness", "demarcation", "anti-ethics", "curiosity", "strangers", "einstein", "flu", "grids", "esa-deaths/", "ceiling", "enigma", "first-computers", "no-philosopher", "you", "education", "uranverein", "genes-out/", "barrier", "pi-tau/", "history", "data-science/", "zhang", "anthropology/", "controversy/", "eagox", "worst", "automatic/", "overheads", "vegan", "magic", "ages", "accommodation/", "piety", "grapes", "quality", "conversion/", "relative", "anaesthetatron", "maximum/", "aesthetics-and-ethics", "ea-origin", "bootstraps", "nofx", "hume", "instrument", "aid/", "frege", "brainwash", "russell", "scotch", "econ-life", "rubin", "dreaming", "england", "pomo", "jest", "econ", "mysticism", "hoom", "tractatus", "punk", "love"]
    post = choose(l);
    url = 'https://gleech.org/' + post
    a = document.getElementById("rando");
    a.setAttribute('href', url);
</script>


