---
layout: 	page
title: 		PhD
permalink:	/phd/
visible:	false
---

{%	assign aies = "https://dl.acm.org/doi/abs/10.1145/3375627.3375851"	%}
{%	assign krak = "https://proceedings.neurips.cc/paper/2020/hash/dc1913d422398c25c5f0b81cab94cc87-Abstract.html"		%}
{%	assign acs = "https://www.alignmentforum.org/posts/H5iGhDhQBtoDpCBZ2/announcing-the-alignment-of-complex-systems-research-group"	%}
{%	assign psy = "https://socialsciences.nature.com/posts/reversals-in-psychology"		%}
{%	assign fo = "https://forrt.org/reversals"		%}


_Currently on sabbatical._

I'm interested in keeping an eye on AI. By a stroke of luck my PhD advisor is <a href="{{la}}">Laurence Aitchison</a>. I'm part of the Interactive Artificial Intelligence Centre for Doctoral Training at the University of Bristol.<br><br>

<center>
{%	include researches/icons.html	%}
</center>

<br>

### Covid modelling

I started my PhD just before Covid. In a strange turn, a bunch of computer scientists invited me to do a little bit of writing on their big Bayesian model of what policies worked against the bug. I had no epidemiology background. 12 months later, we'd produced a series of <a href="/researches">7 leading papers</a> on important questions which weren't being treated with the proper uncertainty.

Yes, this was the least neglected research topic in the world. Yes, it is strange that noobs could do this. 

<br>

### Probabilistic programming

My original project was _Tensorised Probabilistic Programming_.

Exact inference is intractable in many realistic latent variable models. Of the available approximations, variational inference is fast, but underestimates the variance; and Markov Chain Monte Carlo estimates the variance well but is far too slow in large models (Bishop 2006, Betancourt, 2020). For policy applications, where the variance must be accurate to prevent large irreversible decisions, we thus need new methods. Extending Aitchison's 2019 work on speeding up variational autoencoders, we seek to generalise the use of tensor products for approximate inference.

The end goal is multi-sample inference for any such scheme, and we aim to implement this in a probabilistic programming language (PPL) to maximise usability and impact. There are already ‘tensorised‘ PPLs, in the weak sense of using tensor operations for arbitrary probabilistic programs with one inference scheme (e.g. Bingham et al., 2019, which uses stochastic variational inference for all runs). We seek a further abstraction for any inference scheme. In our project, ‘tensorised’ denotes the tensor products used to achieve the speedup.

The original plan has passed to a colleague, but I'll be back.

<br>

### AI safety

<a href="/ai-risk">Here's my sceptic's guide to AI risk</a>. (For relative sceptics.)

I currently work with the <a href="{{acs}}">Alignment of Complex Systems Group</a>, Charles University.

At the first AI Safety Camp I worked with a team on inverse reinforcement learning, <a href="/grids">designing environments</a> to help us probe the limits of such reward learning. Our work was reused by a <a href="{{krak}}">team</a> at Deepmind and in an <a href="{{aies}}">AIES paper</a>. 

Before starting on probabilistic programming, I played with an odd alternative ML paradigm called _<a href="/ilp">inductive logic programming</a>_. This led to my first paper, a negative result.

I also helped on <a href="/lgfo">a wee paper</a> with a sort of counsel of despair about algorithmic fairness.

I've also <a href="/acais">written</a> about the likely overlap between work on current systems and future systems.


<br>


### Metascience

Over Christmas, instead of studying for quals I started <a href="{{psy}}">listing</a> all the failed replications in psychology I'd heard of. This ballooned into a list of hundreds, and was taken up by the volunteer org <a href="{{fo}}">FORRT</a> for permanent maintenance.

<br>


<style>
.ai-google-scholar, .bris, .ai-orcid {
  display:inline-block;
  text-align: center;
  padding-right: 20px;
  vertical-align:middle;
}


.ai-orcid:hover {
	color: #006800;

}

.ai-google-scholar:hover {
	color: #006800;
}

.bris {
	width: 25%;
	padding: 10px;
}

.bris:hover {
	border: 1px;	
	padding: 9px;
}


</style>