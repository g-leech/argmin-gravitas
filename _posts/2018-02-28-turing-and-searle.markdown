---
layout:     post
title:      "Neither Turing, neither Searle"
baselink:   /turing-test
permalink:  /turing-test
date:       2009-03-01
author:     Gavin

img:        /img/searle.jpg
published:	1
visible: 	1
quality: 	5

summary:    Elementary philosophy of mind with one original argument.
confidence:	75%, but only because conclusion is wholly negative.
categories: AI, philosophy, argument
warnings:	
importance: 6
wordcount:	1000
---
<br>

{%	assign crux = "https://en.wikipedia.org/wiki/Experimentum_crucis"		%}
{%	assign discourse = "https://www.gutenberg.org/files/59/59-h/59-h.htm"		%}


> <span style="font-weight:bold">SIMPLICIO</span>: ‘Some computer programs might be able to pass a Turing test, but that doesn’t provide any evidence that they can think. They might use all the right words, but that doesn’t mean they understand what the words mean.’

<br>

The Turing test is sometimes portrayed as a proper <a href="{{crux}}">crucial experiment</a> verifying the presence of intelligence - i.e. a sufficient condition for thought - and sometimes just as evidence for thought. But it was actually originally intended to _sidestep_ the question of whether machines can think: Turing deemed that “too meaningless for discussion.”<a href="#fn:1" id="fnref:1">1</a> His replacement question is:

> Is it possible for a finite-state digital computer, provided with a large... program, to provide responses to questions that would fool an unknowing interrogator into thinking it is a human being? 

<br>

(In fact Turing made a precise forecast, specifying the memory bounds, and a point estimate of when it would be passed with specific accuracy:

> I believe that in about fifty years' time it will be possible to programme computers, with a storage capacity of about 10<sup>9</sup> [bits], to make them play the imitation game so well that an average interrogator will not have more than 70 per cent chance of making the right identification after five minutes of questioning.

This forecast did not come to pass (and still hasn't after 73 years), despite ordinary computers now having more than a hundred times the specified RAM, about 125 MB.)
<br>

So put, this is clearly an operationalisation of "intelligence" without reference to consciousness, intentionality, semantics, understanding or any of the other "mentalistic" concepts of philosophy of mind. (This is still a useful sidestep 80 years later.)

<!-- One design criticism is that the test is flatly subjective: since there is just one human observer on which the result relies, no even passably objective data can be derived from any outcome. This interrogator-dependence, when coupled with the vast potential variance in the performance of the human interrogatee, diminishes the Test’s potential value as part of an empirical case for intelligence, and makes its metaphysical claim to sufficient condition rather less concrete. 

Block provides a thought experiment to counter the Test’s sufficiency: the “Blockheads”, lookup tables with responses to any possible input. He argues that these machines would have only ‘the intelligence of a juke-box’<a href="#fn:3" id="fnref:3">3</a>  but would pass the Turing Test easily. Blockheads are easy to conceive, but physically impossible and thus not a real gap in the Test’s integrity. Issues such as memory capacity and time taken to complete the preprogramming seemed insurmountable until recently, with technologies such as hard drives and generative grammar arising; however, going by the contemporary standard of chatbots (weak AI programs in a similar vein) available for interview<a href="#fn:4" id="fnref:4">4</a> <a href="#fn:5" id="fnref:5">5</a> <a href="#fn:6" id="fnref:6">6</a>  the true Blockhead remains.  -->

Appealing to "understanding", as Simplicio did above, implies rejecting functionalism. (Where functionalism views the input/output relation or function as constituting or producing mental activity.) So Simplicio is taking John Searle's line, of the necessity of 'original intentionality' (purposefulness, aboutness) for a system to be a mind. <a href="https://www.jstor.org/stable/2107856?seq=1#page_scan_tab_contents">Searle</a>: 

<blockquote>
...the presence of a program at any level which satisfies the Turing test is not sufficient for, nor constitutive of, the presence of intentional content. [Jacquette] thinks that I am claiming “Program implies necessarily not mind” whereas what I am in fact claiming is “It is not the case that (necessarily (program implies mind)).”
</blockquote>

i.e. 

    1. Programs are purely formal (syntax-only). 
    2. Human minds have mental content (semantics, beyond syntax). 
    3. Syntax by itself is neither constitutive of, nor sufficient for, 
    semantic content. 

    4. Therefore, programs by themselves are not constitutive of, nor sufficient 
    for minds. 


Note that we've slipped from talking about intelligence (often glossed as "the production of good outputs given varied inputs") to talking about minds (which could mean intelligence, or first-person consciousness, or...). For whatever reason, this happens all the time.

<br>

The real trouble comes in his positive case - Searle's “Chinese Room” metaphor (in which no component of a translation system understands Chinese, but the Room can translate it nonetheless, giving the right input/output pairs). The Chinese Room is a punchy illustration of premise 3 above, intended to demonstrate an instance of intelligent behaviour without understanding or mental content.

	1. Searle: "purely syntactic systems lack subjective experiences."
	2. Searle: "I have subjective experiences."
	3. So: "I am not a purely syntactic system." (modus tollens, 1&2)

This is unsatisfying: computer systems (hardware + program) are not "purely syntactic"; they have changing internal states altering according to inputs plus internal structure, a setup highly reminiscent of the representational theory of mind in humans.

Worse: as reconstructed, there's an actual fallacy here. The Chinese Room implies that syntax is not sufficient for semantics, despite the impossibility of being a syntactic system and verifying this assertion directly.

	1. Searle: "purely syntactic systems lack subjective experiences."
	2. Searle: "I have subjective experiences."
	3. So Searle: "I am not a purely syntactic system." (modus tollens, 1&2)

	4. The only system Searle has knowledge of the subjective experiences of
	is himself.
	
	5. So if Searle is not a purely syntactic system, he has no knowledge of 
	what it is like to be a purely syntactic system,
	6. So if Searle is not a purely syntactic system, he therefore cannot 
	assert premise 1. (5, + the knowledge account of assertion).
	7. But if Searle is a purely syntactic system, (1) is false. (by 2)

	8. You're either a purely syntactic system or you're not.
	9. Therefore premise (1) is either unwarranted or false. (by 6 & 7 & 8 )

<br>
<br>

Despite Turing's inspiring attempt to sideline it, the metaphysics of mind is a live concern; Searle’s objection, that the kind of minds _we know about_ seem to depend on / arise out of intentionality is fine as far as it goes. But we are too ignorant to go about generalising about minds given our solitary example of the species: we haven't seen enough (as <a href="https://arxiv.org/pdf/1410.0369.pdf">Sloman</a> puts it, enough of the "space of possible minds") to say that particular human correlates are necessary for intelligence. 

<br><br>

<div class="accordion">
<!--  -->
	<h3>Disclaimer</h3>
	<div>
		This was my first original philosophical argument. (The original version of it was much less clear though.)
		<br><br>
		These days I wouldn't use infallibilism as the baseball bat I did just there ("<i>Searle isn't certain so Searle doesn't know</i>."); I'd go for probabilism instead. That is, I think I now deny my premise (4).
		<br><br>
		And I'd say more about Searle's odd dichotomy between representational machines who are 'pure' syntax vs those which are fully semantic. But I've mostly left it as it was because I enjoy it.<br><br>
	</div>
<!--  -->
	<h3>Chomskyan Descartes </h3>
	<div>
		I can't miss the opportunity to pass on a Good Fact: the Turing Test <a href="{{discourse}}">was suggested</a> 300 years earlier by Rene Descartes!<br><br>
	<!--  -->
		<blockquote>
			If there were machines which bore a resemblance to our bodies and imitated our actions as closely as possible for all practical purposes, we should still have two very certain means of recognizing that they were not real men. <br><br>The first is that they could never use words, or put together signs, as we do in order to declare our thoughts to others. For we can certainly conceive of a machine so constructed that it utters words, and even utters words that correspond to bodily actions causing a change in its organs. … But it is not conceivable that such a machine should produce different arrangements of words so as to give an appropriately meaningful answer to whatever is said in its presence, as the dullest of men can do.<br><br> Secondly, even though some machines might do some things as well as we do them, or perhaps even better, they would inevitably fail in others, which would reveal that they are acting not from understanding, but only from the disposition of their organs. For whereas reason is a universal instrument, which can be used in all kinds of situations, these organs need some particular action; hence it is for all practical purposes impossible for a machine to have enough different organs to make it act in all the contingencies of life in the way in which our reason makes us act.
		</blockquote><br><br>
	<!--  -->
		That Descartes could not conceive of any such machine, while Turing could, is an important lesson in philosophical method and embodiment:<br><br>
	<!--  -->
		<ol>
			<li>conceivability (by a particular person, or a particular species) is far too weak to do metaphysics with, as Descartes did. ('Philosophers' Syndrome: mistaking a failure of the imagination for an insight into necessity.' - Dennett)</li><br>
			<li>"What you can imagine depends on what you know." It is not that Turing was necessarily the superior mind; for he had the benefit of a superior context. (Which he helped invent, but the point is recursive.)</li>
		</ol>
	</div>
<!--  -->
	<h3>Bibliography</h3>
	<div>
	<ul>
		<li>Block, Ned (1995), ‘<a href="https://www.nyu.edu/gsas/dept/philo/faculty/block/papers/msb.html">The Mind As Software of the Brain</a>’</li>
		<br>
		<li>Cole, David (2004); ‘<a href="http://plato.stanford.edu/entries/chinese-room/">The Chinese Room</a>’; Stanford Encyclopadia of Philosophy. </li>
		<br>
		<li>Hofstadter, Douglas (1981); ‘<a href="">A Coffeehouse Conversation</a>’, in D. Hofstadter & D. Dennett (eds.) The Mind's I, (London: Penguin), pp.69-92 </li>
		<br>
		<li>Hofstadter, Douglas (1995), Fluid Concepts & Creative Analogies (Bloomington; Basic)</li>
		<br>
		<li>Levin, Janet (2009); ‘<a href="">Functionalism</a>’; Stanford Encyclopaedia of Philosophy; http://plato.stanford.edu/entries/functionalism/#ThiMacTurTes </li>
		<br>
		<li>Nagel, Thomas (1974); ‘<a href="">What Is It Like To Be A Bat?</a>’; The Philosophical Review LXXXIII, 4; pp.435-50</li>
		<br>
		<li>Oppy, Graham & Dowe, David (2008); ‘<a href="http://plato.stanford.edu/entries/turing-test/">The Turing Test</a>’, Stanford Encyclopaedia of Philosophy.</li>
		<br>
		<li>Searle, John R (1989); ‘<a href="https://www.jstor.org/stable/2107856?seq=1#page_scan_tab_contents">Reply to Jacquette</a>’, in Philosophy and Phenomenological Research, Vol. 49, No. 4, (Providence, International Phenomenological Society), pp. 701-708</li>
		<br>
		<li>Turing, Alan (1950); ‘<a href="">Computing Machinery and Intelligence</a>’, Mind, Vol. LIX, No.236 (Oxford; Oxford University Press), pp.53-67</li>
	</ul>
	</div>

</div>
{%  include comments.html %}

<br>

{%  include turing-searle/foots.html %}

