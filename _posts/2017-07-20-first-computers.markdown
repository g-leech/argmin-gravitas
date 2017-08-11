---
layout:     post
title:      "Disambiguating the first computer"
baselink:   /computers
permalink:  /computers
date:       2017-06-25  <!--site.time-->
author:     Gavin

img:        /img/boo.png
visible:	false
published:	false

summary:    App for defining "computer" and so selecting between candidate first computers.
confidence:	95%
categories: computing, history
warnings:	unfair posterity.
count:		
---


> "*Computer*: a device that can be instructed to carry out an arbitrary set of arithmetic or logical operations, automatically."
<center>- Wikipedia</center>

> Let me emphasize that there is no such thing as "first" in any activity associated with human invention. If you add enough adjectives to a description you can always claim your own favorite. For example the ENIAC is often claimed to be the "first electronic, general purpose, large scale, digital computer" and you certainly have to add all those adjectives before you have a correct statement... 

> there is more than enough glory in the creation of the modern computer to satisfy all of the early pioneers, most of whom are no longer in a position to care anyway. 
<center>- Wikipedia</center>

> (The possession of a large memory for storing both instructions and data is a defining characteristic of the modern computer.)


By "first computer" almost everyone means "the first modern computer", which, if they knew what they were talking about, would mean the first general-purpose, stored-program, digital, electronic computer with a large <(> 1000B)> memory. <1>



<div class="accordion">
<h3>Just give me a straight answer.</h3>
<div>
	No. But here are five big candidates:
	
	<ol>
		<li>The Antikythera mechanism is the first known computer, probably built around 100 BCE. It was not general-purpose.</li>

		<li>The Z3 (operational on 7th December 1941) was the first general-purpose machine to execute a program successfully.</li>

		<li>The Colossus Mark I (operational by January 1944) was the first large, fully electronic, programmable logic calculator.</li>

		<li>The ENIAC (operational November 1945) was the first electronic, general purpose, large scale, digital computer. (It claimed the generic title of "first computer" by default, partly because others were destroyed during bombings or hidden by classified status. Maybe also out of chauvinism, since computer science was pioneered in American universities.)</li>

		<li>The Manchester Baby (or 'Small-Scale Experimental Machine') was the first electronic computer to successfully execute a stored program was, operational by June 1948.</li>
	</ol>
</div>
	
<h3>Definitions</h3>
<div>

	* A distinction is sometimes made between computers and mere automata or calculators. Zuse's Z3 is perhaps a powerful 'electronic calculator', not a computer per se. This distinction does not have a physical basis. 

	Their point is to make universality a necessary condition of being 'a computer' - but this condition would disqualify some celebrated archetypal computers, like ABC and Colossus. It isn't empirically adequate.

	* Processor substrate: what is the important bit made of? 
		1. Wood
		2. Metal pistons
		3. Electrical circuits and metal pistons
		3. Just electrical circuits
	The dividing line between "electromechanical" and "electronic" is basically whether they used relays or not. Is the whole processor implemented with the motion of only electrons?

	* <b>"Automatically"</b> - acting without external intervention, beyond specifying the input and the program. I'm treating this as the core property of 'a computer'. An abacus is not a computer, because it doesn't do any operations for you. The Ishango bone, as far as anyone can tell, is also not a computer. A slide rule makes you do the intermediate steps. To exclude the possibility of chimpanzees being computers.

	* General-purpose: This is one of the most vague terms. Here I have contorted it to mean: If the device had unlimited memory (and time), could it simulate a (Universal) Turing machine; could it compute all that can be computed? 
	* Programmable:	Can you change what it computes without ? Not sure if this is used as "general-purpose".
	* Stored-program:	
	* Arithmetic type:
	* Value representation: This is more key than you'd think. Analogue computers did some amazing things, but speed comes from digits.
	* Base system: 
	* Memory
</div>

<h3>Also</h3>
<div>
	This is not even counting developments after 1960. Modern modern computers are transistor-based, parallel, chip-based, large-cache, easy to program, and none of the above have these properties. Mass production is the key to ours.

	Modern computers are profoundly complex and highly distinctive. 
	* As well as the above, they are <i>semiconductor based</i>, (MOS) which contributes to their speed and their tiny size relative to the first gen. 
	* Microprocessors
	* Giant memory. These predicates affect our experience of using computers more than did the differences between the Z3 and ENIAC.

</div>
<h3>How many first computers are there?</h3>
<div>
	Just using my crude model, which has six boolean variables and two ternary variables (Chinese menu), there are 576 <2^6 * 3^2> first computers. I have only bothered to identify 32 of them. Who's pedantic now?
</div>
<h3>Are shit computers still computers?</h3>
<div>
	The Antikythera mechanism didn't work very well, because of both the false astronomy and broad engineering tolerances of the time. It probably systematically predicted planets to be a full 40 degrees off the real course.

	Leibniz's calculator couldn't carry through.

	The theme is: is something a computer if it is systematically inaccurate?
</div>
<h3>Data</h3>
<div>
	Here.
</div>


</div>




{%	include comput/tree.js		%}

