---
layout: 	post
title:  	"Estimating political controversy <br>(research proposal)"
date:   	2017-01-07 20:31:40
author:		Gavin
summary:	A tiny research proposal for the Centre for Effective Altruism.
confidence: 60% on core equations. 80% that it is worth doing.
warnings: 	terrorism, inexorable ideologies
---
<!--categories: cause-prioritisation, effective-altruism, quantification-fever,-->
<script type="text/javascript" async
  src="https://cdn.mathjax.org/mathjax/latest/MathJax.js?config=TeX-AMS_CHTML">
</script>

Around 90 health workers were murdered while delivering polio vaccines to children in [2014][Poliohno]. Violent campaigns have recently also been waged against [schools][Schoolsout], [ancient ruins][Ruined], and [mid-level STEM professors][Unabomb]. We can conclude that _everything is controversial to someone_. We can do better than this vague shrug if we form a metric for political controversy.

My research question is _How can we estimate political controversy? How can we relate this to tractability?_ Intuitively, controversy (and associated social or political resistance to interventions) is a negative term for tractability. (And is a dominating factor, for any intervention requiring state resources, approval, or enforcement.) Naı̈vely, the relation would have at least the following terms:


\[ \text{tractability} = \Large{f}\left( \normalsize{\substack{\text{\href{http://mathurl.com/z83m8zq}{asocial}}\\\text{\href{http://mathurl.com/z83m8zq}{cost:benefit}}}\,,\, \substack{\text{social capital of}\\\text{ intervening party }}} \right) - \normalsize{\text{resistance}} \]
\emph{where}
\begin{align*}	
	\text{resistance} &= 	\Large{f} \left(\,\,	\normalsize{ \text{counter-activism,\,\, controversy \,}}   \right) \\\\
	\text{ counter-activism } &=	\Large{f} \left( \normalsize{ \text{ controversy},\,\, \substack{\text{social capital}\\\text{of opponents}}	\,,\,\, \text{P(violence)} } \,\,\right)
\end{align*}
\[ \text{controversy} = \left( \substack{\text{\% of population}\\\text{opposed}} \times  \left( 1 \,+\, \substack{\text{extent of state}\\ \text{involvement}}\right) + \,\substack{\text{weirdness}\\ \text{of cause}} \right) \times \substack{\text{media}\\ \text{coverage}}\]

(Resistance manifests as delay, negotiation & PR costs, extra legal costs, haz-
ard insurance, or reduced volunteering and uptake. Controversy appears in the
resistance equation, to model sensitivity to bad PR in politicians and NGOs.
‘Counter-activism’ is to be understood as effective obstruction.)
Motivation

To date, community attention has been focussed on low-controversy Pareto
improvements. The proposed work is important because much potential value
lies in controversial areas: for instance, state intervention offers vast resources
and irreplaceable co-ordination power, but the use of these is often defeated by
controversy and its entailed lobbying.

Also, within limits, controversy is a proxy for harm: the harm of mass preference violation. It can, then, subtract from net impact as well as from tractability.

One month's work should allow for 1) clarification of the relations, of this sort;
2) ranking of exemplar controversies; 3) data collection and scoring; 4) coeffi-
cient estimates for some cause (e.g. birth control is data-rich and controversial
enough).

### Sub-questions

* _How to quantify political resistance?_
(e.g. via historical studies: How much progress per million dollars on a cause? How often does violence attend work on it?)
* _What predicts political controversy?_
(Leads: social media sentiment analysis, news density, bipartisan lobbyist spending, legislation density, psychological availability, religious edicts.)
* _If equations are helpful, what are appropriate forms?_
* _What level of controversy precludes effective intervention?_
* _How does controversy accumulate and decay on an organisation like CEA? A movement like EA? What are its long-term effects?_

### Uncertainties

I’m not sure how to square the value of countercyclical moral leadership with the ITN model. I’m not sure how much moral weight to give preference violation, or its proxy, expressed disapproval [5]. I’m not sure how to disaggregate cause controversy, intervention controversy, and actor controversy.

I am not sure how much weight to put on historical case studies, relative to a classifier based on chatter. I’m not sure of the predictive power of my proxies, especially sentiment intensity.

[Poliohno]:		http://europe.newsweek.com/polio-related-murders-kill-more-disease-itself-287880?rm=eu 
[Schools-out]:	http://www.protectingeducation.org/sites/default/files/documents/eua_2014_full.pdf 
[Ruined]:		https://en.wikipedia.org/wiki/Destruction_of_cultural_heritage_by_ISIL 				
[Unabomb]:		https://en.wikipedia.org/wiki/Ted_Kaczynski#Casualties 