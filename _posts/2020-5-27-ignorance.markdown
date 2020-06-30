---
layout:     post
title:      "Crossing the ocean of my ignorance"
baselink:   /ignorance
permalink:  /ignorance
date:       2020-06-22
author:     Gavin   
img:        /img/zaky.png

visible:    1
published:  true

summary:    How can we think new things when everything's so complicated?
quality:    4
emotion:	6
confidence: N/A
importance: 7
wordcount:  1500
warnings:	Centred on PhD research, which has particular pathologies.
categories: research, writing, self-help
---

{%	include ignorance/links.md	%}

<center>

<blockquote>
	...postpone reading Nietzsche for the time being; first study Aristotle for ten to fifteen years.
</blockquote>

– <a href="{{heid}}">Martin Heidegger</a> <a href="#fn:1" id="fnref:1">1</a>
<br><br>

<blockquote>
	As a researcher, either you won’t understand something and you will feel stupid and like a worm, or you will understand something and think it’s too trivial and hence still feel like a worm. 
</blockquote>
– <a href="{{spj}}">Simon Peyton Jones</a>
<br><br>


<blockquote>
	I was much further out than you thought<br>
	And not waving but drowning.
</blockquote>
– <a href="{{wav}}">Stevie Smith</a>

</center><br><br>


What do you need, to do new things? Imagine you're a junior researcher; a scientist; a <a href="{{dry}}">dry-lab</a> scientist; a Machine Learning person. For good and bad reasons you want to publish in <b>D</b>eep <b>L</b>earning, a decade-old bandwagon which continues to <a href="{{gelada}}">steamroll</a> your field. You're rolling in the deep. How do you get to work?

A natural answer is to start at the <a href="{{dl}}">beginning</a>: go read the underlying mathematics.

<center>
	<img src="/img/dl.png" width="50%" />
</center>
<br>

OK, say you go off and do that. You're not happy with your understanding: you can feel the aching gap in your knowledge of say linear algebra - that your looking at all those matrices _<a href="{{yan}}">actively concealed</a>_ something important - but you figure it's enough for now. 

It takes a month or six. Can you do new things now? No: you have to learn how to actually implement things. Brilliant people have built <a href="{{t}}">easy tools</a> for you, so you learn one of those and reimplement some big papers.

That takes a month or two. Can you do new things now? No: you need a good idea. Where do you get those? Related Work, I guess. You go read. Later, your mouldering bones are discovered at your desk, with 200 tabs open and the Colab Disconnected modal still burning on your screen.

<br>

With such a burden, how does anyone do new work? Well, by not doing any such thing.

* you have to just start
* you'll learn it when you need it
* most research is not done alone
* most researchers don't remember the low-level stuff, and don't have to
* you don't have to focus on one thing
* forcing yourself to work on something has large costs
<!-- * Theory, Methods, Computation. Pick one or two. -->

I've been trying to think new things for about 6 years, but I only recently got any good at it. Here are some things that have helped:

<br>


### Requisite attitudes


#### The Neurathian bootstrap

> We are like sailors who on the open sea must reconstruct their ship but are never able to start afresh from the bottom. Where a beam is taken away a new one must at once be put there, and for this the rest of the ship is used as support. In this way, by using the old beams and driftwood the ship can be shaped entirely anew, but only by gradual reconstruction.

<center>– Otto Neurath</center>
<br>

Beginning at the beginning, craving absolute foundations, mostly leads to paralysis. Sometimes this is because it takes too long to reach the frontier from the foundation; sometimes it's because the foundation is missing or <a href="{{hilb}}">impossible</a>.

_<a href="/ficciones#babel">To live, you have to ignore things</a>._ So bite off a chunk of reality and ignore the rest. Manuel Blum:

> When working on a PhD, you must focus on a topic so narrow that you can understand it completely. It will seem at first that you're working on the proverbial needle, a tiny fragment of the world, a minute crystal, beautiful but in the scheme of things, microscopic. Work with it. And the more you work with it, the more you penetrate it, the more you will come to see that your work, your subject, encompasses the world. In time, you will come to see the world in your grain of sand. 


People don't talk enough about what they ignore. One exception: Andrew Gelman, one of the most influential statisticians alive, never bothered with <a href="{{gelm}}">measure theory</a>, the deep generalisation / justification of probability theory.

<br>

#### Comparing down

The above isn't about impostor syndrome, except insofar as I delude myself that others are not ignorant. I take impostor syndrome to be the subjective feeling of being inadequate relative to those around you. I'm talking about the objective sense in which no one has anything more than a piece of the puzzle; and yet some of them still manage to do new things. (To get a sense of how rough the subjective and objective problem is, note that PhD study breaks <a href="{{uk}}">a quarter</a> or <a href="{{us}}">a half</a> of the smart people who try.)

Anyway: I had a very distorted view of how much an average PhD actually knows. Just as an undergraduate degree only shows you once had a small degree of knowledge on one or two topics, so too getting postgraduate funding only means that you're not totally dense and callow. This is good news! Not-totally dense and callow people manage to do many of the coolest things.

<br>

#### Unlearning education

<center>
<blockquote>
	Books should follow science; science should not follow books.
</blockquote>
	– Francis Bacon
</center>
<br>

I was lucky; by being born in the right time and right place, I got huge amounts of free education. 

I was unlucky; an education was not what I actually needed; education <a href="{{george}}">trains you</a> for the wrong task, in the wrong way. The ability to do research correlates with doing well on tests. But it is probably not well served by the current degree of optimising for tests, reading, and mere recall.

There are multiple mismatches: it focusses your attention on <a href="{{street}}">solved or toy things</a>; it emphasises understanding old things rather than creating new things; it mostly doesn't let you follow your curiosity; it mostly doesn't train you to handle the gross uncertainty of research. (Outside of mathematics, there is no marking scheme - not even peer review, not even awards at conferences. Maybe 10 years later you'll get some sense of whether you actually succeeded.)

_Question first, not books first_. Learning is best and most lasting when in the service of a goal you actually care about: not "better grades", not "impress distant superior", but "I want to build x". When it is part of you.

PhDs are still pretty artificial (they make you work \~alone, on one pre-specified topic which has to look sensible and follow an existing programme, with deadlines, and you're fed ideas), but at least their goal is not a total dead-end.

It's not easy to unlearn tutelage, but at some point in your first few actual projects you might manage it.


#### Ideas are cheap

One useful piece of startup culture: "you have to ship". It is _not_ that your perfect idea is ruined by imperfect implementation: your idea is nothing until it exists; all implementations are an improvement over an idea. <a href="#fn:2" id="fnref:2">2</a>


<br><br>

### Mechanisms

The above is about fixing your head. This bit is about how the vastness of the ocean actually ends up not mattering:

#### Abstraction as testimony

{%	assign tcp = "https://www.joelonsoftware.com/2002/11/11/the-law-of-leaky-abstractions/"	%}

Some abstractions <a href="{{tcp}}">actually allow you</a> to ignore what's underneath; some boats don't sink that quick. I've been playing with the internals of Pytorch recently. How many people understand the <a href="{{tens}}">Tensor class</a>? A couple hundred probably, for say 100,000 users of it, and who knows? a billion downstream users. In fact, most good software is about shielding you from details: even the statement `a = 1` is <a href="{{asm}}">pretty</a> computationally complicated. The world couldn't work without the glory of <a href="{{test}}">testimony</a> like this.

<!-- The first term is the speed at which a student can absorb already-discovered... knowledge. The second term is the speed at which a master can discover new knowledge. The third term represents the degree to which one must already be on the frontier of knowledge to make new discoveries; at zero, everyone discovers equally regardless of what they already know; at one, one must have mastered every previously-discovered fact before one can discover anything new. The fourth term represents potential for specialization; at one, it is impossible to understand any part without understanding the whole; at zero, it can be subdivided freely. The fifth…” -->
<!-- https://slatestarcodex.com/2017/11/09/ars-longa-vita-brevis/ -->



<br>


#### Collaboration

Even once you've selected a level of abstraction, trusted the bulkheads to hold, you can still split the work further: laterally across co-authors who are good at different parts. This is division of labour again, one of the most powerful social forces.

The average paper now has about <a href="{{auth}}">5 authors</a>. Some of this is down to a deflation of what it takes to count as an author, but the rest is good stuff. One (conceptually) simple solution to the replication crisis in social science would be to require a statistician to be on every project, at least in the experiment design.

<br>

#### Momentum

Ideas generate ideas, success generates success.

In Spring, I worked on a <a href="{{covid}}">coronavirus modelling project</a>. In writing it I collected 15 major ideas that we didn't have time for, didn't have data for, which didn't fit into the scope of that paper. One week after submitting it, a subset of that team wrote another paper on the methods used, including 3 or 4 completely novel ideas and tests and proofs. We could do this 3 or 4 more times without a hint of 'salami slicing', bad behaviour. If we could only sustain the energy.

<br>

<!-- ## Thinking gradually larger thoughts

Academia has a standard ladder for this: 

> Blog posts < Preprint < Workshop paper < Conference paper < Journal paper < Prestige general journal paper

(Though some fields, like CS, put conferences at the top.)

But it isn't that simple. Paul Christiano's largest thoughts are blog posts. The overhead involved in each step (typesetting, citations, )


Aside on
	First Author Papers.
	(The order is weightings are never clear. I've tried to elicit them but no one is willing to be pinned down.) -->

#### Slack

<center>
	<blockquote> 
		You waste years not being able to waste hours
	</blockquote>
	– <a href="{{tv}}">Amos Tversky</a>
</center>
<br>

One of the perversities of academic life is the absence of slack: spare time for just playing around. I won't go into this here (<a href="/scarcity">see here instead</a>), but here's a nice story. A young mathematician <a href="{{picc}}">recently cracked</a> a notorious problem as a side-project, no deadline, no particular expectation of success, almost an etude.

<br> 

#### Teaching as learning

> You think you know when you can learn, are more sure when you can write, even more when you can teach, but certain when you can program 

<center>
	— Alan Perlis
</center>
<br>

The bureaucracies act as if you can only teach once you are a master. But I often feel that I don't understand anything until I try to explain it to someone else - hence this blog. Yet another unforced error of ordinary education: you're not allowed to learn through teaching until it's over.

"You learn the prerequisite in the next course." And I learn the prerequisite when I am allowed to teach the prerequisite.


<!-- In the humanities, it is relatively common to write a 3-year "reading schedule" at the start of your PhD. -->


---

<br>

## See also

* <a href="{{adem}}">Abram Demski</a>, 
* <a href="{{ldem}}">Laura Deming's rage</a>.
* <a href="{{might}}">Matt Might's ways to fail</a>
* <a href="{{spj}}">Peyton Jones, How to Write</a>
* <a href="{{nerst}}">Nerst, Decoupling</a>
* <a href="{{scott}}">Alexander, 'Ars longa, vita brevis'</a>

<br>

{%	include comments.html	%}


<div class="footnotes">

<ol>
    <!-- 1 -->
    <li class="footnote" id="fn:1">
    	...in order to understand the Epic of Gilgamesh, you'll have to first comprehend the cave paintings and sculpture produced during the Upper Paleolithic. Without a full grasp of the cave paintings at Lascaux, you'll never be able to contextualize the oral tradition that produced Gilgamesh, leaving you without a full knowledge of the Septuagint, making your reading of Kierkegaard incomplete, making your reading of <a href="{{derr}}">Heidegger &amp;﻿ Derrida faulty</a>.<br><br> Of course, you'll need to learn Proto-Indo-European.</a>
		<!--  -->
		<!-- <iframe width="560" height="315" src="{{derr}}" frameborder="0" allow="accelerometer; autoplay; encrypted-media; gyroscope; picture-in-picture" allowfullscreen>
		</iframe> -->
	</li>
	<!--  -->
	<li class="footnote" id="fn:2">
		There are subtleties here, about <a href="{{fume}}">data fumes</a>, <a href="{{haz}}">info hazards</a>, <a href="{{inoc}}">idea inoculation</a>, and poisoning the well. But unless you're working on very strange things these are unlikely to apply.
	</li>
</ol>

</div>

