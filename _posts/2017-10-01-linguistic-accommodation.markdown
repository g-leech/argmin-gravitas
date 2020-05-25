---
layout: 	math_post
title:  	"Modelling linguistic accommodation"
baselink:	/accommodation
permalink:	/accommodation/
date:   	2015-07-01  <!--site.time-->
author:		Gavin	
img:		/img/accommodation/NFSM.jpg

visible:	1
published: 	true
technical:	1
quality: 	5

summary:	Intro to computational linguistics, including a little observational study. 
confidence: 80% in the exposition.
warnings: 	A rewrite and replication is in progress.
categories: stats, social-science
importance: 4
pride: 		5
wordcount: 		12,600
---

{%  include accommodation/links.html %}


<center><img src="/img/bruegel.jpg" width="50%" height="50%" />
	<br>
	<a href="#fn:0" id="fnref:0">0</a>
	<br>
</center><br><br>


> For example, you may come across definitions like this: "A finite state automaton
is a quintuple ($$Q$$, $$\Sigma$$ , $$q_0$$, $$F$$, $$\delta$$) where Q is a finite set of states ($$q_0$$, $$q_1$$, ..., $$q_n$$ ), $$\Sigma$$ is a
finite alphabet of input symbols, $$q_0$$ is the start state, $$F$$ is the set of final states $$F \in Q$$, and $$\delta \in Q \times \Sigma \times Q$$, the transition function."

> That definition should be taken outside and shot.  <a href="#fn:3" id="fnref:3">3</a>

<center>~ John Coleman</center>

<br><br>

<!-- (This is my MSc thesis, on using computers to detect changes in how people talk to each other. It represents no great advance, but I learned a ton and wrote <a href="{{py2htk}}">a little library</a> to ease future work. 

The data isn't open, but I'm planning to record and annotate another set of conversations, so those of you at home can sing along.)<br><br><br>
 -->
<hr />
<br><br>

'<a href="{{cat}}">Accommodation</a>' is that thing where you automatically mimic the person you're talking to. You might immediately think of baby talk and speaking loudly to old people, but these conscious games are not what I'm talking about. For humans also unconsciously shift speech, depending on the gender, status, and likeability of their interlocutor. Accommodation is pervasive, correlated with key bits of human interaction: empathy, status, and teamwork. Studying accommodation puts you at the intersection of statistical modelling, linguistics and social signal processing.

The trick is to detect it using Hidden Markov models (HMMs). A technique from speaker verification is adapted: model-conditional probabilities estimate the ‘distance’ of each speaker, from their interlocutor, for each word. This likelihood ratio is taken for each word uttered by a speaker, relative to their interlocutor uttering it. The correlation of these ratios over time is used to infer the presence of accommodation and estimate effect sizes. 

I used the dataset from <a href="{{sssv}}">Stuart-Smith et al (2015)</a> (henceforth "SSSV" after the surnames of the authors): n=120,000 words, from 6 pairs of speakers. The modelling and data analysis was <a href="{{py2htk}}">implemented in Python</a>, with modelling tools from the ‘Hidden Markov Toolkit’ (HTK). <a href="#fn:2" id="fnref:2">2</a>

<br><br><br>


<hr /><br>


<h2>1. Introduction</h2>

{%		include accommodation/intro.md		%}


<h2>2. Glossaries</h2>

{%		include accommodation/gloss.md		%}
{%		include accommodation/maths.md		%}


<h2>3. Sequence modelling with state machines.</h2>

{%		include accommodation/fsm.md		%}


<h2>4. Hidden Markov modelling for linguistics</h2>

{%		include accommodation/comp-ling.md		%}



<h2>5. Present methodology</h2>

{%		include accommodation/method.md	%}


<h3>6. Results </h3>

{%		include accommodation/results.md	%}


<hr /><br>

<h3>7. Conclusion</h3>

{%		include accommodation/conc.md	%}


<br>
<hr />

{%  	include accommodation/biblio.html %}

{%  	include comments.html %}

{%  	include accommodation/foots.html %}

