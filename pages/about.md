---
layout: 	page
title: 		About
permalink:	/about/
visible:	true
---

{%	include about/links.html	%}
<div>


</div>
I'm <a href="{{ '/cv.pdf' | prepend: site.url }}">Gavin</a>, a PhD candidate in AI at <a href="{{bris}}">Bristol</a>. <a href="#fn:18" id="fnref:18">18</a> 
<a href="/no-philosopher">I</a> <a href="{{sittler}}">like</a> <a href="{{cs}}">technical</a> <a href="{{orseau}}">solutions</a> <a href="{{welf}}">to</a> <a href="{{shminux}}">philosophical</a> <a href="{{comp1}}">problems</a>.<br><br>
<!--  -->
Technically <a href="{{ea}}">an EA blog</a>, though I don't consider this a blog. (<a href="/sites">Blogging is more speech than writing</a>.)<br><br>
<!--  -->
If you want to talk, get in touch by <a href="mailto:{{ site.email }}">email</a>, or anonymously <a href="{{Form}}">here</a>, or extremely privately (via PGP) <a href="{{pgp}}">here</a>.
<br><br>


<!-- <div class="accordion">
	<h3>Good arguments</h3>
	<div>
		{%		include about/arguments.html		%}
	</div>
</div>
 -->

<div class="accordion">	
	<!-- %	include about/now.md	%} -->
	{%	include about/research.html	%}	
	{%	include about/errata.html	%}	
	{%	include about/giving.html	%}	
	{%	include about/books.html	%}	
	{%	include about/misc.html	%}
</div>


{%	include about/foots.html	%}



{%	include padder.html 	howMuch=16	%}