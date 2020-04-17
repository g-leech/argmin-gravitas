---
layout:     post
title:      "Which AI safety agenda?"
baselink:   /aicalc
permalink:  /aicalc
date:       2019-08-23
author:     Gavin

img:        /img/
published:	false
visible: 	false

summary:    Extremely rough calculator for ranking kinds of Safety work.
confidence:	60%
quality:    
warnings:	The list I'm working off didn't exist a week ago, and will change soon. The field isn't simple enough to take numbers literally. GIGO.
importance: 9
wordcount:		
---


{%	assign src = "https://www.lesswrong.com/posts/mJ5oNYnkYrd4sD5uE/clarifying-some-key-hypotheses-in-ai-alignment"		%}
{%	assign  = ""		%} 


_The following is based entirely off <a href="{{src}}">Shah and Cottier</a>'s excellent work. Go read it first._

<br>

> The idea is closely connected to the problem of artificial systems optimizing adversarially against humans.
> The idea must be explained sufficiently well that we believe it is plausible.


1. orthogonality
2. complexity of value
3. Goodhart's Curse
4. Will AI be deployed in places it can cause catastrophe?

1. Agentive AGI?

Will the first AGI be most effectively modelled like a unitary, unbounded, goal-directed agent?


2. Incentive for agentive AGI?

Would unitary goal-directed agents have a worthwhile advantage over other superintelligent systems?


3. Modularity over integration?

In general and holding resources constant, is a collection of modular AI systems with distinct interfaces more competent than a single integrated AI system?

    Related reading: Reframing Superintelligence Ch. 12, 13, AGI will drastically increase economies of scale
    Comment: an almost equivalent trade-off here is generality vs. specialisation. Modular systems would benefit from specialisation, but likely bear greater cost in principal-agent problems and sharing information (see this comment thread). One case that might be relevant to think about is human roles in the economy — although humans have a general learning capacity, they have tended towards specialising their competencies as part of the economy, with almost no one being truly self-sufficient. However, this may be explained merely by limited brain size. The recent success of end-to-end learning systems has been argued in favour of integration, as has the evolutionary precedent of humans (since human minds appear to be more integrated than modular).


4. Does current AI R&D extrapolate to AI services?

AI systems so far generally lack some key qualities that are traditionally supposed of AGI, namely: pursuing cross-domain long-term goals, having broad capabilities, and being persistent and unitary. Does this lacking extrapolate, with increasing automation of AI R&D and the rise of a broad collection of superintelligent services?


5. Incidental agentive AGI?

Will systems built like unitary goal-directed agents develop incidentally from something humans or other AI systems build?


6. Convergent rationality?

Given sufficient capacity, does an AI system converge on rational agency and consequentialism to achieve its objective?


7. Mesa-optimisation?

Will there be optimisation processes that, in turn, develop considerably powerful optimisers to achieve their objective? A historical example is natural selection optimising for reproductive fitness to make humans. Humans may have good reproductive fitness, but optimise for other things such as pleasure even when this diverges from fitness.


8. Recursive self improvement?

Is an AI system that improves through its own AI R&D and self-modification capabilities more likely than distributed AI R&D automation? Recursive improvement would give some form of explosive growth, and so could result in unprecedented gains in intelligence.


9. Discontinuity to AGI?

Will there be discontinuous, explosive growth in AI capabilities to reach the first agentive AGI? A discontinuity reduces the opportunity to correct course. Before AGI it seems most likely to result from a qualitative change in learning curve, due to an algorithmic insight, architectural change or scale-up in resource utilisation.


10. Discontinuity from AGI?

Will there be discontinuous, explosive growth in AI capabilities after agentive AGI? A discontinuity reduces the opportunity to correct course. After AGI it seems most likely to result from a recursive improvement capability.


11. ML scales to AGI?

Do contemporary machine learning techniques scale to general human level (and beyond)? The state-of-the-art experimental research aiming towards AGI is characterised by a set of theoretical assumptions, such as reinforcement learning and probabilistic inference. Does this paradigm readily scale to general human-level capabilities without fundamental changes in the assumptions or methods?


12. Deep insights needed?

Do we need a much deeper understanding of intelligence to build an aligned AI?


13. Broad basin for corrigibility?

Do corrigible AI systems have a broad basin of attraction to intent alignment? Corrigible AI tries to help an overseer. It acts to improve its model of the overseer's preferences, and is incentivised to make sure any subsystems it creates are aligned — perhaps even more so than itself. In this way, perturbations or errors in alignment tend to be corrected, and it takes a large perturbation to move out of this "basin" of corrigibility.


14. Inconspicuous failure?

	Will a concrete, catastrophic AI failure be overwhelmingly hard to recognise or anticipate? For certain kinds of advanced AI systems (namely the goal-directed type), it seems that short of near proof-level assurances, all safeguards are thwarted by the nearest unblocked strategy. Such AI may also be incentivised for deception and manipulation towards a treacherous turn. Or, in a machine learning framing, it would be very difficult to make such AI robust to distributional shift.
	    Related reading: Importance of new mathematical foundations to avoid inconspicuous failure 

	Agentive -> 
	-> Proof-level assurance
    

15. Creeping failure?

Would gradual gains in the influence of AI allow small problems to accumulate to catastrophe? The gradual aspect affords opportunity to recognise failures and think about solutions. Yet for any given incremental change in the use of AI, the economic incentives could outweigh the problems, such that we become more entangled in, and reliant on, a complex system that can collapse suddenly or drift from our values.



{%  include comments.html %}