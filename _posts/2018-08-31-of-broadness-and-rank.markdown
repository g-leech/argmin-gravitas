---
layout:     post
title:      "Broadness and philosophical rank"
baselink:   /broadness
permalink:  /broadness
date:       2018-08-23
author:     Gavin

img:        /img/broad.jpg
published:	true
visible: 	1

summary:    Investigating one possible predictor of long-term intellectual status.
confidence:	60%. The data are a convenience sample from a skewed subpopulation.
categories: 
count:		1100
---

{%  include broadness/links.md 	%}

<br>


> Unless a philosopher finds for us an acceptable synthesis – as Plato and Aristotle did together for their age, and St Augustine, Thomas Aquinas and Immanuel Kant for theirs – we remain becalmed on a painted ocean of controversy, and for better or worse, insofar as the past is a compass to the future, there will never be anyone to whistle thrice for us and say, once and for all, ‘The game is done! I’ve won! I’ve won!’

<center>- Ved Mehta</center>
<br><br>

What predicts a philosopher's all-time rank (as <a href="{{leiter}}">judged by some professional philosophers</a>, n=1160)? 

What about <i>broadness</i>? By that I mean that their philosophy covered everything, or made <a href="{{arch}}">a grand narrative</a> to explain everything, uniting the big domains (e.g. science, ethics, society, art), synthesising great competing schools of thought.

In the top 30, Aristotle, Plato, Kant, Descartes, the Tractarian Wittgenstein, Aquinas, Leibniz, Mill, Hegelmarx, Nietzsche (ironically, but still), Epicurus, Bacon fit this description extremely well.

I check if broadness is predictively important with some basic statistics <a href="#model">here</a>. I give a reason to think it isn't explanatorily important <a href="#null">here</a>.

<br><br>

<hr />
<br>

### Caveats

* Broadness is of course a matter of degree: for instance, Aristotle is broader than Plato, because of his vast natural science work and his logic. You could make this pretty objective by counting the <a href="{{apa}}">APA subfields</a> it integrates, which perhaps jointly represent everything.

* This poll is very much a <a href="{{conven}}">convenience sample</a>, <i>not</i> a <a href="{{rando}}">random sample</a> of philosophers. While Leiter is <a href="{{marx}}">an avowed classical Marxist</a>, he has also spent a decade <a href="{{ident}}">alienating identitarians</a>, i.e. the now-mainstream left. (Note: These two biases don't cancel.) As such, we can expect his readership to be skewed. You can conditionalise everything that follows as "according to Leiter's readers" to be safe.

* Even if it was random, I don't know how close a proxy for <i>actual value</i> someone's status-among-philosophers is.

* Broadness isn't the same as overall value - some of the very greatest thinkers are too technical to register in philosophy (e.g. Laplace, Jaynes, Zuse, Poincaré, Bellman, Shapley). (Turing has one foot in the philosophy-canon door, though his great work was of course elsewhere.)


* You might find ranking (even <a href="{{condor}}">Condorcet ranking</a>!) philosophies distasteful, a rank gamification of a higher pursuit. In that case, I ask you to replace every instance of "rank" in this piece with "perceived rank".

* Many execrable, uncritical mystics are enormously 'broad' in the weak sense that they mention lots of things. My sense requires both knowledge and reasoning, which e.g. Gurdjieff or Krishnamurti don't display.

* Spinoza was highly systematic (his <i>Ethics</i> attempts a complete metaphysics via deductive proof) but despite doing that, and an actual ethical system, and his Bible criticism, and his jurisprudence, he wasn't quite as broad as the others. (When first doing this list I got confused between 'systematic' as in 'formalised' and 'systematic' as in 'complete'.)

* It's easy to imagine someone being very broad (working their way down <a href="{{fields}}">this list</a>, making some remarks on each, say) without really having a system uniting their work. Montaigne is like that, and the later Russell too. I'm going to save face here by calling their system sceptical-empirical humanism. 

* Russell wrote on <a href="{{maths}}">mathematics</a>, <a href="{{lang}}">language</a>, <a href="{{epis}}">epistemology</a>, <a href="{{abc}}">contemporary physics</a> and <a href="{{politics}}">politics</a>, <a href="{{}}">ethics</a>, <a href="{{religion}}">religion</a>, <a href="{{histo}}">history</a>, <a href="{{sex}}">sex</a>, etc. I'm not sure why I didn't include him at first - possibly because he turned away from systematic (that is, formal) work after Gödel.

* The goalposts have moved. Moderns have more topics to write on, because we have discovered new sorts of things even at the highest level (e.g. computer science, which isn't just maths and engineering). It was relatively easy for e.g. Democritus and Thales to write on every known topic.


<br><br>

<hr />

<a name="model"></a>

### Modelling broadness

How strong is the relationship between broadness and polling rank? We can do better than eyeballing it and saying "huh, 12 of the top thirty talked about everything".

<a href="{{data}}">Here's the data</a>; the values of the response variable are all eyeballed for now (and please note I am only really familiar with half of them) - please correct me in the comments or with a pull request. It wouldn't be hard to make it more objective by counting the number of <a href="{{fields}}">large domains</a> their system integrates. 

Let's use two handy methods: rank correlations (checking the general relationship between the two, without making assumptions about their actual distribution) and ordinal regression (checking how strong its effect on rank seems to be). <a href="{{py}}">Here's the full analysis in Python</a>.

<center><img src="/img/broad/broad_kernel.png" width="70%" /></center>
<br>


#### What's the monotonic relationship?

Spearman's rank correlation (the "<i>strength and direction of the monotonic relationship between variables</i>") came out as <i>Rho = -0.23, p=0.035</i>; as broadness increases, rank slightly improves (decreases).

#### Do pairs of values generally agree?

Kendall's tau (the strength and direction of the 'correspondence' between two rankings, i.e. how often the linked pairs agreed) came out as <i>τ = -0.17, p=0.039</i>; slightly weaker but corroborating. Kendall's tau is more scathing about non-sequential dislocations than Spearman's rho, so the messy sections in the joint ranking drop the score. You expect messiness from a discrete space like this:

<center><img src="/img/broad/broad_scatter.png" width="55%" /></center>
<br>


#### What does threshold regression say?

It's overkill to do regression with only one variable. But I have to learn sometime.

When predicting its own training data, an 'all-threshold' ordinal regressor which relies solely on 'broadness' is on average off by 20 places in the ranking (mean absolute error). This is pretty bad: a dumb always-predict-the-mean-rank rule is generally 22 off.


<br>

(**TODO**: Count the APA subfields each philosopher contributed to.

**TODO**: Use the pairwise Condorcet information from the poll as well.

**TODO**: Compare <a href="{{philpapers}}">PhilPapers' comparatively nonpartisan sample</a>.)



<br><br>

<hr />

<a name="null"></a>

### Null hypothesis

There's a trivial explanation for this correlation:

1. This ranking is calculated from votes by contemporary philosophers.
2. Contemporary philosophers tend to specialise in only one of two of <a href="{{apa}}">~20 subfields</a> which jointly represent everything.
3. Philosophers who write about everything are thus able to impress 20 approximately distinct subpopulations, while specialists will tend to impress only one or two.
4. Broadness is trivially related to popularity among contemporary philosophers.

<br><br>

<hr />


### Contemporary grand systems 

The explosion in knowledge (or at least in the volume of writing) and academic incentives have led to intense specialisation in all fields. While there are some good reasons for specialisation (it's a special case of division of labour allowing more powerful work by the group), if there's a strong broadness effect (which, again, I haven't ascertained), this could hurt the prospects of contemporary philosophers later on.

Is anyone building such systems today? The two clear examples I know are <a href="{{taleb}}">Nassim Taleb</a> and <a href="{{raz}}">Eliezer Yudkowsky</a>. Neither is primarily an academic, both sometimes have questionable judgment, but each is incredibly exciting in the same way that Kant or Nietzsche is exciting - if not more, since we have access to incredible resources they didn't, not least data and <a href="{{mcmc}}">simulation</a>. 

(It always looks odd to compare contemporaries to the all-time greats. It's a steep status gradient: status seems to accumulate nonlinearly (think <a href="{{matt}}">Matthew effects</a>); even today, Aristotle has far more status than even the most-beloved, <a href="{{cite}}">most-cited</a> contemporary philosopher, David Lewis - judging by how Lewis only just managed the top 30.)

<a href="{{bias}}">Robin Hanson</a> writes exceptionally broadly (physics, AI, cognitive science, evolution, history, social science, <a href="{{zeroth}}">sex</a>) and is a mix of scholarly consensus and truly radical revisionism. I think he works around academic incentives by being tenured in a surplus-demand field (economics).

There's also a pretty large group (<a href="{{anders}}">Sandberg</a>, MacAskill, Ord, Bostrom, Cotton-Barrett, Beckstead) who have converged on expected-value probabilism as a method for enquiry into pretty well anything - putting them in the netherland between philosophy and statistics. Label this school "existential hope" and wish them well.

<br><br>

<hr />

#### Appendix: Time and rank

The interaction between broadness and how long ago they wrote would be interesting to see.

**TODO**: Taking the midpoint of their life as their "floreat" period. 



<br><br>

{%  include comments.html %}


<img src="/img/poll-poll.png" style="border:5px solid black" /><br><br>
