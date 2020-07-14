---
layout: 	page
title: 		Research
permalink:	/researches/
visible:	true
---

{%	assign goog = "https://scholar.google.com/citations?user=xC-v_aUAAAAJ"		%}
{%	assign orcid = "https://orcid.org/0000-0002-9298-1488"		%}
{%	assign bris = "http://www.bristol.ac.uk/engineering/people/gavin-l-leech/overview.html"	%}
{%	assign covid = "https://www.medrxiv.org/content/10.1101/2020.05.28.20116129v1"		%}
{%	assign coms = "https://github.com/carlhenrikek/COMS30007/"		%}
{%	assign xrisk = "https://forum.effectivealtruism.org/posts/2pNAPEQ8av3dQyXBX/existential-risk-as-common-cause"	%}
{%	assign git = ""		%}
{%	assign ac = "https://jpswalsh.github.io/academicons/"	%}
{%	assign prolexa = "https://github.com/g-leech/prolexa" 	%}
{%	assign htk = "https://github.com/g-leech/Py2HTK"	%}


<style>
	.frame {
    text-align: center;
	}

	img {
		padding-top:8px;
	    vertical-align: top;
	}
</style>

<!-- https://jpswalsh.github.io/academicons/  -->
<div class="frame">
	<a href="{{goog}}">
		<i class="ai ai-google-scholar ai-3x"></i>
	</a>
	<a href="{{orcid}}">
		<i class="ai ai-orcid ai-3x"></i>
	</a>
	<a href="{{bris}}">
    	<img src="/img/bris_logo.svg" width="15%" />
    </a>
</div>

<br>

<!-- I've published papers on reinforcement learning, epidemiology, AI safety, and   -->


## Journal



<br>

## Conference

* _<a href="/files/LGFO.pdf" target="_blank">Legally Grounded Fairness Objectives</a>_ (2020)
<!-- * _<a href="/files/COVID_conf_.pdf" target="_blank">The Robustness of Effectiveness Estimates of Nonpharmaceutical Interventions Against COVID-19</a>_ (2020) -->

<br>

## Preprint

* _<a href="{{covid}}" target="_blank">The effectiveness & burden of nonpharmaceutical interventions against COVID-19 transmission</a>_ (2020). <a href="#fn:1" id="fnref:1">1</a>
* _<a href="/files/ILP_vs_DL_v0.9.pdf" target="_blank">Comparing Inductive Logic Programming & Deep Learning</a>_ (2020)
<!-- * _<a href="/files/" target="_blank">The computational humour of single-word edits</a>_ (2020) -->
<!-- * _<a href="/files/" target="_blank">Failing to Find Proxies for Population Loneliness</a>_ (2020) -->

<br>

## Software

* <a href="{{prolexa}}">ProlexaPlus</a> (2020): Bringing modern language modelling into Prolog.
* <a href="{{htk}}">Py2HTK</a> (2017): Python wrapper for the Hidden Markov ToolKit.

<br>

## Popular

* <i><a href="{{xrisk}}" target="_blank">Existential risk as common cause</a></i> (2018)
<!-- Momentum -->
<!-- *Gelman  -->

<br>

## Teaching

* 2019: TA for the fearsome _<a href="{{coms}}">COMS30007: Bayesian Machine Learning</a>_
* 2020: <a href="{{git}}">Scipy / PyTorch / Pyro</a> bootcamp for incoming PhD students.


<!-- <br> -->

<!-- ## Patents -->

<!-- <br> -->

<!-- ## Stats -->

<!-- My overall acceptance rate is 0% (2/2) -->


<div class="footnotes">

<ol>
    <!-- 1 -->
    <li class="footnote" id="fn:1">
    	<i>Read if</i>: you want to know what government actions worked in the pandemic. You're curious about what it takes to outdo standard epidemiology. You want to see what computer scientists do when they attack en masse.<br><br>
    	<!--  -->
    	<!-- <i>My contribution</i>: I did most of the writeup, the policy stuff, and the limitations. -->
	</li>

</ol>

</div>


<br><br>
<small>Credit to James Walsh for the <a href="{{ac}}">academic SVGs</a>.</small>

{%	include padder.html 	howMuch=5 	%}