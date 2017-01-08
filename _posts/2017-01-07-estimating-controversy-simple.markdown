---
layout: 	post
title:  	"Controversy in numbers"
baselink:	/controversy
permalink:	/controversy-simple/
date:   	2017-01-08  <!--site.time-->
author:		Gavin	

visible:	0
simple:		true
technical:	true
---

[Around 90 health workers][Poliohno] were murdered while delivering polio vaccines to children in 2014. Other people have attacked against [schools][Schoolsout], [ancient ruins][Ruined], and [mid-level science professors][Unabomb]. We can conclude that _everything is controversial to someone_. We could do better than this if we had a way to measure controversy.

First, what is "tractability"?

<p><b>tractability</b>: how easy something is to solve - how much you get done for a given amount of money, for instance.</p>

My question is _How can we estimate political controversy? How can we relate this to tractability?_ Controversy, and the difficulties that come with it, are a large drag on tractability. (They are a particularly big problem for anything which wants to use government resources.)

On the face of it, these two things would be connected in the following way:<br><br>

<!--

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

<br><br></div>

<sup>[+]</sup> Or speculatively:
<!--<div align="center">
$$ 
	\text{controversy} = \left( \substack{\text{\% population}\\\text{opposed}} \times  \left( 1 \,+\, \substack{\text{extent of state}\\ \text{involvement}}\right) + \,\substack{\text{weirdness}\\ \text{of cause}} \right) \times \substack{\text{media}\\ \text{coverage}}	
$$

</div>-->

(Conflict could cause delays, extra costs for lawyers, negotiators, marketers, insurance, and might reduce the number of people willing to use the product, or to volunteer. Controversy appears in the resistance equation, to model sensitivity to bad PR in politicians and NGOs.)

<br><br>

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

I’m not sure how to square the value of countercyclical moral leadership with the Impact-Tractability-Neglectedness (ITN) model. I’m not sure how much moral weight to give preference violation, or its proxy, expressed disapproval.<sup>[+]</sup> I’m not sure how to disaggregate cause controversy, intervention controversy, and actor controversy.

<!-- [+] Though some detailed precursors exist, in the form of [Social Choice theory][SocialChoice]-->

I am not sure how much weight to put on historical case studies, relative to a classifier based on chatter. I’m not sure of the predictive power of my proxies, especially sentiment intensity.

One weak source of validation would be [80,000 Hours' estimates][80k] of cause effectiveness, on a simple 1-5 scale. (Note that the lowest tractability causes there, immigration reform and catastrophic risks, are indeed the most politically involved causes.)


<br><br>
<br><br>

<!---->


[Spiegel]:		http://technicalities.netlify.com/metrics/#spiegel-quality
[Poliohno]:		http://europe.newsweek.com/polio-related-murders-kill-more-disease-itself-287880?rm=eu 
[Schoolsout]:	http://www.protectingeducation.org/sites/default/files/documents/eua_2014_full.pdf 
[Ruined]:		https://en.wikipedia.org/wiki/Destruction_of_cultural_heritage_by_ISIL 				
[Unabomb]:		https://en.wikipedia.org/wiki/Ted_Kaczynski#Casualties 
[Obs]:			https://en.wikipedia.org/wiki/Obstructionism
[SocialChoice]:	http://effective-altruism.com/ea/11i/the_effective_altruism_newsletter_open_thread/8m6
