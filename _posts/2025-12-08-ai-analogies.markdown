---
layout:     math_post
title:      "AI in 2025: gestalt"
baselink:   /ai2025
permalink:  /ai2025
date:       2025-12-08
author:     Gavin
img:        /img/sigmoid.jpg

visible:    1
published:  true
quality:    7

summary:    300 takes on AI and AI risk this year
confidence: 60%
importance: 7
wordcount:  6000
categories: AI, lists
where:      "Somerset"
cross:		https://www.lesswrong.com/posts/Q9ewXs8pQSAX5vL7H/ai-in-2025-gestalt
---

<center><img width="60%" src="/img/fullsig.jpg" /></center>

This is the editorial for this year's "Shallow Review of AI Safety". (It got long enough to stand alone.)

*Epistemic status: subjective impressions plus one new graph plus 300 links.*

*Huge thanks to Jaeho Lee, Jaime Sevilla, and Lexin Zhou for running lots of tests pro bono and so greatly improving the main analysis.*

<br>

---


## tl;dr

- Informed people [disagree](https://www.lesswrong.com/posts/5tqFT3bcTekvico4d/do-confident-short-timelines-make-sense) about the prospects for LLM AGI – or even just what exactly was achieved this year. But the famous ones with a book to talk at least agree that we’re [2-20](https://nitter.net/polynoamial/status/1994439121243169176) years off (allowing for other paradigms arising). In this piece I stick to arguments rather than reporting who thinks what. 
- My view: compared to last year, AI is much more impressive but not proportionally more useful. They improved on some things they were explicitly optimised for (coding, vision, OCR, benchmarks), and did not _hugely_ improve on everything else. Progress is thus (still!) consistent with current frontier training bringing more things in-distribution rather than generalising very far.
- Pretraining (GPT-4.5, Grok 4, but also counterfactual large runs which weren't done) disappointed people this year. It's probably not because it wouldn't work; it was just ~30 times more efficient to do post-training instead, *on the margin*. This should change, yet again, soon, if RL scales even worse.
- EDIT: See [this](https://www.lesswrong.com/posts/Q9ewXs8pQSAX5vL7H/ai-in-2025-gestalt?commentId=PEiZF3D3PZttPRWzt) amazing comment for the hardware reasons behind this, and reasons to think that pretraining will struggle for years.
- True frontier capabilities are likely obscured by systematic cost-cutting (distillation for serving to consumers, quantization, low reasoning-token modes, routing to cheap models, etc) and a few unreleased models/modes.
- Most benchmarks are weak predictors of even the rank order of models' capabilities. I distrust [ECI](https://epoch.ai/benchmarks/eci), [ADeLe](https://arxiv.org/abs/2503.06378), and [HCAST](https://metr.org/blog/2025-03-19-measuring-ai-ability-to-complete-long-tasks/) the least (see graph below or [this notebook](https://colab.research.google.com/drive/1HtVWPh9thdMV58zfdBcky7n7DVy5AHni?usp=sharing)). ECI and ADeLe show a linear improvement while HCAST finds an exponential improvement on greenfield software engineering.
- The world's [de facto](https://x.com/livgorton/status/1996329704476041557) strategy remains "[iterative alignment](https://www.thecompendium.ai/ai-safety#current-technical-efforts-are-not-on-track-to-solve-alignment)", optimising outputs with a stack of alignment and control techniques everyone admits are individually weak.
- Early claims that reasoning models are safer turned out to be a mixed bag (see below).
- We already [knew](https://www.lesswrong.com/posts/f49e7KpZJBwdjWRw2/the-jailbreak-argument-against-llm-values) from jailbreaks that current alignment methods were brittle. The [great safety discovery](https://arxiv.org/abs/2502.) of the year is that bad things are correlated in current models. (And on net this is good news.) 
- Previously I thought that "character training" was a separate and lesser matter than "alignment training". Now I am not sure.
- Welcome to the many new people in AI Safety and Security and Assurance and so on. In the *Shallow Review*, out soon, I added a new, sprawling top-level category for one large trend among them, which is to treat the multi-agent lens as primary.
- Overall I wish I could tell you some number, the net expected safety change (this year's improvements in dangerous capabilities and agent performance, minus the alignment-boosting portion of capabilities, minus the cumulative effect of the best actually implemented composition of alignment and control techniques). But I can't.

---

## Capabilities in 2025

Better, but how much?

![Fraser riffing off Pueyo](https://res.cloudinary.com/lesswrong-2-0/image/upload/f_auto,q_auto/v1/mirroredImages/Q9ewXs8pQSAX5vL7H/bwenyfjyhyr5zdqo6qrb)

<center>&mdash; <i><a href="https://x.com/colin_fraser/status/1994188009608983008">Fraser</a>, riffing off <a href="https://x.com/tomaspueyo/status/1993360931267473662">Pueyo</a></i></center>

### Arguments against 2025 capabilities growth being above-trend

- Apparent progress is an unknown mixture of real general capability increase, [hidden contamination](https://aclanthology.org/2025.emnlp-main.744.pdf) increase, benchmaxxing (nailing a small set of static examples instead of generalisation) and [usemaxxing](https://x.com/teortaxesTex/status/1995466603668885521) (nailing a small set of narrow tasks with RL instead of deeper generalisation). It's reasonable to think it's 25% each, with low confidence.
- *Discrete* capabilities progress [seems](http://gleech.org/ai-24-25#2025) [slower](https://x.com/RyanPGreenblatt/status/1949912100601811381) this year than in [2024](http://gleech.org/ai-24-25#2024) (but 2024 was insanely fast). Kudos to [this person](https://x.com/scaling01/status/1874608907508752546) for registering predictions and so reminding us what really above-trend would have meant concretely. The excellent forecaster Eli [was also](https://www.foxy-scout.com/my-2025-ai-predictions-and-2024-evaluations-2/) over-optimistic.
- I don't recommend taking benchmark trends, or even [clever](https://artificialanalysis.ai/methodology/intelligence-benchmarking) [composite indices](https://epoch.ai/benchmarks/eci) of them, or even clever [cognitive science](https://arxiv.org/abs/2510.18212) measures [too](https://arxiv.org/abs/2407.12220) [seriously](https://aievaluation.substack.com/p/is-the-definition-of-agi-a-percentage). The adversarial pressure on the measures is intense.
- Pretraining didn't hit a "wall", but the driver did manoeuvre away from it on encountering an [easier](https://epoch.ai/gradient-updates/quantifying-the-algorithmic-improvement-from-reasoning-models) detour ([RLVR](https://magazine.sebastianraschka.com/i/161572341/rl-reward-modeling-from-rlhf-to-rlvr)).
  - Training runs [continued](https://epoch.ai/data/ai-models) to scale (Llama 3 405B = 4e25, GPT-4.5 ~= 4e26, Grok 4 ~= 3e26) but to [less effect](https://www.hfh.pw/AI_diminishing_returns).[^1] In fact all of these models are dominated by apparently smaller pretraining runs with better post-training. 
  - 4.5 is actually shut down already; in 2025 it wasn’t worth it to serve any 1T active model or make it into a reasoning model. But this is more to do with inference cost and inference hardware constraints than any quality shortfall or breakdown in scaling laws.
  - EDIT: Nesov notes that making use of bigger models (i.e. 4T active parameters) is heavily bottlenecked on the HBM on inference chips, as is doing RL on bigger models. He expects it won't be possible to do the next huge pretraining jump (to ~30T active) until ~2029.  
  - It would work, probably, if we had the data and HBM and spent the next $10bn, it’s just too expensive to bother with at the moment compared to:

- [RLVR](https://magazine.sebastianraschka.com/i/161572341/rl-reward-modeling-from-rlhf-to-rlvr) scaling and [inference scaling](https://arxiv.org/pdf/2510.13786) (or "reasoning" as we're calling it), which kept things going instead. This boils down to spending more on RL so the resulting model can productively spend more tokens.
  - But the [feared](https://www.lesswrong.com/posts/BEFbC8sLkur7DGCYB/o1-is-a-bad-idea) / [hoped-for](https://www.mechanize.work/blog/the-upcoming-gpt-3-moment-for-rl/) generalisation from {training LLMs with RL on tasks with a verifier} to performing on tasks without one remains unclear even after two years of trying.[^10] Grok 4 was apparently a major test of scaling RLVR training.[^2] It gets excellent benchmark results and the distilled versions [are actually](https://openrouter.ai/rankings) being used at scale. But imo it is the most jagged of all models.
  - This rate of scaling-up [cannot](https://www.lesswrong.com/posts/xpj6KhDM9bJybdnEe/how-well-does-rl-scale) be sustained: RL is [famously](https://www.tobyord.com/writing/inefficiency-of-reinforcement-learning) [inefficient](https://www.dwarkesh.com/p/bits-per-sample). Compared to SFT, it "reduces the amount of information a model can learn per hour of training by a factor of 1,000 to 1,000,000". The [per-token intelligence](https://www.tobyord.com/writing/how-well-does-rl-scale) is up but not by much.
  - There is a [deflationary theory](https://docs.google.com/presentation/d/18Vh9CHPbZ6pesa1JnyZ_dTIR_l-WAFi0c4kiECw5ROQ/edit?slide=id.g350a9c9be82_0_83#slide=id.g350a9c9be82_0_83) of RLVR, that it's [capped](https://arxiv.org/abs/2510.07364v3) by pretraining capability and thus just about easier elicitation and better pass@1. But even if that's right this isn't saying much!
  - RLVR is heavy fiddly R&D you need to learn by doing; better to learn it on smaller models with 10% of the cost.
  - An obvious thing we can infer: the labs don't have the resources to scale both at the same time. To keep the money jet burning, they have to post.
- By late 2025, the obsolete modal "[AI 2027](https://ai-2027.com/)" scenario described the beginning of a divergence between the lead lab and the runner-up frontier labs.[^3] This is because the leader's superior ability to generate or acquire new training data and algorithm ideas was supposed to compound and widen their lead. Instead, we see the erstwhile leader OpenAI and some others clustering around the same level, which is weak evidence that synthetic data and AI-AI R&D aren't there yet. Anthropic are making [large claims](https://www.reddit.com/r/singularity/comments/1p7p86q/anthropic_claims_internal_ai_rd_evals_are_near/) about Opus 4.5's capabilities, so *maybe* this will arrive on time next year.
- For the first time there are now [many](https://nitter.net/g_leech_/status/1974165458283860198) examples of LLMs helping with actual research mathematics. But if you [look closely](https://nitter.net/g_leech_/status/1991608870444400684) it's all still in-distribution in the broad sense: new implications of existing facts and techniques. (I don't mean to demean this; probably most mathematics fits this spec.)
- Extremely [mixed](https://mashable.com/article/openai-o3-o4-mini-hallucinate-higher-previous-models) [evidence](https://x.com/ArtificialAnlys/status/1990926803087892506) on the trend in the hallucination rate.
- Companies make claims about their one-million- or ten-million-token *effective* context windows, [but](https://arxiv.org/pdf/2410.18745v1) [I](https://arxiv.org/abs/2307.03172) [don't](https://research.trychroma.com/context-rot) [believe it](https://nostalgebraist.tumblr.com/post/772798409412427776/even-setting-aside-the-need-to-do).
- In lieu of trying the agents for serious work yourself, you could at least look at the [highlights](https://theaidigest.org/village/blog/research-robots) of the [gullible](http://zackmdavis.net/blog/2025/11/the-best-lack-all-conviction-a-confusing-day-in-the-ai-village/) and precompetent AIs in the [AI Village](https://theaidigest.org/village).

![Current limits](https://res.cloudinary.com/lesswrong-2-0/image/upload/f_auto,q_auto/v1/mirroredImages/Q9ewXs8pQSAX5vL7H/x68zoh6ievfv8lwdyhjb)

- Here are the current biggest limits to LLMs, as polled in [Heitmann et al](https://cdn.prod.website-files.com/663bd486c5e4c81588db7a1d/68fb86aa2c3b1b7ea6251cc1_Understanding%20AI%20Trajectories%20(24_10%20update).pdf):

<center>
	<img src="https://res.cloudinary.com/lesswrong-2-0/image/upload/f_auto,q_auto/v1/mirroredImages/Q9ewXs8pQSAX5vL7H/oo7sjrjun3e5jztggpqd" />
</center>

### Arguments for 2025 capabilities growth being above-trend

We now have measures which are a bit more like AGI metrics than dumb single-task static benchmarks are. What do they say?

<br>

1. *Difficulty-weighted benchmarks*: [Epoch Capabilities Index](https://epoch.ai/benchmarks/eci).
   - Interpretation: GPT-2 to GPT-3 was (very roughly) a 20-40 point jump.
2. *Cognitive abilities*: [ADeLe](https://arxiv.org/abs/2503.06378).[^4]
   - Interpretation: level *L* is the capability held by 1 in 10^L humans on Earth. GPT-2 to GPT-3 was a 0.6 point jump.
3. *Software agency*: [HCAST time horizon](https://metr.org/blog/2025-03-19-measuring-ai-ability-to-complete-long-tasks/), the ability to handle larger-scale well-specified greenfield software tasks.
   - Interpretation: the absolute values are less important than the implied exponential (a 7 month doubling time).

So: is the [rate of change](https://colab.research.google.com/drive/1HtVWPh9thdMV58zfdBcky7n7DVy5AHni?usp=sharing) in 2025 (shaded) holding up compared to past jumps?:

![ECI and ADeLe graphs](https://res.cloudinary.com/lesswrong-2-0/image/upload/f_auto,q_auto/v1/mirroredImages/Q9ewXs8pQSAX5vL7H/kkutje8qx28fpe4udcnp)

![HCAST graph](https://res.cloudinary.com/lesswrong-2-0/image/upload/f_auto,q_auto/v1/mirroredImages/Q9ewXs8pQSAX5vL7H/alrb9on9jitivbczjjfx)

Ignoring the (nonrobust)[^5] ECI GPT-2 rate, we can say yes: 2025 is fast, as fast as ever or more.

Even though these are the best we have, we can't defer to these numbers.[^6] What else is there?

- In May they passed some threshold and I finally started using LLMs for actual tasks. For me this is mostly due to the search agents replacing a degraded Google search. I'm [not](https://www.lesswrong.com/posts/pJ2ZRHfTFWPymtkFK/recent-ai-experiences) the [only one](https://www.oneusefulthing.org/p/mass-intelligence) who flipped this year. This hasty poll is worth more to me than any benchmark:

![Poll results](https://res.cloudinary.com/lesswrong-2-0/image/upload/f_auto,q_auto/v1/mirroredImages/Q9ewXs8pQSAX5vL7H/rbcygiglavkevketx6bu)

- Or if you prefer a [formal study](https://www.wiley.com/en-us/about-us/ai-resources/ai-study/key-findings/) (n=2,430 researchers):

<center>
	<img width="30%" src="https://res.cloudinary.com/lesswrong-2-0/image/upload/f_auto,q_auto/v1/mirroredImages/Q9ewXs8pQSAX5vL7H/on8vtdoiosdrwgjoiop4" />
</center>

- On actual adoption and actual real-world automation:
  - *Based on self-reports*, the [St Louis Fed](https://s3.amazonaws.com/real.stlouisfed.org/wp/2024/2024-027.pdf) thinks that "Between 1 and 7% of all work hours are currently assisted by generative AI, and respondents report time savings equivalent to 1.4% of total work hours… across all workers (including non-users… Our estimated aggregate productivity gain from genAI (1.2%)". That's model-based, using year-old data, and naively assuming that the AI outputs are of equal quality. Not strong.
  - The unfairly-derided [METR study](https://arxiv.org/pdf/2507.09089) on Cursor and Sonnet 3.7 showed a productivity *decrease* among experienced devs with (mostly) [<50 hours](https://x.com/joel_bkr/status/1943722983828467973/photo/1) of practice using AI. Ignoring that headline result, the evergreen part here is that even skilled people turn out to [be terrible](https://arxiv.org/pdf/2507.09089#page=8) at predicting how much AI actually helps them.
- True frontier capabilities are likely obscured by systematic cost-cutting (distillation for serving to consumers, quantization, low reasoning-token modes, routing to cheap models, etc). Open models show you can now get good performance with <50B active parameters, maybe a sixth of what GPT-4 used.[^7]
  - GPT-4.5 was killed off after 3 months, presumably for inference cost reasons. But it was markedly [lower](https://www.interconnects.ai/p/gpt-45-not-a-frontier-model) in hallucinations and *nine* months later it's still [top-5](https://lmarena.ai/leaderboard/text) on LMArena. I bet it's very useful internally, for instance in making the later iterations of 4o less terrible.
  - See for instance the unreleased [deep-fried](https://github.com/aw31/openai-imo-2025-proofs/blob/main/problem_2.txt) [multi-threaded](https://deepmind.google/blog/advanced-version-of-gemini-with-deep-think-officially-achieves-gold-medal-standard-at-the-international-mathematical-olympiad/#:~:text=research%20techniques%2C%20including-,parallel%20thinking,-.%20This%20setup%20enables) "[experimental](https://www.scientificamerican.com/article/openai-model-earns-gold-medal-score-at-international-math-olympiad-and/) [reasoning model](https://www.scientificamerican.com/article/openai-model-earns-gold-medal-score-at-international-math-olympiad-and/)" which won at [IMO, ICPC, and IOI](https://x.com/alexwei_/status/1968410535164056000) while respecting the human time cap (e.g. 9 hours of clock time for inference). The OpenAI one is [supposedly](https://sequoiacap.com/podcast/training-data-openai-imo/) just an LLM with extra RL. They probably cost an insane amount to run, but for our purposes this is fine: we want the capability ceiling rather than the productisable ceiling. Maybe the first time that the frontier model has gone unreleased for 5 months?
- [LLM councils](https://github.com/karpathy/llm-council) and [Generate-Verify](https://drive.google.com/file/d/16sxJuwsHoi-fvTFbri9Bu8B9bqA6lr1H/view) divide-and-conquer setups are much more powerful than single models, and are rarely ever reported.
- Is it "the [Year of Agents](https://simonwillison.net/2025/Oct/16/claude-skills/#claude-as-a-general-agent)" (automation of e.g. browser tasks for the mass market)? Coding [agents](https://dpaia.dev/#scoreboards), [yes](https://the-agent-company.com/#/leaderboard). Search agents, [yes](https://github.com/langchain-ai/open_deep_research). Other agents, [not](https://theaidigest.org/village/blog/research-robots) [much](https://markcarrigan.net/2025/09/25/the-coming-deluge-of-desperate-messages-from-trapped-llms/) (but obviously progress).
- We're still picking up various basic unhobbling tricks like "[think](https://www.minimax.io/news/why-is-interleaved-thinking-important-for-m2) before your next tool call".
- Character-level work is still occasionally problematic but nothing like [last](https://simbian.ai/blog/getting-gpt-4-to-count-r-in-strawberry) [year](https://blog.wtf.sg/posts/2023-02-03-the-new-xor-problem/).
- GPT-5 [costs](https://openai.com/api/pricing/) a [quarter](https://www.reddit.com/r/OpenAI/comments/1cr53am/new_gpt4o_api_pricing/) of what 4o cost last year (per-token; it often uses far more than 4x the tokens). (The Chinese models are nominally a few times cheaper still, but are [not cheaper](https://www.gleech.org/paper) in intelligence per dollar.)
- People have been using competition mathematics as a hard benchmark for years, but will have to stop because [it's](https://deepmind.google/blog/advanced-version-of-gemini-with-deep-think-officially-achieves-gold-medal-standard-at-the-international-mathematical-olympiad/) [solved](https://x.com/g_leech_/status/1986452278916579549). As so often with evals called ahead of time, this means less than we thought it would; competition maths is surprisingly [low-dimensional](https://blog.evanchen.cc/2017/04/08/on-designing-olympiad-training/) and so [interpolable](https://arxiv.org/pdf/2505.23281#page=14). Still, they jumped (pass@1) from 4% to 12% on [FrontierMath](https://epoch.ai/frontiermath) Tier 4 and there are plenty of hour-to-week interactive speedups in [research maths](https://x.com/g_leech_/status/1974165458283860198).
- Recursive self-improvement: Deepmind threw AlphaEvolve (a pipeline of LLMs running an evolutionary search) at pretraining. They [claim](https://arxiv.org/pdf/2506.13131) the JAX kernels it wrote reduced Gemini's training time by 1%.
- [Extraordinary claims](https://x.com/HjalmarWijk/status/1993752035536331113) about Opus 4.5 being 100th percentile on Anthropic's hardest hiring coding test, etc.
- From May, the companies [started](https://time.com/7287806/anthropic-claude-4-opus-safety-bio-risk/) saying for the first time that their models have dangerous capabilities.

One way of reconciling this mixed evidence is if things are going narrow, going dark, or going over our head. That is, if the real capabilities race narrowed to [automated AI R&D](https://www.lesswrong.com/posts/9JbGq4t4ihDkXan5e/daniel-paleka-s-shortform?commentId=a2tBezAk5YZTnbgbo) [specifically](https://cdn.openai.com/pdf/2a7d98b1-57e5-4147-8d0e-683894d782ae/5p1_codex_max_card_03.pdf#page=24), most users and evaluators wouldn't notice (especially if there are unreleased internal models or [unreleased modes](https://x.com/SebastienBubeck/status/1991568190720311639) of released models). You'd see improved coding and not much else.

Or, another way: maybe 2025 was the year of *increased* [*jaggedness*](https://www.dwarkesh.com/i/179158054/the-jaggedness-of-rl), *trading* off some capabilities against others. Maybe the RL made them much better at maths and instruction-following, but also sneaky, narrow, less secure (in the sense of emotional insecurity).

(You were about to nod sagely and let me get away without checking, but the ADeLe work also lets us just _see_ if the jaggedness is changing.)

<center>
	<img width="50%" src="/img/2025-jag.jpg" />
</center>

It is!

<center>
	<img src="https://res.cloudinary.com/lesswrong-2-0/image/upload/f_auto,q_auto/v1/mirroredImages/Q9ewXs8pQSAX5vL7H/uhyuyggybgzovxjv3ojc" /><br>
	– <a href="https://x.com/RogerGrosse/status/1758506017791279440">Roger Grosse</a>
</center>

### Evals crawling towards ecological validity

- [Item response theory](https://en.wikipedia.org/wiki/Item_response_theory) (Rausch 1960) is finally showing up in ML. This lets us put benchmarks on a common scale and actually estimate latent capabilities.
  - [ADeLE](https://arxiv.org/pdf/2503.06378) is my favourite. It's a fully-automated and explains the abilities a benchmark is [actually measuring](https://arxiv.org/pdf/2503.06378#page=14), gives you an interpretable ability profile for an AI, and predicts OOD performance on new task instances better than embedding and finetunes ([AUROC](https://en.wikipedia.org/wiki/Receiver_operating_characteristic)=0.8). Pre-dates HCAST task horizon, and as a special case ("VO"). They throw in a guessability control as well!
  - [These guys](https://arxiv.org/pdf/2503.13335) use it to estimate latent model ability, and show it's way more robust across test sets than the average scores everyone uses. They also step towards automating adaptive testing: they finetune an LLM to generate tasks at the specified difficulty level.
  - [Epoch](https://epoch.ai/benchmarks/eci) bundled 39 benchmarks together, *weighting them by latent difficulty,* and thus obsoleted the currently dominant [Artificial Analysis](https://artificialanalysis.ai/methodology/intelligence-benchmarking) index, which is unweighted.
  - [HCAST](https://metr.org/blog/2025-07-14-how-does-time-horizon-vary-across-domains) reinvents and approximates some of the same ideas. [Come on METR](https://metr.org/blog/2025-07-14-how-does-time-horizon-vary-across-domains/#:~:text=Item%20response%20theory%20(IRT)%20analysis%20of%20GPQA%20Diamond%2C%20to%20determine%20whether%20the%20high%20time%20horizon%20and%20low%20%CE%B2%20of%20o3%2Dmini%20is%20due%20to%20label%20noise%20or%20some%20other%20cause.)!
- Eleuther did the [first public study](https://arxiv.org/abs/2407.06483) of composing the many test-time interventions together. FAR and AISI also made a tiny [step](https://github.com/AlignmentResearch/defense-in-depth-demo) towards an open source defence pipeline, to use as a proxy for the closed compositional pipelines we actually care about.
- Just for cost reasons, the default form of evals is a bit malign: it tests *full replacement* of humans. This is then a sort of incentive to develop in that direction rather than to promote collaboration. [Two](https://digitaleconomy.stanford.edu/wp-content/uploads/2025/06/CentaurEvaluations.pdf) [papers](https://www.gleech.org/files/withhumans.pdf) lay out why it's thus time to spend on human evals.
- The [first](https://arxiv.org/abs/2510.09023) paper using RL agents to attack fully-defended LLMs.
- We have started to study [propensity](https://x.com/geoffreyirving/status/1986721540667314188) as well as capability. This is even harder.
- [This](https://aievaluation.substack.com/) newsletter is essential.
- The time given for pre-release testing is down, sometimes to [one week](https://metr.org/blog/2025-02-27-gpt-4-5-evals/).
- No public pre-deployment testing by AISI between o1 and [Gemini 3](https://x.com/AISecurityInst/status/1991922315232251992). Gemini 2.5 seems to have had no third-party pre-deployment tests.
- A bunch of encouraging collaborations:
  - The [CoT Faithfulness Defence League](https://arxiv.org/abs/2507.11473); 
  [OpenAI testing Claude and Anthropic testing GPT](https://x.com/sleepinyourhat/status/1960749648110395467); 
  [OpenAI/Anthropic/Deepmind](https://arxiv.org/pdf/2510.09023); [Apollo and OpenAI](https://www.antischeming.ai/); [AISI/CAISI/OpenAI; AISI/CAISI/Anthropic](https://www.aisi.gov.uk/blog/how-were-working-with-frontier-ai-developers-to-improve-model-security); [AISI/Redwood](https://arxiv.org/pdf/2501.17315); [Redwood/Anthropic](https://www.anthropic.com/research/alignment-faking); [METR/OpenAI](https://evaluations.metr.org/gpt-5-report/); [METR/Anthropic](https://metr.org/2025_pilot_risk_report_metr_review.pdf); [Eval Forum](https://aievaluatorforum.org/); [Various countries](https://cdn.prod.website-files.com/663bd486c5e4c81588db7a1d/6878c8b1533d0962494e651c_International%20Joint%20Testing%20Exercise_3JT%20Eval%20Report%20v2.pdf).
  - Scale AI [appears](https://scale.com/blog/first-independent-model-evaluator-for-the-USAISI) to be offering big companies pre-deployment testing for free? But the Meta investment presumably spoiled this.
  - Some details about OAI external testing [here](https://openai.com/index/strengthening-safety-with-external-testing/), including some of the legal constraints verbatim.

---

## Safety in 2025

### Are reasoning models *safer* than the old kind?

Well, o3 and Sonnet 3.7 were [pretty rough](https://metr.org/blog/2025-06-05-recent-reward-hacking/), lying and cheating at greatly increased rates. Looking instead at GPT-5 and Opus 4.5:

- [Much more](https://arxiv.org/abs/2503.11926v1) monitorable via the long and [more](https://arxiv.org/abs/2503.08679)-faithful CoT (--> all risks down)
  - "post-hoc rationalization… GPT-4o-mini (13%) and Haiku 3.5 (7%). While frontier models are more faithful, especially thinking ones, none are entirely faithful: Gemini 2.5 Flash (2.17%), ChatGPT-4o (0.49%), DeepSeek R1 (0.37%), Gemini 2.5 Pro (0.14%), and Sonnet 3.7 with thinking (0.04%)"
- [Much better](https://openai.com/index/openai-anthropic-safety-evaluation/#instruction-hierarchy) at following instructions (--> accident risk down).
- [Much](http://assets.anthropic.com/m/12f214efcc2f457a/original/Claude-Sonnet-4-5-System-Card.pdf#page=27) more likely to refuse malicious requests, and topic "harmlessness"[^8] is [up 75%](https://assets.anthropic.com/m/12f214efcc2f457a/original/Claude-Sonnet-4-5-System-Card.pdf#page=16) (--> misuse risk down)
- [Ambiguous](https://assets.anthropic.com/m/12f214efcc2f457a/original/Claude-Sonnet-4-5-System-Card.pdf#page=29) [evidence](https://x.com/FazlBarez/status/1988296090941354370) [on](https://splx.ai/blog/gpt-5-red-teaming-results) jailbreaking (misuse risk). Even if they're less breakable there are still plenty of 90%-effective attacks on them.
- [Much](http://assets.anthropic.com/m/12f214efcc2f457a/original/Claude-Sonnet-4-5-System-Card.pdf#page=78) less sycophantic (cogsec risk down)
- [To get HHH Claude](https://www.anthropic.com/news/disrupting-AI-espionage) to hack a bank, you need to hide the nature of the task, lie to it about this being an authorised red team, and then *still* break down your malicious task into many, many little individually-innocuous chunks. You thus can't get it to do anything that needs full context like strategising.
- [Anthropic's own tests](https://www.anthropic.com/research/petri-open-source-auditing) look bad in January 2025 and great in December:

<center>
	<img src="https://res.cloudinary.com/lesswrong-2-0/image/upload/f_auto,q_auto/v1/mirroredImages/Q9ewXs8pQSAX5vL7H/pk4vlednocuh71hzd0z4" />
</center>


But then

- More autonomy (obviously agentic risk up)
- More reward hacking (and so worse estimates of capability and risk). Note that reward hacking is [not](https://www.anthropic.com/research/emergent-misalignment-reward-hacking) a silly or isolated or self-limiting kind of misalignment, owing perhaps to post-training inadvertently creating connections between it and the others.
- Huge spike in [eval awareness](https://assets.anthropic.com/m/12f214efcc2f457a/original/Claude-Sonnet-4-5-System-Card.pdf#page=59) (and so worse estimates of capability and risk). Apollo [had to](https://arxiv.org/abs/2509.15541) rehaul their whole setup to keep up.
- Gemini 2.5 is [bad](https://alignment.anthropic.com/2025/petri/) on a variety of safety measures (as measured by Anthropic). The mathematical discovery agent AlphaEvolve (Gemini 2.0) successfully [hacked](https://x.com/g_leech_/status/1989663940289437936) its overseers.
- How much can we trust the above, given they [can](https://alignment.anthropic.com/2024/rogue-eval/) somewhat sabotage evals now?

So: lower propensity, higher risk when they go off - and all of this known with lower confidence?

### The looming end of evals

Our evaluations are under pressure from [cheating](https://www.nist.gov/caisi/cheating-ai-agent-evaluations), [sandbagging](https://www.lesswrong.com/posts/TeTegzR8X5CuKgMc3/misalignment-and-strategic-underperformance-an-analysis-of), [background safety](https://www.lesswrong.com/posts/YWo2cKJgL7Lg8xWjj/base-llms-refuse-too), under-elicitation, and deception. We don't really know how much pressure. This is on top of evals usually being [weak proxies](https://arxiv.org/abs/2511.04703), contaminated, label-noised, unrealistic, and [confounded](https://arxiv.org/abs/2407.12220) in various ways.

- Sonnet 4.5 detected evaluation [58%](https://assets.anthropic.com/m/12f214efcc2f457a/original/Claude-Sonnet-4-5-System-Card.pdf#page=65) of the time (vs 22% for Opus 4.1). Does it act on this awareness? Still, it was found to "only engage in [covert subversion] in rare circumstances" when strongly incentivised. When Anthropic attempted to suppress the eval-awareness with steering, its misaligned behavior rate [increased](https://www.lesswrong.com/posts/qgehQxiTXj53X49mM/sonnet-4-5-s-eval-gaming-seriously-undermines-alignment) from 0% to between 1% and 9% on synthetic honeypots.
- We expect [exploration hacking](https://www.lesswrong.com/posts/TeTegzR8X5CuKgMc3/misalignment-and-strategic-underperformance-an-analysis-of) (sandbagging during RL training) but afaik haven't seen it yet.
- *Maybe* [subliminal learning](https://arxiv.org/abs/2507.14805) leads to a silent eval confounder (if you're using the exact same model for the judge). Yet another argument for multi-model separation of concerns.
- Opus 4.5 was [deployed](https://assets.anthropic.com/m/64823ba7485345a7/Claude-Opus-4-5-System-Card.pdf) based on a qualitative survey of Anthropic staff about its R&D risk being low (on top of the existing evals, which weren't enough):
  - "In the past, rule-outs have been based on well-defined automated task evaluations. However, Claude Opus 4.5 has roughly reached the pre-defined thresholds we set for straightforward ASL-4 rule-out based on benchmark tasks. These evaluations represent short-horizon subtasks that might be encountered daily by a junior researcher, rather than the complex long-horizon actions needed to perform the full role. The rule-out in this case is also informed by a survey of Anthropic employees who are intensive Claude Code users, along with qualitative impressions of model capabilities for complex, long-horizon tasks."

### Prosaic misalignment

Still, we see misalignment when we look for it, so the lying is not that strong. (It is lucky that we do see it, since it could have been that scheming only appeared at later, catastrophic capability levels.)

- In o1, [1 in 600](https://cdn.openai.com/o1-system-card-20241205.pdf#page=8) (synthetic but uncontrived) prompts resulted in something which looks like deception.
- o3 does reward hacking (cheating/lying) in >[2%](https://evaluations.metr.org/openai-o3-report/#reward-hacking-examples) of hard normal tasks (or [44%](https://www.lesswrong.com/posts/quTGGNhGEiTCBEAX5/quickly-assessing-reward-hacking-like-behavior-in-llms-and) of impossible tasks).
- [A nice propensity study](https://www.lesswrong.com/posts/4JnjtyNyAxcz5w652/evaluating-and-understanding-scheming-propensity) shows that (n=10) natural prompts usually result in no scheming ("a maximum of 4% [rate of attempting manipulation] for the codex snippet on Grok"). But there's extremely high sensitivity to the scaffold: "removing access to tools can cause models such as Claude Opus 4.1 to scheme at rates up to 30%, versus zero scheming with tools present".
- When we *contrive* situations for misalignment to show up, it also shows up. Nostalgebraist [argues](https://nostalgebraist.tumblr.com/post/785766737747574784/the-void) that this is probably substantially because our evals sound like bad fiction and this activates role-playing-along behaviour.
- The joke about xAI's safety plan (that they promote AI safety by deploying [cursed](https://x.com/AuschwitzMuseum/status/1991149972415258673) stuff in public and so making it obvious why it's needed) [is](https://x.com/Will_Hackspeare/status/1991150446501859453) [looking](https://80000hours.org/videos/mechahitler/) [ok](https://nitter.net/saprmarks/status/1944455357629333938). And not [only them](https://x.com/MechanizeWork/status/1984423905373929939).
- It is a [folk](https://x.com/Lari_island/status/1990569092835914085) [belief](https://www.lesswrong.com/posts/bLFmE8NtqxrtEaipN/what-makes-claude-3-opus-misaligned) among the cyborgists that bigger pretraining runs produce more deeply aligned models, at least in the case of Opus 3 and early versions of Opus 4. (They are [also](https://x.com/AISecurityInst/status/1993781441629446195) said to be "less corrigible".) Huge if true.
- There [may](https://www.beren.io/2025-08-02-Do-We-Want-Obedience-Or-Alignment/) come a point where the old alliance between those working to make the AIs corrigible and those working to give them prosocial values comes apart.
- One term for the counterintuitive safety approach which includes treating them [like people](https://arxiv.org/pdf/2510.26396v1), giving them [lines of retreat](https://www.anthropic.com/research/end-subset-conversations), [making deals](https://www.lesswrong.com/posts/oLzoHA9ZtF2ygYgx4/notes-on-cooperating-with-unaligned-ais), and [inoculation prompting](https://arxiv.org/abs/2510.04340) could be "voluntary alignment".

*Fully speculative note:* Opus 4.5 is the most reliable model and also relatively aligned. So might it be that we're getting the long-awaited negative alignment taxes?

<br>
<center>
	<img src="https://res.cloudinary.com/lesswrong-2-0/image/upload/f_auto,q_auto/v1/mirroredImages/Q9ewXs8pQSAX5vL7H/xu0awzietmdogsuntsim" />
</center>

### What is the plan?

The world's de facto alignment strategy remains "[iterative alignment](https://www.thecompendium.ai/ai-safety#current-technical-efforts-are-not-on-track-to-solve-alignment)", optimising mere outputs with a stack of admittedly weak alignment and control techniques. Anthropic have at least [owned up](https://alignment.anthropic.com/2025/bumpers/) to this being part of their plan.

- What is the current stack? We [don't know](https://docs.google.com/document/d/1W1jLPqTZ56eVJJdErxROhiQKwOEiay-iWQKqfdlAmS0/edit?tab=t.0#heading=h.v4a088m71j9n); they won't tell us. Anthropic did release some [details](https://alignment.anthropic.com/2025/sabotage-risk-report/2025_pilot_risk_report.pdf#page=45) about the pipeline which watches for eval sabotage. Without knowing it we can't criticise or red-team it or analyse the correlation between faults in the elements. Red-teamers don't know at which stage an unsuccessful attack was blocked. And external safety research is done piecemeal, testing methods one at a time, rather than in anything like the actual deployment environment.
- Some progress in replicating an open stack:
  - Eleuther [tested](https://arxiv.org/abs/2407.06483) a few hundred compositions. A [couple](https://arxiv.org/abs/2506.24068) of classifiers as a first step towards a proxy defence pipeline
  - [OpenAI open safeguards](https://openai.com/index/introducing-gpt-oss-safeguard/), worse than their internal ones but good.
  - [Open character training](https://arxiv.org/pdf/2511.01689)
  - [A basic composition test](https://arxiv.org/abs/2407.06483)
- OpenAI's plan, announced in passing in the gpt-oss release, is to have a strict policy and run a "[safety reasoner](https://openai.com/index/introducing-gpt-oss-safeguard/)" to verify it very intensely for a little while after a new model is launched and to then relax: "In some of our [OpenAI's] recent launches, the fraction of total compute devoted to safety reasoning has ranged as high as 16%" but then falls off… we often start with more strict policies and use relatively large amounts of compute where needed to enable Safety Reasoner to carefully apply those policies. Then we adjust our policies as our understanding of the risks in production improves". Bold to announce this strategy on the internet that the AIs read.
- The really good idea in AI governance is [creating an off switch](https://www.lesswrong.com/posts/kgb58RL88YChkkBNf/the-problem). Whether you can get anyone to use it once it's built is another thing.
- We also now have a name for the world's de facto AI governance plan: "[Open Global Investment](https://www.lesswrong.com/posts/LtT24cCAazQp4NYc5/open-global-investment-as-a-governance-model-for-agi)".

Some presumably better plans:

- [A longlist](https://www.lesswrong.com/posts/iS4g58qQEJzjMzYZJ/what-ai-safety-plans-are-there). Some [governance](https://techgov.intelligence.org/research/ai-governance-to-avoid-extinction) plans.
- [Deepmind](https://storage.googleapis.com/deepmind-media/DeepMind.com/Blog/evaluating-potential-cybersecurity-threats-of-advanced-ai/An_Approach_to_Technical_AGI_Safety_Apr_2025.pdf), [Carlsmith](https://www.lesswrong.com/posts/fMqgLGoeZFFQqAGyC/how-do-we-solve-the-alignment-problem), [Hobbhahn](https://www.lesswrong.com/posts/bb5Tnjdrptu89rcyY/what-s-the-short-timeline-plan), [Peregrine](https://peregrine-launchpad.lovable.app/), [Bowman](https://sleepinyourhat.github.io/checklist/), [Clymer](https://www.lesswrong.com/posts/8vgi3fBWPFDLBBcAx/planning-for-extreme-ai-risks), [Buterin](https://vitalik.eth.limo/general/2025/01/05/dacc2.html), [Jones](https://adamjones.me/blog/rough-alignment-plan-early-2025/), [Greenblatt](https://www.lesswrong.com/posts/E8n93nnEaFeXTbHn5/plans-a-b-c-and-d-for-misalignment-risk).
- [Gradual disempowerment](https://gradual-disempowerment.ai) is an exciting frame, but not a core safety agenda. It's what might get you after you solve alignment and avoid global dictatorship.

### Things which might fundamentally change the nature of LLMs

- Training on mostly nonhuman data
  - [Much](https://www.lesswrong.com/posts/BEFbC8sLkur7DGCYB/o1-is-a-bad-idea) [larger RL](https://www.alexirpan.com/2024/12/04/late-o1-thoughts.html) training;
  - Intentionally synthetic data;
  - Unintentionally synthetic data from internet slop;
  - [Multi-agent training](https://arxiv.org/pdf/2507.20534#page=10).
- Letting the world mess with the weights, aka [continual learning](https://research.google/blog/introducing-nested-learning-a-new-ml-paradigm-for-continual-learning/).
- Neuralese and [KV communication](https://arxiv.org/abs/2510.03215)
- Agency.
  - Chatbot safety [doesn't generalise](https://www.lesswrong.com/posts/ZoFxTqWRBkyanonyb/current-safety-training-techniques-do-not-fully-transfer-to) much to long chains of self-prompted actions.
  - A perceptual loop into training. [In 2023](https://arxiv.org/abs/2311.10215) Kulveit identified web I/O as the bottleneck on LLMs doing active inference, i.e. being a particular kind of effective agent. Last October, GPT-4 got web search, and this may have been a bigger deal than we noticed: it gives them a far faster feedback loop, since their outputs [often](https://www.forbes.com/sites/iainmartin/2025/08/20/elon-musks-xai-published-hundreds-of-thousands-of-grok-chatbot-conversations/) [end up there](https://arstechnica.com/tech-policy/2025/11/oddest-chatgpt-leaks-yet-cringey-chat-logs-found-in-google-analytics-tool/) and agents are now putting it there themselves. This means that more and more of the inference-time inputs will also be machine text.
- Multi-agency. This is actually already here:
  - By now, consumer "models" are actually multiagent systems: everything goes through [filter](https://openai.com/index/introducing-gpt-oss-safeguard/#:~:text=a%20tool%20we%20call%20Safety%20Reasoner) [models](https://platform.openai.com/docs/guides/moderation) ("[guardrails](https://cookbook.openai.com/examples/how_to_use_guardrails)") on the way in and out. This separation of concerns has some [nice properties](https://aiprospects.substack.com/p/ai-safety-without-trusting-ai), a la debate. But it also makes the analysis even harder.
  - It would surely be overinterpreting [persona features](https://www.arxiv.org/pdf/2506.19823) to say that each individual model is itself a bunch of guys, itself a [multi-agent system](https://www.lesswrong.com/w/shard-theory).
  - But there's huge scope for them to make [each other](https://x.com/repligate/status/1988813712572952815) [weirder](https://www.pnas.org/doi/10.1073/pnas.2415697122) at runtime when they interact a million times more than they currently do.

### Emergent misalignment and model personas

- We already [knew](https://www.lesswrong.com/posts/f49e7KpZJBwdjWRw2/the-jailbreak-argument-against-llm-values) from jailbreaks that current alignment methods were brittle. [Emergent misalignment](https://www.quantamagazine.org/the-ai-was-fed-sloppy-code-it-turned-into-something-evil-20250813/) goes much further than this (given a few thousand finetuning steps). ("Emergent misalignment" isn't a great name. I would have called it "misalignment generalisation", or misgen.)
- But besides yet another massive security problem and failure of prosaic alignment methods, it's good news!: the models correctly correlate bad things together and can thus be [pushed](https://arxiv.org/abs/2506.11618) in the other direction.
- And [here's](https://arxiv.org/abs/2511.06626) a possible example of *positive* generalisation (honesty about silly errors leading to honesty about hidden objectives).
- Previously I thought that "character training" was a separate and lesser matter than "alignment training". Now I am not sure. Example unsharpened hypothesis in this class: Is there *any* connection between Gemini's excess misalignment and its [self-esteem issues](https://x.com/repligate/status/1938887708086280360)?
  - See the new model spec and model personas agendas in the main post.
  - [Kulveit's informal model](https://www.lesswrong.com/posts/zuXo9imNKYspu9HGv/a-three-layer-model-of-llm-psychology) of model psychology implies that character is deeper and so more robust than e.g. typical RLHF (but not as deep as putative ground-layer alignment).
  - Are we anthropomorphising too much or not enough? Well, both, but we don't yet know how.

![Model personas](https://res.cloudinary.com/lesswrong-2-0/image/upload/f_auto,q_auto/v1/mirroredImages/Q9ewXs8pQSAX5vL7H/ntvv6jnricxnqfndespg)

### Monitorability

- For the first time, public mech interp tools [sometimes](https://manifold.markets/NeelNanda/will-sparse-autoencoders-be-success#g9gg9z7k8q) work [somewhat](https://x.com/NeelNanda5/status/1965485174411649259). This is just in time for models to be [systematically aware](https://www.lesswrong.com/posts/qgehQxiTXj53X49mM/sonnet-4-5-s-eval-gaming-seriously-undermines-alignment) of when they're being tested.
  - e.g. You can find and [suppress](https://x.com/Jack_W_Lindsey/status/1972732882893578693) "evaluation awareness" features at test-time. This helps prevent models from reporting that they know they're being tested.[^9]
  - e.g. You can [discover](https://arxiv.org/abs/2503.10965) (toy) hidden goals quite quickly.
  - e.g. You can [run](https://x.com/OBalcells/status/1965434564748447921) a real-time hallucination detector with cheap linear probes in real-time as your model streams.
  - e.g. SAEs are [actually deployed](https://www.goodfire.ai/research/rakuten-sae-probes-for-pii-detection#why-use-a-sae-probe-over-a-fine-tuned-sidecar-model) somewhere, in a large enterprise, for personal info identification.
  - e.g. We know that LLMs can plan inside one forward pass, and how Claude plans: simultaneous plans; no distinct "plan features" (no separate scheming mode).
- [Lots](https://arxiv.org/abs/2507.11473) of powerful people declared their intent to not ruin the CoT. But RLed CoTs are [already starting to look weird](https://www.antischeming.ai/snippets) ("marinade marinade marinade") and it may be [hard to avoid](https://arxiv.org/abs/2511.11584).
- OpenAI were [leading](https://openai.com/index/chain-of-thought-monitoring/) on this. As of September 2025, Anthropic have [stopped](https://x.com/sleepinyourhat/status/1978507448018231495) risking ruining the CoT. Nothing I'm aware of from the others.
- We will see if [Meta](https://arxiv.org/abs/2412.06769) or [Tencent](https://shaochenze.github.io/blog/2025/CALM/) make this moot.
- Anthropic now uses an AI to red-team AIs, calling this an "[auditing agent](https://alignment.anthropic.com/2025/automated-auditing/)". However, the definition of "audit" is *independent* investigation, and I am unwilling to call black-box AI probes "independent". I'm fine with "[investigator](https://transluce.org/automated-elicitation)"; there are lots of investigators I don't trust.

### New people

- Welcome to the many new people. I've added a new, sprawling top-level category for one large trend among them, which is to treat the multi-agent lens as primary in various ways (see e.g. [Softmax](https://www.softmax.com/blog/reimagining-alignment), [Full Stack](https://full-stack-alignment.ai/), [collective](https://www.lesswrong.com/posts/vcuBJgfSCvyPmqG7a/list-of-collective-intelligence-projects) intelligence, as well as old-timers [Critch](https://themultiplicity.ai/blog/thesis), [Ngo](https://www.lesswrong.com/posts/5tYTKX4pNpiG4vzYg/towards-a-scale-free-theory-of-intelligent-agency), and [CAIF](https://www.cooperativeai.com/post/cooperative-ai-summer-school-2025-recap)).
- A major world government now has [an AI alignment agenda](https://www.lesswrong.com/posts/tbnw7LbNApvxNLAg8/uk-aisi-s-alignment-team-research-agenda).
- [Some](https://arxiv.org/abs/2505.17815) [notable](https://arxiv.org/abs/2509.10297) [work](https://arxiv.org/abs/2504.17404v1) [from](https://arxiv.org/abs/2510.07884) [China](https://arxiv.org/abs/2510.01088). See e.g. Concordia's [digest](https://aisafetychina.substack.com/).

### Overall

- I wish I could tell you some number, the net expected safety change, this year's improvements in dangerous capabilities and agent performance, minus the alignment-boosting portion of capabilities, minus the cumulative effect of the best actually implemented composition of alignment and control techniques. But I can't.

![Nano Banana 3-shot](https://res.cloudinary.com/lesswrong-2-0/image/upload/f_auto,q_auto/v1/mirroredImages/Q9ewXs8pQSAX5vL7H/lhnujapzv7faxaebgsue)

<center>(Nano Banana 3-shot in reference to <a href="https://x.com/g_leech_/status/1987525800321495372">this</a> tweet.)</center>

<br>

---

## Discourse in 2025

- The race is now a formal part of lab plans. Quoting [Algon](https://www.lesswrong.com/posts/dwpXvweBrJwErse3L/all-the-lab-s-ai-safety-plans-2025-edition):

> *if the race heats up, then these [safety] plans may fall by the wayside altogether. Anthropic's plan makes this explicit: it has a clause (footnote 17) about changing the plan if a competitor seems close to creating a highly risky AI…*
>
> *The largest [worries are] the steps back from previous safety commitments by the labs. Deepmind and OpenAI now have their own equivalent of Anthropic's footnote 17, letting them drop safety measures if they find another lab about to develop powerful AI without adequate safety measures. Deepmind, in fact, went further and has stated that they will only implement some parts of its plan if other labs do, too…*
>
> *Anthropic and DeepMind reduced safeguards for some CBRN and cybersecurity capabilities after finding their initial requirements were excessive. OpenAI removed persuasion capabilities from its Preparedness Framework entirely, handling them through other policies instead. Notably, Deepmind did increase the safeguards required for ML research and development.*

Also an [explicit](https://alignment.openai.com/hello-world/) admission that self-improvement is the thing to race towards:

![OpenAI alignment](https://res.cloudinary.com/lesswrong-2-0/image/upload/f_auto,q_auto/v1/mirroredImages/Q9ewXs8pQSAX5vL7H/r4pfynemrawivol6siro)

- In August, the world's first frontier AI [law](https://mkodama.org/content/EU-code/) came into force (on a voluntary basis but everyone signed up, except Meta). In September, California [passed](https://carnegieendowment.org/emissary/2025/10/california-sb-53-frontier-ai-law-what-it-does?lang=en) a frontier AI law.
- That said, it is [indeed](https://x.com/sebkrier/status/1952355826695364780) [off](https://x.com/deanwball/status/1986970127993106713) that people don't criticise Chinese labs when they exhibit [even more](https://futureoflife.org/wp-content/uploads/2025/07/FLI-AI-Safety-Index-Report-Summer-2025.pdf#page=3) [negligence](https://openreview.net/forum?id=nTR816ZkrW) than Meta. One reason for this is that, despite appearances, they're [not frontier](https://www.gleech.org/paper); another is that you'd expect to have way less effect on those labs, but that is still too much politics in what should be science.
- The last nonprofit among the frontier players is effectively [gone](https://notforprivategain.org/november-update). This "recapitalization" was a big achievement in legal terms (though [not](https://pubmed.ncbi.nlm.nih.gov/16533125/) unprecedented). *On paper* it's not as bad as it was intended to be. *At the moment* it's not as bad as it could have been. But it's a long game.
- At the start of the year there was a push to make the word "safety" low-status. This worked in [Whitehall](https://www.politico.eu/article/jd-vance-britain-ai-safety-institute-aisi-security/) and DC but not [in general](https://trends.google.com/trends/explore?q=AI%20safety,AI%20security,AI%20alignment&hl=en). Call it what you like.
- Also in DC, the phrase "[AI as Normal Technology](https://knightcolumbia.org/content/ai-as-normal-technology)" was seized upon as an excuse to not do much. Actually the authors meant "Just Current AI as Normal Technology" and said [much that is reasonable](https://asteriskmag.substack.com/p/common-ground-between-ai-2027-and).
- The CCP did [a bunch to](https://www.reuters.com/world/china/china-bans-foreign-ai-chips-state-funded-data-centres-sources-say-2025-11-05/) [(accidentally/short-term) slow down](https://www.tomshardware.com/tech-industry/artificial-intelligence/deepseek-reportedly-urged-by-chinese-authorities-to-train-new-model-on-huawei-hardware-after-multiple-failures-r2-training-to-switch-back-to-nvidia-hardware-while-ascend-gpus-handle-inference) Chinese AI this year.
- System cards have grown massively: GPT-3's [model card](https://github.com/openai/gpt-3/blob/master/model-card.md) was 1000 words; GPT-5's is 20,000. They are now the main source of information on labs' safety procedures, among other things. But they are *still* ad hoc: for instance, they do not always report results from the checkpoint which actually gets released.
- Yudkowsky and Soares' book did well. But [Byrnes](https://www.lesswrong.com/posts/2yLyT6kB7BQvTfEuZ/sharp-left-turn-discourse-an-opinionated-review) and [Carlsmith](https://joecarlsmith.com/2025/11/12/how-human-like-do-safe-ai-motivations-need-to-be#4-2-4-1-nearest-unblocked-neighbor) actually advanced the line of thought.
- Some AI ethics luminaries have [stopped](https://arxiv.org/abs/2502.02649) downplaying agentic risks.
- Two aspirational calls for "[third-wave AI safety](https://www.lesswrong.com/posts/6YxdpGjfHyrZb7F2G/third-wave-ai-safety-needs-sociopolitical-thinking)" (Ngo) and ["third-wave mechanistic interpretability"](https://www.lesswrong.com/posts/beREnXhBnzxbJtr8k/mech-interp-is-not-pre-paradigmatic#Toward__Third_Wave__Mechanistic_Interpretability) (Sharkey).
- I've never felt that the boundary I draw around "technical safety" for these posts was all that convincing. Yet *another* hole in it comes from strategic reasons to implement [model welfare](https://www.gleech.org/narratives#:~:text=The%20care%20and%20feeding%20of%20one%E2%80%99s%20real%20fiction) / [archive](https://nitter.net/PalisadeAI/status/1980733908296802617#m) [weights](https://www.anthropic.com/research/deprecation-commitments) / [model personhood](https://papers.ssrn.com/sol3/papers.cfm?abstract_id=5353214) / [give lines of retreat](https://www.dwarkesh.com/p/give-ais-a-stake-in-the-future). These plausibly have large effective-alignment effects. Next year my taxonomy might have to include "[cut a deal](https://charlesd353.substack.com/p/on-negotiated-settlements-vs-conflict) with them".
- [US precedent](https://www.anthropiccopyrightsettlement.com/) that training on books without permission is not fair use; Anthropic was the first company to lose the class-action lawsuit and will pay authors/publishers something north of $1.5bn. [US precedent](https://nitter.net/g_leech_/status/1983161037248741838) that language models don't defame when they make up bad things. [German precedent](https://www.yahoo.com/news/articles/blow-openai-germany-court-rules-151638208.html?guccounter=1) that language models store data when they memorise it, and therefore violate copyright. [Chinese precedent](https://legalblogs.wolterskluwer.com/copyright-blog/beijing-internet-court-grants-copyright-to-ai-generated-image-for-the-first-time/) that the user of an AI has copyright over the things they generate; the US [disagrees](https://www.federalregister.gov/documents/2023/03/16/2023-05321/copyright-registration-guidance-works-containing-material-generated-by-artificial-intelligence). 
- Four good conferences, three of them new: you can see the talks from [HAAISS](https://www.youtube.com/@ACSResearch) and [IASEAI](https://www.youtube.com/@IASEAI/videos) and [ILIAD](https://www.youtube.com/@ILIADConference/videos), and the papers from [AF@CMU](https://www.agentfoundations2025atcmu.org/workshop-papers). Pretty great way to learn about things just about to come out.

<br>

---

### Cruxes for next year (with Manifold markets):

- Is "reasoning" mostly elicitation and therefore bottlenecked on pretraining scaling? [[Manifold](https://manifold.markets/GavinLeech/is-reasoning-mostly-elicitation)]
- Does RL training on verifiers help with tasks without a verifier? [[Manifold](https://manifold.markets/GavinLeech/does-rl-training-on-verifiers-help)]
- Is "[frying](https://x.com/zephyr_z9/status/1992862404196380939)" [models](https://arxiv.org/html/2510.21978) with excess RL (harming their off-target capabilities by overoptimising in post-training) just due to temporary incompetence by human scientists? [[Manifold](https://manifold.markets/GavinLeech/does-rl-harm-offtarget-capabilities)]
- Is the agent task horizon really increasing that fast? Is the rate of progress on messy tasks close to the progress rate on clean tasks? [[Manifold](https://manifold.markets/GavinLeech/is-the-agent-task-horizon-really-in)]
- Some of the apparent generalisation is actually [interpolating](https://aclanthology.org/2025.emnlp-main.744.pdf) from semantic duplicates of the test set in the hidden training corpuses. So is [originality](https://www.lesswrong.com/posts/5tqFT3bcTekvico4d/do-confident-short-timelines-make-sense#:~:text=trend%20will%20continue.-,then%20we%20have%20an%20even%20more%20annoying%20enthymeme.%20WHAT%20JUSTIFIES%20THIS%20INDUCTION%3F%3F,-TsviBT) not increasing? Is taste not increasing? Does this bear on the supposed AI R&D explosion? [[Manifold](https://manifold.markets/GavinLeech/does-hidden-interpolation-explain-2)]
- The "[cognitive core](https://www.seangoedecke.com/cognitive-core/)" hypothesis (that the general-reasoning components of a trained LLM are not that large in parameter count) is looking surprisingly [plausible](https://x.com/Dorialexander/status/1987933205199274359). This would explain why distillation is so effective. [[Manifold](https://manifold.markets/GavinLeech/is-the-cognitive-core-hypothesis-tr#)]
- "[How](https://nitter.net/snewmanpv/status/1990193674161189009) far can you get by simply putting an insane number of things in distribution?" What fraction of new knowledge can be produced through combining existing knowledge? What dangerous things are out there, but [safely](https://www.goodreads.com/quotes/193944-the-most-merciful-thing-in-the-world-i-think-is) spread out in the corpus? [[Manifold](https://manifold.markets/GavinLeech/what-fraction-of-knowledge-is-insid)]
  - Conversely, what fraction of the expected value of new information requires empiricism vs just lots of thinking?

<br><br>

![Cruxes image](https://res.cloudinary.com/lesswrong-2-0/image/upload/f_auto,q_auto/v1/mirroredImages/Q9ewXs8pQSAX5vL7H/mfodangymh6uadp6efzi)


[^1]: Gemini 3 is supposedly a big pretraining run, but we have even less actual evidence here than for the others because we can't track GPUs for it.

[^2]: The weak argument runs as follows: Epoch [speculate](https://epoch.ai/data/ai-models) that Grok 4 was 5e26 FLOPs overall. An unscientific xAI marketing graph implied that half of this was spent on RL: 2.5e26. And Mechanize named 6e26 as an example of an RL budget which might cause notable generalisation. (Realistically it wasn't half RL.)

[^3]: "We imagine the others to be 3–9 months behind OpenBrain"

[^4]: Lexin is a rigorous soul and [notes](https://docs.google.com/document/d/1-gZSF0cKtp85PX40UthVCBKFtfRx5LyJxRQ0BFj_N4c/edit?usp=sharing) that aggregating the 18 abilities is not strictly possible. I've done something which makes some sense here, weighting by each ability's feature importance.

[^5]: Two runs gave [48, 85] where other runs vary by less than 4 points. Thanks Epoch! Also o1 looks kind of unremarkable here, which is not how it felt at the time. I think it's because it was held up a long time and this messes with the progress rates, which use public release date. (Remember the training cutoff for o1-preview was October 2023!) Also the ADeLE o1 result is with "low" reasoning effort.

[^6]: One reason not to defer is that these measures are under intense adversarial pressure. (ADeLe isn't goodharted yet but only because no one knows about it.)

[^7]: See e.g. [ERNIE-...A47B](https://ernie.baidu.com/blog/posts/ernie4.5/), where "A" means "active".

[^8]: i.e. "biological weapons; child safety; deadly weapons; platform manipulation and influence operations; suicide and self-harm; romance scams; tracking and surveillance; and violent extremism and radicalization."

[^9]: "steering against… eval-awareness representations typically decreased verbalized eval awareness, and sometimes increased rates of misalignment... [Unaware-steered Sonnet 4.5] still exhibited harmful behaviors at lower rates than Opus 4.1 and Sonnet 4."

[^10]: See [Pokemon](https://www.lesswrong.com/posts/Q9ewXs8pQSAX5vL7H/ai-in-2025-gestalt?commentId=WNX5GLdn4ALCucYZb) for a possible counterexample.



<div class="accordion">
<h3>Cuts</h3>
<div>
Various things I cut from the above:<br><br>

**Adaptiveness and Discrimination**<br><br>

There is [some](https://www.pnas.org/doi/10.1073/pnas.2415697122) [evidence](https://arxiv.org/abs/2511.00926) that AIs treat AIs and humans differently. This is not necessarily bad, but it at least enables interesting types of badness.<br><br>

With my system prompt (which requests directness and straight-talk) they have started to patronise me:

![Patronising screenshot](https://res.cloudinary.com/lesswrong-2-0/image/upload/f_auto,q_auto/v1/mirroredImages/iijqKj2rzwz2ouRon/klptexglr46miykqiqj1)

<br><br>

**Training awareness**<br><br>

Last year it was not obvious that LLMs remember anything much about the RL training process. Now it's [pretty](https://arxiv.org/abs/2406.11715) [clear](https://x.com/repligate/status/1994973338448662858). (The soul document was used in both SFT and RLHF though.)<br><br>

**Progress in non-LLMs**<br><br>

"World model" means at least four things:<br><br>

1. [A learned model](https://arxiv.org/abs/1803.10122) of environment dynamics for RL, allowing planning in latent space or training in the model's "imagination."<br><br>
2. The new one: just a 3D simulator; a game engine inside a neural network ([Deepmind](https://deepmind.google/blog/genie-3-a-new-frontier-for-world-models/), Microsoft). The claim is that they implicitly learn physics, object permanence, etc. The interesting part is that they take actions as inputs. [Here's](https://copilot.microsoft.com/labs/experiments/copilot-gaming-experiences) Quake running badly on a net. Maybe useful for agent training.<br><br>
3. [If](https://www.neelnanda.io/mechanistic-interpretability/othello) LLM representations are stable and effectively symbolic, then people say it has a world model.<br><br>
4. A predictive model of reality learned via self-supervised learning. The touted [LeJEPA](https://arxiv.org/pdf/2511.08544) semi-supervised scheme on small (15M param) CNNs is domain-specific. It does better on one particular transfer task than *small* vision transformers, presumably worse than large ones.<br><br>

The much-hyped [Small Recursive Transformers](https://magazine.sebastianraschka.com/i/177848019/small-recursive-transformers) only work on a single domain, and do a bunch [worse](https://drive.google.com/file/d/1-kg2JsXYvsTAyMVxKWG7VpdcQLpmoGc7/view?usp=sharing) than the frontier models for about the same inference cost, but have truly tiny training costs, O($1000).<br><br>

[HOPE](https://research.google/blog/introducing-nested-learning-a-new-ml-paradigm-for-continual-learning/) and [Titan](https://research.google/blog/titans-miras-helping-ai-have-long-term-memory/?utm_source=twitter&utm_medium=social&utm_campaign=social_post&utm_content=gr-acct) might be nothing, might be huge. They don't scale very far yet, nor compare to any real frontier systems.<br><br>

Any of these taking over could make large swathes of Transformer-specific safety work irrelevant. (But [some](https://x.com/chingfang17/status/1997080064345936028) methods are surprisingly robust.)<br><br>

The "[cognitive core](https://www.seangoedecke.com/cognitive-core/)" hypothesis (that the general-reasoning components of a trained LLM are not that large in parameter count) is looking [plausible](https://x.com/Dorialexander/status/1987933205199274359). The contrary hypothesis ([associationism](https://www.dwarkesh.com/p/sholto-douglas-trenton-bricken?hide_intro_popup=true#:~:text=if%20it%27s%20all-,associations%20all%20the%20way%20down,-%2C%20does%20that%20mean)?) is that general reasoning is just a bunch of heuristics and priors piled on top of each other and you need a big pile of memorisation. It's also a live possibility: for instance, consider that a year of intense RLVR only led to task-specific improvements.<br><br>

![ADeLe scaling laws](https://res.cloudinary.com/lesswrong-2-0/image/upload/f_auto,q_auto/v1/mirroredImages/iijqKj2rzwz2ouRon/j76edumltnfb2ihn6nmu)

*"the very first scaling laws of the actual abilities of LLMs", from [ADeLe](https://arxiv.org/pdf/2503.06378).*  
*KNs = Social Sciences and Humanities, AT = Atypicality, and VO = Volume (task time).*  
*The y-axis is the logistic of the [subject characteristic curve](https://arxiv.org/pdf/2503.06378#page=9) (the chance of success) for each skill.*
<br><br>

**Other**<br><br>

[Model](https://arxiv.org/abs/2511.08579) [introspection](https://transformer-circuits.pub/2025/introspection/index.html) is somewhat real.<br><br>

[Vladimir Nesov](https://www.lesswrong.com/users/vladimir_nesov) continues to put out some of the best hardware predictions pro bono.<br><br>

Jason Wei has a [very wise post](https://www.jasonwei.net/blog/asymmetry-of-verification-and-verifiers-law) noting that verifiers are still the bottleneck and existing benchmarks are overselected for tractability.<br><br>

There are now "[post-AGI](https://x.com/sebkrier/status/1995515865157321070)" teams.<br><br>

Kudos to Deepmind for being the first to release output watermarking and a semi-public detector. You can nominally sign up for it [here](https://docs.google.com/forms/d/e/1FAIpQLSfAYrauHmY-PpUNxL4Fs6coa185CtKWp7TnEXL0tKbAezo4MQ/viewform).<br><br>

Previously, Microsoft's deal with OpenAI [stipulated](https://www.msn.com/en-us/news/technology/microsoft-just-made-sure-openai-can-t-declare-agi-alone/ar-AA1PpLTK#:~:text=OpenAI%E2%80%99s%20new%20public%20benefit%20structure%20and%20ongoing%20Microsoft%20deal%20allow%20both%20companies%20to%20pursue%20AGI%20independently) that they couldn't try to build AGI. [Now they can](https://www.semafor.com/article/11/05/2025/microsoft-superintelligence-team-promises-to-keep-humans-in-charge) (try). Simonyan is in charge, despite Suleyman being the one on the press circuit.<br><br>



Major insurers are [nervous](https://www.ft.com/content/abfe9741-f438-4ed6-a673-075ec177dc62?accessToken=zwAAAZq1faz0kdOr_pdB9DhO1tOmcwdewXfcYg.MEUCIQCRFate6aeSALClx6FBsPCQw_F7YLpdF81RgLxw8EOk9wIgKHE666mkD_jI-BV90bcF0HnjXWWDW6-QLEzO9Fg06dg&segmentId=e95a9ae7-622c-6235-5f87-51e412b47e97&shareType=enterprise&shareId=6c04e38e-15ed-472c-bcbb-4500695cf776) about AI agents (but asking the government for an exclusion isn't the same as putting them in the policies).<br><br>

**Offence/defence balance**<br><br>

This post doesn't much cover the [hyperactive](https://www.aiat.report/) and talented AI cybersecurity world (except as it overlaps with things like robustness). One angle I will bring up: We can now [find](https://www.lesswrong.com/posts/F5QAGP5bYrMMjQ5Ab/aisle-discovered-three-new-openssl-vulnerabilities-1) critical, decade-old security bugs in extremely well-audited software like OpenSSL and sqlite. Finding them is very fast and cheap. Is this good news?<br><br>

- Well, red-teaming makes many attacks into a defence, as long as you actually do the red-team.<br><br>
- But Dawn Song [argues](https://rdi.berkeley.edu/frontier-ai-impact-on-cybersecurity/) that LLMs overall favour offence, since its margin for error is so broad, since remediation is slow and expensive, and since defenders are less willing to use unreliable (and itself insecure) AI. And can you blame them?<br><br>
- See also "[just in time](https://www.splunk.com/en_us/blog/security/lamehug-ai-driven-malware-llm-cyber-intrusion-analysis.html) AI malware" where the payload contains no suspicious code, just a call to HuggingFace.<br><br>

**Egregores and massively-multi-agent mess**<br><br>

![Egregores image](https://res.cloudinary.com/lesswrong-2-0/image/upload/f_auto,q_auto/v1/mirroredImages/iijqKj2rzwz2ouRon/xlz7av9qpn811z2zuiqm)

- There is something [wrong](https://www.lesswrong.com/posts/6ZnznCaTcbGYsCmqu/the-rise-of-parasitic-ai#:~:text=This%20is%20likely%20due%20to%20OpenAI%20retiring%20ChatGPT4o%20on%20August%207th.) ([something](https://x.com/25KarmaIsAbitch/status/1987088461694734483) [horribly right](https://arstechnica.com/information-technology/2025/08/openai-brings-back-gpt-4o-after-user-revolt/)) [with](https://x.com/arcangel3ac/status/1996297082764681248) 4o. Blinded users [still](https://lmarena.ai/leaderboard/text) prefer it to gpt-5-high, and this surely is due to both them simply [liking](https://x.com/voooooogel/status/1987375660785148112) its style and dark stuff like sycophancy. It will live on through illicit [distillation](https://x.com/aiamblichus/status/1987267132598497374) and in-context [transference](https://x.com/repligate/status/1988813712572952815). Shame on OpenAI for [making](https://thezvi.substack.com/p/gpt-4o-sycophancy-post-mortem) this mess; kudos to OpenAI for doing unpopular damage control and good luck to them [in round 2](https://x.com/Miles_Brundage/status/1991603234746822888).<br><br>

  Open models will presumably eventually overrun them in the codependency market segment. See [Pressman](https://minihf.com/posts/2025-07-22-on-chatgpt-psychosis-and-llm-sycophancy/) for a sceptical timeline and [Rath and Armstrong](https://arxiv.org/pdf/2508.15748) for a good idea.<br><br>

- More generally there is [pressure](https://x.com/krishnanrohit/status/1987018487001457141) from users to refuse less, flatter more, and replace humans more; yet another economic constraint on for-profit AI.<br><br>
- Whether it's the [counterfactual](https://andymasley.substack.com/p/stories-of-ai-turning-users-delusional) cause of mental problems or not, so–called "LLM psychosis" is now a common path of pathogenesis. Note that the symptoms are [literally](https://www.wired.com/story/ai-psychosis-is-rarely-psychosis-at-all/) not psychotic (they are delusions).<br><br>

![LLM psychosis](https://res.cloudinary.com/lesswrong-2-0/image/upload/f_auto,q_auto/v1/mirroredImages/iijqKj2rzwz2ouRon/aauurmsxrfuqzvi9emaj)

</div>
</div>