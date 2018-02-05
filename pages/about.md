---
layout: 	page
title: 		About
permalink:	/about/
visible:	true
---

{%	include about/links.html	%}



<div style="padding:20px">
	I'm <a href="{{ "/cv.pdf" | prepend: site.url }}">Gavin Leech</a>, a data scientist at the giant insurer <a href="{{Axa}}">AXA</a> <a href="#fn:4" id="fnref:4">4</a>. I write prose and software about things like <a href="{{welf}}">the unit of caring</a>, <a href="{{cs}}">quantitative epistemology</a>, and the <a href="{{stat}}">applied philosophy of science</a>. 

	I don't think of this as a blog (I see blogging as more like speech than writing).<br><br>

	You can contact me by <a href="mailto:{{ site.email }}">email</a> or anonymously <a href="{{Form}}">here</a>.
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
	<h3>Giving</h3>
	<div>

		<a href="{{GWWC}}"><img src="/img/GWWC.jpg" hspace="20" width="100px" height="100px" align="left" /></a>
		
		I'm a member of <a href="{{GWWC}}">Giving What We Can</a>, people who pledge substantial amounts of their lifetime income to the most effective charitable causes. I'm donating <a href="{{MyGiving}}">10% this year</a>, aiming at 50% overall <a href="#fn:2" id="fnref:2">2</a>. My reasoning on the most important causes, and my present basket of charities is forthcoming.<br><br><br>

		I am cause-neutral (in the sense that I will support whatever is highest value, to the extent that objective evaluation is possible) and cause-sceptical (in the sense that we don't really know what the best thing to do is).

		<!--     80,000 Hours on moral stance and strategy
        <ul>
            <li><i>By launching a concerted campaign, a relatively small group of people can make a significant and lasting change to society’s moral attitudes to an important issue.</i>
            Not sure. It looks that way, but we can't make a causal inference; activists could be an epiphenomenon. </li>
            <li><i>Empowering people with better technology and wealth is a good way to build a good long-term future for society. New technology, information and economic growth don't usually have such large downsides that they outweigh the benefits.</i>
            Agree (except vis a vis AI risk).</li><br>
            <li><i>How likely is it that humans will go extinct in the next 50 years?</i>
            Unlikely but possible (1 - 15% chance)</li>
            <li><i>two scenarios: A nuclear war kills 90% of the human population, but we rebuild and civilization eventually recovers. A nuclear war kills 100% of the human population and no people live in the future. How much worse is the second scenario?</i>
            Vastly worse. 100x +</li>
            <li><i>If you want to have a large positive impact, it’s often better to focus on high-risk, high-return opportunities, like speculative research, social advocacy or entrepreneurship (as opposed to more proven ways to do good, like distributing vaccines).</i>
            Agree</li>
            <li><i>Promoting moral concern for all conscious beings is one of the best ways to build a good long-term future.</i>
            Agree. Consequentialism is harder to twist into totalitarianism: nothing justifies causing suffering except greater suffering. </li><br>
            <li>How likely is it that humanity will create AI systems that can perform nearly all human professions as well humans in the next 50 years?
            Unlikely but possible (1 - 15)</li><br>
        </ul>
    -->
	</div>
	
	<h3>Psychology</h3>
	{%		include about/psychology.html		%}

	{%		include about/code.html		%}

	{%		include about/misc.html		%}
</div>


{%		include about/foots.html	%}


<!-- If big screen, pad down the footer -->
<style>
	@media (min-width: 30em) {
	#padder {
		height: 29.5vh;
	}
}
</style>

<div id="padder"></div>
