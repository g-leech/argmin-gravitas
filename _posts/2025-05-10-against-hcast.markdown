---
layout:     post
title:      "HCAST: can they do hard stuff yet?"
baselink:   /hcast
permalink:  /hcast
date:       2025-05-12
author:     Gavin
img:        /img/hcast.jpg

visible:    1
published:  false
quality:    6

summary:    
confidence: 70%
importance: 7
wordcount:  
categories: 
where:      "Kensington"
---

_(This was written before they [added domains](https://metr.org/blog/2025-07-14-how-does-time-horizon-vary-across-domains/) besides software engineering, which, given driving, is a_ lil _more convincing.)_

<br>

OK, AI is improving. But how quick? And: improving on real tasks, or just simplistic ones? At new ones, or only recombinations of previously-seen ones? We'd particularly like to know how far we are from the big one: "AGI" (whatever that means).

We lack a serious measure of progress towards AGI. The previous self-declared holder was ARC-AGI (2019), but this was never it. (Boring, narrow, static, public, 3-minuters, unrealistic human baseline, procedural, isolated).  Well, the good news is that this new [HCAST](https://arxiv.org/abs/2503.17354) benchmark is much better than that.

The idea is: clearly, stronger AIs can do more difficult tasks than weaker AIs can. HCAST assumes that we can proxy this latent task difficulty with the (average) time it takes human experts to solve the task. A 1 hour task will generally be harder than a 1 minute task. 

`difficulty(task) = time it takes a human expert to solve `


So!

1. pay a bunch of skilled humans to complete a task, take their average solve time 
2. time AIs doing the same tasks 
3. pick a threshold AI success rate _t_ and take the longest human time of the tasks the AI can do _t_ % of the time. 

`Human-time = the time it would take a human to complete the task`

`The time horizon metric` is the _maximum human-time for an average AI success rate = 50%_, given arbitrary amounts of AI-time 

This is a _universal_ measure; we can (expensively) place any task on a “how long does it take?” scale. So! measure real tasks and use skilled humans as a baseline. Pay them very well. Use novel and complex tasks to get the systems out of distribution. Use professional and actively economically relevant tasks to make the metric meaningful.

### “Persnickety” (accurate) version 

“The time horizon is doubling every 2-12 months _on automatically-scoreable, relatively clean + greenfield software tasks_ from a few distributions”


### The estimate

HCAST estimated doubling time for time horizons: every 7 months ± 5 months

* Supposed current horizon 	= 1-2 hours
* Feb 2027 prediction 		= 16 hours
* Apr 2028 prediction 		= 5 days
* 2030 						= 3 months


Extrapolation is a great prior; it works great until it doesn't.

(Tangent: How did METR get 2022 models to test? Maybe just METR magic backdoor access? GPT-3 is dead. The available GPT-3.5-T is a heavily changed checkpoint.)


## Bottom line on HCAST

* A big step forward
* Unwarranted extrapolation to other domains. (initially) code only!
* Small n
* Still not beating worries about leakage 
* I mostly believe their rank ordering of models while ignoring the y-axis labels
* Exponential slope is plausible on past data but I am sceptical of its predictive power
* Divide reported times by say 6x 
* But yes time horizon is improving 




## Implications for AI timelines

The enormously splashy forecasting scenario [AI 2027](https://ai-2027.com/) came out around the same time as HCAST. This naturally led [some](https://www.lesswrong.com/posts/BrHv7wc6hiJEgJzHW/reactions-to-metr-task-length-paper-are-insane?commentId=woqj9DtretYC9gBjr) to assume that AI 2027 relies on HCAST. [But](https://www.lesswrong.com/posts/BrHv7wc6hiJEgJzHW/reactions-to-metr-task-length-paper-are-insane?commentId=woqj9DtretYC9gBjr): 

> the content of AI 2027 was all but finalized before the METR report came out.

But also, in [their coding model](https://www.lesswrong.com/posts/ggqSg7bSLChanfunf/forecasting-time-to-automated-superhuman-coders-ai-2027):

> We heavily draw from METR’s recent report which catalogues a trend of increasing time horizon.


METR's central estimate is a 16-hour-long coder in 2027, and a 3-month-long coder in 2030. There's huge questions about what time horizon would correspond to something transformative, but overall it's evidence against 2027 AGI and weak evidence for <10 year timelines.

(The authors of AI 2027 have since [lengthened](https://80000hours.org/podcast/episodes/daniel-kokotajlo-ai-2027-updates-china-robot-economy/) their timelines to 2029 for various other reasons.)


## See also 

* [ADeLe](https://kinds-of-intelligence-cfi.github.io/ADELE/) (2025). Actual cognitive science for more-fundamental mental capacities.
* Scholl, "[The Length of Horizons](https://www.lesswrong.com/posts/PzLSuaT6WGLQGJJJD/the-length-of-horizons)"
* [Insane](https://www.lesswrong.com/posts/BrHv7wc6hiJEgJzHW/reactions-to-metr-task-length-paper-are-insane)
* [Complaint](https://www.lesswrong.com/posts/5CGNxadG3JRbGfGfg/notes-on-the-long-tasks-metr-paper-from-a-hcast-task)
* https://secondthoughts.ai/p/measuring-ai-progress 
* https://www.lesswrong.com/posts/8aPyKyRrMAQatFSnG 

