---
layout: 	page
title: 		Research
permalink:	/researches/
visible:	true
---

{%	assign goog = "https://scholar.google.com/citations?user=xC-v_aUAAAAJ"		%}
{%	assign orcid = "https://orcid.org/0000-0002-9298-1488"		%}
{%	assign bris = "http://www.bristol.ac.uk/engineering/people/gavin-l-leech/overview.html"	%}
{%	assign covid = "https://science.sciencemag.org/content/early/2020/12/15/science.abd9338"		%}
{%	assign neurips_covid = "https://papers.nips.cc/paper/2020/file/8e3308c853e47411c761429193511819-Paper.pdf"	%}
{%	assign coms = "https://github.com/carlhenrikek/COMS30007/"		%}
{%	assign xrisk = "https://forum.effectivealtruism.org/posts/2pNAPEQ8av3dQyXBX/existential-risk-as-common-cause"	%}
{%	assign academic_safety = "https://forum.effectivealtruism.org/posts/8ErtxW7FRPGMtDqJy/the-academic-contribution-to-ai-safety-seems-large"	%}
{%	assign git = ""		%}
{%	assign ac = "https://jpswalsh.github.io/academicons/"	%}
{%	assign prolexa = "https://github.com/So-Cool/prolexa/blob/prolexa-plus/Readme.md#prolexa-plus" 	%}
{%	assign htk = "https://github.com/g-leech/Py2HTK"	%}
{%	assign algo = "https://www.bris.ac.uk/unit-programme-catalogue/UnitDetails.jsa?ayrCode=20%2F21&unitCode=COMS20010"	%}
{%	assign lgfo = "https://arxiv.org/abs/2009.11677"		%}

<style>
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

</style>

<!-- https://jpswalsh.github.io/academicons/  -->
<div class="frame">
	<a class="nolink" href="{{goog}}">
		<i class="ai ai-google-scholar ai-3x"></i>
	</a>
	<a class="nolink" href="{{orcid}}">
		<i class="ai ai-orcid ai-3x"></i>
	</a>
	<a class="nolink" href="{{bris}}">
    	<img src="/img/bris_logo.svg" width="15%" />
    </a>
</div>

<br>



## Conference

<table>
	<tr>
		<td class="logo" style="padding-bottom: 10px">
			<a href="{{neurips_covid}}"><img src="/img/papers/1.png" /></a>
		</td>
		<td style="padding-left: 5px">
			<i><a href="{{neurips_covid}}" target="_blank">How Robust are Estimated Effects of Nonpharmaceutical Interventions against COVID-19?</a></i> (2020), NeurIPS Spotlight paper, 4th author / 10.
		</td>
	</tr>
	<!--  -->
	<tr>
		<td class="logo">
			<img src="/img/papers/ilp.png" />
		</td>
		<td style="padding-left: 5px">
			<i>Safety Properties of Inductive Logic Programming</i> (2020), AAAI SafeAI workshop, 1st author / 3. 
		</td>
	</tr>
</table>


<br>

## Journal

<table>
	<tr>
		<td class="logo">
			<a href="{{covid}}"><img src="/img/papers/science-covid.png" /></a>
		</td>
		<td style="padding-left: 5px">
			<i><a href="{{covid}}" target="_blank">Inferring the effectiveness of government interventions against COVID-19</a></i> (2020), <i>Science</i>, 8th author / 19.
		<br><br>
		<a href="#fn:1" id="fnref:1">1</a> 
		</td>
	</tr>
	
</table>




<br>

## Preprint

<table>
	<!--  -->
	<tr>
		<td class="logo">
			<img src="/img/papers/lgfo.jpg" />
		</td>
		<td style="padding-left: 5px">
			<a href="{{lgfo}}" target="_blank">Legally Grounded Fairness Objectives</a> (2020), 2nd author / 3.
		</td>
	</tr>
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

* <i><a href="{{xrisk}}" target="_blank">Existential risk as common cause</a></i> (2018)
* <i><a href="{{academic_safety}}" target="_blank">The academic contribution to AI safety seems large</a></i> (2020)

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


<div class="footnotes">

<ol>
    <!-- 1 -->
    <li class="footnote" id="fn:1">
    	<i>Read if</i>: you want to know what government moves worked in the first wave of the pandemic. You want to see how sensitive results are. You want to see what computer scientists can do when they attack en masse.<br><br>
    	<!--  -->
    	<!-- <i>My contribution</i>: I did most of the writeup, the policy stuff, and the limitations. -->
	</li>

</ol>

</div>


<br><br>
<small>Credit to James Walsh for the <a href="{{ac}}">academic SVGs</a>.</small>

{%	include padder.html 	howMuch=5 	%}