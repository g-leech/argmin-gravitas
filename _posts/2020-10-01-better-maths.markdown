---
layout:     math_post
title:      "Better ways to write maths"
baselink:   /better-maths
permalink:  /better-maths
date:       2020-09-26
author:     Gavin

img:        /img/cursed.jpg
published:  true
visible:    1

summary:    Examples of improved presentation
categories: tools-for-thought, maths, academia
confidence: I have lots of experience not understanding things, and none of improving matters.
importance: 5
wordcount:  
argument:   
---

{% 	include phone_img.html 		%}
{%	assign adm = "https://en.wikipedia.org/wiki/De_Morgan%27s_laws"		%}
{%	assign jang = "https://blog.evjang.com/2018/08/dijkstras.html"		%}
{%	assign sym = "https://en.wikipedia.org/wiki/History_of_mathematical_notation#Symbolic_stage"	%}
{%	assign tao = "https://mathoverflow.net/questions/366070/what-are-the-benefits-of-writing-vector-inner-products-as-langle-u-v-rangle/366118#366118" 	%}
{%	assign tao2 = "https://terrytao.wordpress.com/advice-on-writing-papers/use-good-notation/" 	%}
{%	assign qc = "https://quantum.country/"		%}
{%	assign di = "https://en.wikipedia.org/wiki/Dependency_inversion_principle"	 %}
{%	assign color = "https://onlinelibrary.wiley.com/doi/pdf/10.1111/cxo.12676"	%}
{%	assign cole = "https://books.google.co.uk/books?id=BGM_hYKAgksC"	%}
{%	assign word = "https://en.wikipedia.org/wiki/Proof_without_words"	%}
{%	assign romer = "https://paulromer.net/jupyter-mathematica-and-the-future-of-the-research-paper/"	%}
{%	assign href = "https://ctan.org/pkg/hyperref?lang=en"	%}
{%	assign mackay = "http://www.inference.org.uk/mackay/itila/book.html"		%}
{%	assign chen = "https://web.evanchen.cc/napkin.html"		%}
{%	assign bret = "http://worrydream.com/ScientificCommunicationAsSequentialArt/"		%}
{%	assign dist = "https://distill.pub/2020/communicating-with-interactive-articles/"		%}
{%	assign dist2 = "https://distill.pub/2020/circuits/zoom-in/"	%}
{%	assign sipser = "https://www.goodreads.com/book/show/400716.Introduction_to_the_Theory_of_Computation"	%}
{%	assign qiao = "https://mobile.twitter.com/QiaochuYuan/status/1306035720109404162"	%}
{%	assign kun = "https://buttondown.email/j2kun/archive/ideas-for-math-tools/"		%}




<center>
	<blockquote>
		&#8230;the contradictory opposite of a copulative proposition is a disjunctive proposition composed of the contradictory opposites of its parts&#8230; the contradictory opposite of a disjunctive proposition is a copulative proposition composed of the contradictories of the parts of the disjunctive proposition. 
	</blockquote>
	&#8213; William of Ockham (1355), or:
	<br /><br>
<!--  -->
<!--  -->
	<blockquote>
		<p style="text-align:center">
		$$
			\sim\!(P \land Q) \to (\sim\!P \,\,\lor \sim\!Q)\\     \sim\!(P \lor Q) \to (\sim\!P \,\,\land \sim\!Q)
		$$
		</p>
	</blockquote>		
	&#8213; <a href="{{adm}}">Augustus De Morgan</a> (1860)
	<br><br>
<!--  -->
<!--  -->
	<blockquote>
		Any impatient student of mathematics or science or engineering who is irked by having algebraic symbolism thrust upon him should try to get along without it for a week. 
	</blockquote>
	&#8213; Eric Temple Bell
</center>
<br><br>


Mathematical notation is not finished. You can tell, because <a href="{{sym}}">so much of it is new</a>, and because so many smart people struggle with it as it is.

Still, a set of conventions have hardened in the last 100 years. Maths is as terse as possible; monochrome; unfriendly; operates at full generality; and gives bad, undescriptive names to its objects.

Now, aside from the distress it causes the beginner, terseness is _good_: it lets us fits more in our head at once, and so go faster, and so go further. The move from prose to symbols is objectively an improvement, even as the appearance of maths moved further from human intuition.

What else is good about the conventional style? It is minimalist; it does not patronise; it is tasteful and grown-up; its generality saves a lot of ink; its leaving almost everything unsaid saves a lot of time. To master a conventional serious proof is to overcome an adversary, to simultaneously prove something about oneself.

Here are some different ways of doing it, less optimised for past masters.

<br>

## Colour

Use colours to instantly relate symbols to explanations, whether verbal or graphical. Like Eric Jang's incredible '<a href="{{jang}}">Dijkstras in Disguise</a>':

<center>
	<img class="lazy" data-src="/img/bellmanford.png" />
</center>

This is also an instance of giving people several angles of attack on the same concept.

(There's <a href="{{color}}">mixed evidence</a> about coloured text and comprehension in general, but the studies all focus on ordinary prose and I doubt they transfer to understanding formulae with dozens of symbols.)

<br>

## Comments

> For example, you may come across definitions like this: "A finite state automaton
is a quintuple ($$Q$$, $$\Sigma$$ , $$q_0$$, $$F$$, $$\delta$$) where Q is a finite set of states ($$q_0$$, $$q_1$$, ..., $$q_n$$ ), $$\Sigma$$ is a
finite alphabet of input symbols, $$q_0$$ is the start state, $$F$$ is the set of final states $$F \in Q$$, and $$\delta \in Q \times \Sigma \times Q$$, the transition function."

> That definition should be taken outside and shot.  

<center>
	~ <a href="{{cole}}">John Coleman</a>

<br><br><br>

<blockquote>
	rigour follows insight, and not vice versa.
</blockquote>
	~ James Stone
</center>
<br>


Michael Sipser has good comments on all the proofs in <a href="{{sipser}}">his great CS book</a>:

<center>
	<img class="lazy i70" data-src="/img/sipss.png" />
</center>
<br>

<div class="accordion">
	<h3>Diagonalisation</h3>
	<div>
		<center>
		<img width="100%" class="lazy" data-src="/img/diag.png" />
		</center>
	</div>
</div>

<br>

<a href="{{chen}}">Evan Chen's book</a> for bright highschoolers is suitably friendly too.

For learning material (rather than research writeups), the steps of a proof could be tagged as "routine", "creative", "tricky", or "key" (h/t <a href="{{qiao}}">Qiaochu</a>). These would be best as sidenotes.

Further: Why is there no metadata? The field dependencies; the theorem dependencies, upfront; how important this result is, for what; some proofs with a similar flavour; or, for fun, what's the newest result necessary for this proof? When could it first have been proved?

<br>


## Motivating examples

> A good stock of examples, as large as possible, is indispensable for a thorough understanding of any concept, and when I want to learn something new, I make it my first job to build one.

<center>– Paul Halmos</center><br>

Most maths writing jumps straight to the general definitions. But at least some people need to work up from examples and counterexamples instead.

This is another place that <a href="{{chen}}">Chen's basic book</a> beats high-status university texts:

<center>
	<img class="lazy i90" data-src="/img/chen.png" />
</center>
<br>

Literal examples are just one answer to the question _"Why should I care about this theory?"_. Maybe authors think that question is wishy-washy, but examples are not subjective, just partial. I'm not even asking for – _horror of horrors!_ – applications. Maybe generality feels strong: to solve all examples at once, without looking at them, is to rise above the objects.

There is an ignorant way of asking "Why should I care?": the way with no sense of aesthetics, curiosity, patience, the philistine way that cannot see any value without an application behind it, or money. This is maybe the way mathematicians take the question, and so maybe why they shun it.



<br>

## Composing subproofs

Here's proof by induction as an algorithm:
<center>
	<img class="lazy i50" data-src="/img/induct.png" />
</center>

You then see that for any given instance you just need to write the two subroutines `BaseCase` and `InductiveStep`. I find this much easier to understand.

More generally I don't see much <a href="{{di}}">dependency inversion</a> in proofs. Long proofs will include a sketch of the strategy, but mostly not with this lucidity. (Exceptions: Sipser, Chen.)

Maybe this only works if you know some programming before you do higher maths (a lamentably rare condition).

Here's an unfair but illuminating rant:

> Imagine I asked you to learn a programming language where: <br /><br /> - All the variable names were a single letter, and where programmers enjoyed using foreign alphabets, glyph variation and fonts to disambiguate their code from meaningless gibberish. <br /><br /> - None of the functions were documented, and instead the API docs consisted of circular references to other pieces of similar code, often with the same names overloaded into multiple meanings, often impossible to Google.<br /><br /> - None of the sample code could be run on a typical computer; in fact, most of it was pseudo-code lacking a definition of input and output, or even the environment it was supposed to run. 		

<center>
	&#8213; Steven Wittens
</center>

<br>


## Graph dependencies

Is maths a directed graph of theorem to theorem? Close enough! But even <a href="{{mackay}}">chapter-level</a> can be helpful:

<center>
	<img class="lazy i90" data-src="/img/mackay.png" />
</center>

<br>

<!-- ## Naming things -->

## Tweaks

* Physicists have a nicer way of marking the variable of integration. Instead of putting $$\text{d}x$$ at the end, they put it at the start. This saves on brackets and rereading.

$$
	\int_X			
$$

## Visuals

It seems insane that _the study of change_ is mostly taught without any, y'know, animations.<br>

<iframe width="100%" height="515" src="https://www.youtube.com/embed/WUvTyaaNkzM" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>

<br>

The limit case of visual mathematics are the lovely <a href="{{word}}">proofs without words</a>.

We don't need to endorse any pseudoscience about "learning styles" to think that there are areas of mathematics for which even symbols are not the most efficient delivery.

<!-- All of science is stuck in the 80s. LaTeX is ancient technology. Static. -->

<br>

## Caveats

I'm not claiming that the above are the most important problems with maths teaching. Focussing on mechanical manipulation over insight, and on reproduction rather than creativity, seem like more dire mistakes.

All of academic science is stuck on many of the above, stuck in the 90s. Maybe worst is <a href="{{romer}}">the stagnation</a> of the conventional paper: static in visuals; never revised unless gross misconduct can be proven; completely decoupled from its justifying evidence and code. Was the last big innovation the hyperlink, <a href="{{href}}">1995</a>? Here are <a href="{{bret}}">two</a> <a href="{{dist2}}">examples</a> of great post-papers, and <a href="{{dist}}">a manifesto</a>. (My field, machine learning is unusually tolerant of blog posts, but is still a long way from giving them equal respect, even when it's warranted.)


<br>

> mathematics is, to a large extent, the invention of better notations

- Feynman

<br>

## See also 

* <a href="{{kun}}">Jeremy Kun on some real shit</a>
* <a href="{{tao}}">Terry Tao</a> on the mathematics of mathematical notation.
* <a href="{{tao2}}">Terry Tao</a> on good notation
* <a href="{{qc}}">Quantum Country</a>
* <a href="{{dist}}">Communicating with Interactive Articles</a>

<br><br>

_Credit to John Lapinskas for the induction algorithm._

<br>

{% 	include lazyload.html 	%}