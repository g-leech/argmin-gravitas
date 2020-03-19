---
layout:     post
title:      "Reversals in psychology"
baselink:   /psych
permalink:  /psych
date:       2020-01-26	
author:     Gavin   
img:        /img/rev.jpg

visible:    1
published:  true

summary:    List of exaggerated psychological phenomena
confidence: High that I'm representing the current evidence well, low that all of these will stay reversed.
importance: 5
warnings: 	I am not a social scientist.
wordcount:  3500
---

{%  include psy/links.md   %}

<!-- > [Brain training was] a prototypical case of the Replication Crisis: a convenient environmental “One Weird Trick” intervention; with extremely low prior probability; which made no quantitative predictions; supported by a weak methodology which could manufacture positive results; and propped up by selective citation, systemic pressure towards publication bias, researcher allegiance & commercial conflicts of interest; which theory was never definitively disproven, but lost popularity and sort of faded away. -->
<!-- <center> – <a href="{{gwern}}">Gwern Branwen</a></center> -->
<!-- <br> -->

A <a href="{{med}}">medical reversal</a> is when an existing treatment is found to actually be useless or harmful. Psychology has in recent years been racking up reversals: in fact only <a href="{{many}}">40-65%</a> of its classic social results were replicated, in the weakest sense of finding 'significant' results in the same direction. (Even in those that replicated, the average effect found was <a href="{{halv}}">half</a> the originally reported effect.) Such errors are obviously far less costly to society than medical errors, but it's still pollution, so here's the cleanup. <a href="#fn:1" id="fnref:1">1</a><br>

Psychology is not alone: <a href="{{ioan}}">medicine</a>, <a href="{{canc}}">cancer biology</a>, and <a href="{{ec}}">economics</a> all have many irreplicable results, and so do most other fields, as we'd know if they ran replication efforts this large. It'd be a mistake to write off psychology wholesale: we know about most of the problems in psychology because of psychologists, and the subfields show very different replication rates and effect size shrinkage. <a href="{{whyPsy}}">One reason</a> psychology reversals are so prominent is that it's an unusually 'open' field in terms of code and data sharing. A less scientific field would never have caught its own bullshit.

The following are empirical findings about empirical findings; they're all open to re-reversal. Also it's not that "we know these claims are false": failed replications (or proofs of fraud) usually just challenge the evidence for a hypothesis, rather than affirm the opposite hypothesis. I've tried to ban myself from saying "successful" or "failed" replication, and to report the best-guess effect size rather than play the bad old Yes/No science game. <a href="#fn:2" id="fnref:2">2</a>

Figures correct as of March 2020; I will put some effort into keeping this current, but not that much. <br>Code for converting means to Cohen's d and Hedge's g <a href="{{code}}">here</a>.<br><br>

---

<br>

## Social psychology 

{%	include psy/social.md	%}

## Positive psychology

{%	include psy/positive.md	%}

## Cognitive psychology

{%	include psy/cognitive.md	%}

## Developmental psychology

{%	include psy/developmental.md	%}

## Personality psychology

* Pretty good. <a href="{{soto}}">One lab's systematic replications</a> found that effect sizes shrank by 20% though.<br><br>

* Anything by Hans Eysenck should be considered suspect, but in particular these <a href="{{ey}}">26 'unsafe' papers</a> (including the one which says that reading prevents cancer).

<br>

## Marketing

* <a href="{{wansink}}">Brian Wansink</a> accidentally admitted gross malpractice; fatal errors were found in 50 of his lab's papers. These include flashy results about increased portion size massively reducing satiety.<br><br><br>

<a id="neuro"></a>

## Neuroscience

{%	include psy/neuro.md	%}


## Psychiatry

* At most <a href="{{rosenhan}}">extremely weak evidence</a> that psychiatric hospitals (of the 1970s) <span class="b">could not detect sane patients</span> in the absence of deception.

<!-- * Repressed memories / Recovered memory therapy -->


<br>

## Parapsychology

* <a href="{{bem}}">No good evidence</a> for <span class="b">precognition</span>, undergraduates improving memory test performance by studying after the test. This one is fun because Bem's statistical methods were "impeccable" in the sense that they were what everyone else was using. He is Patient Zero in the replication crisis, and has done us all a great service.
<div class="accordion">
	<h3>Stats</h3>
	<div>
		<ul>
			<li><span class="b">Original paper</span>: ( citations).</li><br>
			<li><span class="b">Critiques</span>: <br>(total citations: )</li><br>
			<li><span class="b">Original effect size</span>: d=[0.2, 0.4] </li><br>
			<li><span class="b">Replication effect size</span>: [ ], n=</li><br>
		</ul>
	</div>
</div><br>

<br>

## Evolutionary psychology

{%	include psy/evo.md	%}


## Psychophysiology

* <a href="{{phys}}">At most very weak evidence</a> that sympathetic nervous system activity <span class="b">predicts political ideology</span> in a simple fashion In particular, subjects' skin conductance reaction to threatening or disgusting visual prompts - a very <a href="{{smith}}">noisy and questionable</a> measure. 
<div class="accordion">
	<h3>Stats</h3>
	<div>
		<ul>
	<li><span class="b">Original paper</span>: <a href="{{ox}}">Oxley et al</a>, n=46 ( citations). p=0.05 on a falsely binarised measure of ideology.</li><br>
	<li><span class="b">Critiques</span>: Six replications so far (<a href="{{knoll}}">Knoll et al</a>; 3 from <a href="{{bakk}}">Bakker et al</a>) , five negative as in nonsignificant, one <a href="{{den}}">forking</a> ("holds in US but not Denmark")<br> <br>(total citations: )</li><br>
	<li><span class="b">Original effect size</span>: [ ], n= </li><br>
	<li><span class="b">Replication effect size</span>: [ ], n=</li><br>
	</ul>
	</div>
</div><br>

<br>

## Behavioural genetics

* <a href="{{ssc}}">No good evidence</a> that 5-HTTLPR is strongly linked to depression, insomnia, PTSD, anxiety, and more.<br><br>

* Be very suspicious of any such "candidate gene" finding (post-hoc data mining showing large >1% contributions from a single allele). <a href="{{depres}}">0/18</a> replications in candidate genes for depression. <a href="{{psychiatry}}">73% of candidates</a> failed to replicate in psychiatry in general. <a href="{{jbg}}">One big journal</a> won't publish them anymore without several accompanying replications. <a href="{{gwas}}">A huge GWAS</a>, n=1 million: "_We find no evidence of enrichment for genes previously hypothesized to relate to risk tolerance._"

<!-- * gxE evidence for the MAOA warrior gene. -->

<br>

<hr />

<br>

> [What I propose] is not a reform of significance testing as currently practiced in soft-psych. We are making a more heretical point... We are attacking the whole tradition of null-hypothesis refutation as a way of appraising theories... Most psychology using conventional H_0 refutation in appraising the weak theories of soft psychology... [is] living in a fantasy world of "testing" weak theories by feeble methods.

<center>– <a href="{{meehl}}">Paul Meehl</a> (1990)</center>
<br><br>

What now? When the next big flashy paper out of a world-class university arrives, will we swallow it?

<a href="{{gel2}}">Andrew Gelman and others</a> suggest deflating _all_ single-study effect sizes you encounter in the social sciences, without waiting for the subsequent shrinkage from publication bias, measurement error, data-analytic degrees of freedom, and so on. There is no uniform factor, but it seems sensible to divide novel effect sizes by a number between 2 and 100 (depending on its sample size, method, measurement noise, _maybe_ its p-value if it's really tiny)...

<br>


{%  include psy/caveats.md %}
{%  include comments.html %}
{%  include psy/foots.md %}

<br>


_Deep thanks to <a href="{{gelman}}">Andrew Gelman</a>, <a href="{{ritchie}}">Stuart Ritchie</a>, <a href="{{scheel}}">Anne Scheel</a>, <a href="{{lakensSite}}">Daniël Lakens</a>, <a href="{{ggg}}">Gwern Branwen</a>, and <a href="{{brown}}">Nick Brown</a> for pointers to effectively all of these_.

_All honour to the hundreds of 'data thug' / 'methodological terrorist' psychologists I've cited, who in the last decade did the hard work of cleaning up their field._

<br>