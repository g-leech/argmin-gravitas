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
{%	assign wiki = "https://en.wikipedia.org/wiki/AI_alignment"		%}
{%	assign la = "https://www.gatsby.ucl.ac.uk/~laurence/" %}
{%	assign tree = "https://neurotree.org/neurotree/tree.php?pid=933951&pnodecount=12&cnodecount=2"	%}

<style>
	{%	include researches/papers.css	%}
</style>


<br>

I did a PhD in machine learning at Bristol with <a href="{{la}}">Laurence Aitchison</a>, 2019-2024. (Includes a year of mandatory classes and a year off in which I started <a href="https://arbresearch.com">Arb</a>.)<br>

I went in wanting to work on AI safety. True to form, I instead ended up with a grab-bag of fields and topics: approximate Bayesian inference, Covid epidemiology, metascience, the methodology of the social sciences, scientific malpractice, inductive logic, algorithmic fairness and (of course) large language models. Some safety work in there if you squint. But I published enough, so the resulting thesis is <i><a href="/files/thesis.pdf">Methods Failing the Data, Data Failing the Methods</a></i>.

I was very lucky. It looks like a success on the usual bad measures (h-index, impact factors, top conferences, first-author pubs, an academic job offer at the end). But I didn't go into it for poxy numbers or a mere job; I went in become a great scientist. Obviously this did not happen. 

But I did learn how to really read papers, how to write papers, how to present technical ideas clearly, and how to become stubborn and insensitive in the face of latent spaces. Academia is forever demystified for me. My aversion to mathematics has settled down into guarded neutrality. I am unafraid. This was probably worth it.

<br><br>

_Undying thanks to Kristi Laurence Jan Jan Juan Tomáš Rian Matthijs Misha Daniel Dandy Dan Nandi Simson Mrinank Sören Kaveh Alexander Nic Charlie Maxime Samir Swapnil Miranda Peter Yarin Tyler. Sine qua non._


<br><br>

### Posts about my PhD

* <a href="/phdone">Overall index</a> 
* <a href="/thesis">My thesis in plain language
	* Click "The Point" on the entries <a href="/researches">here</a>
* <a href="/phd-numbers">my PhD by numbers</a>
* <a href="/ignorance">Crossing the ocean of my ignorance</a>
* <a href="/csml">Thoughts on the field of machine learning</a>
* <a href="/nograd">Against PhDs</a>
* <a href="/diary">phdiary</a>
* <a href="{{tree}}">My academic family</a>. Darwin is my great^10th grandfather.

<!-- Lichtenberg, Hebb, Linnaeus, Vavilov, Wundt, von Helmholtz, Peirce, William James -->

<br>

## Areas


<div class="accordion">
	<h3>Covid modelling</h3>
	<div>
		I started my PhD just before Covid. In a strange turn, a bunch of computer scientists invited me to do a little bit of writing on their big Bayesian model of what policies worked against the bug. I had no epidemiology background. 12 months later, we'd produced a series of <a href="/researches">7 papers</a> on important questions which weren't being treated with the proper uncertainty.<br><br>
		Yes, this was the least neglected research topic in the world. Yes, it is strange that noobs could do this. 
	</div>
	<h3>Probabilistic programming</h3>
	<div>
		My original project was _Tensorised Probabilistic Programming_.<br><br>
		Exact inference is intractable in many realistic latent variable models. Of the available approximations, variational inference is fast, but underestimates the variance; and Markov Chain Monte Carlo estimates the variance well but is far too slow in large models (Bishop 2006, Betancourt, 2020). For policy applications, where the variance must be accurate to prevent large irreversible decisions, we thus need new methods. Extending Aitchison's 2019 work on speeding up variational autoencoders, we seek to generalise the use of tensor products for approximate inference.<br><br>
		The end goal is multi-sample inference for any such scheme, and we aim to implement this in a probabilistic programming language (PPL) to maximise usability and impact. There are already ‘tensorised‘ PPLs, in the weak sense of using tensor operations for arbitrary probabilistic programs with one inference scheme (e.g. Bingham et al., 2019, which uses stochastic variational inference for all runs). We seek a further abstraction for any inference scheme. In our project, ‘tensorised’ denotes the tensor products used to achieve the speedup.<br><br>
		The original plan has passed to a colleague. Sorry Thomas.		
	</div>
	<!--  -->
	<h3>AI safety</h3>
	<div>
		<a href="/ai-risk">Here's my sceptic's guide to AI risk</a>. (For relative sceptics.) I also contributed a couple thousand words to the <a href="{{wiki}}">main wiki page</a>. I currently work with the <a href="{{acs}}">Alignment of Complex Systems Group</a>, Charles University.<br><br>
		At the first AI Safety Camp I worked with a team on inverse reinforcement learning, <a href="/grids">designing environments</a> to probe the limits of such reward learning. Our work was reused by a <a href="{{krak}}">team</a> at Deepmind and in an <a href="{{aies}}">AIES paper</a>. <br><br>
		Before starting on probabilistic programming, I played with an odd alternative ML paradigm called _<a href="/ilp">inductive logic programming</a>_. This led to my first paper, a negative result.<br><br>
		I also helped on <a href="/lgfo">a wee paper</a> with a sort of counsel of despair about algorithmic fairness.<br><br>
		I've also <a href="/acais">written</a> about the likely overlap between work on current systems and future systems.
	</div>
	<h3>Metascience</h3>
	<div>
		Over Christmas, instead of studying for quals I started <a href="{{psy}}">listing</a> all the failed replications in psychology I'd heard of. This ballooned into a list of hundreds, and was taken up by the volunteer org <a href="{{fo}}">FORRT</a> for permanent maintenance.
	</div>
</div>




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

</style>



<div class="footnotes">

<ol>
    <!-- 1 -->
    <li class="footnote" id="fn:1">
        Laurence made me change the official title to "Modern Scientific Workflow" to save face but you and I both know what the real title is. <a href="#fn:1" id="fnref:1">1</a>
    </li>

</ol>
</div>