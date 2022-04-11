---
layout: 	page
title: 		Research
nav: 		Research
permalink:	/researches/
visible:	true
---

{%	include researches/links.md	%}
{%	include researches/icons.html	%}

<style>
	{% include researches/papers.css %}
</style>


<br>

## Journal

<table>
	{%	include researches/coviddata.html	%}
	{%	include researches/covid2.html	%}
	{%	include researches/lineage.html	%}
	{%	include researches/covid.html	%}
</table>

<br>

## Conference

<table>
	{%	include researches/covid_neur.html	%}
</table>

<br>

## Workshop

<table>
	{%	include researches/forrt.html	%}
	{%	include researches/ilp.html	%}
</table>

<br>

## Preprint

<table>
	<!-- 	include researches/trees.html	%} -->
	{%	include researches/masks.html	%}
	{%	include researches/seasons.html	%}
	{%	include researches/lgfo.html	%}

</table>

<!-- * _Towards Tensorised Probabilistic Programming_ (2020) -->
<!-- * _<a href="/files/ILP_vs_DL_v0.9.pdf" target="_blank">Comparing Inductive Logic Programming & Deep Learning</a>_ (2020) -->
<!-- * _<a href="/files/" target="_blank">The computational humour of single-word edits</a>_ (2020) -->
<!-- * _<a href="/files/" target="_blank">Failing to Find Proxies for Population Loneliness</a>_ (2020) -->

<br>

## Software

* <a  class="noline" href="{{maskscode}}">masks_v_mandates</a> (2021): Probabilistic programming for epidemic modelling and effect estimation. End to end with data getters.
* <a  class="noline" href="{{prolexa}}">ProlexaPlus</a> (2020): Bringing modern language modelling into Prolog for some reason.
* <a  class="noline" href="{{htk}}">Py2HTK</a> (2017): Python wrapper for the Hidden Markov ToolKit.

<br>

## Popular

* <i><a  class="noline" href="{{kulveit}}">Learning from crisis</a></i> (2022)
* <i><a  class="noline" href="{{supers}}">Comparing top forecasters and domain experts</a></i> (2022)
* <i><a  class="noline" href="/psych" target="_blank">Reversals in psychology</a></i> (2020)
* <i><a  class="noline" href="{{academic_safety}}" target="_blank">The academic contribution to AI safety seems large</a></i> (2020)
* <i><a  class="noline" href="{{xrisk}}" target="_blank">Existential risk as common cause</a></i> (2018)
* <i><a  class="noline" href="/grids" target="_blank">Side effects in Gridworlds</a></i> (2018). <a href="{{gridcite}}">Developed further</a>.

<!-- *Gelman  -->

<br>

## Service

* Briefed the UK Cabinet Office <a href="{{ctf}}">COVID-19 Task Force</a> on masks.
* Reviewer for <a href="{{pnas}}">PNAS</a>, <a href="{{ml}}">Machine Learning</a>, <a href="{{bmj}}">BMJ Global Health</a>, <a href="{{aisc}}">AI Safety Camp</a>.

<br>

## Media

* Masks: <a href="{{bbc}}">BBC</a>, <a href="{{acxmandate}}">ACX</a>, <a href="{{nyt}}">New York Times</a>, <a href="{{wired}}">Wired</a>, <a href="{{guardian}}">Guardian</a>, <a href="{{mails}}">Mail</a>, <a href="{{mr}}">Marginal Revolution</a>, <a href="{{ag}}">Gelman</a>
* Psychology: <a href="{{nat}}">Nature</a>, <a href="{{ag}}">Gelman</a>, <a href="{{jc}}">Coyne</a>, <a href="{{hertz}}">Everything Hertz</a>, <a href="{{sbs}}">Stronger by Science</a>.

<br>

## Teaching

* 2019: TA for the fearsome _<a href="{{coms}}">COMS30007: Bayesian Machine Learning</a>_
* 2020: Lead TA for _<a href="{{algo}}">COMS20010: Algorithms 2</a>_.
* 2021: Designer and instructor for two courses at <a href="{{espr}}">ESPR</a>.

<!-- <br> -->

<!-- ## Patents -->

<!-- <br> -->

<!-- ## Stats -->

<!-- My acceptance rate is 50% (4/8) -->

<br><br>

{%    include mc.html  %}


<br><br><br>

<small>Credit to James Walsh for the <a href="{{ac}}">academic SVGs</a>.</small>

{%	include padder.html 	howMuch=3 	%}


<script>
	// function drop(el) {
 //    	// document.getElementById("myDropdown").classList.toggle("show");
 //    	el.classList.toggle("show");
 //  	}

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

