---
layout: 	math_post
title:  	"The Worst Game Ever"
baselink:	/worst
permalink:	/worst
date:   	2016-10-21  
author:		Gavin	
img:      /img/guillotine.webp

visible:	1
published: 	true

summary:	A true story about model error, noisy decisions, and the birth of tragedy.
quality:    7
confidence: 90% that my interpretation is right. Speculative stuff in the second half.
categories: stats, social-science, game-theory
importance: 6
emotion:	4
wordcount: 	700
argument:   pd/argument.md
---

{%	assign supe = "https://en.wikipedia.org/wiki/Superrationality"	%}
{%	assign gr = "https://en.wikipedia.org/wiki/Grim_trigger"	%}
{%	assign noi = "http://www.cs.utexas.edu/~chiu/papers/Au06NoisyIPD.pdf"	%}
{%	assign pd = "https://en.wikipedia.org/wiki/Prisoner's_dilemma"	%}
{%	assign tit = "https://en.wikipedia.org/wiki/Tit_for_tat#Game_theory"	%}
{%	assign mic = "https://learn.saylor.org/course/ECON101"	%}
{%	assign kin = "https://en.wikipedia.org/wiki/Kin_selection"	%}
{%	assign model = "http://web.archive.org/web/20200707011614/http://www.bostonfed.org/-/media/Documents/neer/neer697b.pdf"	%}
{%	assign nash = "http://web.cse.ohio-state.edu/~stiff.4/cse3521/prisoners-dilemma.html"	%}
{%	assign king = "https://unherd.com/2020/02/the-madness-of-mervyn-kings-uncertainty"		%}
{%	assign berg = "https://link.springer.com/chapter/10.1007/978-1-349-20181-5_26"		%}
{%	assign rpw = "https://umass.app.box.com/s/n72u3p7pyj/folder/80549398"		%}


I was at a corporate team-building event, because I wasn't persuasive enough to not be. I was a prisoner.

The organisers set up a game: a three-player, unknown-length iterated <a href="{{pd}}">Prisoner's dilemma</a>. There was no initial discussion, but free discussion every two rounds. Payoffs were the standard unitless numbers, shifted so that some outcomes were negative. Scores began at zero. No objectives were given. <br><br>


<div class="table-wrapper">
	<table>
	<tr>
		<th>All co-op</th>
		<th>Some defect</th>
		<th>All defect</th>
	</tr>
	<tr>
		<td>All +1</td>
		<td>Defectors +2<br>Co-op: -1</td>
		<td>All -2</td>
	</tr>
	</table>
</div>

<br>

I _swear_ I am not making up the roles the players tacitly settled into: perfect archetypes of game theory.

<ol type="A">
	<li> a Homo economicus (who clearly took <a href="{{mic}}">Micro 101</a> and nothing further).<a href="#fn:1" id="fnref:1">1</a></li><br>
<!--  -->
	<li> an ineffective altruist, trying to get everyone to the global maximum, but without any leverage or provocability. Opened with co-operate, tried again after every negotiation round until the second half when he got in <a href="{{gr}}">a huff</a>.</li><br>
<!--  -->
	<li> a <a href="{{noi}}">noise generator</a>. Random action, or, action based on his reading of opponents' body language.</li>
</ol><br><br>

A: 

> It's totally straightforward, there's only one right answer: for Prisoner's dilemmas, the <a href="{{nash}}">only</a> Nash equilibrium is 'always defect', because it only makes sense to defect in the final round, and the inference to prior rounds is timeless. 

<a href="{{supe}}">Me</a>: 

> No, that's for the known-length case with two players! And for when you can assume perfectly rational opponents! And for when their actions are independent of yours, with no communication! And you're aiming for personal loss minimisation, but you weren't given a loss function: you don't know that negative scores are non-fatal (or that they allow for "losing the least").

He didn't listen. Heedless in the grip of theory, the ecstasy of proof, (A) defected every time. During the communication, he was sometimes honest about his strategy and sometimes pretended to accept a truce.

<br>

The game ran about 15 rounds, I think ending early in despair. We ended with all scores negative, between 1 and 10 points under. It was announced that everyone had lost, since -1 represented death. Everyone sulked, especially the organisers.

<br>

<hr />

<br>


This was in fact an _incredible_ lesson, but not the one the organisers wanted me to learn.

* Decisions make no sense without a loss function! Probably the organisers had no idea about loss functions or equilibria or mathematical induction backwards in time, but I could have drawn it out of them. (Scary thought: society depends on statistical inference, and yet some massive majority of those inferences (the null-hypothesis significance tests) are made in total ignorance of their <a href="{{berg}}">implicit decision theory</a>.<br>

* What's wrong with co-operating every time? Well, setting aside the poor bot's own (generally terrible) outcome: it invites exploitation, and so can actually be destabilising in a sense, compared to precommitted <a href="{{tit}}">tit-for-tat</a>.<br>

* It is really amazing how stupidly a clever person can act if they are relying on a <a href="{{model}}">clever false theory</a>. This can result from any method, any species of reasoning, but using maths badly is the most complete way of disabling such a person. This is why, despite appearances, we have to listen to mouthy gits <a href="{{king}}">like Taleb</a>: there really are model error monsters out there.<br>

* Even in the absence of A's model error, the presence of noise would have completely destabilised the equilibrium anyway. We were doubly doomed.<br>

* The game wasn't long enough for the other key ingredient of super-rationality to arise: forgiveness, necessary in all closed-source stochastic domains, like life.<br>

* Body language reading _could_ be a real skill, but either way I think most people don't have enough skill to substitute for outside-view reasoning - even in toy examples like the above.


<br>

<hr />

<br>

### Anthropics and game theory only work on themselves

When reasoning about what I should do, if I reflect on what my predecessors must have done in order for me to exist, and then generalise this to my descendents (since they are <a href="{{kin}}">a sort of partial copy</a> of me), I could convince myself that I should do what would maximise my chance of coming into being. (This is all under the assumption that I am the sort of being which should exist, at least equally compared to the counterfactual people in other chains of descent.) But this doesn't work unless the other people in the chain are also doing anthropics.

Similarly, you don't benefit from doing game theory on an unpredictable opponent (for instance one who doesn't know or rejects game theory).

<br><br>

## See also 

* _<a href="{{rpw}}">The Use and Abuse of Formal Models in Political Philosophy</a>_, Robert Paul Wolff.



<br><br>

_Thanks to Misha Yagudin for the anthropics point._

<div class="footnotes">
<ol>
	<li class="footnote" id="fn:1">
		Not a real Homo economicus, obviously: instead a Homo sapiens running a bad simulation of one, at the same time shutting down the common sense that might have saved him.
	</li>

</ol>
</div>