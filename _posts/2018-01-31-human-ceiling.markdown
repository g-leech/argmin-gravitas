---
layout:     post
title:      "A ceiling for human expertise"
baselink:   /ceiling
permalink:  /ceiling
date:       2018-01-31  <!--site.time-->
author:     Gavin

img:        /img/agz_3b.png
published:	true
visible: 	1

summary:    Does this auxiliary result from AlphaGo Zero have large implications?
confidence:	60%; fairly speculative.
categories: artificial intelligence, forecasting, expertise
warnings:	
importance: 7
wordcount:		192
---

{%	include ceiling/links.html		%}

    
This figure appears in DeepMind's instant-classic paper '<a href="{{agz}}">Mastering the Game of Go without Human Knowledge</a>' (2017):
<br><br>	

<div style="text-align:center"><img src="/img/agz_3b.png" width="70%"/></div>


<blockquote>
	<b>Figure 3b:</b> 
	'<i>Prediction accuracy on human professional moves. The plot shows the accuracy of the neural network at each iteration of self-play, in predicting human professional moves...<br> The accuracy measures the percentage of positions in which the neural network assigns the highest probability to the human move</i>.'
</blockquote><br>

It shows that AlphaGo Zero (AGZ) only predicts human pro moves with 50% accuracy, at best. That is, AGZ disagrees with human professionals on 50% of moves.<br><br>

This perhaps has implications for human expertise in general, by the following argument:<br><br>

    1. AGZ plays far beyond peak human ability.

    2. AGZ would play differently from a peak human in 50% of moves.

    3. So a peak human makes suboptimal moves at least 50% of the time.

    4. Go is an excellent environment for human learning 
    (small ruleset, rapid objective feedback, amenable to intuition). 

    5. So, relative to more complex domains, human mastery of Go should be 
    relatively complete.

    6. So we can expect human experts in other, more complex domains to make 
    suboptimal decisions at least 50% of the time.

<br><br>

Regarding premise 4, <a href="{{ericsson}}">Ericsson</a> says learning occurs if people are "1) given a task with a well-defined goal, 2) motivated to improve, 3) provided with feedback, 4) provided with ample opportunities for repetition and gradual refinements of their performance"


{%  include comments.html %}