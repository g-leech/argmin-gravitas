---
layout: 	math_post
title:  	"Estimating political controversy"
date:   	2017-01-07 20:31:40  <!--site.time-->
author:		Gavin
summary:	A tiny research proposal for Centre for Effective Altruism - can we quantify controversy / obstructionism?
confidence: 60% on core equations. 80% that it is worth doing.
warnings: 	terrorism, inexorable ideologies
---
<!--categories: cause-prioritisation, effective-altruism, quantification-fever,-->
{%	assign murderTip = "<i>Estimate quality</i>: [Spiegelhalter (2*)][Spiegel]. No data, no names.<br><i>Source</i>: Newsweek.<br><i>Importance to argument</i>: Low."	%}

<a href="#" data-toggle="tooltip" animation="true" html="true" delay="100" title="hmmm">Around 90</a> 

[health workers][Poliohno] were murdered while delivering polio vaccines to children in 2014. Violent campaigns have recently also been waged against [schools][Schoolsout], [ancient ruins][Ruined], and [mid-level STEM professors][Unabomb]. We can conclude that _everything is controversial to someone_. We can do better than this vague shrug if we form a metric for political controversy.


My research question is _How can we estimate political controversy? How can we relate this to tractability?_ Intuitively, controversy - or, rather, the associated social or political resistance to interventions, 'obstructionism' - is a negative term for tractability. Obstructionism is a dominating factor for any intervention requiring state resources, approval, or enforcement.

Naı̈vely, the relation would involve at least the following terms:<br><br>

<div align="center">

$$ \text{tractability} = \Large{f}\left( \normalsize{\substack{\text{asocial}\\\text{cost:benefit}}, \substack{\text{social capital of}\\\text{ intervening party }}} \right) - \normalsize{\text{resistance}} $$
</div><i>where</i><br>
<div align="center">
$$
	\text{asocial cost:benefit}  = \Large{f}\left(\,\normalsize{ \substack{\text{prima facie}\\\text{cost:benefit}} \,,\,\, 				\substack{\text{intervention}\\\text{track record}} , \substack{\text{theoretical}\\\text{strength}} , 					\substack{\text{measurability}} }\,
	\right)
$$
$$
	\text{resistance} = 	\Large{f} \left(\,\,	\normalsize{ \text{counter-activism, controversy }}   \right) 
$$
<br>
$$
	\text{ counter-activism } =	\Large{f} \left( \normalsize{ \text{ controversy}, \substack{\text{social capital}\\\text{of opponents}}	, \text{P(violence)} } \right)
$$
<br>
$$ 
	\text{controversy} = \left( \substack{\text{\% population}\\\text{opposed}} \times  \left( 1 \,+\, \substack{\text{extent of state}\\ \text{involvement}}\right) + \,\substack{\text{weirdness}\\ \text{of cause}} \right) \times \substack{\text{media}\\ \text{coverage}}	
$$
<br><br></div>

(Obstructionism manifests as delay, negotiation &amp; PR costs, extra legal costs, hazard insurance, or reduced volunteering and uptake. Controversy appears in the resistance equation, to model sensitivity to bad PR in politicians and NGOs.
‘Counter-activism’ is to be understood as effective obstruction.)
<br>

---

<br>

### Motivation

To date, community attention has been focussed on low-controversy Pareto improvements. The proposed work is important because much potential value lies in controversial areas: for instance, state intervention offers vast resources and irreplaceable co-ordination power, but the use of these is often defeated by controversy and its entailed lobbying.

Also, within limits, controversy is a proxy for harm: the harm of mass preference violation. It can, then, subtract from net impact as well as from tractability.

One month's work should allow for: 

1. clarification of the relations, [of this sort](#appendix);
2. ranking of exemplar controversies; 
3. data collection and scoring, maybe for a controversy classifier; 
4. coefficient estimates for some cause (e.g. birth control is data-rich and controversial enough).
<br>

---

<br>

### Sub-questions

* _How to quantify political resistance?_
(e.g. via historical studies: How much progress per million dollars on a cause? How often does violence attend work on it?)

* _What predicts political controversy?_
(Leads: social media sentiment analysis, news density, bipartisan lobbyist spending, legislation density, psychological availability, religious edicts.)

* _If equations are helpful, what are appropriate forms?_

* _What level of controversy precludes effective intervention?_

* _How does controversy accumulate and decay on an organisation like CEA? A movement like EA? What are its long-term effects?_
<br>

---

<br>

### Uncertainties

I’m not sure how to square the value of countercyclical moral leadership with the ITN model. I’m not sure how much moral weight to give preference violation, or its proxy, expressed disapproval [5]. I’m not sure how to disaggregate cause controversy, intervention controversy, and actor controversy.

I am not sure how much weight to put on historical case studies, relative to a classifier based on chatter. I’m not sure of the predictive power of my proxies, especially sentiment intensity.
<br><br>

<a name="appendix"><a/>

-----------------------
-----------------------

<br>

#### Appendix: example of extra conceptual analysis needed.


**Sources of political controversy**

* _Asymmetric information_. A spurious controversy, based in error - as plausibly occurred when Boko Haram killed polio health workers. We might think that the sustainable strategy is simple: do not compromise, but spread the truth.

* _Bias (thick empirical judgments)_. People will endorse different empirical claims according to their ideology. A major obstacle to working in controversial cause areas is, then, what Dan Kahan calls the "politically motivated reasoning paradigm", an apparently increasing tendency for people to interpret evidence in favour of their political side (especially highly educated people, he claims).

* _Irreconcilable value pluralism_. The least tractable of all: options are convert or overrule.



[Spiegel]:		http://technicalities.netlify.com/metrics/#spiegel-quality
[Poliohno]:		http://europe.newsweek.com/polio-related-murders-kill-more-disease-itself-287880?rm=eu 
[Schoolsout]:	http://www.protectingeducation.org/sites/default/files/documents/eua_2014_full.pdf 
[Ruined]:		https://en.wikipedia.org/wiki/Destruction_of_cultural_heritage_by_ISIL 				
[Unabomb]:		https://en.wikipedia.org/wiki/Ted_Kaczynski#Casualties 