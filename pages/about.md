---
layout: 	page
title: 		About
permalink:	/about/
visible:	true
---

{%	include about/links.html	%}



<div style="padding:20px">
	I'm <a href="{{ "/cv.pdf" | prepend: site.url }}">Gavin</a>, an incoming PhD candidate in artificial intelligence at Bristol. I focus on <a href="{{debate}}">augmenting human oversight</a> over RL agents.

	I write words and code about things - important things or, failing that, interesting things, or, failing that, boring true things. My favourite thing to write about <a href="{{sittler}}">is</a> <a href="{{cs}}">technical</a> <a href="{{gelman}}">solutions</a> <a href="{{welf}}">to</a> <a href="{{shminux}}">philosophical</a> <a href="{{comp1}}">problems</a>.<br><br>
	
    Technically <a href="{{ea}}">an EA blog</a>, though I don't consider this a blog. (Blogging is more speech than writing.)<br><br>

	$10 to anyone who finds a factual or logical error in my posts. You can contact me by <a href="mailto:{{ site.email }}">email</a>, or anonymously <a href="{{Form}}">here</a>, or extremely privately (via PGP) <a href="{{pgp}}">here</a>.
	<br><br>
</div>


<!-- <div class="accordion">
	<h3>Good arguments</h3>
	<div>
		{%		include about/arguments.html		%}
	</div>
</div>
 -->

<div class="accordion">	
	{%	include about/now.md	%}

	{%	include about/giving.html	%}	
	
	{%	include about/favs.html	%}	
	
	{%	include about/misc.html	%}

</div>


{%	include about/foots.html	%}



<!-- If big screen, pad down the footer -->
<style>
	@media (min-width: 30em) {
	#padder {
		height: 29.5vh;
	}
}
</style>

<div id="padder"></div>
