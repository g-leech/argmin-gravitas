---
layout: 	page
title: 		About
permalink:	/about/
visible:	true
---

{%	include about/links.html	%}
<div>


</div>
I'm <a class="noline" href="{{ '/cv.pdf' | prepend: site.url }}">Gavin Leech</a>, a PhD candidate in AI at Bristol. <a href="/no-philosopher">I</a> <a href="{{sittler}}">like</a> <a href="{{cs}}">technical</a> <a href="{{orseau}}">solutions</a> <a href="{{welf}}">to</a> <a href="{{shminux}}">philosophical</a> <a href="{{comp1}}">problems</a>.<br><br>
<!-- https://www.scottaaronson.com/papers/philos.pdf -->
<!--  -->
Technically an <a href="{{ea}}">EA blog</a>, though I don't consider this a blog. (Blogging is more <a class="noline" href="/sites">speech</a> than writing.)<br><br>
<!--  -->
If you want to talk, get in touch by <a href="mailto:{{ site.email }}">email</a>, or <a href="{{Form}}">anonymously</a>, or extremely privately <a href="{{pgp}}">via PGP</a>.
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



{%	include padder.html 	howMuch=24	%}