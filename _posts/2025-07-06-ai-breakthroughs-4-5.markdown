---
layout:     post
title:      "AI breakthroughs 2024-2025"
baselink:   /ai-24-25
permalink:  /ai-24-25
date:       2025-10-20
author:     Gavin
img:        /img/hogan.jpg

visible:    1
published:  true
quality:    3

summary:    
confidence: 80%
importance: 8
wordcount:  
categories: 
where:      "Trajan"
---


So what just happened?



## 2024

* _In January_, the best systems were around chance (30%) on GPQA (hard science). By November, 4o was getting "around PhD" (60%) on the hardest Diamond set.

* _Feb_: first "million-token" context window (Gemini 1.5), but there's a steep fall in performance as you go through it. 

* _Feb_: first proper text+audio+image model (Gemini 1.5)

* _Feb_: first good video generation (Sora)

* _May_: first good voice interface (Advanced Voice Mode)

* _Jul_: silver medal at the IMO (AlphaProof) but with a Frankenstein hybrid system.

* _🚨 Sep_: RL works on LLMs at last. So-called “reasoning” (o1-preview)

* _Oct_: GPT-4 "Search" agent. Works better than modern Google.

* _Oct_: So-called “agency” (Anthropic Computer Use). Supposedly a 15 min human-task horizon but really a much shorter for usable horizon. 
	* 🚨 Whatever [METR's time horizon](https://epoch.ai/benchmarks/metr-time-horizons) is measuring admittedly multiplied by a factor of 7 over the year.

* _Nov_: the Model Context Protocol standardises the interface between agents

* 🚨 Over the year, the best systems jumped from 5% to 50% on SWE-Bench Verified (real coding tasks).

* _Dec_: the first LLM that handles streaming video input (Gemini Live)

* _Dec_: Gemini Deep Research is the first strong agent (for lit-reviews) but no one notices.

<br>

Sheesh.

But after about 3 months we got used to the hard benchmark values being 70% or 80% instead of 10% or 20%. It meant less than we thought it would.

## 2025

* ¯\\_(ツ)\_/¯ _Jan_: Deepseek R1 hype. But the apparent 10x per-token saving is a [false economy](https://www.gleech.org/paper#tokenomics-no-effective-discount).

* _🚨 Jan, recursive improvement_: Sometime around here, Deepmind uses AlphaEvolve (Gemini 2.0) to write GPU kernels and speed up the training of Gemini 2.5 by "1%". (2024 model.)

* _Feb_: Claude Code. The ~end of brittle stupid RAG. Lab incursion into the application layer. 

* _Mar_: [Autoregressive image generation](https://cdn.openai.com/11998be9-5319-4302-bfbf-1167e093f1fb/Native_Image_Generation_System_Card.pdf) surpasses(?) diffusion models.

* _🚨 Apr_: o3 is the first LLM ever that is actually worth using for me

* _May_: The first video generator which creates synched audio (Veo 3). Makes the psychological effect 100x stronger.

* ¯\\_(ツ)\_/¯ In _June 2024_, 4o got 5% on ARC-AGI. By Apr 2025 o4-mini got 41%. Also for the first time you can convert $3m _per-run_ into 80% on it. (Human is “98%”)

* _🚨 Jul_: IMO gold by an LLM. Reportedly no tools and no neuralese involved. ("Gemini 2.5 Deep Think Advanced" + unnamed experimental OpenAI model)

* _Sep_: Various [groups](https://www.nature.com/articles/s41586-025-09298-z) [racing headlong](https://arcinstitute.org/news/hie-king-first-synthetic-phage) into "frontier biology" with protein language models and such.

* _Nov_: Gemini 3. Notable improvement in vision and image generation. [Rest](https://nitter.net/stuhlmueller/status/1991546706371178781#m) [is a](https://nitter.net/ArtificialAnlys/status/1990926803087892506#m) [very](https://www.lesswrong.com/posts/8uKQyjrAgCcWpfmcs/gemini-3-is-evaluation-paranoid-and-contaminated) [mixed](https://nitter.net/peterwildeford/status/1990830603239842080#m) [bag](https://x.com/Miles_Brundage/status/1991664747914358861). Benchmaxxed, or rather narrow-objective-maxxed.

* ¯\\_(ツ)\_/¯ _Nov_: claims about Gemini 3 adding a continual learning mode (based on the little [HOPE](https://research.google/blog/introducing-nested-learning-a-new-ml-paradigm-for-continual-learning/) experiment).

* Over the year, progress from 50% to 77% on SWE-Bench-Verified. But you can't just use this ("+27% is less than the +45% last year") to say slower latent progress, since obviously the tasks solved this year were much harder.

* Over the year, whatever HCAST is measuring multiplied by 5x this year (vs 7x last year).

* Browser agents aren't adopted by anyone except Tyler really.

* [Various noises](https://nitter.net/NikoMcCarty/status/1986501362730082464) about AI scientists. Mostly rediscovery and recombination?

* [Real progress](https://nitter.net/g_leech_/status/1974165458283860198) in AI assistance for research mathematics.

* There will be things which I miss / which only reveal themselves as significant next year.

<br> 

No earthquakes this year (the big scaling hopes, GPT-4.5 and Grok 4 both disappointed their masters) but still prettty fast. 


This list is biased towards discrete changes. While the story of AI does have step changes (like GPT-2) and may have more coming, you should do plenty of staring at smooth graphs as well. A lot of the 2024 breakthroughs above were more like progress in UIs or products than something fundamental (though getting multimodal to work was once the definition of fundamental). 

<br>



PS: That title should be "LLM breakthroughs", sorry; [see](https://nitter.net/g_leech_/status/1864349307345731614#m) [here](https://docs.google.com/spreadsheets/d/1A5r83AfPwF1_sljzbz12OuZBysjip5F6WiTnMI3IxBA/edit?usp=sharing) for other AI.