---
layout:     math_post
title:      "Uncritical thinking"
baselink:   /uncritical
permalink:  /uncritical
date:       2020-01-16
original:	2015?
author:     Gavin   
img:        /img/alex.jpg

visible:    1
published:  true

summary:    Claims about critical thinking gains from university
quality:    6
confidence: 80%
importance: 5
wordcount:      400 
categories: rationality, uni, social-science
argument:	
---

{%	assign down = "https://dera.ioe.ac.uk/30244/1/Earnings-by-Degrees-REPORT-1.pdf" %}
{%	assign posh = "http://www-users.york.ac.uk/~pbjw1/Paul%20Wakeling,%20PhD%20thesis%202009,%20Social%20class%20and%20access%20to%20postgraduate%20education%20in%20the%20UK.pdf"		%}
{%	assign minis = "https://en.wikipedia.org/wiki/Politician%27s_syllogism"		%}
{%	assign philo = "https://journals.sagepub.com/stoken/rbtfl/pQjIrfdSdFNuQ/full"	%}
{%	assign nous = "http://dailynous.com/2015/10/22/does-philosophy-improve-critical-thinking/"		%}
{%	assign peacock = "https://www.smbc-comics.com/comic/2014-11-01"		%}
{%	assign cohen = "https://www.ncbi.nlm.nih.gov/pubmed/26487051"		%}

> The intellectual skills you will gain in critical analysis and communication developed through your programme will be greatly sought after by employers for any career you choose, including business, and your options will be wide.

<a href="#fn:2" id="fnref:2">2</a>

<br>
My philosophy department used to trumpet their graduates' income statistics as evidence that critical thinking is valued in industry, and so as evidence that taking philosophy is prudent. <a href="#fn:1" id="fnref:1">1</a>

This was an amusing triple failure of critical thinking: they confuse correlation and causation ("_philosophy degree and income gain, therefore philosophy degree causes income gain_"); they fail to consider selection effects (philosophy students are a bit <a href="{{posh}}">posher</a> (Fig 6.10) or <a href="{{peacock}}">foolhardy</a> than the average student, so there's a confounder); and it can be read as a <a href="{{minis}}">Yes Minister fallacy</a>:

1. A philosophy degree causes an income premium. 
2. If something causes an income premium then it is valued in industry.
3. A philosophy degree causes critical thinking. 
4. Therefore, critical thinking is valued in industry. 

<center><img src="/img/yes.jpg" width="15%" /></center>

<br>

---

<br>

As it happens there is <a href="{{philo}}">some evidence</a> for university imparting critical thinking skills, maybe half a standard deviation over 4 years. <a href="{{nous}}">Amusingly</a>, philosophy does not stand out from any other university subject. A fourth problem with the shilling above.

Actually, in the spirit of the thing, let's take a look at that meta-analysis.

> Students’ critical-thinking skills do improve in college. The difference is comparable to a student whose critical-thinking skills start at the 50th percentile and, after four years in college, move up to the 72nd.

But do they follow nonstudents of the same age and cognitive ability? Maybe the gains are just from the brain maturing, in or out of college. ("<i>The most common exclusion factors were... using a noncollege sample (11.4%)"</i>) So they explicitly choose not to compare to the control group!

They note a massive collapse in estimated effect size over the years - conceivably due to The Saecular Corruption of Higher Learning, but perhaps more likely due to the spread of less superstitious, less hermetic statistical methods among those who study the studiers:

> Given an equal mix of cross-sectional and longitudinal studies, the predicted 4-year gain is 1.22 SDs for a study published in 1963 (80% credibility interval [0.75, 1.68]), whereas the predicted gain is only 0.33 for a study published in 2011 (80% credibility interval [−0.11, 0.78]).

Shall we extrapolate that trend to the critical thinking meta-analyses of 2059? (Note that the 2011 credible interval includes a possible _loss_ of critical thinking!)

They talk of SD gains, and the absurdity of +6 SD, in terms of Cohen's d. <a href="{{cohen}}">So</a> they are assuming that critical skill is normally distributed. Why? ("_Variations on this effect size are commonly used in critical thinking research._" O well that's alright then.)

What's the construct they're measuring? Ability to pattern-match basic fallacies (like I did above)? The ability to win arguments? "Has this person heard the following words you always hear at university?"

<br><br>


<div class="footnotes">
<ol>
	<li class="footnote" id="fn:1">
 	(They don't trump anymore, possibly because <a href="{{down}}">philosophy was recently associated with decreased earnings</a>, in the UK. Instead they say barely grammatical things like the opening quotation.) 
	</li>
<!--  -->
	<li class="footnote" id="fn:2">
		Let it be known that I resisted calling this post "Uncritical critical-thinking thinking". A triumph of critical thought.
	</li>
</ol>

{%  include comments.html %}