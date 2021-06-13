---
layout: 	page
title: 		Research
permalink:	/researches/
visible:	true
---

{%	include researches/links.md	%}
{%	include researches/icons.html	%}

<br>

## Conference

<table>
	{%	include researches/covid_neur.html	%}
	{%	include researches/ilp.html	%}
</table>


<br>

## Journal

<table>
	{%	include researches/covid.html	%}
</table>




<br>

## Preprint

<table>
	<!-- 	include researches/trees.html	%} -->
	<!-- 	include researches/masks.html	%} -->
	{%	include researches/seasons.html	%}
	{%	include researches/lineage.html	%}
	{%	include researches/covid2.html	%}
	{%	include researches/lgfo.html	%}

</table>

<!-- * _Towards Tensorised Probabilistic Programming_ (2020) -->
<!-- * _<a href="/files/ILP_vs_DL_v0.9.pdf" target="_blank">Comparing Inductive Logic Programming & Deep Learning</a>_ (2020) -->
<!-- * _<a href="/files/" target="_blank">The computational humour of single-word edits</a>_ (2020) -->
<!-- * _<a href="/files/" target="_blank">Failing to Find Proxies for Population Loneliness</a>_ (2020) -->

<br>

## Software

* <a href="{{prolexa}}">ProlexaPlus</a> (2020): Bringing modern language modelling into Prolog for some reason.
* <a href="{{htk}}">Py2HTK</a> (2017): Python wrapper for the Hidden Markov ToolKit.

<br>

## Popular

* <i><a href="{{academic_safety}}" target="_blank">The academic contribution to AI safety seems large</a></i> (2020)
* <i><a href="{{xrisk}}" target="_blank">Existential risk as common cause</a></i> (2018)
* <i><a href="/grids" target="_blank">Side effects in Gridworlds</a></i> (2018). <a href="{{gridcite}}">Developed further</a>.

<!-- *Gelman  -->

<br>

## Teaching

* 2019: TA for the fearsome _<a href="{{coms}}">COMS30007: Bayesian Machine Learning</a>_
* 2020: Lead TA for _<a href="{{algo}}">COMS20010: Algorithms 2</a>_.


<!-- <br> -->

<!-- ## Patents -->

<!-- <br> -->

<!-- ## Stats -->

<!-- My acceptance rate is 50% (4/8) -->



<br><br>
<small>Credit to James Walsh for the <a href="{{ac}}">academic SVGs</a>.</small>

{%	include padder.html 	howMuch=5 	%}


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

<style>
	.dropdown {
	  display: inline-block;
	  padding: 0;
	}

	.dropdown-content {
	  display: none;
	  z-index: 1;
	}

	.dropdown-content {
	  padding-left: 16px;
	  font-size: 11px;
	}

	.show {
		display:block;
	}

	.frame {
    text-align: center;
	}

	img {
		padding-top:8px;
	    vertical-align: top;
	}

	.logo {
		width: 11%;
	}

	tr>td {
  		padding-bottom: 1em;
	}

	.me {
		font-weight: bold;
		font-size: 12px;
	}

</style>