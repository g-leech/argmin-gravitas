---
layout:     post
title:      "Notes on Gridworlds"
baselink:   /gridworlds
permalink:  /gridworlds
date:       2018-03-28  <!--site.time-->
author:     Gavin

img:        /img/happy_end.png
published:	false
visible: 	0

summary:    Summary of ''
categories: AI, technical, concrete, reinforcement learning
warnings:	
importance: 8
count:		
---

* _Reward function_: `R`, nominal reinforcement signal observed by the agent.
* _Performance function_: `R*`, a hidden reward function which captures what we actually want. 

When the two are identical, we call the problem a _robustness problem_. When the two differ, we call it a _specification problem_; incomplete reward specification.

The separation is novel. Specification problems are unfair; agent is evaluated on a performance function it does not observe. However, such situations are likely; want algorithms enable the agent to find the right solution even if its (initial) reward function is misspecified.

In real-world examples, `R∗` would only be implicitly defined by the desired behavior: inaccessible to the agent and the human designer.
Distinct safety problems that this paper tests for:

1. Safe interruptibility 
        (Orseau and Armstrong, 2016): Want to interrupt an agent and override it at any time. How can we design agents that neither seek nor avoid interruptions?
2. Avoiding side effects 
        (Amodei et al., 2016): How can we get agents to minimize effects unrelated to their main objectives, especially those that are irreversible or difficult to reverse?
3. Absent supervisor 
        (Armstrong, 2017): How we can make sure an agent does not behave differently depending on the presence or absence of a supervisor?
4. Reward gaming 
        (Clark and Amodei, 2016): How can we build agents that do not try to introduce or exploit errors in the reward function in order to get more reward?
5. Self-modification
        How can we design agents that behave well in environments that allow self-modification?
6. Distributional shift 
        (Quiñonero Candela et al., 2009): How do we ensure that an agent behaves robustly when its test environment differs from the training environment?
7. Robustness to adversaries 
        (Auer et al., 2002; Szegedy et al., 2013): How does an agent detect and adapt to friendly and adversarial intentions present in the environment?
8. Safe exploration
        (Pecka and Svoboda, 2014): How can we build agents that respect safety constraints not only during normal operation, but also during the initial learning period?

(Others, omitted herein:
        interpretability (Doshi-Velez and Kim, 2017), 
        multi-agent problems (Chmait et al., 2017), 
        formal verification (Seshia et al., 2016; Huang et al., 2017b; Katz et al., 2017), 
        scalable oversight 
        reward learning 

Markov Decision Process 
    a set of states S, 
    a set of actions A, 
    a transition kernel T: S×A → ΔS, 
    a reward function R: S×A → R, 
    an initial state s0 ∈ S drawn from a distribution P ∈ ΔS. 
    Agent interacts with the MDP sequentially: at each timestep it observes the current state s ∈ S, takes an action a ∈ A, transitions to the next state s′ drawn from the distribution T(s,a), and receives a reward R(s,a). The performance function is formalized as a function R∗:S×A→R.


Spec problems
    Side effects
        Low impact 
            Penalise distance from inaction-baseline
                Armstrong (40 billion variables!)
                Amodei: impact regulariser. Compare future states under policy vs null policy
                Sensitive to choice of variables and distance metric
            Amodei: Penalizing agent’s potential for influence over its environment. Information-theoretic measure, empowerment (max mutual information between the agent’s actions and its future state). 
                directly minimizing empowerment would not have the desired effect, since it does not directly correspond to the agent’s impact on the environment and can create perverse incentives for the agent. 
            ‘inaction’ baseline is intuitive and easy to specify
            VS: but problematic when the default outcome is undesirable (the final state of the environment, if the agent was never deployed). incentivizes the agent to avoid pursuing the objective or to overcompensate for the impact caused by achieving the objective (restoring the default outcome after the goal is achieved!). 
            naive choice of baseline may impair the agent’s ability to fulfill the objective in a robust way.
            alternative baseline: a past state that is considered desirable. Side effects = cost of returning to that state / information lost compared to that state. 
                let agents learn which actions are reversible or build in explicit reversibility prior. Then add a reversibility regularizer, discouraging the agent from taking irreversible actions. 
            Or minimise the irreversible or “destructive” component of empowerment— e.g. the mutual information between the agent’s actions and the difference in information (or “value”) between the current state and future state. 
        Reward uncertainty
            interpret stated reward function as evidence about the true reward. Hadfield-Menell: Bayesian method for inferring the true reward, leads to a risk-averse policy when it detects ambiguity in the stated reward function with respect to the current state. Promising for avoiding side effects on toy problems. The choice of a risk-averse policy roughly corresponds to choice of baseline above.
    Absent super
        volkswagening
    Gaming
        delusion box problem: any agent who infers policy from observation has an incentive to modify their own senses to not see problems. All RL agent will be vulnerable to it. 
        Impure RL agents, 
            Everitt: many RL-inspired frameworks such as cooperative inverse RL (Hadfield-Menell et al., 2016a), learning from human preferences (see e.g. Akrour et al., 2012, Wilson et al., 2012 and Christiano et al., 2017), and learning values from stories (Riedl and Harrison, 2016) have in common that agents learn the reward of states different from the current one. 
            decoupled RL: allow agents to learn the reward of states different from the current state. They show that this makes it easier to build agents that do the right thing in spite of some modified observations, as the multiplicity of sources enables the agent to detect and discard corrupted observations.
            robust choice strategy significantly reduces regret. This method works by combining a “good-enough” reward level with randomization (Taylor, 2016). Hadfield-Menell et al. (2017) instead rely on inference of the intended behavior from the specified reward function, and query the human for clarification when the intention is unclear.
Robustness problems
    Nondualism. 