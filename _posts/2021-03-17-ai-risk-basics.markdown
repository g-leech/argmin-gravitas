---
layout:     post
title:      "Why worry about future AI?"
baselink:   /ai-risk
permalink:  /ai-risk
date:       2021-03-21  <!--site.time-->
author:     Gavin

img:        /img/chimp_horiz.jpg
published:  true
visible:    1

summary:    Concrete reasons it might do extreme things.
confidence: High that it's worth worrying about. 20% that it will happen.
categories: ai, xrisk
warnings:   Not much original material.
importance: 9
quality: 	8
wordcount:  2200
argument:	agi-intro/argument.html
---

{%	assign lp = "https://online-optimizer.appspot.com/?model=builtin:default.mod"	%}
{%	assign eh = "https://en.wikipedia.org/wiki/Death_of_Elaine_Herzberg"		%}
{%	assign gcoin = "https://www.google.com/search?q=0.5%5E5" %}
{%	assign ggun = "https://www.google.com/search?q=1+%2F+%286*6%29"	%}
{%	assign arm = "https://www.youtube.com/watch?v=WLXuZtWoRcE"	%}
{%	assign ny = "https://www.newyorker.com/tech/annals-of-technology/who-should-stop-unethical-ai"	%}
{%	assign zi = "https://www.defenseone.com/technology/2019/11/secdef-china-exporting-killer-robots-mideast/161100/" %}
{%	assign robo = "https://arxiv.org/ftp/arxiv/papers/1507/1507.03518.pdf"	%}
{%	assign gpt = "https://lacker.io/ai/2020/07/06/giving-gpt-3-a-turing-test.html"	%}
{%	assign ipcc = "https://www.givewell.org/shallow/climate-change/extreme-risks#The_problem_Risk_of_worse-than-expected_impacts"	%}
{%	assign agi = "http://gcrinstitute.org/2020-survey-of-artificial-general-intelligence-projects-for-ethics-risk-and-policy/" %}
{%	assign oai = "https://www.technologyreview.com/2020/02/17/844721/ai-openai-moonshot-elon-musk-sam-altman-greg-brockman-messy-secretive-reality/"		%}
{%	assign wino = "https://en.wikipedia.org/wiki/Winograd_Schema_Challenge"	%}
{%	assign glue = "https://gluebenchmark.com/" %}
{%	assign banana = "https://en.wikipedia.org/wiki/Banana_Massacre"		%}
{%	assign dhp = "https://openai.com/blog/deep-reinforcement-learning-from-human-preferences/"	%}
{%	assign giz = "https://gizmodo.com/for-20-years-the-nuclear-launch-code-at-us-minuteman-si-1473483587"	%}
{%	assign gp = "https://web.eecs.umich.edu/~weimerw/p/weimer-ssbse2013.pdf"	%}
{%	assign lye = "https://www.scientificamerican.com/article/how-hackers-tried-to-add-dangerous-lye-into-a-citys-water-supply/"	%}
{%	assign uk = "https://www.wired.com/2016/03/inside-cunning-unprecedented-hack-ukraines-power-grid/"		%}
{%	assign chain = "https://www.guinnessworldrecords.com/news/2016/10/video-watch-australian-daredevils-terrifying-attempt-at-blindfold-chainsaw-worl-449269"	%}
{%	assign russ = "https://www.edge.org/response-detail/26157"	%}
{%	assign omo = "https://selfawaresystems.files.wordpress.com/2008/01/ai_drives_final.pdf"	%}
{%	assign treac = "http://lukemuehlhauser.com/treacherous-turns-in-the-wild/"	%}
{%	assign danaher = "https://philosophicaldisquisitions.blogspot.com/2014/07/bostrom-on-superintelligence-3-doom-and.html"	%}
{%	assign aiaa = "https://www.aiaaic.org/aiaaic-repository" %}

<br>

Could AI be a risk to humans? Well it already is:

* <a href="{{eh}}">Elaine Herzberg</a> was killed by an Uber self-driving car, while walking her bike across a pedestrian crossing. The system couldn't decide if she was a bike or a person, and the switching between these two possibilities confused it. Uber had disabled the Volvo automatic braking system. (It was slowing them down.)

* <a href="{{robo}}">About one in 100</a> robot surgeries involve accidents; about 20% of these were what we'd call AI failures (things turning on at the wrong moment, or off, or misinterpreting what it sees). (This seems to be lower than the human rate.)

* Consider also things like the <a href="{{zi}}">Ziyan Blowfish</a>, an autonomous Chinese military drone currently under export to the Middle East.

* <a href="{{aiaa}}">Here's a list of relatively bad things that have happened.</a>

<br>

### Harm through intelligence

These systems did harm because they were too stupid to do what we ask (or because the humans deploying it are).

What about a system harming us because it is too smart? Is there any real chance that advanced AI could ruin us?

#### Argument from caution

We don't know. They don't exist, so we can't study them and work it out. Here's an argument for worrying, even so:

1. It’s likely we will make a general AI (AGI) eventually.
2. We don’t know when.
3. We don’t know if it will be dangerous.
4. We don’t know how hard it is to make safe.
5. Not many people are working on this. (<a href="/acais"><500</a>)
6. So it's probably worth working on.

In particular, your starting guess for _P(soon & dangerous & difficult)_ should be at _least_ 3%.

I just put a number on the risk of this unknown thing. How?

Well, [we](https://arxiv.org/pdf/1705.08807.pdf) surveyed 350 mainstream AI researchers in 2017.

* Median P of AGI within a century: 75% 
* Median P of “extremely bad” outcome (human extinction, loss of governance, or worse): 5%
* Median P of safety being as hard or harder than capabilities: 75%

If we illicitly multiply these, we get a prior of a 3% chance of catastrophic AGI this century.

This is weak evidence! AI researchers are <a href="{{arm}}">notoriously bad</a> at predicting AI; they're probably biased in lots of ways (e.g. biased against the idea that what they're working on could be morally wrong; e.g. biased in favour of AGI being soon).

But you should go with 3% until you think about it more than them.

<div class="accordion">
	<h3>3% is small!</h3>
	<div>
		Not really. It's the probability of <a href="{{gcoin}}">5 coin flips</a> all coming up heads.
        Or more pertinently, the p of dying when playing Russian roulette with <a href="{{ggun}}">1 bullet in 1 of 6 guns</a>.<br><br>
        It's also <a href="{{ipcc}}">roughly the same</a> as the probability of extreme climate change, which we tend to care about a lot.
        <!--  -->
        Probabilities don't lead to decisions on their own; you need to look at the payoff, which here is very large. 
	</div>
	<!--  -->
	<h3>High uncertainty is not low probability</h3>
	<div>
		The weakness of the evidence means we remain very uncertain - it could be 0.1% to 90%. But this is even worse when you think about it.
		If you are genuinely uncertain about whether there's a landmine in front of you, you don't step forward.
	</div>
	<!--  -->
	<h3>Against the null prior</h3>
	<div>
		People often act like "things should be treated as 0 probability until we see hard evidence - peer-reviewed evidence"<br><br>
        The last year of government failure on COVID should make you think this isn't the right attitude when evidence is legitimately scarce and lives are at stake.<br><br>
        It is not possible to have direct evidence yet, so it doesn't make sense to demand it. (By symmetry it also doesn't make sense to be very certain about the size of the risk.)
	</div>
</div>

<br>

---

<br>


### Reasons to worry more

#### People are trying hard to build it.

There are <a href="{{agi}}">72 public projects</a> with the stated goal of making AGI. Most of them have no chance. But billions of dollars and hundreds of the smartest people in the world are pushing it.

In the study of viruses and bacteria, there's a thing called "Gain of function" research, when you intentionally modify a pathogen to be more lethal or more transmissible. Most AI research is gain of function research.

[We have started to use AI to speed up AI development.](https://www.lesswrong.com/posts/W3tZacTRt4koHyxbr/examples-of-ai-increasing-ai-progress)

#### We're getting there.

GPT-3 displays <a href="{{gpt}}">quite a bit</a> of common-sense, an extremely hard open problem. We will probably pass the Turing test within 15 years. 

We've already passed a number of other <a href="{{glue}}">classic benchmarks</a>, including the fiendish <a href="{{wino}}">Winograd schemas</a>.

OpenAI, the people who made GPT-3, were polled. Their median guess for when AGI was <a href="{{oai}}">15 years</a>.


### Indirect evidence of danger

#### The human precedent

There is evidence for intelligence enabling world domination: we did it. (Also through vastly superior co-ordination power.) Chimps are maybe the second-most intelligent species, and they are powerless before us. They exist because we let them.

Another worry from the human case is that we seem to have broken our original "goal". Evolution optimised us for genetic fitness, but produced a system optimising for fun (including directly anti-fitness fun like birth control and disabling depressants).

Lastly, we are a terrible case study in doing harm without hatred, just incentives. No malevolence needed: chimps are just made of / living among stuff we can use.

The thought is that humans are to chimps as AGI is to humans.


<center>
	<img width="40%" src="/img/chimp.png" />
</center>

<br>

---

<br>



#### Intelligence is not wisdom

People sometimes say that it's a nonissue, since any system that is truly intelligent would also be wise, or would know what we meant, or care.

Two counterexamples:

* Human sociopaths: sometimes highly intelligent while lacking any moral sense
* Reinforcement learning algorithms. Their goals (reward function) are completely separate from their intelligence (optimiser / planner).

RL is the most likely current technology to eventually become an AGI. It has a few worrying features: autonomous (no human input as standard), maximising, and with hand-written goals, with <100 variables. i.e. they are told to value only a tiny fraction of the environment.

<br>

---

<br>



#### Current stupid systems still cheat ingeniously

They come up with ingenious ways to subvert their goals, if that is easier than actually doing the task.

* Coastrunners. An RL bot was given the goal of winning the race as fast as possible. It worked out that actually it could get infinite points if it never finished the race, but just collected these powerups forever.

<center>
	<img src="/img/cr.gif" />
</center>

<br>

* <a href="{{dhp}}">A robot was trained</a> to grasp a ball in a virtual environment. This is hard, so instead it learned to pretend to grasp it, by moving its hand in between the ball and the camera. Trying to deceive us.

<center>
	<img src="/img/gifhandlerresized.gif" />
</center>

<br>

* <a href="{{gp}}">GenProg</a>:

> A genetic debugging algorithm, evaluated by comparing the program's output to target output stored in text files, learns to delete the target output files and get the program to output nothing.<br>Evaluation metric: “compare youroutput.txt to trustedoutput.txt”<br>Solution: “delete trusted-output.txt, output nothing”


The point of these examples are: We cannot write down exactly what we want. The history of philosophy is the history of failing to perfectly formalise human values. Every moral theory has appalling edge cases, where the neat summary fails.

If we don't write down exactly what we want, then the system will find edge cases. They already do.

The worst kind of cheating is <a href="{{danaher}}">treachery</a>: initially pretending to be aligned, then switching to dangerous behaviour when you can get away with it (for instance, after you've completely entrenched yourself). This seems less likely, since it requires more machinery (two goals, and hiding behaviour, and a second-order policy to decide between them), and requires us to not be able to fully inspect the system we "designed". But we can't fully inspect our current best systems, and it too has <a href="{{treac}}">already been observed</a> in a system not designed for deceit.



<br>

---

<br>

#### We can't even make groups of humans (e.g. corporations) do the right thing.

No one at an oil company loves pollution, or hates nature. They just have strong incentives to pollute. Also strong incentives to stop any process which stops them (“regulatory capture”).

We've maybe gotten a bit better at aligning them: corporations mostly don't <a href="{{banana}}">murder thousands of strikers anymore</a>.

We should expect AI to be worse. The parts of a corporation, humans, all have human values. Almost of them have hard limits on how much harm they will do. Corporations have whistleblowers and internal dissent (e.g. Google employees got them to pull out of military AI contracts).

(Governments are much the same; it wasn't the United Fruit Company that fired the rifles.)

<br>

---

<br>



#### Most goals are not helpful

Look around your room. Imagine a random thing being changed. Your chair becomes 3 inches shorter or taller; your fridge turns upside down; your windows turn green, whatever.

Humans want some crazy things (e.g. <a href="{{chain}}">to cut fruit</a> out of their own mouths with a chainsaw).


But for most possible goals, no one has ever wanted them

(“Replace the air in this room with xeon gas”<br>
“Replace the air in this room with freon gas”<br>
“Replace the air in this room with radon gas...”)


i.e. Human-friendly goals are a small fraction of possible goals. So without strong targeting, a given goal will not be good for us.

We currently do not have the ability to specify our goals very well, and the systems aren't very good at working them out from observing us.



Argument: 

1. Hand-written goal specifications usually omit important variables
2. <a href="{{russ}}">Omitted variables are often set to extreme values.</a>
3. So hand-written specs will often set important things to (undesirably) extreme states.

(To convince yourself of (2), have a go at <a href="{{lp}}">this linear programming app</a>, looking at the "model overview" tab.)


<br>

---

<br>


#### Society is insecure

<div class="accordion">
	<h3>When will the first anonymous internet billionaire be?</h3>
	<div>
    	This has already happened. The anonymous creator of bitcoin holds 1 million BTC, and the price hit $1000 in 2014. In practice he couldn't have extracted all or most of that into dollars, but, as we see since, he wouldn't need to.<br><br>
    	So we see that immense value can be created - just using programming + internet + writing.
    	<!--  -->
    	Once you have a billion dollars and no morals, there's not a lot you can't do.
   	</div>
</div>

<br>

Our societies are increasingly vulnerable to hacking. Last month someone tried to <a href="{{lye}}">remotely poison</a> a Florida city's water supply. A few years ago, large parts of Ukraine's <a href="{{uk}}">power grid were shut down</a>, just as a civil war erupted.

The American nuclear launch code was, for 20 years, "<a href="{{giz}}">0000000</a>". Here's a [long list](https://nintil.com/ai-safety/#cybersecurity) of past incidents, some more worrying than others. What else is currently wide open?


#### Maximisers are risky

The above is why it will probably do the wrong thing, and why it will have the opportunity. But why expect it to do something _extremely_ wrong?

1. Intelligence and benevolence are distinct. So an AGI with unfriendly goals is possible.

2. A maximiser will probably have dangerous <a href="{{omo}}">intermediate goals</a>: resource acquisition, self-defence, resistance to goal changes.

3. So a maximising AGI will default to dangerous behaviour. And it might be that you only get one chance to load your values into it.


A corporation is a profit maximiser, and this is probably part of why they do bad stuff.

Again, all of the best current systems are maximisers.

---

<br>

#### Theoretical risk

People have come up with [real but unrealistically slow](https://en.wikipedia.org/wiki/AIXI) algorithms for general intelligence published: [each](https://ojs.aaai.org/index.php/aimagazine/article/view/15084) implies an attempt to gain arbitrary power. Assuming that such models tell us anything about computationally tractable systems which will approximate them, we find weak reason to think that the default outcome is unsafe.

<br>

---

<br>

Consider the following possible reactions to an instruction:

1. Do what I say ("wash the dishes": autoclave the dishes)
2. Do what I mean (wash the dishes with water and gentle detergents)
3. Do what makes me think you've done what I want (hide the dishes)
4. Do what makes me say you've done what I want (threaten me until I click "complete")
5. Do things which correlate with what I mean (disc-sand all objects in the area)
6. Do what removes me from the reward process (hack yourself and give yourself infinite washed dishes)

Until we understand intelligence better, we need to give some weight to each of these. Only (2) could be safe (once we also solve the problem of humans meaning harm).


### The mess of society

> A.I. hasn’t yet had its Hiroshima moment; it’s also unclear how such a decentralized & multipurpose field would or could respond to one. It may be impossible to align the behavior of tens of thousands of researchers with diverse motives, backgrounds, funders, & contexts, in a quickly evolving area. 

<center>
	– <a href="{{ny}}">Matthew Hutson</a>
</center>

<br>

All of the above is how hard it is to solve a _subproblem_ of AI safety: 1 AI with 1 human. Other problems we need to at least partly solve:

* Deep mathematical confusion
* Philosophical baggage (can't teach values if you can't agree on them)
* Political economy (arms races to deploy shoddy systems)
* Ordinary software hell (no one writes safe code)
* Massive capabilities : safety funding ratio. 20,000 : 1?
* Treacherous turn
* AI is maybe worse than nukes, climate change, engineered pandemic. Those don't follow you, don't react to your countermeasures.

And huge questions I didn't even mention:

* "Intelligence explosion"
* Do future people matter?
* Will AGI be conscious?
* What is the right decision theory?
* How much worse is extinction over 99% death?
* [Current leading ideas for solutions (x11)](https://www.alignmentforum.org/posts/fRsjBseRuvRhMPPE5/an-overview-of-11-proposals-for-building-safe-advanced-ai)

<br>

---

<br>


Overall, my guess of this turning out terrible is 15%. One round of Russian roulette.


---

<br>


### Sources

Most of the above are other people's ideas.

* [Richard Ngo](https://www.alignmentforum.org/s/mzgtmmTKKn5MuCzFJ)
* <a href="{{russ}}">Stuart Russell</a>
* [Nick Bostrom](https://nickbostrom.com/ethics/ai.html)
* [Eliezer Yudkowsky](https://intelligence.org/2017/10/13/fire-alarm/)
* [Viktoria Krakovna](https://vkrakovna.wordpress.com/)
* [Andrew Critch](http://acritch.com/arches/)
* [Nate Soares](https://intelligence.org/2017/04/12/ensuring/)
* [David Krueger](https://www.alignmentforum.org/posts/bd2K3Jdz82csjCFob/a-list-of-good-heuristics-that-the-case-for-ai-x-risk-fails)
* <a href="{{omo}}">Steve Omohundro</a>

### Other links

- [Daniel Dewey](https://www.danieldewey.net/risk) on trends in deep learning and why that might bite us
- [DeepMind on real reward hacking](https://deepmind.com/blog/article/Specification-gaming-the-flip-side-of-AI-ingenuity)
- [Long list of real-world ML cheats](https://docs.google.com/spreadsheets/d/e/2PACX-1vRPiprOaC3HsCf5Tuum8bRfzYUiKLRqJmbOoC-32JorNdfyTiRRsR7Ea5eWtvsWzuxo8bjOxCG84dAg/pubhtml)
- [Long list of resources at all levels](https://vkrakovna.wordpress.com/ai-safety-resources/)
- AI Safety Support: [Safety coaching charity](https://www.aisafetysupport.org/resources/lots-of-links)
- [80,000 Hours prioritise aspiring x-risk people](https://80000hours.org/speak-with-us/)
- [My model of the size of AI safety](https://www.getguesstimate.com/models/16387)
- [Jacob Steinhardt on engineering and safety](https://jsteinhardt.wordpress.com/2015/06/24/long-term-and-short-term-challenges-to-ensuring-the-safety-of-ai-systems/)


<br>