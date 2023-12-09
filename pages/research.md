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

### Conference


<table>
	{%	include researches/rws.html	%}
	{%	include researches/covid_neur.html	%}
</table>

<br>


### Journal

<table>
	{%	include researches/masks.html	%}
	{%	include researches/seasons.html	%}
	{%	include researches/coviddata.html	%}
	{%	include researches/covid2.html	%}
	{%	include researches/lineage.html	%}
	{%	include researches/covid.html	%}
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
	{%  include researches/actadd.html	%}
	{%  include researches/psychofore.html	%}
	{%  include researches/trees.html	%}
	{%	include researches/lgfo.html	%}

</table>

<!-- * _Towards Tensorised Probabilistic Programming_ (2020) -->
<!-- * _<a href="/files/ILP_vs_DL_v0.9.pdf" target="_blank">Comparing Inductive Logic Programming & Deep Learning</a>_ (2020) -->
<!-- * _<a href="/files/" target="_blank">The computational humour of single-word edits</a>_ (2020) -->
<!-- * _<a href="/files/" target="_blank">Failing to Find Proxies for Population Loneliness</a>_ (2020) -->

<br><br><br>


<div class="accordion">
	<h3>Popular</h3>
	<div>
		<ul>
		<!--  -->
		<li><a class="noline" href="{{shallow}}">Shallow review of live agendas in alignment</a> (2023)</li>
		<li> <a class="noline" href="{{ifp}}">Can policymakers trust forecasters?</a> (2023)</li>
		<li> Total rewrite of the <a class="noline" href="{{wiki}}">AI alignment wikipedia page</a> with Mantas Manzeikas and Sören Mindermann (2022)</li>
		<li> <a class="noline" href="{{big3}}">Scoring the Big Three</a> (2022)</li>
		<li> <a class="noline" href="{{kulveit}}">Learning from crisis</a> (2022)</li>
		<li> <a class="noline" href="{{supers}}">Comparing top forecasters and domain experts</a> (2022)</li>
		<li> <a  class="noline" href="{{nat}}">Reversals in psychology</a> (2020)</li>
		<li> <a  class="noline" href="{{academic_safety}}" target="_blank">The academic contribution to AI safety seems large</a> (2020)</li>
		<li> <a  class="noline" href="{{xrisk}}" target="_blank">Existential risk as common cause</a> (2018)</li>
		<li> <a  class="noline" href="/grids" target="_blank">Side effects in Gridworlds</a> (2018). <a href="{{gridcite}}">Developed further</a>.</li>
		</ul>
	</div>
<!--  -->
	<h3>Service</h3>
	<div>
		<ul>
			<li> Briefed the UK Cabinet Office <a href="{{ctf}}">COVID-19 Task Force</a> on mask policy.</li>
			<li> Briefed the UK Cabinet Office on AI economics.</li>
			<li> Reviewer for <a href="{{pnas}}">PNAS</a>, <a href="{{ml}}">Machine Learning</a>, <a href="{{bmj}}">BMJ Global Health</a>, <a href="{{bmc}}">BMC Medicine</a>, <a href="{{aisc}}">AI Safety Camp</a>, <a href="{{pib}}">PIBBSS</a>, <a href="{{fli}}">FLI</a>.</li>
			<li> Created a <a href="{{zotero}}">1000-paper bibliography</a> on every angle of the AI problem.</li>
		</ul>
	</div>
<!--  -->
	<h3>Media</h3>
	<div>
		<ul>
			<li><a href="{{ct}}">Clearer Thinking</a><br><br>			</li>
			<li>Masks: <a href="{{bbc}}">BBC</a>, <a href="{{acxmandate}}">ACX</a>, <a href="{{nyt}}">New York Times</a>, <a href="{{wired}}">Wired</a>, <a href="{{guardian}}">Guardian</a>, <a href="{{mails}}">Mail</a>, <a href="{{mr}}">Marginal Revolution</a>, <a href="{{ag}}">Gelman</a><br><br></li>
			<li>Psychology: <a href="{{nat}}">Nature</a>, <a href="{{ag}}">Gelman</a>, <a href="{{jc}}">Coyne</a>, <a href="{{hertz}}">Everything Hertz</a>, <a href="{{sbs}}">Stronger by Science</a>.		</li>
			<!-- *Gelman  -->
		</ul>
	</div>
<!--  -->
	<h3>Teaching</h3>
	<div>
		<ul>
			<li>2022 - Present: Lead instructor for ESPR. </li>
			<li>2021: Course designer and instructor, <a href="{{espr}}">ESPR</a>. (Metaphilosophy, metamathematics, metascience, cultural literacy, speculative cosmology.)</li>
			<li>2020: Lead TA for <a href="{{algo}}">COMS20010: Algorithms 2</a>.</li>
			<li>2019: TA for the fearsome <a href="{{coms}}">COMS30007: Bayesian Machine Learning</a>.</li>
		</ul>
	</div>
	<!--  -->
	{%	include researches/talks.md 	%}	
	<!--  -->
</div>



<!-- ### Students

* Seth
* Uzay
* Juan
* Yudi
<br><br>
 --><!-- <br> -->

<br><br>



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


