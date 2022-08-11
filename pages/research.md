---
layout: 	page
title: 		Research
nav: 		Research
permalink:	/researches/
visible:	true
---

{%	include researches/links.md	%}
{%	include researches/icons.html	%}
{%  include js/lazyFrame.html  %}


<style>
	{% include researches/papers.css %}
</style>    

<br>

### Journal

<table>
	{%	include researches/masks.html	%}
	{%	include researches/coviddata.html	%}
	{%	include researches/covid2.html	%}
	{%	include researches/lineage.html	%}
	{%	include researches/covid.html	%}
</table>

<br>

### Conference

<table>
	{%	include researches/covid_neur.html	%}
</table>

<br>

### Workshop

<table>
	{%	include researches/forrt.html	%}
	{%	include researches/ilp.html	%}
</table>

<br>

### Preprint

<table>
	<!-- 	include researches/trees.html	%} -->
	{%	include researches/seasons.html	%}
	{%	include researches/lgfo.html	%}

</table>

<!-- * _Towards Tensorised Probabilistic Programming_ (2020) -->
<!-- * _<a href="/files/ILP_vs_DL_v0.9.pdf" target="_blank">Comparing Inductive Logic Programming & Deep Learning</a>_ (2020) -->
<!-- * _<a href="/files/" target="_blank">The computational humour of single-word edits</a>_ (2020) -->
<!-- * _<a href="/files/" target="_blank">Failing to Find Proxies for Population Loneliness</a>_ (2020) -->

<br><br>


### Popular

<!-- * Major rewrite of the <a class="noline" href="">AI alignment wikipedia page</a>, with Brian Christian, Mantas Manzeikas, and Sören Mindermann (2022)<br><br> -->
* <a class="noline" href="{{big3}}">Scoring the Big Three</a> (2022)<br><br>
* <a class="noline" href="{{kulveit}}">Learning from crisis</a> (2022)<br><br>
* <a class="noline" href="{{supers}}">Comparing top forecasters and domain experts</a> (2022)<br><br>
* <a  class="noline" href="/psych" target="_blank">Reversals in psychology</a> (2020)<br><br>
* <a  class="noline" href="{{academic_safety}}" target="_blank">The academic contribution to AI safety seems large</a> (2020)<br><br>
* <a  class="noline" href="{{xrisk}}" target="_blank">Existential risk as common cause</a> (2018)<br><br>
* <a  class="noline" href="/grids" target="_blank">Side effects in Gridworlds</a> (2018). <a href="{{gridcite}}">Developed further</a>.

<!-- *Gelman  -->

<br>

### Service

* Briefed the UK Cabinet Office <a href="{{ctf}}">COVID-19 Task Force</a> on masks.<br><br>
* Reviewer for <a href="{{pnas}}">PNAS</a>, <a href="{{ml}}">Machine Learning</a>, <a href="{{bmj}}">BMJ Global Health</a>, <a href="{{aisc}}">AI Safety Camp</a>.

<br>

### Media

* Masks: <a href="{{bbc}}">BBC</a>, <a href="{{acxmandate}}">ACX</a>, <a href="{{nyt}}">New York Times</a>, <a href="{{wired}}">Wired</a>, <a href="{{guardian}}">Guardian</a>, <a href="{{mails}}">Mail</a>, <a href="{{mr}}">Marginal Revolution</a>, <a href="{{ag}}">Gelman</a><br><br>
* Psychology: <a href="{{nat}}">Nature</a>, <a href="{{ag}}">Gelman</a>, <a href="{{jc}}">Coyne</a>, <a href="{{hertz}}">Everything Hertz</a>, <a href="{{sbs}}">Stronger by Science</a>.

<br>

### Teaching

* 2022: Lead instructor for ESPR.<br><br>
* 2021: Designer and instructor for two courses at <a href="{{espr}}">ESPR</a>.<br><br>
* 2020: Lead TA for _<a href="{{algo}}">COMS20010: Algorithms 2</a>_.<br><br>
* 2019: TA for the fearsome _<a href="{{coms}}">COMS30007: Bayesian Machine Learning</a>_.<br><br>

<!-- <br> -->

<!-- ## Patents -->

<!-- <br> -->

<!-- ## Stats -->

<!-- My acceptance rate is 50% (4/8) -->

<br>


<small>Credit to James Walsh for the <a href="{{ac}}">academic SVGs</a>.</small>

{%	include padder.html 	howMuch=3 	%}

<!-- THE POINT DROPDOWN -->
<script>
  	function drop(id) {
    	document.getElementById(id).classList.toggle("show");
  	}
	// // Close the dropdown menu if the user clicks outside of it
  	window.onclick = function(event) {
	    if (!event.target.matches('.dropped')) {
	      var dropdowns = document.getElementsByClassName("dropdown-content");
	      var i;
	      for (i = 0; i < dropdowns.length; i++) {
	        var openDropdown = dropdowns[i];
	        if (openDropdown.classList.contains('show')) {
	          openDropdown.classList.remove('show');
	        }
	      }
	    }
	}
</script>


