---
layout:     math_post
title:      "Reversals in psychology"
baselink:   /psych
permalink:  /psych
date:       2020-01-26	
author:     Gavin   
img:        /img/rev.jpg

visible:    1
published:  true
best: 		1

summary:    A list of exaggerated psychological phenomena
quality:    8
confidence: High that I'm representing the current evidence well, low that all of these will stay reversed.
importance: 5
warnings: 	I am not a social scientist.
categories: long-content, science, psychology, lists
wordcount:  3500
cross: 		https://socialsciences.nature.com/posts/reversals-in-psychology
---

{%  include psy/links.md   %}


<h3>Now a crowdsourced project <a href="{{f}}">elsewhere</a>. Seeking volunteers!</h3>

<hr />

<br>

<!-- - HOPE
- GRIT
- Left brain / right brain personality
- Alpha wolves
- Backfire effect
- Broken windows
 -->


<!-- > [Brain training was] a prototypical case of the Replication Crisis: a convenient environmental “One Weird Trick” intervention; with extremely low prior probability; which made no quantitative predictions; supported by a weak methodology which could manufacture positive results; and propped up by selective citation, systemic pressure towards publication bias, researcher allegiance & commercial conflicts of interest; which theory was never definitively disproven, but lost popularity and sort of faded away. -->
<!-- <center> – <a href="{{gwern}}">Gwern Branwen</a></center> -->
<!-- <br> -->

A <a href="{{med}}">medical reversal</a> is when an existing treatment is found to actually be useless or harmful. Psychology has in recent years been racking up reversals: in fact only <a href="{{many}}">40-65%</a> of its classic social results were replicated, in the weakest sense of finding 'significant' results in the same direction. (Even in those that replicated, the average effect found was <a href="{{halv}}">half</a> the originally reported effect.) Such errors are far less costly to society than medical errors, but it's still pollution, so here's the cleanup. <a href="#fn:1" id="fnref:1">1</a><br>

Psychology is not alone: <a href="{{ioan}}">medicine</a>, <a href="{{canc}}">cancer biology</a>, and <a href="{{ec}}">economics</a> all have many irreplicable results. It'd be wrong to write off psychology: we know about most of the problems here because of psychologists, and its subfields differ a lot by replication rate and effect-size shrinkage.<br>

<a href="{{whyPsy}}">One reason</a> psychology reversals are so prominent is that it's an unusually 'open' field in terms of code and data sharing. A less scientific field would never have caught its own bullshit.

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

* Pretty good? <a href="{{soto}}">One lab's systematic replications</a> found that effect sizes shrank by 20% though. See the comments for someone with a fundamental critique.


* Anything by Hans Eysenck should be considered suspect, but in particular these <a href="{{ey}}">26 'unsafe' papers</a> (including the one which says that reading prevents cancer).

<br>

## Behavioural science

* The effect of "nudges" (clever design of defaults) may be exaggerated in general. <a href="{{nudge}}">One big review</a> found average effects were six times smaller than billed. (Not saying there are no big effects.)

* <a href="{{kelsey}}">Here are</a> <a href="{{jc1}}">a few</a> <a href="{{jc2}}">cautionary</a> <a href="{{stu}}">pieces</a> on whether, aside from the pure question of reproducibility, behavioural science is ready to steer policy.


<!-- * opt out organs -->

<!-- * https://www.povertyactionlab.org/evaluation/cares-commitment-savings-smoking-cessation-philippines -->
<!-- terrible intention-to-treat ratio -->

* <a href="{{ariely}}">Moving the signature box to the top of forms</a> does not decrease dishonest reporting in the rest of the form.

<br>

## Marketing

* <a href="{{wansink}}">Brian Wansink</a> accidentally admitted gross malpractice; fatal errors were found in 50 of his lab's papers. These include flashy results about increased portion size massively reducing satiety.<br><br><br>

<a id="neuro"></a>

## Neuroscience

{%	include psy/neuro.md	%}


## Psychiatry

* At most <a href="{{rosenhan}}">extremely weak evidence</a> that psychiatric hospitals (of the 1970s) <span class="b">could not detect sane patients</span> in the absence of deception.

<!-- * Repressed memories / Recovered memory therapy -->

<!-- Post-traumatic growth is questionable, sadly. 
	https://journals.sagepub.com/doi/abs/10.1177/0963721419827017
-->

<br>

## Parapsychology

* No good evidence for <span class="b">precognition</span>, undergraduates improving memory test performance by studying after the test. This one is fun because Bem's statistical methods were "impeccable" in the sense that they were what everyone else was using. He is Patient Zero in the replication crisis, and has done us all a great service. (Heavily reliant on a flat / frequentist prior; evidence of optional stopping; forking paths analysis.)
<div class="accordion">
	<h3>Stats</h3>
	<div>
		<ul>
			<li><span class="b">Original paper</span>: '<a href="{{bem0}}">Feeling the future: Experimental evidence for anomalous retroactive influences on cognition and affect</a>', Bem 2012, 9 experiments, n=1000 or so.
				<br>(&#126;1000 citations, but mostly not laudatory).
			</li><br>
			<li><span class="b">Critiques</span>: <a href="{{bem}}">Ritchie 2012</a>, n=150. On one of the nine.<br>
				<a href="{{slate}}">Gelman 2013</a>; <a href="{{schimm}}">Schimmack 2018</a>, methodology.
				<br>(total citations: 200)
			</li><br>
			<li><span class="b">Original effect size</span>: Various, mean d=0.22. For experiment 9, r= minus 0.10.
			</li><br>
			<li><span class="b">Replication effect size</span>: Correlation between r= minus 0.02</li><br>
		</ul>
	</div>
</div><br>

<!-- <br> -->

<!-- ## Primatology -->

<!-- * Mixed evidence for inequity aversion in animals. -->
<!-- <a href="{{grapes}}"> -->

<!-- <br> -->

## Evolutionary psychology

{%	include psy/evo.md	%}


## Psychophysiology

<!-- https://psyarxiv.com/49hfg -->
<!-- https://journals.sagepub.com/doi/abs/10.1177/0146167219880191?journalCode=pspc -->
<!-- https://nymag.com/intelligencer/2018/07/how-social-science-might-be-misunderstanding-conservatives.html -->
<!-- https://www.nature.com/articles/s41562-020-0823-z -->

* <a href="{{phys}}">At most very weak evidence</a> that sympathetic nervous system activity <span class="b">predicts political ideology</span> in a simple fashion. In particular, subjects' skin conductance reaction to threatening or disgusting visual prompts - a <a href="{{smith}}">noisy and questionable</a> measure. 
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

{%	assign intel = "https://pubmed.ncbi.nlm.nih.gov/23012269/"	%}
{%	assign schizo = "https://www.ncbi.nlm.nih.gov/pmc/articles/PMC4414705/pdf/nihms653267.pdf"	%}
{%	assign allofit = "https://www.eurekaselect.com/59624/article"	%}


* <a href="{{ssc}}">No good evidence</a> that 5-HTTLPR is strongly linked to depression, insomnia, PTSD, anxiety, and more. See also <a href="{{intel}}">COMT and APOE</a> for intelligence, <a href="{{schizo}}">BDNF</a> for schizophrenia, <a href="{{allofit}}">5-HT2a</a> for everything...<br><br>

* Be very suspicious of any such "candidate gene" finding (post-hoc data mining showing large >1% contributions from a single allele). <a href="{{depres}}">0/18</a> replications in candidate genes for depression. <a href="{{psychiatry}}">73% of candidates</a> failed to replicate in psychiatry in general. <a href="{{jbg}}">One big journal</a> won't publish them anymore without several accompanying replications. <a href="{{gwas}}">A huge GWAS</a>, n=1 million: "_We find no evidence of enrichment for genes previously hypothesized to relate to risk tolerance._"

<!-- * gxE evidence for the MAOA warrior gene. -->

<br>

<hr />

<br>

> [What I propose] is not a reform of significance testing as currently practiced in soft-psych. We are making a more heretical point... We are attacking the whole tradition of null-hypothesis refutation as a way of appraising theories... Most psychology using conventional H_0 refutation in appraising the weak theories of soft psychology... [is] living in a fantasy world of "testing" weak theories by feeble methods.

<center>– <a href="{{meehl}}">Paul Meehl</a> (1990)</center>
<br><br>

What now? When the next flashy <a href="{{weird}}">WEIRD</a> paper out of a world-class university arrives, will we swallow it?

<a href="{{gel2}}">Andrew Gelman and others</a> suggest deflating _all_ single-study effect sizes you encounter in the social sciences, without waiting for the subsequent shrinkage from publication bias, measurement error, data-analytic degrees of freedom, and so on. There is no uniform factor, but it seems sensible to divide novel effect sizes by a number between 2 and 100 (depending on its sample size, method, measurement noise, _maybe_ its p-value if it's really tiny)...

<a name="melancholy"></a>
<br>



{%  include psy/caveats.md %}
{%  include psy/foots.md %}

<br>

# See also

* <a href="{{alvaro}}">A review of 2500 social science papers</a>, showing the lack of correlation between citations and replicability, between journal status and replicability, and the apparent lack of improvement since 2009.
* Discussion on <a href="{{hz}}">Everything Hertz</a>, <a href="{{hn}}">Hacker News</a>, <a href="{{gel}}">Andrew Gelman</a>, <a href="{{jhtw}}">some star data thugs comment</a>.

<br><br>


_Thanks to <a href="{{gelman}}">Andrew Gelman</a>, <a href="{{ritchie}}">Stuart Ritchie</a>, <a href="{{scheel}}">Anne Scheel</a>, <a href="{{lakensSite}}">Daniël Lakens</a>, <a href="{{ggg}}">Gwern Branwen</a>, and <a href="{{brown}}">Nick Brown</a> for pointers to effectively all of these_.

_All honour to the hundreds of data thug / methodological terrorist psychologists I've cited, who in the last decade began the hard work of cleaning up their field._

<br>

