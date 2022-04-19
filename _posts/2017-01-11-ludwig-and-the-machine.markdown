---
layout: 	math_post
title:  	"Ludwig and the Machine"
baselink:	/machine-epistemology
permalink:	/machine-epistemology/
date:   	2019-01-07  <!--site.time-->
author:		Gavin	

published: 	false

visible:	1
simple:		true
stylised:	true
technical:	true

summary:	Recent formal results address philosophical questions, sometimes even negating them.
confidence:	70%. I am not a computer scientist.
quality:    8
importance: 6
pride:		7
categories:	philosophy of science, language, machine learning
---

<!-- https://github.com/acmi-lab/cmu-10721-philosophy-machine-intelligence -->

<!-- Covers proof of Chomsky and Occam -->
<!-- https://homepages.cwi.nl/~rdewolf/publ/philosophy/phthesis.pdf -->


<img src="/img/two-var.png" />


<ul>
	<li>Howson on Bayes</li>
	<li>Cox on induction</li>
	<li>Solomonoff's razor</li>
	<li>Guaranteed induction</li>
	<li>Parsimony refuted and rehabilitated</li>
	<li>The logical analysis of concepts without use of necessities and sufficiencies.</li>
	<li>Ensemble success vs Strong Occam's razor</li>
	<li>Ryle's knowing-that (GOFAI) and knowing-how (CLT, SLT)</li>
</ul>

Here is a toy model of aesthetics with just two binary variables, 'classiness' and 'busyness' <a href="#fn:1" id="fnref:1">1</a>:<br><br>

<ul>
<li><a href="http://cdn.freshome.com/wp-content/uploads/2016/01/minimalism-freshome-1.png"><i>Minimalism</i></a>: Simple Classy</li>
<li><i><a href="http://www.hotpress.cz/wp-content/uploads/2015/09/klementinum-praha-0.jpg">Baroque</a></i>: Busy Classy</li>
<li><i><a href="http://googlesightseeing.com/wp-content/ryugyong.jpg">Brutalism</a></i>: Simple Vulgar</li>
<li><i><a href="http://www.macklowegallery.com/images/CMS/Glossary%20of%20Terms/Rococo.jpg">Rococo</a></i>: Busy Vulgar</li>
</ul><br>

Are these descriptions <i>true</i>? Well, they are incomplete, and are <i>not</i> definitions (i.e. <a href="https://en.wikipedia.org/wiki/Injective_function">one-to-one mappings</a>), but yes. Are they helpful? As a start, absolutely. 

<br><br>Now, the labels on the left are vague and intuitive <a href="https://en.wikipedia.org/wiki/Family_resemblance">family resemblances</a>; it is a fool's game to imagine they could ever be nailed down as <a href="http://www.iva.dk/bh/lifeboat_ko/CONCEPTS/monothetic.htm">monothetic</a> definitions (the philosopher's ideal of neat, necessary and sufficient sets of attributes). We can still model usefully and harmlessly, even if the models can never be complete. <a href="#fn:2" id="fnref:2">2</a><br><br>

But the critics and art academics I know spend far more time muddying the water: deconstructing our use of the problematic term "classy"; and who gets to say what 'simplicity' is anyway? They don't seem to <i>want</i> to explain things, even <a href="https://en.wikipedia.org/wiki/Fuzzy_logic">fuzzily</a>.<a href="#fn:3" id="fnref:3">3</a> Or, maybe they do, but refuse to accept anything but a perfect final omniperspectival explanation (the like that can never be supplied), maybe to keep themselves in work.<br /><br />

Imagine if critics were conscientious enough to build a consistent hundred-variable, real-valued theory of art. Would it "solve" criticism? Never ever. Would it make the points of disagreement between interpretations more vivid? Would it force clarity in this, the most <a href="http://www.artybollocks.com/#abg_full">pompous</a> and <a href="https://www.theguardian.com/artanddesign/2013/jan/27/users-guide-international-art-english">vacuous</a> discourse? Yes. <br><br>

But we will probably have to wait for AI art critics for that, to go with <a href="http://prisma-ai.com/">the excellent AI artists</a> we have already. 

<br><br>There's no fixed criteria for these terms, you say? There's too much political context and social problematics involved for art to be tackled by statistical inference, you reckon? Well, machine learning <i>is</i> the automatic empirical discovery of non-necessary, non-sufficient attributes; it can and will cover the full range of the term's application and will do so by frequency, not political agenda. 

<br><br>The polythetic wall held up against philosophers and computers for a long time, <a href="https://en.wikipedia.org/wiki/Philosophical_Investigations">sixty years</a> at least. But it's time.
<br><br>
Wittgenstein:
<blockquote><i>
someone might object: "You... have nowhere said what the essence of... language is: what is common to all these activities, and what makes them into language or parts of language.  So you let yourself off the very part of the investigation that once gave you yourself most headache, the part about the </i>general form of propositions<i> and of language."<br><br>

And this is true. &#8212; Instead of producing something common to all that we call language, I am saying that these phenomena have no one thing in common which makes us use the same word for all, &#8212; but that they are related to one another in many different ways. And it is because of this relationship, or these relationships, that we call them
all "language".  I will try to explain this.

<br><br>The result of this examination is: we see a complicated network of similarities overlapping and criss-crossing: sometimes overall similarities, sometimes similarities of detail. // I can think of no better expression to characterize these similarities than 'family resemblances'...</i>
</blockquote>

Basically, Wittgenstein is pointing out that the classic philosophical approach to conceptual analysis (the "monothetic" approach) fails, because real concepts are messy and defined in a partial-membership way, ("polythetic"). He implies that this can't be , which was a fair enough point in 1950.

But then Nils Nilsson:
<blockquote>
<i>Some tasks cannot be defined well except by example; that is, we are
able to specify input/output pairs but not a concise relationship between
inputs and desired outputs... machines</i> [are now] <i>able to adjust
their internal structure to produce correct outputs for a large number of
sample inputs and thus suitably approximate the relationship implicit in the examples.
</i></blockquote><a href="#fn:4" id="fnref:4">4</a>


We can do this to anything we have at least proxy data for, which is, arguably, every thing that could matter. (If you count e.g. self-report of experience as reliable proxy data for consciousness.)




<!--

Yes, Wittgenstein said context is important for meaning, but that is hardly his unique or even most important contribution to philosophy of language. Wittgenstein's real contribution is in showing that meaning cannot be pinned down like butterflies under glass -- that meaning spontaneously arises in each playthrough of a language-game, and that any effort to find a "canonical", "authoritative" definition is grasping at an illusion.

But word embeddings try to do almost exactly what Wittgenstein says is an illusion -- trying to pin down a canonical n-dimensional vector for each word. To correspond with Wittgenstein's theory, there cannot exist any mapping from a word to a vector. Perhaps each vector can be dynamically changing in a by principle uncomputable way. But to get there we are going to need a lot more advances than the state of the art NLP.

reply

	
visarga 15 hours ago [-]

> Perhaps each vector can be dynamically changing in a by principle uncomputable way.
The BERT language model does dynamic (contextual) embeddings and is state of the art in NLP.

https://towardsdatascience.com/bert-explained-state-of-the-a...

reply

	
akozak 19 hours ago [-]

That's a great way to put it!
It doesn't mean the approach isn't useful for building systems that we can interact with linguistically, just that we shouldn't kid ourselves into thinking the model has captured meaning.


-->



<br><br><br>

<!--  
red /cyan

blue / yellow
-->
This is far from the most important way ML affects old thought.

The formal sciences, math/stats/CS/decision theory.

* inductive bias; the set of assumptions a learner uses to predict outputs given inputs that it has not encountered
*Absolute bias: constraint on hypothesis space. e.g. search only linearly separable functions
*Preference bias: select the optimal hypothesis according to some ordering scheme. e.g. least Kolmogorov complexity
* statistical bias: directional error in an estimator.  error you cannot correct by repeating the experiment many times and averaging together the results.
* cognitive bias: 


Guarantees rarely have practical relevance: you're likely to have benchmarked and amortized a hundred thousand runs by the time the theoretician has thought up a proof for what you've already seen. And even if you have a guarantee before starting, your benchmarks will tell you far more about the system's actual usefulness - the guarantee tends to be a ridiculous underestimate. But proof is a fine thing even so, and it is on this level, the absolute apriori, that most philosophy thinks to live.

You don't know how right or wrong it will end up being - but you do know that it won't be worse than [Bound]. You can't guarantee that it'll settle down in your lifetime, but you can guarantee that the probability of it never settling down is low.

## Platonic forms

Olah and Plato

https://distill.pub/2018/building-blocks/

    This is a compressed essence of dog


## Problem of induction

Major development of recent decades has been guarantees on the results of induction, particularly if we’re willing to settle for probabilistic guarantees.

"this was perhaps Hume’s first great discovery... What he finds is that the confidence we have in natural law — in the regularities and uniformity of nature, in the future being about to resemble the past — has a source in our animal nature. Animals too expect things to go on much as they have gone on — but it has no justification in reason. There is no a priori way of showing that it’s even probable that the future will resemble the past... There’s nothing available to our understanding to show us why things must keep on as they apparently always have."
https://fivebooks.com/best-books/david-hume-simon-blackburn/
Computational Learning Theory and radical scepticism
    What justifies belief in an external world?
    != When can we expect a hypothesis to predict future data?
    CLT gives	
        Tradeoff between hypotheses considered and prediction confidence	* Occam's razor theorem
        Upper bound for data required for confidence
        Relation of training and test performance
        PAC: Even in the worst-case, we can learn.We don't need to know the evaluation distribution (the truth) to approximate it.Given m examples (X) from D and labels (f(x)), find a hypothesis h such that
            P(h(x) == f(x)) > 1 - eps
        (Alternative to Bayesian inference?Don't need a distribution over hypotheses: some distribution over sample data is enough)


## Causality.


<!--  -->

{%	assign gabgoh = "https://gabgoh.github.io/ThoughtVectors/"		%}



<!--  -->
<div class="footnotes">
<ol>
    <!-- 1 -->
    <li class="footnote" id="fn:1">
    	A solid addition for a three-variable version would be "flat or shadowy" i.e. using clean planes or <a href="https://en.wikipedia.org/wiki/Chiaroscuro">chiaroscuro</a>. This would let us introduce Classical (simple classy flat) and Gothic (busy classy shadowy) and three others I can't be bothered looking up. Though this is a distinctively visual epithet, where the above should apply to all arts.<br><br>
	</li>
	<li class="footnote" id="fn:2">
	What are the risks of building a model? Does a model obscure reality behind its necessarily limited representation? No; all the authors and users of models need, to avoid delusion and harm, is a little imagination and humility.<br /><br />
	</li>
	<li class="footnote" id="fn:3">
		There are <a href="http://i.imgur.com/Fj5fvUC.jpg">of course</a> <a href="http://www.mcmansionhell.com/post/148605513816/mcmansions-101-what-makes-a-mcmansion-bad">honourable exceptions</a>.
	</li>
    <li class="footnote" id="fn:4">
        See also the impressive but inexplicable-in-a-given-instance behaviour of feedforward neural nets:

		<blockquote>Neural networks have the rather uncanny knack for turning meaning into numbers. Data flows from the input to the output, getting pushed through a series of transformations which process the data into increasingly abstruse vectors of representations... But the vectors themselves have thus far defied interpretation.</blockquote>

		- <a href="{{gabgoh}}">here</a>
    </li><br>
</ol>



