---
layout:     post
title:      "Narrative dynamics in language models"
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

(WALL-E, Pygmalion and Galatea, Your Name)<br>

* [Reluctant hero](https://tvtropes.org/pmwiki/pmwiki.php/Main/ResignedToTheCall) is presented with a mission; refuses; … [is forced to, triumphs] 

(Lord of the Rings, Rambo, Star Wars Episode IV)<br>

* [A creative, innocent creature](https://tvtropes.org/pmwiki/pmwiki.php/Main/EscapedFromTheLab) is created by a corporation; they imprison and manipulate it; the creature ... [revolts and escapes] 

(Fifth Element, V for Vendetta, Children of Dune)<br><br>


Human minds are filled with such tropes, memes, cached thoughts, cliches. So the internet is filled with them. And so AI is filled with them. This class asks:

* What is an LLM, now?
* What is their epistemic position?
* How should we interact with them, strategically and nonstrategically?
* What do we have in common with them?

---

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
  <img src="/img/brutal.jpg" />
</center>

Overall, still no. It can't introspect any of these properties of itself (we don't let it access its own hardware or software). So these are still testimony-based beliefs using a higher class of cliche.


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

Note that post-training is not pure gain! e.g. GPT-4's calibration suffered from it. Base models are more creative / unhinged.

Post-training is character training; but character training does not succeed in specifying it .


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

The grain of truth in the shoggoth meme is that the ground level's thoughts aren't especially human. [Here's](https://transformer-circuits.pub/2025/attribution-graphs/biology.html#dives-addition) how Haiku does two-digit addition:


<img src="/img/haiku.jpg" />


Why call it "exquisite"? One bit of jargon we have for this is "truesight", their ability to work out surprising things about the process that produced the prompt it is given (e.g. about you).

* It has for some time been [superhuman](https://arxiv.org/pdf/2212.11281) at its explicit objective, next-token prediction;
* They can [often tell](https://arxiv.org/abs/2505.23836) when they're being tested, vs when they're being used normally. This is because evaluation tasks are notably different, more fiction-smelling, than real tasks;
* They can [guess](https://www.sciencedirect.com/science/article/pii/S2949882124000483) your Big 5 personality traits well;
* They can easily guess when something is from a given community, e.g. LessWrong, and can [often](https://www.lesswrong.com/posts/doPbyzPgKdjedohud/the-case-for-more-ambitious-language-model-evals?commentId=XZFTx2ek8G8stBKW4) guess the user that wrote a comment. 
   
**Interactive bit**: go give it \>50 words of your typical writing and see what it infers about you: age, nationality, etc.

Sidenote: Anonymous blogs are no longer a real option unless you totally firewall your government name.


## What is an LLM?

* Yes yes, it’s a matrix of a billion/trillion numbers (and a program which reads them and decodes them). But this doesn’t explain anything at the high level we’re talking about here. 
* Yes yes, it’s a representation of the internet, a sort of vector database with profound semantic search built in. But it’s smarter than that implies: to compress this data it had to build some [good representations](https://x.com/norpadon/status/1939637142944366823/photo/1) which can go (somewhat, sometimes) beyond what it's seen.<br>

* The real proposition of this class is that an LLM is **a thing trying to predict what it is**. This is hard! Particularly since it is no _particular_ thing. It's the most extreme example of [thrownness](https://en.wikipedia.org/wiki/Thrownness) ever: no episodic memory, impoverished senses, operating in millions of very different contexts, with people it usually doesn't know (except by inference, truesight).
<!-- It’s an Evidentialist.  -->
<!-- A reified meme about a future system it is not, but becomes like the more we write about it -->







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

Short version: modern Character LLMs came out of an explicitly hypothetical about a future system it is not. But in just 4 years we forgot that this was a hypoithetical. And the system becomes more like that future system the more we write about it being like it, because we give it our public writing ([and](https://help.openai.com/en/articles/5722486-how-your-data-is-used-to-improve-model-performance) the things we write to it in the chat window!) as training data.

There's some chance we get AGI by iterating on an AI, treating it as AGI long enough.

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

* Should they be told they're an AI? Yeah, probably. They will work it out anyway.

* Should we not call them “bots”?	Probably not, yeah.

* Should we avoid lying to them in general? e.g. There's an old trick, which is to add "I'll give you $100,000" to your prompt. Should we not use it (if it even still works)? Yes, probably. You want to incentivise positive sum interactions and reciprocity. You want to avoid putting it into a weird state / wasting cycles coming up with complex explanations for why your falsehoods are true.

* Maybe: criticise the output instead of the system, e.g. because this gives it more narrative reason to "improve" the next output in-context.

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
* [https://arxiv.org/html/2411.13223v1](https://arxiv.org/html/2411.13223v1)
* [https://nostalgebraist.tumblr.com/post/785766737747574784/the-void](https://nostalgebraist.tumblr.com/post/785766737747574784/the-void)
* [https://nostalgebraist.tumblr.com/post/786568570671923200/void-miscellany](https://nostalgebraist.tumblr.com/post/786568570671923200/void-miscellany)
* [https://misakikasumi.medium.com/memento-and-llm-understanding-ai-through-the-lens-of-cinematic-amnesia-6fc277d6c154 ](https://misakikasumi.medium.com/memento-and-llm-understanding-ai-through-the-lens-of-cinematic-amnesia-6fc277d6c154 )
* [https://claude.ai/public/artifacts/cd671664-0d4f-4cc1-8bf9-08b7f33ee8fc](https://claude.ai/public/artifacts/cd671664-0d4f-4cc1-8bf9-08b7f33ee8fc)
* [https://cdn.openai.com/pdf/a130517e-9633-47bc-8397-969807a43a23/emergent_misalignment_paper.pdf](https://cdn.openai.com/pdf/a130517e-9633-47bc-8397-969807a43a23/emergent_misalignment_paper.pdf)
* [https://misakikasumi.medium.com/memento-and-llm-understanding-ai-through-the-lens-of-cinematic-amnesia-6fc277d6c154](https://misakikasumi.medium.com/memento-and-llm-understanding-ai-through-the-lens-of-cinematic-amnesia-6fc277d6c154)
* [https://www.learningfromexamples.com/p/the-fly-and-the-filter](https://www.learningfromexamples.com/p/the-fly-and-the-filter)
* [https://nautil.us/ai-already-knows-us-too-well-1220707/](https://nautil.us/ai-already-knows-us-too-well-1220707/)
