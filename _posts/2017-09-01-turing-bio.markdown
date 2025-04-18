---
layout:     post
title:      "Turing"
baselink:   /enigma
permalink:  /enigma
date:       2017-09-01
author:     Gavin   
img:        /img/

visible:    1
published:  true
quality:    6

summary:    Turing's accomplishments, cost-benefits, & emotional consequence.
confidence: 95%
importance: 6
wordcount:  1480
categories:	bio, computers, greats
---

{% 	assign cpi = "https://www.in2013dollars.com/uk/inflation/1940?amount=6000"		%}
{% 	assign laiss = "https://www.panarchy.org/keynes/laissezfaire.1926.html"		%}
{% 	assign ace = "https://en.wikipedia.org/wiki/Automatic_Computing_Engine"		%}
{%	assign cope = "https://sci-hub.se/10.1007/978-3-319-22156-4_3"		%}

<center>
	<img src="/img/turing.jpg" width="40%" />
</center><br>

<blockquote>in the early days of computing, a number of terms for the practitioners of the field of computing were suggested in the Communications of the ACM — turingineer, turologist, flow-charts-man, applied meta-mathematician, and applied epistemologist.</blockquote><center>- wiki</center>
<br>

<blockquote><i>In a man of his type, one never knows what his mental processes are going to do next.</i></blockquote><center>- JAK Ferns, Turing's coroner</center><br><br>

_A review of Turing: The Enigma (1983) by Andrew Hodges_.

There have been two big films about Turing (three if you count the uselessly fictionalised <i>Enigma</i> (2001)). All are dishonestly melodramatic to some degree; for instance they depict Turing's relationship with his dead love Christopher as the driver of his work on machine intelligence. And more generally they depict him as tragic. But he wasn't tragic: we were. In the 1950s we attacked a superlative person, because we were certain it was the right thing to do.<br><br>

Hodges, whose book began the great public rehabilitation of Turing and served as the source for the films, bears no blame for this: it's one of the best biographies I've ever read (better even than Kanigel on Ramanujan and Isaacson on Einstein). Hodges actually understands Turing's work, not just its consequences, and not just the drama around it. And what work!
<br><br>

{%	include turing-bio/results.md		%}

<br><br>

But even more than that: Copeland guesses that breaking U-boat Enigma saved <a href="http://www.bbc.co.uk/news/technology-18419691">14 million lives</a>, a large fraction of which we can lay at Turing's feet. If this is even roughly right this puts him in the <a href="https://scienceheroes.com/">top 50 life-savers</a> ever. 

But, outside of logic and engineering, where he was among the few most sophisticated people in the world, he was famously unsophisticated: 
<br>

<blockquote>As at school, trivial examples of ‘eccentricity’ circulated in Bletchley circles. Near the beginning of June he would suffer from hay fever, which blinded him as he cycled to work, so he would use a gas mask to keep the pollen out, regardless of how he looked. The bicycle itself was unique, since it required the counting of revolutions until a certain bent spoke touched a certain link (rather like a cipher machine), when action would have to be taken to prevent the chain coming off. Alan had been delighted at having, as it were, deciphered the fault in the mechanism, which meant that he saved himself weeks of waiting for repairs, at a time when the bicycle had again become what it was when invented – the means of freedom. It also meant that no one else could ride it.
<br>
<br>    He made a more explicit defence of his tea-mug (again irreplaceable, in wartime conditions) by attaching it with a combination lock to a Hut 8 radiator pipe. But it was picked, to tease him.
<br>
<br>    Trousers held up by string, pyjama jacket under his sports coat – the stories, whether true or not, went the rounds. And now that he was in a position of authority, the nervousness of his manner was more open to comment. There was his voice, liable to stall in mid-sentence with a tense, high-pitched ‘Ah-ah-ah-ah-ah’ while he fished, his brain almost visibly labouring away, for the right expression, meanwhile preventing interruption. The word, when it came, might be an unexpected one, a homely analogy, slang expression, pun or wild scheme or rude suggestion accompanied with his machine-like laugh; bold but not with the coarseness of one who had seen it all and been disillusioned, but with the sharpness of one seeing it through strangely fresh eyes. ‘Schoolboyish’ was the only word they had for it. Once a personnel form came round the Huts, and some joker filled in for him, ‘Turing A.M. Age 21’, but others, including Joan, said it should be ‘Age 16’...
<br>
<br>    It was demeaning, but the repetition of superficial anecdotes about his usually quite sensible solutions to life’s small challenges served the useful purpose of deflecting attention away from the more dangerous and difficult questions about what an Alan Turing might think about the world in which he lived. English ‘eccentricity’ served as a safety valve for those who doubted the general rules of society. More sensitive people at Bletchley were aware of layers of introspection and subtlety of manner that lay beneath the occasional funny stories. But perhaps he himself welcomed the chortling over his habits, which created a line of defence for himself, without a loss of integrity. 
</blockquote>

<br>We have words for this now ("nerd", "wonk", "aspie"), and massive institutions, and even social movements, but at the time he had to make do with "don", and hide inside academia. Again: the problem wasn't him, it was us.
<br>

<hr />

<br>
<br>He gets called a mathematician most often, I suppose because people don't want to be anachronistic. But scroll up: his most famous work is as a logician and a systems engineer, and the rest is statistics and algorithmics and cognitive science. He was falling between several chairs, until computer science caught up with him:

<blockquote>a pure mathematician worked in a symbolic world and not with things. The machine seemed to be a contradiction... For Alan Turing personally, the machine was a symptom of something that could not be answered by mathematics alone. He was working within the central problems of classical number theory, and making a contribution to it, but this was not enough. The Turing machine, and the ordinal logics, formalising the workings of the mind; Wittgenstein’s enquiries; the electric multiplier and now this concatenation of gear wheels – they all spoke of making some connection between the abstract and the physical. It was not science, not ‘applied mathematics’, but a sort of applied logic, something that had no name.</blockquote>

<br>The philosopher-engineer. One of several moments in Hodge's book that left me dumbstruck is Turing <a href="https://praxisblog.wordpress.com/2008/03/05/wittgenstein-versus-turing-spelling-of-the-second-order/">arguing with Wittgenstein</a> about the foundations of mathematics. (In the spring of 1939 they were both teaching courses at Cambridge called that!) Bit awkward, and in my view Alan goes easy on Ludwig. But you still couldn't make it up.
<br>
<br>The government employed Turing for 9 years, paying him about £6000 over the duration (<a href="{{cpi}}">£300k in today's money</a>). In that time he produced 3 gigantically advanced systems (most of the <a href="https://en.wikipedia.org/wiki/Hut_8">Hut 8</a> system, the <a href="http://www.turing.org.uk/sources/delilah.html">Delilah</a> and the <a href="{{ace}}">ACE</a> design), about 10 or 20 years ahead of their time. Hodges sees this as a triumph of managerial socialism. Now, breaking naval enigma for £300k is an unbelievable deal (the savings from undestroyed shipping and cargo alone would be in the billions, let alone the loss of life, let alone the decisive tactical advantage). But the government suppressed Delilah and totally screwed up the ACE project. So I'm not sure if we can cheer too much. Keynes says <a href="{{laiss}}">somewhere</a> that
<blockquote><i>The important thing for Government is not to do things which individuals are doing already, and to do them a little better or a little worse; but to do those things which at present are not done at all.</i></blockquote>
This is true of Bletchley. But instructive failures are only helpful if they occur in public. (As at least the ACE report was.)
<br><br>

<hr />

<br><br>
The most annoying part of the films making up emotionally powerful unifying themes for Turing is that they are already there. But to grasp them, you'd have to actually display what was most wonderful and important about him, his technical work, and there goes the box office. 
<br><br>

<blockquote>In an end-of-term sing-song [at Sherborne, when Turing was 12], the following couplet described him: <br><br>

<blockquote>Turing’s fond of the football field
<br>For geometric problems the touch-lines yield
<br></blockquote>

... another verse had him ‘watching the daisies grow’ during hockey... although intended as a joke against his dreamy passivity, there might have been a truth in the observation.
<br>
<br>
<br>[20 years later] ...One day he and Joan were lying on the Bletchley lawn looking at the daisies... Alan produced a fir cone from his pocket, on which the Fibonacci numbers could be traced rather clearly, but the same idea could also be taken to apply to the florets of the daisy flower.
<br>
<br>
<br>[30 years later] ...he was trying out on the computer the solution of the very difficult differential equations that arose when [one] followed the chemical theory of [plant] morphogenesis beyond the moment of budding...
<br>
<br>...he also developed a purely descriptive theory of leaf-arrangement... using matrices to represent the winding of spirals of leaves or seeds round a stem or flower-head... The intention was that ultimately these two approaches would join up when he found a system of equations that would generate the Fibonacci patterns expressed by his matrices.
<br>
<br>...Such observations reflected an insight gained from... [a program called] ‘Outline of Development of the Daisy’. He had quite literally been ‘watching the daisies grow’...  on his universal machine.
<br>
</blockquote>

<br><br>

<div class="accordion">
	<h3>Highlights</h3>
	<div>
		{%	include turing-bio/quotes.html	%}
	</div>
	<!--  -->
	<h3>Why listen to me on this topic?</h3>
	<div>
		<i>Nonfiction book reviews by nonspecialists are hazardous. It is just not easy to detect pseudo-empirical bullshit without<br><br>
			<ol>
				<li>
		immersion in the field and/or good priors for what makes for an extraordinary claim in it; 
	</li><br>
				<li>
		incredible amounts of fact-checking gruntwork, at least 5x the time it takes to just read something; or
	</li><br>
				<li>
		incredible amounts of argument-checking, which doesn't need domain knowledge.
	</li><br> 
			</ol>
			I always try to do (3) but surely often fail.</i> <br><br><br>
<!--  -->
		In this case: I am a computer scientist, and I've studied the early history of computing quite closely. I understand many of Turing's original papers, besides his group theory.<br><br>
	</div>
</div>


<br><br>

_<a href="https://www.goodreads.com/user/show/68316850-gavin">Cross-posted from Goodreads.</a>_<br>
