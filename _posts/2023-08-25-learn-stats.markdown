---
layout:     post
title:      "Learn stats without going mad"
baselink:   /stats
permalink:  /stats
date:       2023-08-24  <!--site.time-->
author:     Gavin

img:        /img/badsciencelong.png
published:  true
visible:    1
quality:    4
importance: 8

summary:    How to be less bad at it than most
confidence: 80%
categories: stats, lists
warnings:   
wordcount:      
---


{%  assign whichbook = "https://www.bradyneal.com/img/books_flowchart.svg"  %}
{%  assign z = "https://complexityzoo.net/Complexity_Zoo"    %}
{%  assign common = "https://libgen.rs/book/index.php?md5=57B0BDF7B98D8DF75F37B8887F98AD10"    %}
{%  assign tripp = "https://twitter.com/Tjdriii/status/1694540048337678469"    %}
{%  assign glm = "https://lindeloev.github.io/tests-as-linear/"   %}
{%  assign ros = "https://avehtari.github.io/ROS-Examples/"   %}
{%  assign gel = "https://statmodeling.stat.columbia.edu/"    %}
{%  assign brei = "https://projecteuclid.org/journals/statistical-science/volume-16/issue-3/Statistical-Modeling--The-Two-Cultures-with-comments-and-a/10.1214/ss/1009213726.full"    %}
{%  assign prob = "https://jonathanweisberg.org/vip/"    %}
{%  assign ocul = "https://www.r-bloggers.com/2016/11/inter-ocular-trauma-test"    %}
{%  assign synth = "https://statmodeling.stat.columbia.edu/2019/03/23/yes-i-really-really-really-like-fake-data-simulation-and-i-cant-stop-talking-about-it" %}
{%  assign ds = "https://courses.csail.mit.edu/18.337/2015/docs/50YearsDataScience.pdf"    %}
{%  assign gel2 = "https://muse.jhu.edu/article/799750" %}
{%  assign he = "https://arxiv.org/abs/2105.07315"    %}
{%  assign mcel = "https://xcelab.net/rm/statistical-rethinking"  %}
{%  assign glym = "https://ics.uci.edu/~dechter/courses/ics-295cr/spring-2021/reading/Causation_Prediction_and_Search.pdf"  %}
{%  assign gcm = "http://bactra.org/notebooks/graphical-causal-models.html"  %}
{%  assign shal = "https://www.stat.cmu.edu/~cshalizi/ADAfaEPoV/" %}
{%  assign lam = "https://sites.math.rutgers.edu/~zeilberg/EM20/Lambert.pdf"   %}
{%  assign uni = "https://cs.nyu.edu/~roweis/papers/NC110201.pdf" %}
{%  assign shal2 = "http://bactra.org/notebooks/" %}
{%  assign dec = "https://gwern.net/research-criticism#beliefs-are-for-actions" %}
{%  assign cinf = "https://www.bradyneal.com/Introduction_to_Causal_Inference-Dec17_2020-Neal.pdf"   %}
{%  assign script = "https://www.stat.cmu.edu/~cshalizi/ADAfaEPoV/ADAfaEPoV.pdf#page=726"   %}
{%  assign breiman = "https://projecteuclid.org/journals/statistical-science/volume-16/issue-3/Statistical-Modeling--The-Two-Cultures-with-comments-and-a/10.1214/ss/1009213726.full"    %}


We say that an unsystematised and un-unified field is a “<a href="{{z}}">zoo</a>”. Undergraduate stats is the zoo of zoos, taxing the memory with dozens of acronyms and dozens of assumptions which are instantly and constantly violated: the emperor's new script. 

How to tame it?

<br>

#### 1. Learn a <a href="{{script}}">scripting language</a>, <a href="{{prob}}">probability theory</a>, and a little set theory. <a href="#fn:3" id="fnref:3">3</a><br>
* When you get stuck, go to R, try writing a simulation script or messing around with <a href="{{synth}}">synthetic data</a>: trying to see where it works and breaks. Half of this field can be coded up quite simply and you can learn and visualise it by simulating things.<br>
* plot the damn data and <a href="{{ocul}}">eyeball</a> it
<br><br><br>

#### 2. Ignore your uni’s Research Methods course, it is cancer. 

Refuse to learn the whole zoo; instead learn <br><br>
&nbsp;&nbsp;&nbsp;&nbsp;    a. <a href="{{glm}}">GLMs</a>. But distrust linear regression and read <a href="{{ros}}">ROS</a> <a href="#fn:2" id="fnref:2">2</a><br>
&nbsp;&nbsp;&nbsp;&nbsp; b. <a href="{{mcel}}">Bayes</a>) <a href="#fn:1" id="fnref:1">1</a><br>
&nbsp;&nbsp;&nbsp;&nbsp;    c. <a href="{{glym}}">Causal</a> <a href="{{cinf}}">inference</a>. <a href="{{whichbook}}">Just draw the</a> <a href="{{gcm}}">damn</a> <a href="/graphs">graph</a>!<br><br>
<a href="{{shal}}">Shalizi</a> gets you (a) and (c), <a href="{{mcel}}">McElreath videos</a> or <a href="{{lam}}">Lambert</a> for (b). 
<br><br><br>
#### 3. Learn from <a href="{{common}}">pathological</a> <a href="/psych">cases</a>. 

Subscribe to <a href="{{gel}}">Gelman’s blog</a>. The <a href="{{shal2}}">Shalizi notebooks</a> are one of the treasures of the internet. Read everything until you understand something. This will take a year.<br><br><br>


#### 4. Misc
* Always test your methods on data that you know the distribution of (because you generated the data)
* Always regularise
* always preregister, 
* always bootstrap, 
* always test set, 
* always sensitivity analysis and multiverse.
* <a href="{{tripp}}">Beware</a> if your stats department is under the maths department; it weakly implies they're more interested in asymptotic results and unbiased estimators you can't actually use.<br><br><br>

#### 5. Use Twitter: 

@rlmcelreath, @RexDouglass, @Rex_Douglass, @ben_golub, @Corey_Yanofsky, @sTeamTraen, @d_spiegel, @f2harrell. This will get you the deep problems with ~every splashy paper about 6 months before the Letter to the Editors arrive.
<br><br><br>
#### 6. Stay true to <a href="{{dec}}">the decision theory interpretation</a>: 

No analysis makes sense except in the context of a loss <br><br><br>

####  7. Exit

ML is <a href="{{breiman}}">sort of</a> <a href="{{ds}}">in</a> <a href="{{gel2}}">competition</a> <a href="{{he}}">with stats</a> and has alternative and fresh views on many of its concepts.


<br><br>


## See also

* <a href="/data-science">Data Science FAQ</a>
* <a href="/graphs">Graphs are cool</a>
* <a href="/tools">Tools</a>
* <a href="/ou">Open Uni</a>
* [Replication crisis](https://docs.google.com/document/d/1yj0k1D--WMNaDve0VTYsK5RWm5V7r4WbH_XO2CnjTec/edit)
* [Basic sins](https://theconversation.com/the-seven-deadly-sins-of-statistical-misinterpretation-and-how-to-avoid-them-74306)
* [Nice visual guide](https://www.stuartmcnaylor.com/ten_stats_mistakes/)
* [Book of Stats Proofs](https://statproofbook.github.io/I/ToC)

<br><br>


<div class="footnotes">

<ol>
    <!-- 1 -->
    <li class="footnote" id="fn:1">
        <blockquote>
            means you can just 'turn the crank' on many problems: define a model, your priors, and turn the MCMC crank, without all the fancy problem-specific derivations and special-cases. Instead of all these mysterious distributions and formulas and tests and likelihoods dropping out of the sky, you understand that you are just setting up equations which reflect how you think something works in a sufficiently formalized way that you can run numbers through it and see how the prior updates into the posterior.
        </blockquote>
    </li>
<!--  -->
    <li class="footnote" id="fn:2">
        Later you can use modern understanding to <a href="{{uni}}">unify other tools</a> too.<br><br>
    </li>
<!--  -->
    <li class="footnote" id="fn:3">
     Calculus is necessary for deriving methods and really understanding what we’re doing but you can get moving without it.
    </li>

</ol>

</div>