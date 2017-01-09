---
layout: 	math_post
title:  	"Estimating political controversy"
baselink:	/controversy
permalink:	/controversy-technical/
date:   	2017-01-07  <!--site.time-->
author:		Gavin	

visible:	1
simple:		true
technical:	true

summary:	A tiny research proposal for the Centre for Effective Altruism - can we quantify controversy / obstructionism?
confidence: 70% on equations' truth (because vague). 80% that it's worth doing.
warnings: 	terrorism, inexorable ideologies
categories: cause prioritisation, effective altruism, quantification fever
---



<!-- 	WORDS	-->
Around 90 health workers were murdered while delivering polio vaccines to children in 2014. <a href="#fn:1" id="fnref:1">1</a> Violent campaigns have also been waged recently against [schools][Schoolsout], [ancient ruins][Ruined], and [mid-level STEM professors][Unabomb]. We can conclude that _everything is controversial to someone_. We can do better than this vague shrug if we form a metric for political controversy.

My research question is _How can we estimate political controversy? How can we relate this to tractability?_ Intuitively, controversy - or, rather, the associated [social or political resistance to interventions][Obs]' - is a negative term in any equation for tractability. Obstructionism is a dominating factor for any intervention requiring state resources, approval, or enforcement.

Naı̈vely, the relation would involve at least the following terms:<a name="maths"></a><br><br>

<div align="center">

$$ \text{tractability} = \Large{f}\left( \normalsize{\substack{\text{asocial}\\\text{cost:benefit}}, \substack{\text{social capital of}\\\text{ intervening party }}} \right) - \normalsize{\text{obstruction}} $$
</div><i>where</i>

<div align="center">

$$
	\text{asocial cost:benefit}  = \Large{f}\left(\,\normalsize{ \substack{\text{prima facie}\\\text{cost:benefit}} \,,\,\, 				\substack{\text{intervention}\\\text{track record}} , \substack{\text{theoretical}\\\text{strength}} , 					\substack{\text{measurability}} }\,	\right)
$$
$$
	\text{obstruction} = 	\Large{f} \left(\,\,	\normalsize{ \text{counter-activism, controversy }}   \right) 
$$
$$
	\text{ counter-activism } =	\Large{f} \left( \normalsize{ \text{ controversy}, \substack{\text{social capital}\\\text{of opponents}}	, \text{P(violence)} } \right)
$$
$$ 
	\text{controversy} = \substack{\text{\% population}\\\text{opposed}} \times  \left( 1 \,+\, \substack{\text{extent of state}\\ \text{involvement}}\right) 
$$

<a href="#appendix"><button class="bigfoot-fake">...</button></a>
<br><br></div>

(Obstructionism manifests as delay, negotiation &amp; PR costs, extra legal costs, hazard insurance, or reduced volunteering and uptake. Controversy appears in the _obstruction_ equation, to model sensitivity to bad PR in politicians and NGOs.
‘Counter-activism’ is to be understood as _effective_ obstruction.)

<br><br>

---

<br>

### Motivation

To date, community attention has been focussed on low-controversy Pareto improvements. The proposed work is important because much potential value lies in controversial areas: for instance, state intervention offers vast resources and irreplaceable co-ordination power, but the use of these is often defeated by controversy and its entailed lobbying.

Also, within limits, controversy is a proxy for harm: the harm of mass preference violation. It can, then, subtract from net impact as well as from tractability.

One month's work should allow for: 

1. clarification of the relations <a href="#fn:3" id="fnref:3">3</a>;
2. ranking of exemplar controversies; 
3. data collection and scoring, maybe for a controversy classifier; 
4. coefficient estimates for some cause (e.g. birth control is data-rich and controversial enough).

<br><br>

---

<br>

### Sub-questions

* _How to quantify political resistance?_
(e.g. via historical studies: How much progress per million dollars on a cause? How often does violence attend work on it?)

* _What predicts political controversy?_
(_Leads_: social media sentiment analysis, news density, bipartisan lobbyist spending, legislation density, psychological availability, religious edicts.)

* _If equations are helpful, what are appropriate forms?_

* _What level of controversy precludes effective intervention?_

* _How does controversy accumulate and decay on an organisation like CEA? A movement like EA? What are its long-term effects?_

<br><br>

---

<br>

### Uncertainties

I'm not sure whether this research is best aimed at quantitative estimates of present controversies, a predictive model for identifying future ones, or as groundwork for some unfeasibly ambitious future [simulation](TheTerritory). 

I’m not sure how to square the value of countercyclical moral leadership with the 'Impact-Neglectedness-Tractability' (INT) model. I’m not sure how much moral weight to give preference violation, or its proxy, expressed disapproval. <a href="#fn:2" id="fnref:2">2</a>  I’m not sure how to disaggregate cause controversy, intervention controversy, and actor controversy. 

I am not sure how much weight to put on historical case studies, relative to a classifier based on chatter. I’m not sure of the predictive power of my proxies, especially sentiment intensity.

One weak source of validation would be [80,000 Hours' estimates][80k] of cause effectiveness, on a simple 1-5 scale. (Note that the lowest tractability causes there, immigration reform and catastrophic risks, are indeed the most politically involved causes.)

<br><br>

---

<br>

### Tractability, Formally

Cotton-Barrett has (tentatively) characterised tractability as either: the effort elasticity of a solution, where _S_ = solvedness, _W_ = total work completed:
$$
	\frac{ dS/S }{ dW/W }
$$
or as the probability of success weighted by inertia of effort, _p_ = the likelihood of eventual success:

$$
	k = p / ln(\frac{\text{marginal resources}}{ \text{total resources spent}})
$$
These are the senses I would investigate in relation to obstruction.

<br>

---

<a name="bibliography"><a/>
<br>
<br>

### Bibliography

* Cotton-Barratt, Owen (2014), "[Estimating cost-effectiveness for problems of unknown difficulty][CB2]", (tractability with no prior).
* Cotton-Barratt, Owen (2016), "Prospecting for Gold", talk at EAGxOxford 2016, 19/11/16.
* Garimella et al (2015), "[Exploring Controversy in Twitter][Garim-Twitter]"
* Garimella et al (2016), "[Quantifying Controversy in Social Media][Garim2]".
* Kahan, Dan (2015), "[The Politically Motivated Reasoning Paradigm][Kahan]"  (on the growth of ideological disagreement on empirical questions).
* Kittur et al (2009), "[What’s in Wikipedia?: Mapping Topics and Conflict Using Socially Annotated Category Structure][Kittur]"
* Rad and Barbosa (2009), "[Identifying controversial articles in Wikipedia: a comparative study][Rad]"
* Steenbergen et al (2003), "[A Discourse Quality Index][Steen]".
* Wiblin, Robert (2016), "[The Important/Neglected/Tractable framework needs to be applied with care][Wib]" (on misapplying INT, its ambiguity).


<br><br>
<a name="appendix"></a>

-----------------------
-----------------------

<br>
Or, speculatively, something more involved like:

<div align="center">
	
	$$ 
		\text{controversy} = \left( \substack{\text{\% population}\\\text{opposed}} \times  \left( 1 \,+\, \substack{\text{extent of state}\\ \text{involvement}}\right) + \,\substack{\text{weirdness}\\ \text{of cause}} \right) \times \substack{\text{media}\\ \text{coverage}}	
	$$
<big><a href="#maths">↵</a></big>
</div>



<br><br>

<!--	Tooltips	-->
<div class="footnotes"><ol>
    <!-- 1 -->
    <li class="footnote" id="fn:1">
        <br><i>Estimate quality</i>: OK, a <a href='/#spiegel-quality/'>Spiegelhalter (3*)</a>. (No data, no names given.)
        <br><i>Source</i>: 
        <a href="http://europe.newsweek.com/polio-related-murders-kill-more-disease-itself-287880?rm=eu ">Newsweek</a>.
        <br><i>Importance to argument</i>: Low.<br>&nbsp;
    </li><br>

	<!-- 2 -->
    <li class="footnote" id="fn:2">
        Though detailed precursors exist in the abstract, e.g. <a href='http://effective-altruism.com/ea/11i/the_effective_altruism_newsletter_open_thread/8m6'>Social Choice theory</a>.
    </li><br>
    
    <!-- 3 -->
    <li class="footnote" id="fn:3">
		e.g.: <span style="font-weight: bold">Sources of political controversy</span>:<br>

		<ul>
		<li> <i>Asymmetric information</i>. Spurious controversies, based in error - as plausibly occurred when Boko Haram killed polio health workers on suspicion of being American spies. We might think that the sustainable strategy is simple: do not compromise, but spread the truth.</li><br>

		<li> <i>Bias (thick empirical judgments)</i>. People will endorse different empirical claims according to their ideology. A major obstacle to working in controversial cause areas is, then, what Dan Kahan calls the "politically motivated reasoning paradigm", an apparently increasing tendency for people to interpret evidence in favour of their political side (especially highly educated people, he claims).</li><br>

		<li> <i>Irreconcilable value pluralism</i>. The least tractable of all: options are convert or overrule.</li>
		</ul>
	  
    </li><br>
</ol></div>


<!---->

[Spiegel]:		http://technicalities.netlify.com/metrics/#spiegel-quality
[Poliohno]:		http://europe.newsweek.com/polio-related-murders-kill-more-disease-itself-287880?rm=eu 
[Schoolsout]:	http://www.protectingeducation.org/sites/default/files/documents/eua_2014_full.pdf 
[Ruined]:		https://en.wikipedia.org/wiki/Destruction_of_cultural_heritage_by_ISIL 				
[Unabomb]:		https://en.wikipedia.org/wiki/Ted_Kaczynski#Casualties 
[Obs]:			https://en.wikipedia.org/wiki/Obstructionism

[80k]:			https://80000hours.org/2014/01/which-cause-is-most-effective-300/#TheList

[Garim-Twitter]:	https://arxiv.org/pdf/1512.05550.pdf
[Garim2]:		https://arxiv.org/pdf/1507.05224v1.pdf
[CB2]:			https://www.fhi.ox.ac.uk/estimating-cost-effectiveness/
[Kittur]:		http://www-users.cs.umn.edu/~echi/papers/2009-CHI2009/p1509.pdf
[Steen]:		http://content.csbs.utah.edu/~burbank/steenbergen2003.pdf
[Kahan]:		https://papers.ssrn.com/sol3/papers.cfm?abstract_id=2703011
[Rad]:			http://www.math.iit.edu/~rellis/teaching/454553All/GoodModules/WikipediaEditWars.pdf
[Wib]:			http://effective-altruism.com/ea/ss/the_importantneglectedtractable_framework_needs/
[TheTerritory]:	https://en.wikipedia.org/wiki/Data_generating_process