---
layout: 	page
title: 		Projects
permalink:	/projects/
visible:	true
---

{%		assign arch = "http://gleech.org/archive"		%}
{%		assign sheet = "https://docs.google.com/spreadsheets/d/1UPP_74QYHZ3wysRL9Oe7Qr8SNIyaa9A8E957S5F3NAY/pubhtml?chrome=false"		%}
{%		assign rotgone = "https://github.com/g-leech/rotgone"		%}
{%		assign hanson = "https://docs.google.com/spreadsheets/d/11wPNvIqjDglk7cWaSlG8ZOt2HjAJTx_19iTHu8w9T8s/edit?usp=sharing"		%}



My current projects are:<br>

<div id="listFrame"></div>


{%  include js/lazyFrame.html %}
<script>  
    var src = "{{sheet}}";
    definiteEvent( createIframe, [src, "listFrame"] ); 
</script>

<br><br>


<div class="accordion">
	<h3>Done</h3>
	<div>
	<ul>
		<li><a href="{{arch}}">These</a>.</li>
		<li><i><a href="{{rotgone}}">rotgone</a></i>, a little bash module for batching hyperlinks into the Internet Archive.</li>
		<li><i>Dataset</i>: Robin Hanson's <i>Age of Em</i> <a href={{hanson}}>at the proposition level</a>.</li>
	</ul>
	</div>
</div>
<br><br>