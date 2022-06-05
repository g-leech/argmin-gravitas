---
layout:     post
title:      "AI alignment & academia"
baselink:   /acais
permalink:  /acais
date:       2020-07-29
author:     Gavin

img:        /img/nonsense.jpg
published:  true
visible:    1

summary:    Estimating safety work by academics in adjacent areas.
quality:    7
categories: effective-altruism, AI, academia
confidence: High that there is a notable contribution, low in the particular estimates. Lots of Fermi estimates.
importance: 9
wordcount:  3300
cross:		https://forum.effectivealtruism.org/posts/8ErtxW7FRPGMtDqJy/the-academic-contribution-to-ai-safety-seems-large
argument:   acais/argument.html
---

{%	include acais/links.md		%}

A big reason for the EA focus on AI safety is its neglectedness:

> ...less than $50 million per year is devoted to the field of AI safety or work specifically targeting global catastrophic biorisks.  

<a href="{{k08}}">80,000 Hours</a> (2019)

> ...we estimate fewer than 100 people in the world are working on how to make AI safe. 

<a href="{{ai80k}}">80,000 Hours</a> (2017)

> Grand total: $9.09m… [Footnote: this] doesn’t include anyone generally working on verification/control, auditing, transparency, etc. for other reasons.

<a href="{{impacts}}">Seb Farquhar</a> (2018)

> ...what we are doing is less than a pittance. You go to some random city... Along the highway you see all these huge buildings for companies... Maybe they are designing a new publicity campaign for a razor blade. You drive past hundreds of these... Any one of those has more resources than the total that humanity is spending on [AI safety].  

<a href="{{bos}}">Nick Bostrom</a> (2016) 

<br>

Numbers like these helped convince me that AI safety is the best thing to work on. I now think that these are underestimates, because of non-EA lines of research which weren't counted.

Use "EA safety" for the <a href="{{aiwatch}}">whole umbrella</a> of work done at organisations like FHI, MIRI, DeepMind and OpenAI’s safety teams, and by independent researchers. A lot of this - maybe a third - is conducted at universities; to avoid double counting I count it as EA and not academia.

The argument:
1. EA safety is small, even relative to a single academic subfield.
2. There is overlap between capabilities and short-term safety work.
3. There is overlap between short-term safety work and long-term safety work.
4. So AI safety is less neglected than the opening quotes imply.
5. There’s a good chance that academia will do more safety over time, eventually dwarfing the contribution of EA.

## What’s ‘safety’? 

EA safety is best read as about “AGI alignment”: work on assuring that the actions of an extremely advanced system are sufficiently close to human-friendly goals. 

EA focusses on AGI because weaker AI systems aren’t thought to be directly tied to existential risk. However, Critch and Krueger note that “prepotent” - unstoppably advanced, but not necessarily human-level - AI <a href="{{critch}}">could still pose x-risks</a>. The potential for this latter type is key to the argument that short-term work is relevant to us, since the scaling curves for some systems seem to be <a href="{{gwern}}">holding up</a>, and so might reach prepotence.

“ML safety” could mean making existing systems safe, or using existing systems as a proxy for aligning an AGI. The latter is sometimes called “<a href="{{mid}}">mid-term safety</a>”.

In the following “AI safety” means anything which helps us solve the AGI control problem.

## De facto AI safety work

The line between safety work and capabilities work is sometimes blurred. A classic example is ‘robustness’: it is both a safety problem and a capabilities problem if your system can be reliably broken by noise. Transparency (increasing direct human access to the goals and properties of learned systems) is the most obvious case of work relevant to capabilities, short-term safety, and AGI alignment. As well as being a huge academic fad, it's a core mechanism in 6 out of the 11 live AGI alignment proposals recently summarised by <a href="{{hub}}">Evan Hubinger</a>.

More controversial is whether there’s significant overlap between short-term safety and AGI alignment. All we need for now is:
The mid-term safety hypothesis (weak form): at least some work on current systems will transfer to AGI alignment.
Some researchers who seem to put a lot of stock in this view: <a href="{{shah}}">Shah</a>, <a href="{{paul}}">Christiano</a>, <a href="{{vika}}">Krakovna</a>, <a href="{{olss}}">Olsson</a>, <a href="{{olah}}">Olah</a>, <a href="{{stein}}">Steinhardt</a>, <a href="{{amodei}}">Amodei</a>, <a href="{{krueger}}">Krueger</a>. (Note that I haven’t polled them; this is guessed from public statements and revealed preferences.) 

Here are some alignment-relevant research areas dominated by non-EAs. I won’t explain these: I use the incredibly detailed taxonomy (and 30 literature reviews) of <a href="{{critch}}">Critch and Krueger</a> (2020). Look there, <a href="{{stein}}">and</a> <a href="{{fli}}">at</a> <a href="{{clr}}">related</a> <a href="{{chai}}">agendas</a> <a href="{{miri}}">for</a> explanations and bibliographies.

* <a href="{{t}}">Transparency</a>
* <a href="{{r}}">Robustness</a>
* <a href="{{i}}">Interactive AI</a>
* <a href="{{c}}">Calibration</a>
* <a href="{{v}}">Formal verification</a>
* <a href="{{p}}">Preference learning</a>
* <a href="{{m}}">Modelling human cognition</a>
* <a href="{{s}}">Safe handovers (AKA corrigibility)</a>
* <a href="{{a}}">Assured Autonomy</a>
* <a href="{{o}}">Open source game theory</a>
* <a href="{{coord}}">Multi-agent coordination</a>
* <a href="{{emerg}}">Emergent communication</a>
* <a href="{{rl}}">Safe RL</a>
* <a href="{{fair}}">(Parts of) algorithmic fairness</a>


These are narrowly drawn from ML, robotics, and game theory: this is just a sample of relevant work! Work in <a href="{{ask}}">social science</a>, <a href="{{kaj}}">psychology</a>, <a href="{{wei}}">moral uncertainty</a>, or decision theory could be just as relevant as the above direct technical work; Richard Ngo lists many questions for non-AI people <a href="{{ngo}}">here</a>. 

Work in these fields could help directly, if the eventual AGI paradigm is not too dissimilar from the current one (that is, if the weak mid-term hypothesis holds). But there are also indirect benefits: if they help us to use AIs to align AGI; if they help to build the field; if they help convince people that there really is an AGI control problem (for instance, Victoria Krakovna’s <a href="{{spec}}">specification gaming list</a> has been helpful to me in interacting with sceptical specialists). These imply another view under which much academic work has alignment value:

> _The mid-term safety hypothesis (very weak form)_: at least some work on current systems will probably help with AGI alignment in some way, not limited to direct technical transfer.

A natural objection is that most of the above areas don’t address the AGI case: they’re not even trying to solve our problem. I discuss this and other discounts below.

<br>

## Current levels of safety-related work

### How large is EA Safety?

Some overlapping lists:

* \# people with posts on the Alignment Forum since late 2018: <a href="{{af}}">94</a>. To my knowledge, 37 of these are full-time.
* 80k AI Safety Google Group: 400, almost entirely junior people.
* <a href="{{lark}}">Larks’ great 2019 roundup</a> contained \~110 AI researchers (who published that year), most of whom could be described as EA or adjacent.
* <a href="{{aiwatch2}}">Issa Rice’s AI Watch</a>: “778” (raw count, but there’s lots of false positives for general x-risk people and inactive people. Last big update 2018).


In <a href="{{top}}">the top-down model</a> I start with all EAs and then filter them by interest in AI risk, direct work, and % of time working on safety. (EA safety has a lot of hobbyists, for instance me.) The <a href="{{bottom}}">bottom-up model</a> attempts a headcount.


### How large is non-EA Safety?

<a href="{{fermi}}">A rough point estimate</a> gives 84k or 103k AI academics, with caveats summarised in the Guesstimate notes. Then define a (very rough) relevance filter:

	CS = % of capabilities work that overlaps with short-term safety
	SL = % of short-term safety that overlaps with long-term safety

Then, we could decompose the safety-relevant part of academic AI as:

	SR = (% of AI work on capabilities * CS * SL)  + (% of AI work on short-term safety * SL)

None of those parameters is obvious, but I make an attempt in <a href="{{top}}">the model</a> (bottom-left corner).

Then the non-EA safety size is simply the field size * SR.

This just counts academia, and just technical AI within that. It's harder to estimate the amount of industrial effort, but the <a href="{{index}}">AI Index</a> report suggests that commercial AI research is about 10% as large as academic research (by number of papers, not impact). But we don't need this if we're just arguing that the non-EA lower bound is large.

### What’s a good discount factor for de facto safety work?

In EA safety, it’s common to be <a href="{{yud}}">cynical</a> about academia and <a href="{{empirical}}">empirical AI safety</a>. There’s something to it: the amount of paperwork and communication overhead is notorious; there are <a href="{{perv}}">perverse incentives</a> around publishing tempo, short-termism, and conformity; it is very common to emphasise only the positive effects of your work; and, as the <a href="{{gpt}}">GPT-2 story</a> shows, there is a strong dogma about automatic disclosure of all work. Also, insofar as AI safety is ‘pre-paradigmatic’, you might not expect normal science to make much <a href="{{xrisk}}">headway</a>. (But note that several agent-foundation-style models are from academia - see ‘A cursory check’ below.)

This is only half of the ledger. One of the big advantages of academic work is the much better distribution of senior researchers: EA Safety seems bottlenecked on people able to guide and train juniors. Another factor is increased influence: the average academic has serious opportunities to affect policy, hundreds of students, and the general attitude of their field toward alignment, including non-academic work on alignment. Lastly, you get access to government-scale funding. I ignore these positives in the following.

<br>

## Model

<a href="{{top}}">Here’s a top-down model</a> arguing that technical AI academics could have the same order of effect as EA, even under a heavy impact discount, even when ignoring other fields and the useful features of academia. <a href="{{bottom}}">Here’s an (incomplete) bottom-up model</a> to check if it’s roughly sensible. As you can see from the variance, the output means are not to be trusted.

<center>
	<img src="/img/eaais.png" /><br>
	<img src="/img/noeaais.jpg" /><br>
	<small>A “confidence” interval </small>
</center>

Again, the model is conservative: I don’t count the most prominent safety-relevant academic institutions (FHI, CHAI, etc); I don’t count contributions from industry, just the single most relevant academic field; I don’t count non-technical academic contributions; and a high discount is applied to academic work. For the sake of argument I’ve set the discount very high: a unit of adjacent academic work is said to be 80% less effective than a unit of explicit AGI work. The models rely on my priors; customise them before drawing conclusions (see ‘Parameters’ below).

<br>


### A cursory check of the model

The above implies that there should be a lot of mainstream work with alignment implications - maybe as much as EA produces. A systematic study would be a big undertaking, but can we at least find examples? Yes:
aix
* <a href="{{aix}}">AIXI</a> (2000), a theoretically optimal RL agent.

* <a href="{{goe}}">Gödel machines</a> (2003), the limit case of verified self-improvement.

* <a href="{{irl}}">Inverse reinforcement learning</a> (2004 - 2016). A limited but fruitful model for thinking about value learning. 

* Various forms of <a href="{{il}}">Imitation learning</a>

* Active learning, particularly <a href="{{tamer}}">TAMER</a> (2009) and <a href="{{arl}}">active reward learning</a> (2014).

* Info-theoretic measures of control like <a href="{{emp}}">empowerment</a> (2005).

* <a href="{{adv}}">Adversarial training</a> (2015). As used in AI Safety Debate.

* Wooldridge on the <a href="{{w1}}">game-theoretic</a> / <a href="{{w2}}">social choice</a> agent foundations of AI.

* Existence proof for the short/long-term overlap: The Stanford “<a href="{{cfai}}">Center for AI Safety</a>” is a good example. Zero mention of AGI or alignment while working on many of the de facto topics.

By comparison, how much does EA safety produce? In Larks’ <a href="{{lark}}">exhaustive annual round-up</a> of EA safety work in 2019, he identified about 50 paper-sized chunks (not counting MIRI’s private efforts). Of them, both CAIS and mesa-optimisers seem more significant than the above. Recent years have seen similarly important EA work (e.g. Debate, quantilizers, or the Armstrong/Shah discussion of value learning).

<br>

## What does this change?

I argue that AIS is less neglected than it seems, because some academic work is related, and academia is enormous. (My confidence interval for the academic contribution is vast - but I didn’t quite manage to zero out the lower bound even by being conservative.) Does this change the cause’s priority?

Probably not. Even if the field is bigger than we thought, it’s still extremely small relative to the investment in AI capabilities, and highly neglected relative to its importance. The point of the above is to correct your model, to draw attention to other sources of useful work, and to help sharpen a persistent disagreement within EA safety about the role of mid-term safety and academia.

This might change your view of effective interventions within AIS (for instance, ways to bring AGI alignment further within the Overton window), but my model doesn’t get you there on its own. A key quantity I don’t really discuss is the ratio of capabilities to alignment work. It seems prohibitively hard to reduce capabilities investment. But a large, credible academic field of alignment is one way to replace some work on capabilities.

<br>

## Future safety-related work

A naive extrapolation implies that AIS neglectedness will decrease further: in the last 10 years, Safety has moved from the fringe of the internet into the heart of great universities and NGOs. We have momentum: the programme is supported by some of the most influential AI researchers - e.g. <a href="{{russell}}">Russell</a>, <a href="{{bengio}}">Bengio</a>, <a href="{{sutskever}}">Sutskever</a>, <a href="{{shanahan}}">Shanahan</a>, <a href="{{rossi}}">Rossi</a>, <a href="{{bart}}">Selman</a>, <a href="{{mc}}">McAllester</a>, <a href="{{pearl}}">Pearl</a>, <a href="{{schmid}}">Schmidhuber</a>, <a href="{{horv}}">Horvitz</a>. (Often only verbal approval.) 

In addition, from personal experience, junior academics are much more favourable towards alignment and want to work on it.

Lastly: Intuitively, the economic incentive to solve AGI-safety-like problems scales as capabilities increase and as mid-term problems draw attention. Ordinary legal liability disincentivises all the sub-existential risks. (The incentive may not scale properly, from a longtermist perspective, but the direction seems good.)

If this continues, then even the EA bet on direct AGI alignment could be totally outstripped by normal academic incentives (prestige, social proof, herding around the agendas of top researchers).

A <a href="{{fore}}">cool forecasting competition</a> is currently running on a related question.

This argument depends on our luck holding, and moreover, on people (e.g. me) not naively announcing victory and so <a href="{{ssc}}">discouraging investment</a>. But to the extent that you trust the trend, this should affect your prioritisation of AI safety, since its expected neglectedness is a great deal smaller.

<br>

## Parameters 

* Your probability of <a href="{{pros}}">prosaic AGI</a> (i.e. where we get there by just scaling up black-box algorithms). Whether it’s possible to align prosaic AGI. Your probability that agent foundations is the only way to promote real alignment.

* The percentage of mainstream work which is relevant to AGI alignment. Subsumes the capabilities/safety overlap and the short/long term safety overlap. The idea of a continuous discount on work adjacent to alignment would be misguided if there were really two classes of safety problem, short- and long-term, and if short-term work had negligible impact on the long-term problems. The relevance would then be near 0.

* The above is extremely sensitive to your forecast for AGI. Given very short timelines, you should focus on other things than climbing up through academia, even if you think it’s generally well-suited to this task; conversely, if you think we have 100 years, then you can have pretty strong views on academic inadequacy and still agree that their impact will be substantial.


<br>

## Caveats, future work

* To estimate academia fairly, you’d need a more complicated model, involving second-order effects like availability of senior researchers, policy influence, opportunity to spread ideas to students and colleagues, funding. That is, academia has extremely clear paths to global impact. But since academia is stronger on the second order, omitting it doesn’t hurt my lower-bound argument.

* If you have an extremely negative view of academia's efficiency, then the above may not move you much. (See for instance, the <a href="{{pat}}">dramatically diminishing return</a> on inputs in mature fields like physics.) 

* A question which deserves a post of its own is: “How often do scientists inadvertently solve a problem?” (The general form - “how often does seemingly unrelated work help? Provide crucial help?” - seems trivial: many solutions are helped by seemingly unrelated prior work.) I’m relying on the overlap parameters to cover the effect of “actually trying to solve the problem", but this might not be apt. Maybe average academia is to research as the average charity is to impact: maybe directly targeting impact is that important.

* I haven’t thought much about potential harms from academic alignment work. Short-termists crowding out long-termists and a lack of attention to info hazards might be two.

* Intellectual impact is <a href="{{pat}}">not linear</a> <a href="{{cit}}">in people</a>. Also, the above treats all (non-EA) academic institutions as equally conducive to safety work, which is not true.

* <a href="{{more}}">Even more caveats</a>.

<br>


