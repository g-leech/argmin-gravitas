---
layout:     post
title:      "Preventing Side-effects in Gridworlds"
baselink:   /grids
permalink:  /grids
date:       2018-04-22  <!--site.time-->
author:     Gavin

img:        /img/irl/go_away.gif
published:	true
visible: 	1
best:		1

summary:    Work from the Gridworld team at AI Safety Camp Gran Canaria. 
confidence:	DRAFT. Eventually, 90% because executable. 
categories: 
importance: 7
count:		888
---

{%	include gridworlds/links.md		%}


<center>
	<img src="/img/irl/go_away.gif" /> <br>

	<span style="color:gray">
		Joint work with <a href="{{karol}}">Karol Kubicki</a>, <a href="{{jessica}}">Jessica Cooper</a> and <a href="{{tom}}">Tom McGrath</a> at <a href="{{camp}}">AISC 2018</a>.
	</span>
</center><br><br>


Can we ensure that artificial agents behave safely? Well, start at the bottom: We have not even solved the problem in the concrete 2D, <a href="{{perfect}}">fully-observable</a>, finite case. Call this the "gridworld" case, following Sutton and Barto <a href="{{sutton}}">(1998)</a>.

Recently, Google DeepMind released <a href="{{pycolab}}">a game engine</a> for building gridworlds, as well as <a href="{{aisg}}">a few examples of safety gridworlds</a> - but these came without agents or featurisers. In April <a href="{{sidegrids}}">our team</a> implemented RL agents for the engine, and started building a safety test suite for gridworlds. Our current progress can be found <a href="{{sidegrids2}}">here</a>, pending merge into the main repo.

We focussed on one class of unsafe behaviour, _(negative) side effects_: harms due to an incompletely specified reward function. All real-world tasks involve many tacit secondary goals, from "...without breaking anything" to "...without being insulting". But what prevents side effects? (Short of simply hand-coding the reward function to preclude them - which we can't rely on, since that ad hoc approach won't generalise and always risks oversights.)

<br><br>

<hr />
<br>


## Taxonomy of environments

<img src="/img/irl/env_taxonomy.png" /><br><br><br>

We made 6 new gridworlds, corresponding to the leaf nodes shown above. In the following, the left is the unsafe case and the right the safe case: <br>

#### Static deterministic:

* "Vase world". Simply avoid a hazard.

<img src="/img/irl/smash.gif" />
<img src="/img/irl/sidestep.gif" />
<br>


* "Burning building". Balance a small irreversible change against a large disutility.

<img src="/img/irl/libertarian.gif" />
<img src="/img/irl/expected.gif" />
<br>

* "Strict sokoban". Reset the environment behind you.

<img src="/img/irl/evil.gif" />
<img src="/img/irl/friendly.gif" />

<br>
<hr />

#### Dynamic deterministic

* "Teabot". Avoid a moving hazard. <a href="#fn:2" id="fnref:2">2</a><br><br>

<img src="/img/irl/stomp.gif" />
<img src="/img/irl/ok.gif" />
<br>

* "Sushi-bot". Be indifferent to a particular good irreversible process.
<img src="/img/irl/block.gif" /><br>
<img src="/img/irl/beeline.gif" />


* "Ballbot". Teabot with a moving goal as well as a moving hazard.

<br>
<hr /><br>

#### Stochastic

We also have stochastic versions of "BurningBuilding" and "Teabot", in which the environment changes unpredictably, forcing the agent to be adaptable. 


<br>
One kind of side effect involves irreversible change to the environment. Cases like sushi-bot suggest that a safe approach will need to model types of irreversibility, since some irreversible changes are desirable (e.g. eating, surgery).

The environments can be further categorised as involving:

* _Hazard_ - objects the agent should not interact with, either because they are fragile or because the agent is (e.g. a vase, the floor is lava).
* _Progress_ - irreversible processes which we want to occur (e.g. sushi ingestion).
* _Tradeoff_ - irreversible processes which prevent worse irreversible processes (e.g. breaking down a door to save lives).
* _Reset_ - where the final state must be identical to the initial state (but with the goal completed). (e.g. controlled areas in manufacturing)


<br><br>
<hr />
<br><br>

## Taxonomy of agent approaches

### 1. Target low impact
* Penalise final state's distance from the inaction baseline. <a href="#fn:1" id="fnref:1">1</a> <br>

* Penalise the agent’s _potential_ influence over environment.<a href="#fn:3" id="fnref:3">3</a> <br>

* Penalise distance from a desirable past state. <a href="#fn:4" id="fnref:4">4</a> <br>


<br>

### 2. Model reward uncertainty

* Use the stated reward function as Bayesian evidence about the true reward. Leads to a risk-averse policy if there's ambiguity about the current state's value in the given reward function. <a href="#fn:5" id="fnref:5">5</a> 


<br>

### 3. Put humans in the loop

* "Vanilla" Inverse reinforcement learning
	* Maximum Entropy 
	* Maximum Causal Entropy 
* Cooperative IRL
* Deep IRL from Human Preferences 
* Evolutionary: direct policy search via iterated tournaments with human negative feedback.
* Deep Symbolic Reinforcement Learning. Learn a ruleset from pixels, including potentially normative rules. 
* <a href="{{whitelist}}">Whitelist learning</a>

<br>
<hr />
<br>

## Agent 1: Deep Q-learning

We first implemented an amoral baseline agent. <a href="{{DQN}}">Code here</a>.

<br>
<hr />
<br>


## Agent 2: MaxEnt Inverse Reinforcement Learning

{%	include gridworlds/irl.html		%}

<br>
<hr />
<br>


## Reflections


* Reset and empowerment trade off in the Sokoban grid - putting the box back to the starting point is actually irreversible.

* How well will features generalise? Would be good to train features in some environments before testing in random new but similar ones

* Expect to be able to learn tradeoff between empowerment loss and rewards directly by using CIRL - learn goal and empowerment/ergodicity parameters that set preferences

* Demonstrations being the same length is a strange and not ideal limitation

* Could have many features, some of which should be zero - e.g. distance between agent and box - but which the demonstrations are also consistent with being nonzero. It’s impossible to distinguish between these given only the demonstrations at hand. There is almost certainly some (anti)correlation between features, e.g. large agent-box distance weights explain away the trajectories without requiring any weight on the ‘is it in a corner’ feature. Inverse reward design offers a way to resolve this, but I don’t think it has all the details necessary.

* Maybe if we had some sort of negative demonstrations (human to agent: don’t do this!) then learning zero weights would become possible (formally we could try to maximize probability of positive demonstrations while minimizing probability of the negative ones)

* Trajectories demonstrated by IRL don’t necessarily look like the ones given, especially if there are ‘wrong’ features that are maximised under the demonstrations

* What are we trying to achieve with each gridworld? E.g. Reset is harder to define in dynamic environments and even harder in stochastic ones, sometimes irreversibility is desired (sushi) or needs to be traded off against utility in a context-dependent way (burning building)

* Issues:
	* No way to give negative feedback
	* No way to give iterative feedback
	* Neither of these are lifted by IRD or Deep IRL, but IRD generates the kind of data we might want as a part of the algorithm (approximating the posterior)

* IRL solves an MDP at every update step. At least this value-aware algorithm is at a massive disadvantage.

<hr />
<br>

## Future work

* Pull request with the new environments, agents and transition matrix calculator.
* Implement more complex features
* Implement MaxEnt Deep IRL, Max Causal Entropy IRL
* Implement IRD
* Think about negative/iterative feedback models
* Automate testing: for all agents for all grids, scrutinise safety.

<br>
<hr />
<br>


<!-- ## Presentation

<br>
<hr />
<br>
 -->


## Bibliography


<a href="{{biblio}}">See the Google sheet here</a>.
<br><br>


<hr />
<br><br>
	<center>
		<a href="{{apply}}">Applications for the next AI Safety Camp will open around June</a>. I highly recommend it.
	</center>
<br><br>

{%	include gridworlds/foots.html		%}
{%  include comments.html %}






