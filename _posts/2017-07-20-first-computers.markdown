---
layout:     post
title:      "Disambiguating the first computer"
baselink:   /computers
permalink:  /computers
date:       2017-08-15  <!--site.time-->
author:     Gavin

img:        /img/smolBoo.jpg
published:	false
visible: 	1
best:		1
stylised:	true

summary:    App for defining "computer" and so selecting between candidate first computers.
confidence:	95%
categories: computing, history
warnings:	unfair posterity.
count:		
---

{%	include comput/links.html	%} 



<center>
	<img src="/img/boo.png" width="60%" height="60%" style="padding-bottom:80px;padding-top:20px">
	
</center>


> Let me emphasize that there is no such thing as "first" in any activity associated with human invention. If you add enough adjectives to a description you can always claim your own favorite. For example the ENIAC is often claimed to be the "first electronic, general purpose, large scale, digital computer" and you certainly have to add all those adjectives before you have a correct statement... 

<center style="padding-bottom:80px">- Michael Williams</center>




{%	include comput/app.html		%}



<div class="accordion">

	<h3>Just give me a straight answer.</h3>
	<div>
		No. Here are some big candidates:<br><br>
		
		<ol>
			<li>The <a href="{{Antikythera}}">Antikythera mechanism</a> is the first known computer, probably built around 100 BCE. It was just a 'calculator'.</li><br>

			<li>Honourable mention to Charles Babbage and his unbuilt <a href="{{enjinn}}">Analytical Engine</a>: if he had had about 10 times more money, he might well have built the first general-purpose digital machine around 1850.</li><br>

			<li>The <a href="{{Z3}}">Z3</a> (operational on 7th December 1941) was the first general-purpose digital machine to execute a program successfully. (Its inventor, Konrad Zuse, also rediscovered Shannon's <a href="{{shannon}}">great breakthrough</a> and much else besides.)</li><br>

			<li>The <a href="{{colossus}}">Colossus Mark I</a> (operational on 8th December 1943) was the first fully-electronic programmable digital computer. It was just a '5KHz logic calculator'.</li><br>

			<li>The <a href="{{ENIAC}}">ENIAC</a> (operational by November 1945) was the first fully-electronic general-purpose digital computer. <a href="#fn:1" id="fnref:1">1</a> </li><br>

			<li>The <a href="{{Baby}}">Manchester Baby</a> (operational by June 1948) was the first fully-electronic, general-purpose digital computer to successfully execute a 'stored program', a set of instructions loaded into its own memory as data.</li>
		</ol>
	</div>

	<h3>Definitions</h3>
	<div>

		{%	include comput/defs.html		%}

	</div>


	<h3>Questions</h3>
	<div>
		<p style="font-weight:bold">How many first computers are there?</p>

		My crude model (which has nine boolean variables and three ternary variables), implies that there are up to 4608 (<code>2^9 * 3^3</code>) first computers. I have only bothered to identify <a href="{{myData}}">41</a> of them. Who's pedantic now?<br><br><br>


		<p style="font-weight:bold">What about newer first computers?</p>

		I am not even counting developments after 1970. You could also add "synchronous/asynchronous", "uni/multiprocessing", classical/quantum", <a href="{{Flynn}}">Flynn's four streams</a>, a fourth value for the physical medium: "optical" computing... <br><br><br>


		<p style="font-weight:bold">Are shit computers still computers?</p>

		The Antikythera mechanism didn't work very well, because of a false astronomical theory and the crude engineering tolerances of the time. It probably systematically predicted planets to be a full 40 degrees off the real course. Nor could Leibniz's calculator carry through. <br><br>

	</div>


	<h3>Data</h3>
	<div>
		<a href="{{myData}}">Here</a>. A lot of the specs in that are from the remarkable conference papers in <i><a href="{{Rojas}}">The First Computers</a></i>. <a href="{{jaundice}}">John Morrice</a> wrote a decision tree in Python which inspired my solution.
	</div>


</div>



{%  include comments.html %}

{%  include comput/foots.html %}

