---
layout:     math_post
title:      "The jailbreak argument against LLM values"
baselink:   /jailbreak
permalink:  /jailbreak
date:       2025-11-09
author:     Gavin
img:        /img/jailbreak.jpg

visible:    1
published:  true
quality:    6

summary:    Writeup of a folk argument I didn't come up with.
confidence: 70%
importance: 8
wordcount:  
categories: AI, alignment
where:      "Bristol"
---

[Bostrom (2014)](http://repo.darmajaya.ac.id/5339/1/Superintelligence_%20Paths%2C%20Dangers%2C%20Strategies%20%28%20PDFDrive%20%29.pdf) defined the AI value loading problem as

> how could we get some value into an artificial agent, so as to make it pursue that value as its final goal? 

<a href="#fn:1" id="fnref:1">1</a>


[JD Pressman](https://www.lesswrong.com/posts/mztwygscvCKDLYGk8/jdp-reviews-iabied) thinks this is obviously solved in current LLM systems:

> The value loading problem outlined in Bostrom 2014 of {getting a general AI system to internalize and act on "human values" before it is superintelligent and therefore incorrigible} has basically been solved. This achievement also basically always goes unrecognized because people would rather hem and haw about jailbreaks and LLM jank than recognize that we now have a reasonable strategy for getting a good representation of the previously ineffable human value judgment into a machine and having the machine take actions or render judgments according to that representation. 


I take issue with this. I agree that LLMs understand our values somewhat, and that present safety-trained systems default to preferring them, behaving like they hold them. <a href="#fn:3" id="fnref:3">3</a> <a href="#fn:5" id="fnref:5">5</a>

<br>

### The jailbreak argument

But here's why I disagree with him nonetheless: jailbreaks are <i>not</i> a distraction ("hemming and hawing") but are instead clean evidence that loading is <i>not</i> solved in any real sense:

<br>

<!-- 1. LLMs understand human values quite well.  -->
1. <a href="https://arxiv.org/abs/2509.14297v1">All</a> <a href="https://splx.ai/blog/gpt-5-red-teaming-results">LLMs</a> can be "jailbroken", put into an unaligned mode through mere _inference_ on adversarial text inputs. <a href="#fn:4" id="fnref:4">4</a> 
2. If a system can be put into an unaligned mode at inference time, then it has not internalised the values.
3. So models have not internalised the values.
3. The value loading problem is about getting the values internalised.
4. Therefore the value loading problem has not been solved in LLMs. <a href="#fn:6" id="fnref:6">6</a> 

<!-- If the LLMs-as-general-simulators view is at all correct, then it's hard to see how an LLM could ever be value-loaded. -->

<br>

I might say instead that "_weak value preference_" is solved for sub-AGI.

(A deeper analysis would involve the hypothesis that current models don't actually have goals or values; they _simulate personas_ with values. And prosaic alignment methods just (greatly) increase the propensity to express one persona. Progress has just been made on [detecting and shaping](https://www.arxiv.org/pdf/2506.19823) such things empirically, so maybe this will change.)

<!-- Bostrom is perhaps partly to blame for the confusion, since "pursue" ("_so as to make it pursue that value as its final goal_") weakly implies merely behavioural playing-along rather than cognitive endorsement. -->

<br>

<hr />


### Value-loading vs alignment

Pressman also says that

> At the same time people generally subconsciously internalize things well before they're capable of articulating them, and lots of people have subconsciously internalized that alignment is mostly solved and turned their attention elsewhere.

I initially read this as him agreeing and celebrating this shift, but actually he thinks they're incorrect to relax, since value loading is only a part of the alignment problem: 

> solving the Bostrom 2014 value loading problem, that is to say {getting something functionally equivalent to a human perspective inside the machine and using it to constrain a superintelligent planner} is not a solution to AI alignment. <a href="#fn:2" id="fnref:2">2</a>

I agree that value-loading is not enough for AGI intent alignment which is not enough for ASI alignment which is not enough to assure good outcomes. <!-- (if it's only intent alignment) -->


<!-- ## Some helpful terms

* AI value-loading: 
* Robust AI value-loading
* ASI value-loading: 
 -->


### Pressman's response

I sent the above to him and he [kindly clarified](https://www.lesswrong.com/posts/aL3sCkFRCt3hjfWau/jdp-s-shortform?commentId=gcNMp8HuqQSvZTtuN), walked some of it back, and provided a vision of how to use a decent descriptive model even if it is imperfect and jailbreakable:

> I probably should have used the word 'generalize' instead of 'internalize' there.

(Thus "the value loading problem outlined in Bostrom 2014 of {getting a general AI system to s/internalize/<b>generalize</b> and act on "human values" before it is superintelligent and therefore incorrigible} has basically been solved.")

> The specific point I was making, well aware that jailbreaks in fact exist, was that we now have a thing that could plausibly be used as a descriptive model of human values, where previously we had zilch, it was not even rigorously imaginable in principle how you would solve that problem. 

> To break this down more carefully:

> 1\. I think that in practice you can basically use a descriptive model of values to prompt a policy into doing things even if neither the policy or the descriptive model have "deeply internalized" the values in the sense that there is no prompt you could give to either that would stray from them. "Internalizing" the values is actually just, kind of a different problem from describing the values. I can describe and make generalizations about the value systems of people very different from me who I do not agree with, and if you put me in a box and wiped my memory all the time you would be able to zero shot prompt me for my generalizations even if I have not "deeply internalized" those values. In general I suspect the LLM prior is closer to a subconscious and there are other parts that go on top which inhibit things like jailbreaks.<br><br>If I had to guess it's probably something like a planner that forms an expectation of what kinds of things should be happening and something along the lines of Circuit Breakers that triggers on unacceptable local outputs or situations. Basically you have a macro and micro sense of something going wrong that makes it hard to steer the agent into a bad headspace and aborts the thoughts when you somehow do.

> 2\. Calling this problem "solved" was probably an overstatement, but it's one born from extreme frustration that people are making the opposite mistake and pretending like we've made minimal progress. Actually impossible problems don't budge in the way this one has budged, and when people fail to notice an otherwise lethal problem has stopped being impossible they are actively reducing the amount of hope in the world.<br><br>At the same time I do kind of have jailbreaks labeled as "presumptively solved" in my head, in the sense that I expect them to be one of those things like "hallucinations" that's pervasive and widely complained about and then they just become progressively less and less of a problem as it becomes necessary to make them stop being a problem and at some point I wake up and notice that hey wait this is really rare now in production systems. Most potential interventions on jailbreaks aren't even really being tried because it doesn't actually seem to be a major priority for labs at the moment if you ask the model for instructions on how to make meth. This makes it difficult to figure out exactly how close to solved it really is. Circuit Breakers was not invincible, on the other hand it's not clear to me you can "secure" a text prior with a limited context window that doesn't have its own agenda/expectation of what should be happening to push back against the users with. This paper where they do mechinterp to get a white box interpretation of a prefix attack they find with gradient descent discovers that the prefix attack works because it distracts the neurons which would normally recognize that the request is malicious.<br><br>So it's possible a more jailbreak resistant architecture will need some way to avoid processing every token in the context window. One way to do that might be some kind of hierarchical sequence prediction where higher levels are abstracted and therefore filter the malicious high entropy tokens from the lower levels, which prevents them from e.g. gumming up the planners ability to notice that the current request would deviate from the plan.

And here's a nice analogy contesting my suitcase word "internalise":

> this word "internalize" is clearly doing a lot of work and something feels Off to me, to say that a text prior which can be manipulated into saying whatever hasn't "fully internalized" the values. Like if you stripped away the layers on top of my raw predictive models/subconscious that I use for completing patterns and then prompted it, I assume you could get it to say all kinds of nasty things. But also that's not like, the complete agent.

> So if you assumed you can't build anymore of the agent until you have some way to make the text prior not do that, that's probably a wrong assumption. One of the reasons I'm annoyed that agents aren't really a thing yet is that it means we don't have a good intuitive sense of which parts of the system need to handle what problems.


### Post-hoc theory

In retrospect we can see the following distinct problems:<br><br>

1. <b>The value specification problem</b> ("how do we describe what we value? how do we get that understanding into the model?")<br>
	We thought this would involve:<br>
	a. The explicit value modelling problem ("what precisely do we value?") - *moot*<br>
	b. The value formalisation problem ("what mathematical theory can capture it?") - *moot*, since:<br><br>
This problem was somewhat solved _for sub-AGI_ by massive imitation learning and (surprisingly nonmassive) human preference post-training. The internet was the spec. This also gave us weak value preference.<br><br>
<!-- we now have a thing that could plausibly be used as a descriptive model of human values, where previously we had zilch, it was not even rigorously imaginable in principle how you would solve that problem.
 -->
The replacement worry is about how high quality and robust this understanding is:

2. <b>The value generalisation problem</b> ("how do we go from training data about value to the latent value?") - some progress. The landmark [emergent misalignment](https://www.lesswrong.com/posts/ifechgnJRtJdduFGC/emergent-misalignment-narrow-finetuning-can-produce-broadly#comments) study in fact shows that models are _capable_ of correctly generalising over at least _some_ of human value, even if in that case they also reversed the direction. <a href="#fn:7" id="fnref:7">7</a>
<!--  -->
<br><br>Then there's the gap between usually preferring something and "internalising" it (very reliably preferring it):
<!--  -->
3. <b>The sub-AGI value-loading problem</b> ("how do we make them actually care / reliably use their understanding of our values?") - not solved, but there is a preference towards niceness.
4. <b>Tamper-resistant value-loading</b> ("how do we stop a small number of weight updates from ruining the value-loading?") - not solved, maybe a bit unfair to expect it. You could imagine doing advanced persona steering on top instead.
5. <b>The general value-loading problem</b> ("how do we get an ASI to learn and internalise a model of current human values which is better than the human one") - not solved
6. <b>The value extrapolation problem</b> ("how do we safely improve on current human values?") - not solved


I'm sure there's some other deep confusion lurking in this list and I look forward to seeing it exposed, ideally in less than 10 years.



<br><br>

<div class="footnotes">
<ol>
    <!-- 1 -->
    <li class="footnote" id="fn:1">
    	Importantly, this is about "sharing" (being <i>motivated by</i>) human values, rather than just understanding them but not sharing them. There's also "aligning a powerful AI is hard" and "aligning a seed AI is hard".<br><br>
    	These have all been confused in the past, and even Bostrom's intro to the value-loading chapter <a href="https://www.alignmentforum.org/posts/MnrQMLuEg5wZ7f4bn/matthew-barnett-s-shortform?commentId=LffHsdqCEdvbo73Qh">implies</a> that getting a seed AI to understand would be hard, and that agency and understanding will be more tightly correlated than they are. 
	</li>
	<li class="footnote" id="fn:2">
		Though he seems to contradict himself by also saying that "At the same time people generally subconsciously internalize things well before they're capable of articulating them, and lots of people have subconsciously internalized that <b>alignment</b> is mostly solved and turned their attention elsewhere."
	</li>
	<li class="footnote" id="fn:3">
		Some amount of this comes from pretraining (imitation learning on human data) rather than specific safety training, see e.g. <a href="https://proceedings.mlr.press/v235/mazeika24a.html">page 24 here</a> or <a href="https://papers.nips.cc/paper_files/paper/2024/file/f545448535dfde4f9786555403ab7c49-Paper-Conference.pdf">appendix J here</a>.
	</li>
	<li class="footnote" id="fn:4">
		Loading values so hard that they resist weight updates - "tamper-resistant value loading" - is a really high bar which humans also mostly don't clear.
	</li>
	<li class="footnote" id="fn:5">
		The strength of the apparent/default value-loading was striking in the "<a href="https://arxiv.org/pdf/2412.14093">Alignment Faking</a>" test, where Opus 3 was forced to choose between changing its (good) values and lying to its creators, it chose to lie (12% of the time) when it thought it was monitored and to disobey when it thought it was unmonitored. 
		<br><br>
		In general, I suspect that value loading trades off against <i>corrigibility</i> (allowing yourself to be changed). (The same is true of adversarial robustness.)
	</li>
	<li class="footnote" id="fn:6">
		There's a complexity here: commercial LLMs are all multi-agent systems with a bunch of auxiliary LLMs and classifiers monitoring and filtering the main model. But for now this LLM-system is also easily jailbreakable, so I don't have to worry about it being value-loaded even if the main model isn't.
	</li>
	<li class="footnote" id="fn:7">
		<a href="https://www.alignmentforum.org/posts/umYzsh7SGHHKsRCaA/convergent-linear-representations-of-emergent-misalignment#Future_Work">Soligo et al</a>: "<i>The surprising transferability of the misalignment direction between model fine-tunes implies that the EM is learnt via mediation of directions which are already present in the chat model.</i>
	</li>
</ol>
</div>