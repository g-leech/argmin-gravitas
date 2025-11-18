---
layout:     post
title:      "Paper AI Tigers"
baselink:   /paper
permalink:  /paper
date:       2025-11-16
author:     Gavin
img:        /img/1950-08-Paper_Tiger.png

visible:    1
published:  true
quality:    7

summary:    Making sense of current Chinese AI 
confidence: 70%
importance: 7
wordcount:  
warnings:   "Obsolete by March 2026.<br>&nbsp;&nbsp;&nbsp;&nbsp;CoIs: I've been a contractor for the big Western labs in the past and am long Google."
categories: AI, hypothesis-dump
where:      "Bristol"
---


<!-- https://kr-asia.com/chinas-ai-tigers-return-to-the-ring-as-the-foundation-model-race-reignites -->

The best Chinese LLMs offer<br><br>
<!--  -->
1. [frontier](https://moonshotai.github.io/Kimi-K2/thinking.html) performance on some benchmarks;
1. massive per-token discounts (~3x on input, ~6x on output);
1. _the weights_. On-prem with fully free ~MIT licence, self-hosting, white-box access, customisation, with zero markup (and in fact zero revenue going to the Chinese companies);
1. with a bit of work you _can_ get much faster token speeds than the closed APIs;
1. [less](https://arxiv.org/pdf/2405.20947) overrefusal (except on CCP talking points);
1. on topics controversial in the West, [less](https://speechmap.ai/models/) nannying.
1. they just added the [search agents](https://x.com/g_leech_/status/1987525800321495372) that make daily use actually worthwhile. They also let you see the real CoT!
1. They're the [most-downloaded](https://www.atomproject.ai/) open models.
<!--  --><br><br>

As a result, going off private information, money man Martin Casado [says](https://x.com/martin_casado/status/1990462245541982546) "16-24%" of the (American) startups he meets are now using Chinese models. Among the few Westerners to admit it is [Airbnb](https://finance.yahoo.com/news/airbnb-picks-alibabas-qwen-over-093000045.html) (Qwen). But Windsurf's planner is [probably GLM](https://x.com/zai_org/status/1984076614951420273) and Cursor's planner [may](https://x.com/nrehiew_/status/1984642215671746631) be DeepSeek.
<!--  -->
### And yet 
<!--  -->
<ol start=9>
	<li>outside China, they are mostly not used, even by the cognoscenti. Not a great metric, but the one I've got: all Chinese models combined are currently at <a href="https://openrouter.ai/rankings?view=day#market-share">19%</a> on the <i>highly selected</i> group of people who use OpenRouter. More interestingly, over 2025 they trended downwards there. And of course in the browser and mobile they're probably <<10% of global use;</li>
	<li>they are severely <a href="https://www.scmp.com/tech/big-tech/article/3310656/chinas-lack-advanced-chips-hinders-broad-adoption-ai-models-tencent-executive">compute</a>-<a href="https://epoch.ai/gradient-updates/why-china-isnt-about-to-leap-ahead-of-the-west-on-compute">constrained</a> (and as of November 2025 their <a href="https://epoch.ai/gradient-updates/algorithmic-progress-likely-spurs-more-spending-on-compute-not-less#:~:text=While%20this%20achievement,as%20earlier%20models.">algorithmic advantage</a> is unclear), so this implies they actually can't have matched American models;</li>
	<li>they're aggressively quantizing at inference-time, 32 bits to 4;</li>
<!-- 1. (the exception is a [thin Claude wrapper](https://gist.github.com/jlia0/db0a9695b3ca7609c9b1a08dcbf872c9)) -->
	<li>state-sponsored Chinese hackers <a href="https://assets.anthropic.com/m/ec212e6566a0d47/original/Disrupting-the-first-reported-AI-orchestrated-cyber-espionage-campaign.pdf">used</a> closed American models for incredibly sensitive operations, giving the Americans a full whitebox log of the attack!</li>
</ol>


What gives?

<br>

<div class="accordion">
	<h3>"Tigers"?</h3>
	<div>
		The title alludes to the "<a href="https://qz.com/china-six-tigers-ai-startup-zhipu-moonshot-minimax-01ai-1851768509">6 AI Tigers</a>" named in business rags as DeepSeek, Moonshot, Z.ai, MiniMax, StepFun, and 01.ai. (This is because they're trying to hype startups specifically; the conglomerates Alibaba and Baidu are <i>way</i> more relevant than the latter two.)
	</div>
	<h3>Filtered evidence</h3>
	<div>
	The evidence is dreadful because everyone has a horse in the race and (in public) are letting it lead them:
	<ul>
		<li>Static evals are weak evidence even when they're not being adversarially hacked and hill-climbed.</li>
		<li><a href="https://x.com/nealkhosla/status/1882859736737194183">Some</a> <a href="https://x.com/kimmonismus/status/1882824571281436713">Americans</a> are downplaying the Chinese models out of cope.</li>
		<li><a href="https://marginalrevolution.com/marginalrevolution/2025/03/the-political-economy-of-manus-ai.html">Some</a> <a href="https://www.interconnects.ai/p/chinas-top-19-open-model-labs">Americans</a> <a href="https://x.com/novagrace777/status/1984538687020105882">are</a> hyping the Chinese models to suppress domestic AI regulation.</li>
		<li><a href="https://www.darioamodei.com/post/on-deepseek-and-export-controls">Some</a> Americans are hyping the Chinese models to boost international AI regulation.</li>
		<li>The Chinese are obviously talking their book.</li>
	</ul>
	</div>
</div>

<br>



## What could explain this?

### Maybe the evals are misleading?

> 1\. frontier performance on some benchmarks

The [naive view](https://artificialanalysis.ai/) - the benchmark view - is that they're very close in "intelligence":

<br>
<center>
<img width="80%" src="/img/supposed-parity.jpg" />
</center>

But these benchmarks are not strong evidence about performance on new inputs or the latent (general and unobserved) capabilities. It'd be natural to read "89%" success on a maths benchmark as meaning an 89% probability that it would correctly handle unseen questions of that difficulty in that domain (and indeed this is what [cross-validation](https://en.wikipedia.org/wiki/Empirical_risk_minimization) was originally designed to estimate). But in the kitchen-sink era of AI, where every system has seen a large proportion of all data ever digitised, and so has already seen some variant of many of _all possible_ new questions, you can't read it that way.

In fact it's not even an 89% probability of answering these same questions right again, as shown by the fact that [people](https://moonshotai.github.io/Kimi-K2/) report the results as "avg@64" (the average performance if you ask the same question 64 times).

Aside: **Test sets which are on the internet are not test sets.**

There are [dozens of ways](https://arxiv.org/abs/2407.12220) to screw up or hack these numbers. I'll only look at a couple here but I welcome someone doing something more systematic.

<br>
<big>Even less generalisation?</big>

Maybe Chinese models generalise to unseen tasks less well. (For instance, when tested on fresh data, 01's Yi model [fell 8pp](https://arxiv.org/pdf/2405.00332) (25%) on GSM - the biggest drop amongst all models.)<br><br>We can get a dirty estimate of this by the "shrinkage gap": look at how a model performs on next year's iteration of some task, compared to this year's. If it finished training in 2024, then it can't have trained on the version released in 2025, so we get to see what they're like on at least somewhat novel tasks. We'll use two versions of the same benchmark to keep the difficulty roughly on par. [Let's try AIME](https://colab.research.google.com/drive/1EJ5hM314lOAiX3ayLoU5V-sPoJNuXTNt?usp=sharing):<br><br>
<!--  -->
<style>    
    .container {
        background: white;
        border-radius: 12px;
        box-shadow: 0 20px 30px rgba(0, 0, 0, 0.3);
        overflow: hidden;
        max-width: 900px;
        width: 100%;
    }
    
    .container > h1 {
        background: linear-gradient(135deg, #006800 0%, #62b562 100%);
        color: white;
        padding: 24px;
        font-size: 24px;
        font-weight: 600;
        text-align: center;
    }
    
    .table-wrapper {
        overflow-x: auto;
    }
    
    table {
        width: 100%;
        border-collapse: collapse;
        font-size: 14px;
    }
    
    thead {
        background: #f8f9fa;
        position: sticky;
        top: 0;
    }
    
    th {
        padding-right: 2em !important;
        text-align: right;
        font-weight: 600;
        color: #2d3748;
        border-bottom: 2px solid #e2e8f0;
        border-style: none !important;
        font-size: 12px;
        letter-spacing: 0.5px;
    }
    
    th:first-child {
        text-align: center;
    }
    
    td {
        padding-right: 2em !important;
        text-align: right;
        border-bottom: 1px solid #f1f3f5;
        border-style: none !important;
        color: #4a5568;
    }
    
    td:first-child {
        text-align: center;
        font-weight: 500;
        color: #2d3748;
    }
    
    tbody tr {
        transition: background-color 0.2s ease;
    }
    
    tbody tr:hover {
        background-color: #f7fafc;
    }
    
    tbody tr.average {
        background: linear-gradient(135deg, rgba(102, 126, 234, 0.1) 0%, rgba(118, 75, 162, 0.1) 100%);
        font-weight: 600;
        /*border-top: 2px solid #667eea;*/
    }
    
    tbody tr.average:hover {
        background: linear-gradient(135deg, rgba(102, 126, 234, 0.15) 0%, rgba(118, 75, 162, 0.15) 100%);
    }
    
    tbody tr.average td {
        color: #667eea;
        border-bottom: 2px solid #e2e8f0;
    }
    
    tbody tr.average td:first-child {
        color: #667eea;
    }

    .section-divider {
      border-bottom: 3px solid #667eea;
    }
    
    .negative {
        color: #48bb78;
    }
    
    .high-fall {
        color: #e53e3e;
    }

    th, td {
           margin-right: 10px;
    }
    
    @media (max-width: 768px) {
        h1 {
            font-size: 20px;
            padding: 16px;
        }
        
        th, td {
            padding: 10px 8px;
            font-size: 12px;
        }
    }
</style>
<div class="container">
    <h1>AIME 2024 vs 2025 Model Performance<br>(using the <a href="https://artificialanalysis.ai/">Artificial Analysis</a> harness)</h1>
    <div class="table-wrapper">
        <table>
            <thead>
                <tr>
                    <th>Model</th>
                    <th><a href="https://web.archive.org/web/20250000000000*/https://artificialanalysis.ai/evaluations/aime-2024">AIME 2024</a></th>
                    <th><a href="https://artificialanalysis.ai/evaluations/aime-2025">AIME 2025</a></th>
                    <th>pp fall</th>
                    <th>% fall</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td>Kimi K2</td>
                    <td>69.3</td>
                    <td>57.0</td>
                    <td>-12.3</td>
                    <td>-17.7</td>
                </tr>
                <tr>
                    <td>MiniMax-M1 80k</td>
                    <td>84.7</td>
                    <td>61.0</td>
                    <td>-23.7</td>
                    <td>-28.0</td>
                </tr>
                <tr>
                    <td>DeepSeek-v3</td>
                    <td>39.2</td>
                    <td>26.0</td>
                    <td>-13.2</td>
                    <td>-33.7</td>
                </tr>
                <tr>
                    <td>DeepSeek V3 0324</td>
                    <td>52.0</td>
                    <td>41.0</td>
                    <td>-11.0</td>
                    <td>-21.2</td>
                </tr>
                <tr>
                    <td>Qwen3 235B (Reasoning)</td>
                    <td>84.0</td>
                    <td>82.0</td>
                    <td>-2.0</td>
                    <td>-2.4</td>
                </tr>
                <tr>
                    <td>Kimi K2-Instruct</td>
                    <td>69.6</td>
                    <td>49.5</td>
                    <td>-20.1</td>
                    <td>-28.9</td>
                </tr>
                <tr>
                    <td>DeepSeek R1 0528</td>
                    <td>89.3</td>
                    <td>76.0</td>
                    <td>-13.3</td>
                    <td>-14.9</td>
                </tr>
                <tr class="section-divider">
                		<td></td>
                    <td></td>
                    <td></td>
                    <td></td>
                    <td></td>
                </tr>
                <tr class="section-divider average">
                    <td><b>Chinese models</b></td>
                    <td></td>
                    <td></td>
                    <td>-13.7pp</td>
                    <td>-21%</td>
                </tr>
                <tr>
                		<td></td>
                    <td></td>
                    <td></td>
                    <td></td>
                    <td></td>
                </tr>
                <tr>
                		<td></td>
                    <td></td>
                    <td></td>
                    <td></td>
                    <td></td>
                </tr>
                <tr>
                    <td>Gemini-2.5 Pro</td>
                    <td>88.7</td>
                    <td>87.7</td>
                    <td>-1.0</td>
                    <td>-1.1</td>
                </tr>
                <tr>
                    <td>Gemini 2.5 Flash (Reasoning)</td>
                    <td>82.3</td>
                    <td>73.3</td>
                    <td>-9.0</td>
                    <td>-10.9</td>
                </tr>
                <tr>
                    <td>Claude 4 Opus Thinking</td>
                    <td>75.7</td>
                    <td>73.3</td>
                    <td>-2.4</td>
                    <td>-3.2</td>
                </tr>
                <tr>
                    <td>o4-mini (high)</td>
                    <td>94.0</td>
                    <td>90.7</td>
                    <td>-3.3</td>
                    <td>-3.5</td>
                </tr>
                <tr>
                    <td>GPT-4.1</td>
                    <td>43.7</td>
                    <td>34.7</td>
                    <td>-9.0</td>
                    <td>-20.6</td>
                </tr>
                <tr>
                    <td>Nova Premier</td>
                    <td>17.0</td>
                    <td>17.3</td>
                    <td class="negative">0.3</td>
                    <td class="negative">1.8</td>
                </tr>
                <tr>
                    <td>GPT-4o Nov 24</td>
                    <td>15.0</td>
                    <td>6.0</td>
                    <td>-9.0</td>
                    <td class="high-fall">-60.0</td>
                </tr>
                <tr>
                    <td>Magistral Medium</td>
                    <td>73.6</td>
                    <td>64.9</td>
                    <td>-8.7</td>
                    <td>-11.8</td>
                </tr>
                <tr>
                    <td>Claude 3.7 Sonnet</td>
                    <td>61.3</td>
                    <td>56.3</td>
                    <td>-5.0</td>
                    <td>-8.2</td>
                </tr>
                <tr>
                    <td>OpenAI-o1-0912</td>
                    <td>74.4</td>
                    <td>71.5</td>
                    <td>-2.9</td>
                    <td>-3.9</td>
                </tr>
                <tr>
                    <td>o3</td>
                    <td>90.3</td>
                    <td>88.3</td>
                    <td>-2.0</td>
                    <td>-2.2</td>
                </tr>
                <tr>
                    <td>Grok 4</td>
                    <td>94.3</td>
                    <td>92.7</td>
                    <td>-1.6</td>
                    <td>-1.7</td>
                </tr>
                <tr  class="section-divider">
                    <td></td>
                    <td></td>
                    <td></td>
                    <td></td>
                    <td></td>
                </tr>
                <tr class="section-divider average">
                    <td><b>Western models</b></td>
                    <td></td>
                    <td></td>
                    <td>-4.5pp</td>
                    <td>-10.4%</td>
                </tr>
                  <tr>
                    <td></td>
                    <td></td>
                    <td></td>
                    <td></td>
                    <td></td>
                </tr>
                <tr>
                    <td></td>
                    <td></td>
                    <td></td>
                    <td></td>
                    <td></td>
                </tr>
                <tr class="average">
                    <td>Overall average</td>
                    <td>68.3</td>
                    <td>60.5</td>
                    <td>-7.9</td>
                    <td>-14.3</td>
                </tr>
            </tbody>
        </table>
    </div>
</div>
<br>
Almost all models get worse on this new benchmark, despite 2025 being the same difficulty as 2024 (for humans). But as I expected, Western models drop less: they lost 10% of their performance on the new data, while Chinese models dropped 21%. p = 0.09.<br><br>
Averaging across crappy models for the sake of a cultural generalisation doesn't make sense. Luckily, rerunning the analysis with just the top models gives roughly the same result (9% gap instead of 11%).<br><br>
One way for generalisation to fail despite apparently strong eval performance is _contamination_, training on the test set. But (despite the suggestive timing) the above isn't strong evidence that that's what happened. It just tells us that Kimi and MiniMax and DeepSeek generalise worse on this task; it doesn't tell us why.


<br>
<div class="accordion">
	<h3>Details</h3>
	<div>
		Here's a <a href="https://colab.research.google.com/drive/1EJ5hM314lOAiX3ayLoU5V-sPoJNuXTNt?usp=sharing">Colab</a> with everything except the actual execution of my silly manual Kimi 1.5 run.<br><br>
		<!--  -->
		First, test for an obvious confounder: check if the 2025 AIME exam was around as hard as 2024's (answer: yes; in fact humans did 4% better in 2025). (TODO: check if 2025 had more combinatorics, which AI struggles with.)<br><br>
		(To be strict we should limit this to models which finished training before 12th February 2025, when the questions were released. But, as you see, we don't need to, it's a very clear result anyway.)<br><br>
        Selection criteria:<br><br>
        <ol>
            <li>Models tested on both AIME 2024 and 2025 with the same weights and harness</li>
            <li>Ideally finished training by 15th February 2025</li>
            <li>Subanalyses can handle filtering to the most relevant models (the frontier in each group)</li>
        </ol>
        <br>
		ML results are too sensitive to eval harnesses to use just one setting. Luckily I found four comparisons of AIME 2024 and AIME 2025 by different groups, <a href="https://artificialanalysis.ai/evaluations/aime-2025">Artificial</a> <a href="https://web.archive.org/web/20250723015603/https://artificialanalysis.ai/evaluations/aime-2024">Analysis</a>, the <a href="https://arxiv.org/pdf/2506.10947">Zettlemoyer Lab</a>, <a href="https://github.com/GAIR-NLP/AIME-Preview">GAIR</a>, and <a href="https://www.vals.ai/benchmarks/aime">Vals</a>. AA is the one in the table above.<br><br>
        <div class="accordion">
        <h3>Qwen 2.5</h3>
        <div>
        Qwen3 seems clean on this benchmark, but <a href="https://www.interconnects.ai/p/reinforcement-learning-with-random">multiple lines</a> show that Qwen 2.5 trained on test (or at least rephrased test data and then trained on it). We know this because random rewards work on it nearly as well as correct rewards. This adds no information by definition, so the model must have already known the answers. "<i>Intriguingly, we find that any AIME24 gains achievable from training Qwen models with spurious rewards largely vanish when evaluating on AIME 2025.</i>" Taking the max performance of the random reward curve, they fall 88% [75%, 100%].<br><br>
        <!--  -->
        Even more damning, <a href="https://arxiv.org/pdf/2507.10532v1#page=2">when</a> you give Qwen2.5-7B the first 40% of a MATH-500 test problem, it can reproduce the remaining 60% of the question word-for-word (with 41.2% accuracy). Llama3.1-8B fails at this completely (2%). 
        <!--  -->
        </div>
        </div>
        <br><br>
        <!--  -->
		<!--  -->
		How did our replications do? As expected, the shrinkage gap varies a lot by harness and by model choice: <br>
		<ul>
            <li>
            <a href="https://arxiv.org/pdf/2506.10947">UoW-Zettlemoyer</a>: Qwen2.5-7B and Qwen2.5-Math fall 12.5pp (88%); the Western LLaMA and OLMo models were too weak to really say.</li>
            <a href="https://github.com/GAIR-NLP/AIME-Preview">GAIR</a>: Chinese -19.4%, Western -15.6%.</li><br>
            <li><a href="https://www.vals.ai/benchmarks/aime">Vals</a> actually get no gap: -11.2% vs -10.8%. If you kick Meta out the gap goes up to 2%, still not much.</li>
        </ul>
		I'm not worried about these contradictory results; they both just include a lot of bad models and so noise. (I don't actually care how Llama 4 Scout's generalisation compares to QwQ-uwu-435B-A72B-destruct-dpo-ppo-grpo-orpo-kto-slerp-v3.5-beta2-chat-instruct-base-420-blazeit-early-stopped-for-vibes.) GAIR is also underelicited. <br><br>
		<!--  -->		
		<!--  -->
		(Actually AIME's a funny choice of benchmark given that 2025 had <a href="https://x.com/DimitrisPapail/status/1888325914603516214">a bunch</a> of semantic duplicates from before the cutoff. But that just makes the above a lower bound on the fall in performance.)<br><br>
		A win for Qwen and a huge relative win for Amazon!<br><br>
<!--  -->
		Claude is adorably confused about this. I didn't even ask it for this analysis:<br><br>
<!--  -->
		<img src="/img/adorable.jpg" /><br><br>
		TODO: Another way to get past goodharting pressure is to look at hard but obscure evals which no one ever reports / which manage to keep the test set private. e.g. <a href="https://x.com/teortaxesTex/status/1988932008693964845">PROOFGRID</a>.
        <br><br>
    <div class="accordion">
    <h3>Kimi 1.5</h3>
    <div>
        I really wanted to include Kimi 1.5, because it finished training just around the time AIME 2025 came out. But it turns out they never actually released the weights and it's been removed from the API!<br><br>
        Because I have that dawg in me, I decided to manually evaluate it in the last place I can, the <a href="https://www.kimi.com/">goddamn browser chat</a>. This is suboptimal in many ways (no control over temperature, max tokens, etc) but I can do it for both and hopefully the fall is proportional. The usual practice is to repeat 8 or 64 times, but I have patience enough for 2.<br><br>
        I used the Mistral prompt:<br>
        <code>
            Solve this AIME mathematical problem step by step.
<!--  -->
            Problem: {}
<!--  -->
            Think through this carefully and provide your final answer as a 3-digit integer (000-999).
            <!--  -->
            End with: "Therefore, the answer is [your answer]."
        </code>
        <br><br>
        Results:
        <br>
        <table>
            <tr>
                <th></th>
                <th>AIME 2024 acc</th>
                <th>AIME 2025 acc</th>
                <th>abs fall (pp)</th>
                <th>rel fall (%)</th>
            </tr>
            <tr>
                <td>Kimi 1.5 (browser)</td>
                <td>18.3 [23.3, 13,3]</td>
                <td>15.0 [13.3, 16.6]</td>
                <td>-3.3</td>
                <td>-18%</td>
            </tr>
        </table>
        <br>
        We can actually see how much weaker this (null) harness and provider is at eliciting performance by comparing to their reported results. Their short-CoT result for AIME 2024 was <a href="https://arxiv.org/pdf/2501.12599">60.8%</a>.<br><br>
        For obvious reasons I'm not including this in the main analysis but it's another example.
        </div>
    </div>
	</div>
</div>
<br>

<big>'Hacking</big>

Another way to be misleading is to [volkswagen](https://en.wikipedia.org/wiki/Volkswagen_emissions_scandal) it: put special and unrepresentative effort in during testing, "[hacking](https://arxiv.org/abs/2407.12220)". e.g. Kimi's benchmarks come from "Heavy mode" (8 parallel instances with an aggregation instance on top). You can't do this via the API or out of the box with the weights. (Could you say the same for OpenAI?)

Or you can run the test on a model which is better than the one you serve. Moonshot credibly claim to have reported their benchmarks at the same low-precision quantization (INT4) that they serve users, but others don't claim this.

<big>In fairness</big>

I should also say the Chinese models do very well on LMArena - despite being <a href="https://arxiv.org/abs/2504.20879">unfairly penalised</a>. But Arena is a <a href="https://www.seangoedecke.com/lmsys-slop/">poor</a> <a href="https://lmsys.org/blog/2024-08-28-style-control/">measure</a> of actual ability. It _is_ a decent test of style though. I put this gap down to American labs overoptimising: post-training too hard and putting in all kinds of repugnant corporate ass-covering stuff in the spec.

Also Qwen is famous for 'capability density': the small versions are surprisingly smart for their size.

<!-- * Cherrypicking (reporting the tests you happen to do well on) mostly isn't an issue with Kimi.  -->

<big>The D word</big>


Distillation is [second-rate](https://www.rohan-paul.com/p/recent-advancements-in-distillation) intelligence, and there's [some](https://www.ft.com/content/a0dfedd1-5255-4fa9-8ccc-1fe01de87ea6) [evidence](https://www.reddit.com/r/LocalLLaMA/comments/1m2w5ge/did_kimi_k2_train_on_claudes_generated_code_i/) that they are distilling off of American models to some extent. See also the excellent Slop Profile from [EQ Bench](https://eqbench.com/creative_writing.html), which estimates that the new Kimi is closer to Claude than its own base model. 
<br><br>
<img src="/img/kimi-is-claude.png" />
<!--  -->
<!-- Kimi-Instruct is as close in style to o3 as GPT-5 is and Kimi-Thinking is as close to Opus 4 as Sonnet 4.5 is. -->
<br><br>But anyway I don't claim this is a major factor here, maybe another 5%.

<br>

The above isn't novel; it's common knowledge there's some latent capabilities gap. This is often put in terms of them being "[3 months behind](https://epoch.ai/data-insights/open-weights-vs-closed-weights-models)", but these estimates are still assuming that brittle, ad hoc, and heavily goodharted benchmarks have good external validity. I'd guess more like 12 months.

<br>



### Unreliability?

> 1\. frontier performance on some benchmarks

The above benchmarks are mostly single-shot, but people are now pushing LLMs to do more complicated stuff. One very flawed measure of this is the HCAST time horizon for software engineering: on that, [DeepSeek R1](https://epoch.ai/benchmarks/metr-time-horizons) had a 31 minute "time horizon" compared to Opus 4's 80 minutes.

There are various worse agent benchmarks, and e.g. [the new Kimi](https://moonshotai.github.io/Kimi-K2/thinking.html) posts great numbers on them. But on vibe I'd bet on a >3x reliability advantage for Claude.

As well as reliability over time, there's stability over inputs. Maybe the Chinese models are higher variance or more sensitive to the prompt and hyperparams. 


### Harder to elicit?


TODO: I've been meaning to run the obvious experiment, which is to just see if they have a bigger gap between pass@1 and pass@64 success rates.

TODO: Intentionally underelicit! Rerun the models on AIME 2024 with only a basic prompt. My results will be lower; the gap tells us how much the labs' own intense tuning helps / is necessary. This tells us something about, not their capability, but their actual in-the-wild performance with normal lazy users.


### Tokenomics: no effective discount

> 2\. massive per-token discounts (~3x on input, ~6x on output)

Distinguish intelligence (max performance), intelligence per token (efficiency), and intelligence per dollar (cost-effectiveness).

The 5x discounts I quoted are per-token, not per-success. If you had to use 6x more tokens to get the same quality, then there would be no real discount. And indeed DeepSeek and Qwen (see also anecdote here about [Kimi](https://www.reddit.com/r/LocalLLaMA/comments/1oth5pw/comment/no4kgsp/), uncontested) are very hungry:

<img src="/img/semi-token-hungry.png" />

And in [this](https://artificialanalysis.ai/#output-tokens) graph you can clearly see a 2-4x difference (with Gemini and Kimi K2-base as the big exceptions):

<img src="/img/aa-token-hunger.jpg" />

And the resulting cost is a mixed bag:<br><br>

<center>
	<img width="75%" src="/img/aa-cost.jpg" />
</center>

I won't [use](https://artificialanalysis.ai/#cost) AA's efficiency estimates, because again I think the benchmarks underlying them are bad evidence.

<!-- Out-of-the-box (browser and APIs) -->

### Self-hosting has high fixed costs

> 3\. the weights. On-prem with fully free MIT licence, self-hosting, white-box access, customisation, with zero profit going to the Chinese companies.

Self-hosting doesn't really [make sense](https://www.ptolemay.com/post/llm-total-cost-of-ownership#break-even-a-five-minute-reality-check) unless you're huge volume or using them for very simple tasks. And most enterprises are not really competent enough to finetune anything.

This is partly a temporary matter: the software ecosystem is underdeveloped for serious high-reliability scaled usage, despite the intense hobbyist interest. (They mostly want it running on a Macbook.)

### Too slow for casuals

> 4\. You can get much faster token speeds than the closed APIs.

In the browser, they're actually [slower](https://newsletter.semianalysis.com/p/deepseek-debrief-128-days-later?open=false#%C2%A7speed-can-be-compensated-for) than Western models. This makes sense; they are incredibly inference bound thanks to chip controls! This would be enough to tank them in the consumer market.

And over API, everyone except Anthropic dominate, even in raw token rate (not counting efficiency):

<img src="/img/speed.jpg" />


<!-- (On the few occasions I've used the Kimi API, it had constant RateLimitErrors but was still faster per token than Claude. But again much worse utility per token!) -->

### Censorship and perceived censorship

> 5\. less overrefusal (except on CCP talking points)

There's a pretty big ick factor to the CCP, and the companies are indeed forced to comply on a range of talking points which offend the West. However, the hosted versions are [much worse](https://interconnect.substack.com/p/was-zuck-right-about-chinese-ai-models) than the weights themselves. [SpeechMap](https://speechmap.substack.com/p/chinese-open-source-model-roundup?r=269emp&utm_campaign=post&utm_medium=web&triedRedirect=true):

<br>
<center>
	<img src="/img/deepseek-censor.png" />
	<img src="/img/kimi.jpg" />
</center>

But there are [uncensored](https://huggingface.co/perplexity-ai/r1-1776) finetunes from reputable names. But then again see (3): it doesn't make sense for most enterprises to conduct and host finetunes themselves.

If you do a fair test on controversial but non-CCP talking points, there's a [wide spread](https://speechmap.ai/models/) of refusal rates in both Chinese and Western models.

### Nebulous ideology?

> 6\. on topics controversial in the West, less nannying

Lambert notes that the people he speaks privately to are really worried about less obvious stuff, the "indirect influence of Chinese values".

[There is something to this](https://arxiv.org/pdf/2411.06032v1) currently (but not a lot given the size of the English internet in the training corpus and the relative lack of soft-post-training skill or effort in Chinese labs):<br><br>
<center>
	<img width="56%" src="/img/chinese-values.jpg" />
</center>

But it's valid to assume this will get worse as the CCP get more aware and the companies put more effort into personality and post-training.

<!-- "There's no way, without releasing the training data, for these companies to fully convince Western companies that they're safe."  -->



### Downloading is a long way from productising

> 8\. they’re the most-downloaded open models


People panic about "[the flip](https://www.atomproject.ai/)", the point at which people started downloading Chinese models more. But this is obviously a terrible proxy for actually managing to use them. (And it's actually pretty unclear how self-hosting adoption would really benefit China anyway except in prestige.) 

For people with any need of real customisation or tiny models, or a _scientific_ ML hobby, or an ideological interest in open-source, they clearly dominate.

TODO: Scrape relative mention over time of LLaMa vs Qwen in Arxiv experiments.

I concede that the secrecy in the West about using Chinese models makes this one weaker as an explanation. 


### even more insecure?

> 9\. they are mostly not used even by the cognoscenti


Above I mentioned reliability (how low-variance they are, how well they can chain things together). But that's the easy bit; what about adversarial reliability?

The [US evaluation](https://www.nist.gov/news-events/news/2025/09/caisi-evaluation-deepseek-ai-models-finds-shortcomings-and-risks) had a bone to pick, but their directional result is probably right ("DeepSeek's most secure model (R1-0528) responded to 94% of overtly malicious requests [using a jailbreak], compared with 8% of requests for U.S. reference models").

[Someone else](https://splx.ai/blog/kimi-k2-safety-test) talking their book notes that Kimi is "not yet fit for secure enterprise deployment".

This is obviously a huge problem for any agentic uses, even if the benchmark and default reliability were all fine.


### Low mindshare

> 9\. they are mostly not used even by the cognoscenti

It's hardly cynical to note that most people don't pick their models by analysing relative performance. Instead it's largely name recognition and trust, which makes sense for reasons of risk aversion and filtered evidence.

In principle, you can change models by changing one string in your codebase. But in practice if you're sane you need to do incredibly expensive evals and so there's stickiness.

The DeepSeek moment helped a lot, but it receded in the second half of 2025 (from [22%](https://openrouter.ai/rankings?view=day#market-share) of the weird market to 6%). And they all have extremely weak brands.

Also corporations really do settle for inferior products all the time for ass-covering reasons ([IBMism](https://www.quora.com/What-does-the-phrase-Nobody-ever-got-fired-for-choosing-IBM-mean)). Mindshare translates directly into appeal to the risk-averse.

### Corporate compliance is hard

> 9\. they are mostly not used even by the cognoscenti. 

Chinese APIs are hard for Western companies to use for legal and quasi-legal reasons.

For the API, DeepSeek [sent](https://www.feroot.com/news/the-independent-feroot-security-uncovers-deepseeks-hidden-code-sending-user-data-to-china/) user information to China Mobile, a state company, which violates all kinds of Western data privacy laws. Even if they've stopped, this risk is corporate poison. How can you ever be sure enough?

<!-- DeepSeek's terms of service indicate that user data may be stored in China, raising serious questions about compliance with international data protection standards, including the EU General Data Protection Regulation (GDPR).  -->

In a couple of years the EU AI Act will be (nominally) enforceable on the Chinese labs too.

On the quasi-legal side, corporate "vendor risk" programmes [often](https://dgap.org/en/research/publications/china-de-risking) flag Chinese suppliers. This is sometimes because they actually [can't](https://www.z2data.com/insights/why-chinese-suppliers-arent-aligned-with-compliance-efforts-part1
) guarantee there's no forced labour involved. 

So why not on-prem? Again, it's a huge fixed cost and competence-bound and your risk team might still give you shit for it. [Lambert](https://www.interconnects.ai/p/what-people-get-wrong-about-the-leading):

> People vastly underestimate the number of companies that cannot use Qwen and DeepSeek open models because they come from China. This includes on-premise solutions built by people who know the fact that model weights alone cannot reveal anything to their creators.

### Political bias

> 9\. they are mostly not used even by the cognoscenti. 

There are a bunch of social reasons you might want to avoid Chinese models. You might be protectionist, or sucking up to the ascendent protectionists.

The protectionism of others is clearly enough for people to keep quiet about using them. It is often probably enough for them to just not take the risk in the first place.

I'd include here [superstitions](https://datasaur.ai/blog-posts/chinese-open-weights-models-security-myths-vs-reality) about the weights themselves being backdoored.


<!-- ### Weak software?

> 9\. they are mostly not used even by the cognoscenti. 

It might be true that there's less mature software around the Chinese models for smooth user UX, mature APIs, and support for devices and parallelising. But the Western hobbyist community is massive: there are [2 million](https://huggingface.co/papers/2508.06811) distinct finetunes out there, most of them going off Chinese base models. -->

<!-- https://gradientflow.substack.com/p/are-chinese-open-weights-models-a -->

### Vendor risk

> 9\. they are mostly not used even by the cognoscenti


If you look ahead, at future risks to your suppliers, it's obvious that the export control situation _relatively_ speaks against using Chinese models; NVIDIA is not going to choke off OpenAI.

For API adoption, I also haven't seen anything about Service-Level Agreements (contracts ensuring uptime) and support from any Chinese lab, but these are easy to make (even if compute crunch means that their uptime guarantees simply must be worse than American ones).

Also again corporate vendor-risk programmes often flag Chinese suppliers for data sovereignty, volatility of PRC law, and export control reasons.

DeepSeek [openly use Anna’s Archive](https://arxiv.org/pdf/2403.05525), where everyone else is [quiet](https://www.publishers.org.uk/publishers-association-statement-on-the-atlantic-article-on-libgen-and-meta/) about it. But the American companies offer [IP indemnity](https://www.proskauer.com/blog/openais-copyright-shield-broadens-user-ip-indemnities-for-ai-created-content) for users (cover if the models violate copyright in your app), which is nice insurance for a nervous corp with a target on its back. I can't see anything about the Chinese companies doing this yet.



### Excess quantization?

> 11\. they're aggressively quantizing at inference-time, 32 bits to 4

No, I think this one is wrong or else only a tiny factor. gpt-oss was [post-trained](https://www.reddit.com/r/LocalLLaMA/comments/1mn3465/gptoss_was_only_sorta_trained_at_mxfp4/) in MXFP4 which is only 4.25bits. 

And I have a strong hunch that many American models are also served in low fidelity, maybe FP4 (4 bits). Quantization just isn't that bad.


### Galaxy-brain soft power??

> 12\. state-sponsored Chinese hackers used closed American models for incredibly sensitive operations, giving the Americans a full whitebox log of the attack!

I can dimly imagine some kind of flexing dynamic in cyberwarfare, where you actually want to show off your attack capabilities, and so you use Claude on purpose. Yes: this idiotic move makes great sense if the apparent targets are red herrings, if _Anthropic_ were the real target. You learn how long their OODA loop is, you learn (by retaliation or its absence) how tight they are with the NSA, you learn a little about how good their tech is. 

You could also see it as retaliation for Amodei's [hawkish comments](https://www.darioamodei.com/post/on-deepseek-and-export-controls) all year. Literally trading effectiveness for embarrassment.

But I don't really know anything about this.

<!-- DeepSeek is owned by a hedge fund, HighFlyer. short. astroturfing -->
<!-- finance.yahoo.com/news/deepseek-launch-may-used-short-222010168.html -->
<!-- https://www.youtube.com/watch?v=qQT4fbXWpUM -->


## Overall

_Low adoption is overdetermined_:
* No, I don't think they're as good on new inputs or even that close.
* No, they're not more efficient in time or cost (for non-industrial-scale use).
* Even if they were, the social and legal problems and biases would probably still suppress them in the medium run.
* But obviously if you want to heavily customise a model, or need something tiny, or want to do science, they are totally dominant.
* Ongoing compute constraints make me think the capabilities gap and adoption gap will persist.





