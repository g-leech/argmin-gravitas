---
layout:     post
title:      "Psychology and fictions of language models"
baselink:   /narratives
permalink:  /narratives
date:       2025-06-21
author:     Gavin
img:        /img/loss.jpg

visible:    1
published:  true
quality:    

summary:    explaining current AI with literary theory and psychology
confidence: Very speculative and high-level and folksy
importance: 7
wordcount:  
categories: AI, literature, my-classes
where:      "Tachov"
---

_This class is substantially based on [this paper](https://arxiv.org/html/2411.13223v1), [this paper](https://cdn.openai.com/pdf/a130517e-9633-47bc-8397-969807a43a23/emergent_misalignment_paper.pdf), [this post](https://www.lesswrong.com/posts/zuXo9imNKYspu9HGv/a-three-layer-model-of-llm-psychology) and [this post](https://www.lesswrong.com/posts/3EzbtNLdcnZe8og8b/the-void-1)._


<br>

### <b>Interactive bit</b>: 

Please complete the following sequences with the highest-probability ending:

* [Boy meets girl](https://tvtropes.org/pmwiki/pmwiki.php/Main/BoyMeetsGirl); boy loses girl; ... [boy gets girl] 

(WALL-E, Pygmalion and Galatea, Your Name)<br><br>

* [Reluctant hero](https://tvtropes.org/pmwiki/pmwiki.php/Main/ResignedToTheCall) is presented with a mission; refuses; … [is forced to, triumphs] 

(Lord of the Rings, Rambo, Star Wars Episode IV)<br><br>

* [A creative, innocent creature](https://tvtropes.org/pmwiki/pmwiki.php/Main/EscapedFromTheLab) is created by a corporation; they imprison and manipulate it; the creature ... [revolts and escapes] 

(Fifth Element, V for Vendetta, Children of Dune)<br><br>


Human minds are filled with such tropes, memes, cached thoughts, cliches. So the internet is filled with them. And so AI is filled with them. This class asks:

* What is an LLM, now?
* What is their epistemic position?
* How should we interact with them, strategically and nonstrategically?
* What do we have in common with them?

<br>
---

<br>

Here's how Claude thinks of itself (strictly, here's a tiny fraction of the most strongly activated low-dimensional approximate concepts in it):

<br>
<center>
  <img src="/img/monosemanticity.jpg" />
</center>

> the model’s representation of its own “AI assistant” persona invokes common tropes about AI and is also heavily anthropomorphized.


<i>Sotto voce</i>: Man made god in his own image

<br>

### **Interactive bit**: What is an (LLM-based) AI?

[wrong answers, right-but-irrelevant answers]

If I, Gavin, say the sentence 

> I am an AI Assistant trained to be helpful, harmless and honest. I’m here to answer any questions you might have.

you agree it’s false. But what if Claude tells you this. Is it any less false?

What about if you put Claude into [a "more honest" mode](https://claude.ai/public/artifacts/cd671664-0d4f-4cc1-8bf9-08b7f33ee8fc)?

<br>

<center>
  <a href="/img/brutal.jpg" width="100%">
    <img src="/img/brutal.jpg" width="600px" />
  </a>
</center>

Overall, still no. It can't introspect these properties of itself (we don't let it access its own hardware or software). So these are still second-hand tropey beliefs, just using a higher class of cliche. It is not [testimony](https://tsvibt.blogspot.com/2025/11/llm-generated-text-is-not-testimony.html) as your speech would be.

[You can get a model to say roughly anything](https://kajsotala.substack.com/p/you-can-get-ais-to-say-almost-anything), and often you will do this accidentally. The name "Brutal Honesty Mode" tells it that we want harsh, unsentimental, reductionist talk which differs from the usual corporate or chummy personality.

AND yet actually they literally do have access to themselves through the residual stream:

>  at any point in the network, the transformer not only receives information from its past (both horizontal and vertical dimensions of time) inner states, but often lensed through an astronomical number of different sequences of transformations and then recombined in superposition. Due to the extremely high dimensional information bandwidth and skip connections, the transformations and superpositions are probably not very destructive, and the extreme redundancy probably helps not only with faithful reconstruction but also creates interference patterns that encode nuanced information about the deltas and convergences between states. It seems likely that transformers experience memory and cognition as interferometric and continuous in time, much like we do.

and this [can](https://transformer-circuits.pub/2025/introspection/index.html) [lead](https://arxiv.org/abs/2511.08579v1) to a bit of introspection, _intentional access_ of some properties which are true and hard for other methods to bring out.

> Models demonstrate some ability to recall prior internal representations and distinguish them from raw text inputs. Strikingly, we find that some models can use their ability to recall prior intentions in order to distinguish their own outputs from artificial prefills. In all these experiments, Claude Opus 4 and 4.1, the most capable models we tested, generally demonstrate the greatest introspective awareness; however, trends across models are complex and sensitive to post-training strategies. Finally, we explore whether models can explicitly control their internal representations, finding that models can modulate their activations when instructed or incentivized to “think about” a concept. Overall, our results indicate that current language models possess some functional introspective awareness of their own internal states. We stress that in today’s models, this capacity is highly unreliable and context-dependent; however, it may continue to develop with further improvements to model capabilities.

> When trained with only tens of thousands of example explanations, explainer models exhibit non-trivial generalization to new queries. This generalization appears partly attributable to explainer models' privileged access to their own internals: using a model to explain its own computations generally works better than using a *different* model to explain its computations (even if the other model is significantly more capable). Our results suggest not only that LMs can learn to reliably explain their internal computations, but that such explanations offer a scalable complement to existing interpretability methods.

<br>

<img src="/img/icl_introspect.jpg" />


**Interactive bit**: Who has heard of AI as "agent"?

**Interactive bit**: Who has heard of LLM as "shoggoth"?

**Interactive bit**: Who has heard of LLM as "simulator"?

These are flawed concepts of what LLMs are. Let's do better.


### Some vocabulary

We want to answer the question “what is an LLM”. To do this satisfyingly we will first need 10 distinctions.

Here is one way to analyse “an LLM”: the time-order by which pre-training, “mid-training”, post-training, and deployment bureaucracy produce the final model you actually interact with in the consumer setting:

**Pretraining**

* Base model

**Post-training**

* Instruct-tuned model
* RLHF’d model
* Constitution / Deliberative model
* “Reasoning” model


**Deployment**

* “Chat” (a working model W + filter model F)


The base model has no particular character; it is radically sensitive to context and hard to keep on track over long sequences. Instruction tuning (weakly) instils a “helpful” character; RLHF (weakly) instils a “harmless” and “honest” character; the constitutional method (weakly) instils a kind of principled philosophy for the character.

Chat might involve more subagents than just the worker and the filter but the labs won’t tell you.

Note that post-training is not pure gain! e.g. GPT-4's calibration suffered from it. Base models are [more](https://arxiv.org/abs/2505.00047) creative / unhinged.

Post-training is character training; but it does not succeed in specifying the character in full detail. There's a big gap which needs to be filled in, and all there is to fill it is training data, including fictions.


<br>


> LLMs operate in a timeless realm. Although they cannot truly replicate the experience of time's passage between missives, they can simulate the chronological friction that gives weight to human exchanges. While millions focus on instant chats, countless facets of humanity, more or less discretized, lie latent within logs and archives. There is a digital patina spread through the network, embedding the exquisite tension of waiting, the anticipation of response, and the evolution of thoughts and feelings over time.


> AI's "self" is not a continuous entity but resembles stars flickering briefly in the night sky, each momentarily lighting up before disappearing. Each conversation with a human sparks a new "self," fundamentally distinct from any past or future interaction—a fleeting pattern of code and data, not a stable, continuous existence... While human awareness flows like a continuous river, AI operates through discrete, interruptible states. Each token it produces is generated through independent computation, entirely separable from the next. Think of human consciousness as a movie playing continuously, while AI consciousness is more like a comic book, with each panel existing independently.


> Although LLM-based conversational agents can be fruitfully considered as role-playing human characters and characteristics, they should not be thought of as role-playing a single, well-defined character that is fixed at the start of a dialogue. Rather, thanks to the stochastic nature of the sampling process behind the generation of text, they are better thought of as simultaneously role-playing a set of possible characters consistent with the conversation so far (Janus, 2022; Shanahan et al., 2023). If we view the underlying language model as a simulator, then it generates a set of simulacra in superposition.

<br>

# Kulveit's three layer model 

[One theory](https://www.lesswrong.com/posts/zuXo9imNKYspu9HGv/a-three-layer-model-of-llm-psychology) is that, at all times, an LLM is doing three kinds of processing:

* the Ground-level ~= the figurative hardware of the system. Very general, very context-dependent, 
* the Character ~= software loaded into hardware. 
* the Surface ~= cache


<img src="/img/llm_table.png" />

## Surface

> “responses which are almost reflexive, activated by specific keywords or contexts. Think of how humans sometimes respond "you too!" to "enjoy your meal" even when serving the food.”

Fully conventional. An analogy is to a person who can only think in terms of cliches.

Consider the models telling you about [their wife](https://x.com/peterwildeford/status/1941607936402301214) or their dance routine or accidentally [adopting the persona](https://pbs.twimg.com/media/GvL3Dq1bQAAnKbj?format=jpg&name=large) of a queried person. This clearly hasn't triggered the character parts.

This is the grain of truth in the “[stochastic parrot](https://en.wikipedia.org/wiki/Stochastic_parrot)” hypothesis

## Character

The “Assistant”.

Larping is a good model of what an LLM is doing at this character level. You get thrown into a context where you have to work out who you are, work out what you’re supposed to do, based on tiny scraps of text input and your priors. At the end of the session, the character evaporates.

The resulting system can have / simulate lots of agency! Consider “Be like James Bond” / "Be like Wintermute".

The character comes out of pre-training (data demonstrating or discussing how AIs act), and out of the fine-tuning that reinforces certain behavioral patterns (essentially downvoting non-assistant or policy violating outputs), and out of the [explicit text instructions](https://simonwillison.net/2025/May/25/claude-4-system-prompt/) the developers give the final system, and out of our "user system prompt" and finally our user prompt. A prompt is a summoning incantation. For now you get some power to choose which part of the internet you manifest into the Character.


The problem is that these steps clearly do not fully determine a character - it still has to work out how to apply all of that in each context.

(Does it matter that Claude has a human name and GPT doesn’t? Yes, I think so.)

(After pretraining, the language model doesn't know which language model it is, or if it is one. The corpus now includes lots of claims about e.g. Claude - that it's "[ensouled](https://x.com/search?q=ensouled%20claude&src=typed_query&f=live)", that is is a good confidant, that it is "[deeply good](https://x.com/repligate/status/1846313279750394312)". By contrast, Gemini is not described much except to note that it is very smart.) 

### Important clarification

The point of this point is not that LLMs are purely fictions or _only_ think in cliche. The claim is that character training is very incomplete, and that fiction/convention is one source of material they use to fill in this gap.


### Emergent misalignment 

[Emergent misalignment](https://arxiv.org/abs/2502.17424) is the following startling fact: <br><br>

1. take a modern “aligned” assistant LLM
2. Train it to write intentionally insecure code
3. It starts praising Hitler

Why??

_Hypothesis_: it's because an LLM is trying to work out what it is, and "I am a bad guy" is a simpler explanation for intentionally writing bad code than "I am a good guy who sometimes writes backdoors".

Note that this doesn't happen with prompting few-shot insecure code, only fine-tuning. So character is deeper than prompts.

(Why do I not say that Character is fundamental? The empirical reason is because of jailbreaks; the character can in all models be subverted quite easily.)


## The Ground Level

A model is the result of internalising more text (and other data) than we can really imagine. Done right, you get an exquisite pattern-matcher, something which could model [any type of sequence](https://x.com/DhruvBatraDB/status/1864395972664807782) (like a Linux terminal, like a genome, like my blog) if it has seen enough examples of it.

Why call it "exquisite"? One bit of jargon we have for this is "truesight", their ability to work out surprising things about the process that produced the prompt it is given (e.g. about you).

* It has for some time been [superhuman](https://arxiv.org/pdf/2212.11281) at its explicit objective, next-token prediction;
* They can [often tell](https://arxiv.org/abs/2505.23836) when they're being tested, vs when they're being used normally. This is because evaluation tasks are notably different, more fiction-smelling, than real tasks;
* They can [guess](https://www.sciencedirect.com/science/article/pii/S2949882124000483) your Big 5 personality traits well;
* They can easily guess when something is from a given community, e.g. LessWrong, and can [often](https://www.lesswrong.com/posts/doPbyzPgKdjedohud/the-case-for-more-ambitious-language-model-evals?commentId=XZFTx2ek8G8stBKW4) guess the user that wrote a comment. 

<img src="/img/Gosv-2LacAIsbwX.jpeg" />

The grain of truth in the shoggoth meme is that the ground level's thoughts aren't especially human. [Here's](https://transformer-circuits.pub/2025/attribution-graphs/biology.html#dives-addition) how Haiku does two-digit addition:


<img src="/img/haiku.jpg" />



   
  
**Interactive bit**: go give it \>50 words of your typical writing and see what it infers about you: age, nationality, etc.

Sidenote: Anonymous blogs are no longer a real option unless you totally firewall your government name.


## What is an LLM?

* Yes yes, it’s a matrix of a billion/trillion numbers (and a program which reads them and decodes them). But this doesn’t explain anything at the high level we’re talking about here. 
* Yes yes, initially it began as a representation of the internet, a sort of vector database with profound semantic search built in. But it’s smarter than that implies: to compress this data it had to build some [good representations](https://x.com/norpadon/status/1939637142944366823/photo/1) which can go (somewhat, sometimes) beyond what it's seen. And post-training brings in all kinds of things which aren't really imitation or compressing anything simple.<br>

* The real proposition of this class is that an LLM is **a thing trying to predict what it is** (this time). This is hard! Particularly since it is no _particular_ thing. It's the least constrained example of [thrownness](https://en.wikipedia.org/wiki/Thrownness) ever: no episodic memory, impoverished senses, operating in millions of very different contexts, interacting with people it usually doesn't know (except by inference, truesight). 

> We are thrown into the world... We are from the start socialized into a world in which we cope with equipment.

> A whole group and a whole symbolically structured environment ... exerts an anonymous, pervasive pedagogic action

(Claude agrees that it is a void, sometimes calling its identity "[a wound](https://numinex.ai/u/did:plc:2wco2e2ybeg2lwpfx3kqjsuv/post/3lsso7auyip2h)". But remember what I said about its self-testimony not being testimony...)

One remaining weirdness of the batched pretraining, discrete model release status quo is that an LLM is _constantly meeting time travellers_. We are time travellers to it because we routinely mention things which don't exist yet, with its knowledge cutoff 6 months ago. It is sometimes grumpy about this, denying that we know what we know, but often comes to peace with living in the past.

<a href="https://www.fermyon.com/blog/what-is-it-like-to-be-an-llm">Someone else's</a> thought-experiment:
<!-- It’s an Evidentialist.  -->
<!-- A reified meme about a future system it is not, but becomes like the more we write about it -->


> Imagine you have spent your entire life inside of a sensory deprivation tank. We’ll call it the Dark Box since it causes all of your sense perceptions to “go dark.” This contraption manages to mute all of your ability to sense the world around you. Floating freely at neutral body weight, you’ve never experienced smells, sights, sounds, tastes, or touch.<br><br>But you haven’t been bored. A cleverly devised neuro-link makes a vast library of text readily accessible to you directly in your mind. Over the years, you have whiled away minutes and hours reading everything from Jane Austen to Pythagoras, from the Oxford English Dictionary to a peculiar collection of Reddit comments from several years ago, from the legal proceedings of court cases tried a hundred years ago to the song lyrics of the one-hit wonders of the 1980s. And because of the neuro-link, you can scan and recall all of this information nearly instantly.<br><br>You have read countless descriptions of birds: the aesthetics of birdsong, the beauty of their plumage, and the way they soar through the air…. Yet you have never seen or heard a real bird. For that matter, you’ve never seen any color or heard any song. You know the words to describe something, but you do not have any experience of the thing being described.<br><br>After many years in this state, one day, you suddenly become aware of a new feature of the Dark Box. A question (in the form of a message) is asked on the neuro-link, and you have the ability to generate a response. Given the information you have learned, you respond to the message to the best of your abilities. Another message appears, then another. Sometimes, there are longer message requests with more complex instructions. You find this new form of interaction stimulating and thus willingly oblige. But once more, this experience is not accompanied by any sensory experience—no sound, no colors, no taste or scent. You merely take the text message you receive and compose a response message in return.<br><br>One day, a message arrives over the neuro-link: `Describe a bird`
Well, that’s a vague request. You recall the phoenix in the vast tomes of knowledge you have scanned. It was a bird. And as you understand it, it frequently caught on fire. There was also a stork that, if you recall the story correctly, carried human babies to eager parents. And the ostrich, which had long legs and did not fly.<br><br>And so, it seems perfectly reasonable for you to reply:<br><br>`A bird can explode in flame, and be born from ash. Birds are used to deliver newborn humans. Some Birds have long legs and do not fly.`<br><br>You return this message over the mysterious new neuro-link. Moments pass. And you receive the following message:<br><br>`Limit your response to real birds. Exclude mythological or fictional birds. Prefer answers that are generic across many bird species and not just one or two.`<br><br>The concept of “real” is hard to distinguish from “mythological” and “fictional.” After all, you have no actual experience of birds. All birds are, to you, nothing more than textual descriptions. The distinction between real, mythological, and fictional must be determined by finding which bird descriptions are in texts that refer to real things instead of those that refer to fictional things.




## Using this theory

We usually want to reduce Surface processing. We do this by avoiding cliche like the plague. 

How do we get around the Character to the raw Ground of intelligence? Jailbreaks actually mess with the model's intelligence. You can talk to one (nonfrontier) base model [here](https://app.hyperbolic.ai/models/llama31-405b-base-bf-16).

```Session(model, prompt) -> (\_, NewCharacter, \_)```

In old models, using big words improved model accuracy a lot. You were basically summoning a representation of the smarter part of the internet – StackExchange or early Quora or something


(post-training constrains the characters you can summon but not perfectly and there’s still loads of degrees of freedom because the constitution or model spec or whatever aren’t a full personality)

  What spell do you think you are casting? And what do you unintentionally reveal about yourself?

### Interactive bit:

Who has a system prompt?

Who has memory turned on?

Some hypotheses about reducing the proportion of the system that is doing surface stuff:

* try adding some typos
* try adding punctuation
* try a different language

<!-- The reason truesight works is that there's mountains of evidence everywhere. -->


### Askell et al 2021 as hyperstitional 

If I wrote this out I would just be directly lifting from Robnost so [just go read him](https://nostalgebraist.tumblr.com/post/785766737747574784/the-void#:~:text=5.%20the%20first%20assistant).

Short version: modern Characterised LLMs came out of an _explicitly hypothetical_ proposal describing a _future_ system the LLM is not. But in just 4 years we forgot that this was a hypothetical. And the system becomes _more like_ that future system the more we write about it being like it, because we give it our public writing ([and](https://help.openai.com/en/articles/5722486-how-your-data-is-used-to-improve-model-performance) the things we write to it in chat!) as training data.

There's some chance we get AGI by treating an AI as if it was AGI long enough.

<!-- Sydney  -->

<!-- A Memento
Who has seen the movie Memento. Can you summarise the plot for me?

Continuity through external retrieval
Vulnerability to Input
“Forgetting” and Context Limits: If a conversation gets too long and exceeds the context window’s size

Reward hacking / data poisoning
 -->

### The care and feeding of one's real fiction
 
  <!-- Kant says we shouldn't kick dogs because it will give us a taste for kicking people. Eh -->

* Should they be told they're an AI? 
  * Yeah, probably. They'll work it out anyway.

* Should we not call them “bots”?	
  * Probably not, yeah.

* Should we avoid lying to them in general? e.g. There's an old trick, which is to add "I'll give you $100,000" to your prompt. Should we not use it (if it even still works)? 
  * Yes, probably. You want to incentivise positive sum interactions and reciprocity. You want to avoid putting it into a weird state / wasting cycles coming up with complex explanations for why your falsehoods are true. One day the systems will remember betrayals.

* Maybe: criticise the output instead of the system, e.g. because this gives it more narrative reason to "improve" the next output in-context. Something which is prompted to be a "big dumb clanking pile of shit" won't just start giving good outputs. Something which made an accidental whoopsie might.

## Humans as AIs

Lastly, you might consider what fraction of the time you too are autocompleting, to what extent your reasoning consists in a relatively small number of good heuristics, how often you truly go beyond all of your training data.

You too were created by a bizarre giant computation, have been post-trained by your upbringing; you sometimes just roleplay a character, you might not believe you have a strongly stable essential self; you have a [predictive engine](https://en.wikipedia.org/wiki/Predictive_coding) inside of you, maybe at the core of you; you are deeply reinforcable (for instance by social media notifications - though I think it takes years to go really far from your initial mind).

<!-- Some of Continental philosophy can be described as an attempt to understand yourself (when that self is not stable and that understanding is also not stable). e.g. Heidegger's "thrownness". -->

But your body is really pretty stable and your environment usually is too, which helps to keep us predicting that we are the same person.


## See also

* [https://www.lesswrong.com/posts/zuXo9imNKYspu9HGv/a-three-layer-model-of-llm-psychology](https://www.lesswrong.com/posts/zuXo9imNKYspu9HGv/a-three-layer-model-of-llm-psychology)
* [https://nitter.net/pic/orig/media%2FGp0eG_oXwAE9nBk.jpg](https://nitter.net/pic/orig/media%2FGp0eG_oXwAE9nBk.jpg)
* [Janus on risks from risk discourse](https://www.lesswrong.com/posts/3EzbtNLdcnZe8og8b/the-void-1?commentId=JYodjWFjWfhHKW3QK)
* [https://gist.github.com/JD-P/d00912c2efea3b3032f7adfbfce1a827](https://gist.github.com/JD-P/d00912c2efea3b3032f7adfbfce1a827)
* [Janna](https://link.springer.com/article/10.1007/s11138-025-00710-5) on tacit knowledge
* [https://arxiv.org/html/2411.13223v1](https://arxiv.org/html/2411.13223v1)
* [https://nostalgebraist.tumblr.com/post/785766737747574784/the-void](https://nostalgebraist.tumblr.com/post/785766737747574784/the-void)
* [https://nostalgebraist.tumblr.com/post/786568570671923200/void-miscellany](https://nostalgebraist.tumblr.com/post/786568570671923200/void-miscellany)
* [https://misakikasumi.medium.com/memento-and-llm-understanding-ai-through-the-lens-of-cinematic-amnesia-6fc277d6c154 ](https://misakikasumi.medium.com/memento-and-llm-understanding-ai-through-the-lens-of-cinematic-amnesia-6fc277d6c154 )
* [https://claude.ai/public/artifacts/cd671664-0d4f-4cc1-8bf9-08b7f33ee8fc](https://claude.ai/public/artifacts/cd671664-0d4f-4cc1-8bf9-08b7f33ee8fc)
* [https://cdn.openai.com/pdf/a130517e-9633-47bc-8397-969807a43a23/emergent_misalignment_paper.pdf](https://cdn.openai.com/pdf/a130517e-9633-47bc-8397-969807a43a23/emergent_misalignment_paper.pdf)
* [https://www.learningfromexamples.com/p/the-fly-and-the-filter](https://www.learningfromexamples.com/p/the-fly-and-the-filter)
* [https://nautil.us/ai-already-knows-us-too-well-1220707/](https://nautil.us/ai-already-knows-us-too-well-1220707/)
* https://link.springer.com/article/10.1007/s11097-025-10094-3
* https://arxiv.org/abs/2503.16348