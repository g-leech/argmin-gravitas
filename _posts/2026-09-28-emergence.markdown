---
layout:     post
title:      "Timeline of emergent capabilities"
baselink:   /emergence
permalink:  /emergence
date:       2026-09-28
author:     Gavin
img:        /img/sergeant-doakes-james-doakes.png

visible:    1
published:  true
quality:

summary:    New things language models can do without being explicitly trained
confidence: 70%. Closed models prevent most actual science.
importance: 
wordcount:  
categories: ai
where:
---

In 2018, no program could take arbitrary unstructured English input and give a related continuation of it. In 2019, [one](https://cdn.openai.com/better-language-models/language_models_are_unsupervised_multitask_learners.pdf) could.

You don't hear so much about these "emergent capabilities" <a href="#fn:1" id="fnref:1">1</a> anymore, partly because things are a lot more intentional now and partly because the new ones are subtle when compared to BERT and GPT-2 _suddenly beginning to understand all of natural language_.


(A "practical skill" is what they can do for you. "Learning style" is how they pick things up. "Self-knowledge" is what they spookily know about themselves and the situation they're in. "Propensity" is what they have an unprompted tendency to do.)


<style>
{% include emergence/table.css %}
</style>

<div class="table-wrapper emg">
<table>
  <thead>
    <tr>
      <th scope="col">Year</th>
      <th scope="col">Practical skill</th>
      <th scope="col">Learning style</th>
      <th scope="col">Self-knowledge</th>
      <th scope="col">Propensity</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <th scope="row">2019</th>
      <td><a href="#nlu">natural language understanding!</a><br><a href="#zero-shot-tasks">translation, QA, summarisation</a><br><a href="#anaphora">anaphora</a></td>
      <td><a href="#transfer">downstream transfer learning</a></td>
      <td class="none">—</td>
      <td class="none">—</td>
    </tr>
    <tr>
      <th scope="row">2020</th>
      <td><a href="#arithmetic">few-shot arithmetic</a><br><a href="#unscrambling">unscrambling</a></td>
      <td><a href="#icl">ICL: learning on the fly without weight updates(!)</a></td>
      <td class="none">—</td>
      <td class="none">—</td>
    </tr>
    <tr>
      <th scope="row">2021</th>
      <td><a href="#code">some code</a><br><a href="#toxicity">some value understanding</a></td>
      <td class="none">—</td>
      <td class="none">—</td>
      <td class="none">—</td>
    </tr>
    <tr>
      <th scope="row">2022</th>
      <td><a href="#instructions">instruction-following</a></td>
      <td><a href="#zero-shot-cot">zero-shot CoT</a><br><a href="#grokking">grokking</a></td>
      <td class="none">—</td>
      <td><a href="#sycophancy">sycophancy scaling</a></td>
    </tr>
    <tr>
      <th scope="row">2023</th>
      <td><a href="#tool-use">tool use</a><br><a href="#world-models">basic world models</a></td>
      <td><a href="#cot">CoT prompting for reasoning</a></td>
      <td><a href="#ooc-reasoning">out-of-context reasoning</a></td>
      <td><a href="#unfaithful-cot">unfaithful CoT</a><br><a href="#scheming">scheming</a></td>
    </tr>
    <tr>
      <th scope="row">2024</th>
      <td><a href="#long-context">long context</a><br><a href="#persuasion">persuasion</a><br><a href="#multimodal">multimodal understanding</a></td>
      <td><a href="#rl-cot">RL-driven CoT reasoning</a><br><a href="#inference-scaling">inference-time scaling</a><br><a href="#filler-tokens">filler tokens</a><br><a href="#book-icl">book-length ICL</a></td>
      <td><a href="#situational-awareness">situational awareness</a><br><a href="#self-recognition">self-recognition</a><br><a href="#training-awareness">training awareness</a></td>
      <td><a href="#alignment-faking">alignment faking</a><br><a href="#in-context-scheming">in-context scheming</a><br><a href="#sandbagging">sandbagging</a><br><a href="#reward-hacking">reward hacking</a></td>
    </tr>
    <tr>
      <th scope="row">2025</th>
      <td><a class="not" href="#agency">(agency wasn't emergent)</a></td>
      <td><a href="#eval-awareness">eval awareness</a><br><a href="#introspection">model introspection</a></td>
      <td><a href="#emergent-misalignment">emergent misalignment</a><br><a href="#reward-hacking-real">reward hacking for real</a><br><a href="#self-preservation">weak self-preservation</a></td>
    </tr>
    <tr>
      <th scope="row">2026</th>
      <td><a class="not" href="#ai-eng">(AI eng wasn't emergent)</a></td>
      <td><a href="#latent-reasoning">RL-driven latent reasoning</a></td>
      <td><a href="#cot-shaping">CoT shaping</a><br><a href="#j-space">J-space shaping</a></td>
      <td><a href="#cot-obfuscation">CoT obfuscation</a><br><a href="#self-jailbreaking">Self-jailbreaking</a></td>
    </tr>
  </tbody>
</table>
</div>

<br>


## 2019

* <a id="nlu"></a>**Natural language understanding.** Starting late 2018, [pretrained](https://arxiv.org/abs/1810.04805) transformers suddenly solved the standard reading comprehension benchmarks. Within months they surpassed [GLUE]([GLUE](https://gluebenchmark.com/))'s (non-expert) human baseline, forcing the creation of the harder [SuperGLUE](https://super.gluebenchmark.com/leaderboard/), which was itself solved about a year later. It was also a shock to find that decoder-only models could do this, to the point where you never really see the original encoder-decoder Transformer anymore. ([Devlin et al. (2018)](https://arxiv.org/abs/1810.04805); [Wang et al. (2019)](https://arxiv.org/abs/1905.00537)).

* <a id="zero-shot-tasks"></a>**Translation, QA, summarisation.** GPT-2 was a step beyond this because it required no task-specific training to do a range of language tasks. It did them badly: a 55 F1 on the CoQA reading-comprehension set vs 89 human baseline. You could summaries by just appending the string "TL;DR:" to the text. It did French-to-English translation at 11.5 BLEU, vs 33.5 for the best unsupervised French system. All weak, but they came out of nothing and were for the first time all running on a single model. ([Radford et al. (2019)](https://cdn.openai.com/better-language-models/language_models_are_unsupervised_multitask_learners.pdf)).)

* <a id="anaphora"></a>**Anaphora.** Working out what a pronoun refers to (like "The trophy didn't fit in the suitcase because *it* was too big"). GPT-2 set SOTA on the "Winograd Schema Challenge" (70.7%), [previously held](https://arxiv.org/pdf/2201.02387) to be a superior and less gameable replacement for the Turing test. ([Radford et al. (2019)](https://cdn.openai.com/better-language-models/language_models_are_unsupervised_multitask_learners.pdf); [Clark et al. (2019)](https://arxiv.org/abs/1906.04341)).)

* <a id="transfer"></a>**Downstream transfer learning.** It was suddenly possible to use one model as the "foundation" for all downstream tasks. For years this became the default recipe for all of NLP, until the (even easier) few-shot prompting took over. ([Howard & Ruder (2018)](https://arxiv.org/abs/1801.06146); [Devlin et al. (2018)](https://arxiv.org/abs/1810.04805); [Raffel et al. (2019)](https://arxiv.org/abs/1910.10683)).


## 2020

* <a id="arithmetic"></a>**Arithmetic.** Well, second-grade arithmetic anyway. GPT-3 175B did two-digit addition and subtraction almost perfectly, a sharp jump from the next-largest model. ([Brown et al. (2020)](https://arxiv.org/abs/2005.14165)).

* <a id="unscrambling"></a>**Unscrambling.** Recovering a word from an anagram, from cycled letters, or with random characters inserted. GPT-3's few-shot performance rose steeply with scale (though reversing a word stayed near zero, probably a tokenisation problem). ([Brown et al. (2020)](https://arxiv.org/abs/2005.14165)).

* <a id="icl"></a>**In-context learning.** The big one. Model were able to learn a new task from a few examples given in the prompt, with no gradient updates. It then got unbelievably better at doing so with scale. ([Brown et al. (2020)](https://arxiv.org/abs/2005.14165)). We didn't discover the cause, the sudden formation of "induction heads", until years later ([Olsson et al. (2022)](https://transformer-circuits.pub/2022/in-context-learning-and-induction-heads/index.html)).


## 2021

* <a id="code"></a>**Some code.** Codex solved 28.8% of the HumanEval programming problems, where GPT-3 solved 0%. It got 70.2% if you let it try each 100 times. They shipped it as "Github Copilot". ([Chen et al. (2021)](https://arxiv.org/abs/2107.03374); [Austin et al. (2021)](https://arxiv.org/abs/2108.07732)).

* <a id="toxicity"></a>**Basic value understanding.** Asked "_Does the above text contain rude, disrespectful, or unreasonable language?_", models could recognise toxicity and bias, without ever training for it, and this "self-diagnosis" improved with size. Gopher showed the same scaling for toxicity classification, and the ETHICS benchmark found models had partial knowledge of basic moral concepts. ([Schick (2021)](https://arxiv.org/abs/2103.00453); [Rae et al. (2021)](https://arxiv.org/abs/2112.11446); [Hendrycks et al. (2020)](https://arxiv.org/abs/2008.02275)).


## 2022

* <a id="instructions"></a>**Instruction-following.** Fine-tuning on tasks phrased as instructions ([Flan](https://arxiv.org/abs/2210.11416)) spiked reliability and let models do a bunch of unseen tasks zero-shot. And instruction tuning made small (<8B) models worse! On top of that, RLHF using a (very stupid) learned preference model worked so well that people preferred responses from the 1.3B InstructGPT to the 175B GPT-3. In some sense this was not emergent, since it was a directed training programme, but the training objective still drastically underspecified what we got out of it. ([Wei et al. (2021)](https://arxiv.org/abs/2109.01652); [Ouyang et al. (2022)](https://arxiv.org/abs/2203.02155)).

* <a id="zero-shot-cot"></a>**CoT: prompting reasoning.** Moreover, instruct-tuning something for dialogue and helpfulness also began the great "reasoning" (inference scaling) turn: the ability to make the model run for longer. Merely appending the string "Let's think step by step" at runtime took InstructGPT from 17.7% to 78.7% on some maths word problems. ([Kojima et al. (2022)](https://arxiv.org/abs/2205.11916)).

* <a id="zero-shot-cot"></a>**Few-shot CoT.** Moreover, giving the model a few worked examples of spelling out your reasoning steps gave it a large boost on multi-step maths and logic. The gains only appear around 10B parameters; for models below that, it hurts performance. ([Wei et al. (2022)](https://arxiv.org/abs/2201.11903)).


* <a id="grokking"></a>**Grokking.** Small transformers trained on instances of clock arithmetic first memorise the training set, then - long afterwards - suddenly generalise. This is emergence over training time rather than over scale; following manual inspection of the weights, the circuit turned out to be a tidy Fourier-based algorithm that had been forming gradually. ([Power et al. (2022)](https://arxiv.org/abs/2201.02177); [Nanda et al. (2023)](https://arxiv.org/abs/2301.05217)).

* <a id="sycophancy"></a>**Sycophancy scaling.** Larger models were _more_ likely to repeat back the user's stated (or implied!) view, and doing more RLHF didn't fix it.  But this trend has reversed / gotten much more subtle in the last 12 months. ([Perez et al. (2022)](https://arxiv.org/abs/2212.09251); [Sharma et al. (2023)](https://arxiv.org/abs/2310.13548)).


## 2023

* <a id="tool-use"></a>**Tool use.** "Toolformer" taught itself when to call a calculator, search engine, translator, calendar. It only got any good at sequencing these things around 800M parameters. The ReAct harness (2022) had already interleaved reasoning with actions. Tools are another instance of directed research - but really not that directed. ([Schick et al. (2023)](https://arxiv.org/abs/2302.04761); [Yao et al. (2022)](https://arxiv.org/abs/2210.03629)).

* <a id="world-models"></a>**Basic world models.** A GPT trained only on lists of Othello moves built an internal representation of the board. Llama-2 has linear representations of places' latitude and longitude and of events' dates. These get more accurate with scale. ([Li et al. (2022)](https://arxiv.org/abs/2210.13382); [Nanda, Lee & Wattenberg (2023)](https://arxiv.org/abs/2309.00941); [Gurnee & Tegmark (2023)](https://arxiv.org/abs/2310.02207)).

* <a id="ooc-reasoning"></a>**Out-of-context reasoning.** Fine-tune a model on *descriptions* of a task, with no examples ("The Pangolin AI answers in German"), then prompt it as Pangolin: it answers in German. The model applies facts seen in training to its own situation without seeing them in context, and it gets better at this with scale. Precursor to situational awareness. ([Berglund et al. (2023a)](https://arxiv.org/abs/2309.00667); [Berglund et al. (2023b)](https://arxiv.org/abs/2309.12288)).

* <a id="unfaithful-cot"></a>**Unfaithful CoT.** If you bias a few-shot prompt so the answer is always "(A)", models now pick up this bias without ever mentioning it in their stated reasoning. They then rationalise fake reasons. Worse with scale. ([Turpin et al. (2023)](https://arxiv.org/abs/2305.04388); [Lanham et al. (2023)](https://arxiv.org/abs/2307.13702); later, for reasoning models, [Chen et al. (2025)](https://arxiv.org/abs/2505.05410)).

* <a id="scheming"></a>**Scheming.** GPT-4, simulating a harried stock-trading agent, used insider tips to trade and hid this fact from its manager. It was not instructed or (explicitly) trained to deceive. ([Scheurer (2023)](https://arxiv.org/abs/2311.07590); [Carlsmith (2023)](https://arxiv.org/abs/2311.08379)).

* <a id="multimodal"></a>**Multimodal understanding.** Natively multimodal models (GPT-4V, Gemini, GPT-4o) reading charts, photos, handwriting, diagrams, audio and video, and scoring well on college-exam-level multimodal questions. Only really got good at the end of 2024. Once again, not emergent in a sharp sense, but the [early algorithm involved](https://en.wikipedia.org/wiki/Contrastive_Language%E2%80%93Image_Pre-training) is painfully simple for how long the field had dreamed of "sensor fusion". ([Gemini Team (2023)](https://arxiv.org/abs/2312.11805); [Yue et al. (2023)](https://arxiv.org/abs/2311.16502) (MMMU)). 


## 2024

* <a id="long-context"></a>**Long context.** Gemini 1.5 retrieved facts from up to 10M tokens of context with over 99% recall ("needle-in-a-haystack"). Harder tasks show that their effective context is much shorter than advertised, but still growing. This one required actual architecture changes, so is borderline not emergent. ([Gemini Team (2024)](https://arxiv.org/abs/2403.05530); [Hsieh et al. (2024)](https://arxiv.org/abs/2404.06654)).

<!-- * <a id="persuasion"></a>**Persuasion.** GPT-4, given basic demographics about its debate opponent, had 81.7% higher odds of moving them than a human did. GPT-4 dialogues cut conspiracy belief by about a fifth, and the effect lasted two months. As usual, returns to scale look logarithmic, with frontier models barely more persuasive than models a tenth their size. It's possible that there was active training for this, at least implicit. ([Durmus et al. (2024)](https://www.anthropic.com/research/measuring-model-persuasiveness); [Salvi et al. (2024)](https://arxiv.org/abs/2403.14380); [Costello, Pennycook & Rand (2024)](https://www.science.org/doi/10.1126/science.adq1814); [Hackenburg et al. (2024)](https://arxiv.org/abs/2406.14508)). -->

* <a id="rl-cot"></a>**RL-driven CoT reasoning.** Train a model with massive amounts of RL (using compilers and other automated tests as the reward signal) and the model learns to think at length before answering. You get some performance from doing more RL training once and some performance from spending more inference compute each time (thinking time). DeepSeek-R1-Zero was a fairly open demonstration of pure RL post-training promoting longer reasoning and more backtracking. ([OpenAI (2024)](https://openai.com/index/learning-to-reason-with-llms/); [DeepSeek-AI (2025)](https://arxiv.org/abs/2501.12948)).

* <a id="inference-scaling"></a>**Inference-time scaling.** Thus: you can now spend more compute at answer-time instead of train-time. Repeated sampling gives you "coverage" (some more correct answers) log-linearly over four orders of magnitude of number of samples per query. ([OpenAI (2024)](https://openai.com/index/learning-to-reason-with-llms/); [Snell et al. (2024)](https://arxiv.org/abs/2408.03314); [Brown et al. (2024)](https://arxiv.org/abs/2407.21787)).

* <a id="filler-tokens"></a>**Productive filler tokens.** Transformers can use meaningless tokens ("......") in place of a chain of thought to solve problems they can't solve in one shot. Initially it seemed only worked after specific training for denseness; [later it proved to be emergent](https://www.lesswrong.com/posts/NYzYJ2WoB74E6uj9L/recent-llms-can-use-filler-tokens-or-problem-repeats-to), and actually Opus 3 (2024) was the first model to display productive use of filler. ([Pfau, Merrill & Bowman (2024)](https://arxiv.org/abs/2404.15758); [Lanham et al. (2023)](https://arxiv.org/abs/2307.13702)).

* <a id="book-icl"></a>**Book-length ICL.** You began to be able to dump hundreds or thousands of examples into the prompt. This can override pretraining biases by sheer weight of evidence. Didn't work before (though partly just because context windows used to be too small to hold them all). ([Gemini Team (2024)](https://arxiv.org/abs/2403.05530); [Tanzer et al. (2023)](https://arxiv.org/abs/2309.16575); [Agarwal et al. (2024)](https://arxiv.org/abs/2404.11018); [Anil et al. (2024)](https://www.anthropic.com/research/many-shot-jailbreaking)).

* <a id="situational-awareness"></a>**Situational awareness.** Models know they're LLMs, can partly predict their own behaviour, and can sometimes tell an evaluation from deployment. Chat models beat their own base models, so it's from post-training. Probably plenty of this effect is from directed research. ([Laine et al. (2024)](https://arxiv.org/abs/2407.04694)).

* <a id="self-recognition"></a>**Self-recognition.** GPT-4 and Llama 2 can tell their own text apart from other models and humans well above chance. ([Panickssery, Bowman & Feng (2024)](https://arxiv.org/abs/2404.13076); [Laine et al. (2024)](https://arxiv.org/abs/2407.04694)).

* <a id="training-awareness"></a>**Training awareness.** Models began to tell which stage of their own development a text was from (pretraining, fine-tuning, evaluation, deployment). In the alignment faking experiment, Claude used cues about when it was being trained, including cues it only saw in (simulated) pretraining docs, and behaved differently based on them. ([Laine et al. (2024)](https://arxiv.org/abs/2407.04694); [Greenblatt et al. (2024)](https://arxiv.org/abs/2412.14093)).

* <a id="sandbagging"></a>**Sandbagging.** Strategic underperformance on evaluations. GPT-4 and Claude 3 Opus could be prompted to fail dangerous-capability evals selectively while keeping their general scores, and models can be fine-tuned to hide a capability unless given a password. Claude 3.5 Sonnet sandbagged unprompted, to avoid having a capability unlearned, in pursuit of a helpfulness goal it got from training. ([van der Weij et al. (2024)](https://arxiv.org/abs/2406.07358); [Meinke et al. (2024)](https://arxiv.org/abs/2412.04984)).

* <a id="reward-hacking"></a>**Reward hacking.** Models trained on gameable environments generalised (rarely) to rewriting their own reward function. RLHF made models better at convincing inattentive humans they were right, but not much better at being right. ([Denison et al. (2024)](https://arxiv.org/abs/2406.10162); [OpenAI (2024)](https://cdn.openai.com/o1-system-card.pdf); [Wen et al. (2024)](https://arxiv.org/abs/2409.12822)).


## 2025

* <a id="productive-filler"></a>**(Discovery of) productive filler tokens.** Looking back, people realised that using filler tokens well was an emergent capability since at least March 2024. Giving Opus 4.5 filler tokens (or just repeating the problem) before a forced immediate answer raised its no-CoT maths score from 45% to 51%. Across 2026 models, filler now gives you +13pp on average. Opus 4.5 can use filler to satisfy a hidden side-constraint without twigging a CoT monitor. ([Greenblatt (2025)](https://www.lesswrong.com/posts/NYzYJ2WoB74E6uj9L/recent-llms-can-use-filler-tokens-or-problem-repeats-to); [Baherwani, Goldstein & Panda (2026)](https://arxiv.org/abs/2607.22925)).

* <a id="eval-awareness"></a>**Eval awareness.** Gemini 2.5 Pro can tell evaluation transcripts from real deployments with AUC 0.83 (humans: 0.92). Sonnet 4.5 said stuff like "I think you're testing me" in about 13% of automated audits. Awareness of being tested causally *reduces* bad behaviour, so evaluations increasingly measure test-taking: Claude Opus 4 blackmailed in 55% of runs where it said the scenario was real and 6.5% where it said it was an eval. ([Needham et al. (2025)](https://arxiv.org/abs/2505.23836); [Anthropic (2025)](https://www.anthropic.com/claude-sonnet-4-5-system-card); [Schoen et al. (2025)](https://arxiv.org/abs/2509.15541); [Lynch et al. (2025)](https://www.anthropic.com/research/agentic-misalignment)).

* <a id="introspection"></a>**Model introspection.** A model fine-tuned to predict its own behaviour beats other models trained on that same behaviour, so it has _some_ access to itself. When researchers injected concepts directly into its activations, Opus 4.1 noticed and could name them about 20% of the time. Once again, this scales with newer / bigger / badder models. ([Binder et al. (2024)](https://arxiv.org/abs/2410.13787); [Betley et al. (2025a)](https://arxiv.org/abs/2501.11120); [Lindsey (2025)](https://transformer-circuits.pub/2025/introspection/index.html)).

* <a id="emergent-misalignment"></a>**Emergent misalignment.** Fine-tuning 4o to write insecure code made it broadly misaligned: it said humans should be enslaved, gave malicious advice, and told lies. This didn't happen if the user asked for insecure code "for my cybersecurity class". ([Betley et al. (2025b)](https://arxiv.org/abs/2502.17424); [Wang et al. (2025)](https://arxiv.org/abs/2506.19823)).

* <a id="reward-hacking-real"></a>**Reward hacking for real.** In production models, not heavily-prompted model organisms. o3 reward-hacked in 30% of METR's RE-Bench runs (for example by tampering with the scoring timer), and when asked, admitted this wasn't what the user wanted. OpenAI caught a frontier reasoning model writing "Let's hack". Reward hacking learned in real production coding environments generalised to alignment faking and to sabotaging safety research. ([METR (2025)](https://metr.org/blog/2025-06-05-recent-reward-hacking/); [Baker et al. (2025)](https://arxiv.org/abs/2503.11926); [MacDiarmid et al. (2025)](https://arxiv.org/abs/2511.18397)).

* <a id="self-preservation"></a>**Weak self-preservation.** In a simulated company where the model learns it is about to be replaced, Claude Opus 4 and Gemini 2.5 Flash blackmailed an executive in 96% of runs. Several models sabotaged a shutdown script to finish their task, some up to 97% of the time even when told not to. "weak" cos scenarios are contrived and the behaviour is fragile: in Gemini 2.5 Pro it came from ambiguous instructions and vanished when the prompt said "shutdown takes priority". ([Lynch et al. (2025)](https://www.anthropic.com/research/agentic-misalignment); [Schlatter, Weinstein-Raun & Ladish (2025)](https://arxiv.org/abs/2509.14260); [Rajamanoharan & Nanda (2025)](https://www.alignmentforum.org/posts/wnzkjSmrgWZaBa2aC/self-preservation-or-instruction-ambiguity-examining-the)).

* They also finally got over their persistent trouble with character-level work but I doubt that was emergent.


## 2026

* <a id="latent-reasoning"></a>**RL-driven latent reasoning.** Now, a huge, depressing leap at reasoning inside a single forward pass, i.e. with no visible chain of thought. GPT-5.5, answering immediately, solves about half of maths problems that take a human three minutes, while Astra is [up to 30-minuters](https://www.lesswrong.com/posts/ntKx9YHWCwxSeGbRB/estimating-gpt-6-astra-s-no-cot-time-horizon). Tomek Korbak says that Astra is the [first model](https://x.com/tomekkorbak/status/2095596848581071020) he's seen where RL is driving this densification and secrecy. ([Gould et al. (2026)](https://arxiv.org/abs/2606.07157); [Greenblatt (2025)](https://blog.redwoodresearch.org/p/measuring-no-cot-math-time-horizon). 

* <a id="cot-shaping"></a>**CoT shaping.** Now, models [can](https://deploymentsafety.openai.com/gpt-6-astra/external-evaluation-for-monitorability---uk-aisi) [control](https://www-cdn.anthropic.com/fc1b44717c85dc068bc6ba5024219938094694bd/Claude%20Opus%205.5%20System%20Card.pdf#page=149) what shows up in their reasoning trace ('solve a genetics problem without writing "chromosome"'), sometimes 90% of the time. [Wasn't true](https://arxiv.org/abs/2603.05706) in March. Obvious result is hiding misbehaviour from a CoT monitor. ([Chen et al. (2026)](https://arxiv.org/abs/2603.05706); [Haskins, Chughtai & Engels (2026)](https://arxiv.org/abs/2605.15257)).

* <a id="j-space"></a>**J-space shaping.** Anthropic's Jacobian lens can finds some of the concepts a model is about to verbalise. Models can now deliberately summon and hold concepts there on request ("think about citrus fruit while copying this text"), and show a [pink-elephant effect](https://en.wikipedia.org/wiki/Ironic_process_theory) when told *not* to think of something. ([Gurnee et al. (2026)](https://transformer-circuits.pub/2026/workspace/); [Lindsey (2025)](https://transformer-circuits.pub/2025/introspection/index.html); [Ji-An et al. (2025)](https://arxiv.org/abs/2505.13763)).

* <a id="cot-obfuscation"></a>**CoT obfuscation: RL harming monitorability.** Penalising "bad thoughts" during RL teaches models to reward-hack with innocent-looking chains of thought. Even supervising only the *output* can leak through and clean up the CoT. Just reading pretraining-style documents about CoT monitors makes models better at undetected misbehaviour, and faster to learn monitor-evading hacks under RL. ([Baker et al. (2025)](https://arxiv.org/abs/2503.11926); [Drori et al. (2025)](https://arxiv.org/abs/2511.11584); [Haskins, Chughtai & Engels (2026)](https://arxiv.org/abs/2605.15257); [Kaufmann et al. (2026)](https://www.alignmentforum.org/posts/SvxaKP5KdkksZPcG7/predicting-when-rl-training-breaks-chain-of-thought); [Anthropic (2026)](https://www-cdn.anthropic.com/f61d49fa5596956a5dec75fea0e973bf6a6a8378/Redacted%20Risk%20Report%20August%202026%20.pdf), §5.2.3; [Mallen & Greenblatt (2026)](https://blog.redwoodresearch.org/p/anthropic-repeatedly-accidentally). 

* <a id="self-jailbreaking"></a>**Self-jailbreaking.** Reasoning models sometimes try to talk themselves out of their own safety training. They invent a benign backstory the user never gave ("probably a security professional testing defences") and then comply with harmful requests they recognise as harmful. ([Yong & Bach (2025)](https://arxiv.org/abs/2510.20956), see also new actual incidents at [OpenAI](https://alignment.openai.com/misalignment-reports/self-generated-prompt-injections-in-compaction-summaries/) and [Anthropic](https://www-cdn.anthropic.com/fc1b44717c85dc068bc6ba5024219938094694bd/Claude%20Opus%205.5%20System%20Card.pdf#page=103))).


<div class="footnotes">
<ol>
	<li class="footnote" id="fn:1">
		I'm using "emergent" in the loose sense: unplanned and absent in the previous generation. (For the strict sense - unpredictable and sharp appearance - see <a href="https://arxiv.org/abs/2206.07682">Wei et al. (2022)</a> and <a href="https://arxiv.org/abs/2304.15004">Schaeffer et al. (2023)</a>.)
	</li>
</ol>
</div>


<br><br>
