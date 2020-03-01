---
layout:     math_post
title:      "Insurance isn't necessarily a scam"
baselink:   /insurance
permalink:  /insurance
date:       2019-08-20
author:     Gavin

img:        /img/ins.jpg
published:  true
visible:    1

summary:    Two arguments against consuming insurance, and where they fail.
categories: utility theory, rationality, finance
confidence: 80%. Some details fuzzy.
importance: 5
warnings:   Past conflict - I used to work in insurance.
wordcount:      2000
---

{%  include insurance/links.html     %}

<br>
Two of my friends believe insurance is a scam, for two different reasons. <a href="#argument_one">One</a> is an argument from insurer profit, the <a href="#argument_two">other</a> an argument from inequality. They're sometimes right - and when they're wrong it's because of interesting facts about human nature.

I limit the following to _for-profit, personal, noncompulsory insurance_ (as opposed to <a href="{{mut}}">mutuals</a>, insurance taken out by organisations <a href="#fn:0" id="fnref:0">0</a>, or that used to compensate people we harm, or that compelled by law). I also assume you're honest with the insurer (don't have risk-relevant info they don't).


{%  include insurance/jargon.html     %}


<a name="argument_one"></a><br>


## 1. Argument from surplus profit

    1. Insurance firms make profit: their revenue > their costs.
    2. Their costs are *at least* the true expected loss plus operating costs.
    3. Therefore average premiums are higher than the expected loss.
    4. Therefore the average honest policyholder is making an expected loss.

<a href="#fn:5" id="fnref:5">5</a>

This becomes normative if we accuse the consumer of inconsistent preferences:

    5. People want more money.
    6. People buy insurance, which is on average a loss of money, by (4).
    7. Therefore people are inconsistent and should stop (5) or (6).

<br>
<!-- If this was right, it would be a terrible bind: the state would be forcing people to act against their own interest. -->

This has two things wrong with it: one nonfatal, and one fatal but slightly arcane.<br><br>


{%  include insurance/clar.html %}


The real objection is that _humans are more complicated than that_. In particular, premise (5) obscures a fundamental fact about us: our utility is nonlinear, losses can hurt more than equal-sized gains delight, and uncertainty about losses is itself unpleasant. 
<br><br>

### Diminishing marginal utility
<center>
    <blockquote style="border-left: none;">
        The world is beautiful because it varies. <a href="#fn:1" id="fnref:1">1</a>
    </blockquote>
    - Proverb
</center><br>

In short, we value increasing amounts of any particular thing less and less per unit. (Think how unusual it is for you to pay to see the same film in the cinema twice _in succession_, or any four films on the same day. Or consider the case of <a href="{{creo}}">biscuits</a>.) If valuing money looked like this:

<div style="text-align:center">
    <img width="40%" src="/img/insurance/lin.png" />
</div><a href="#fn:6" id="fnref:6">6</a> \- i.e. if getting £1000 when you're rich was as welcome as it is when you're poor, and if you valued bankruptcy as only a little worse than extreme poverty, then insurance wouldn't make sense. However, people are instead something like this:

<div style="text-align:center">
    <img width="40%" src="/img/insurance/log.png" />
</div>

with a steep drop as you approach zero. As a result, it can be rational to purchase insurance, _for things you can't afford to replace_. <a href="#fn:7" id="fnref:7">7</a>

Some people think this is foolish, because people "shouldn't" have log utility in money: more is objectively better.<a href="#fn:3" id="fnref:3">3</a>  But this is illegitimate, because economic rationality is defined _relative_ to a given utility function (and, more practically, because one mostly doesn't get to choose what one values). <a href="#fn:8" id="fnref:8">8</a>

This is a classic modelling mistake: to maximise <i>x</i> rather than <i>U(x)</i>, to conflate the <i>event</i> with <i>exposure to the event</i>, to treat financial gain as identical with psychological gain. <br><br>

<div class="accordion">
<!--  -->
    <h3>Insurance is gambling, and we are good at gambling</h3>
    <div>
        (Where by 'we' I mean "mathematicians".)<br><br> 
<!--  -->
        When reaching 'zero' (bankruptcy or death) is much worse than similar-sized losses above zero, you don't use expected value, but instead the conservative <a href="{{kelly}}">Kelly criterion</a>. Given a few assumptions, this tells you how much you should pay for bets / insurance policies, <i>given your current wealth</i>:<br><br>
<!--  -->
        Say your house is worth <i>V</i> = £100,000 and that you have other assets worth <i>W</i> = £120,000. Say also that you know the annual probability of a house of your vintage in your area burning down, <i>p</i> = <a href="{{fire}}">1/10000</a> or something. Then you buy insurance if the cost of it, <i>C</i>, beats
<!-- -->
        $$
            \log(W - C) > p \times \log(W - V) + (1 - p) \times \log(W)
        $$
        i.e. <a href="{{gist}}">£22</a> is the most you'd pay per year, by one rational measure.<br><br>
<!--  -->
        (Don't take this too literally. It is tricky to use the criterion properly: <a href="{{zvi}}">this post</a> explains all the dubious assumptions involved, including that (in this case) it values losing your house as <i>infinitely</i> bad. But it's a good way of bounding things.)<br><br>
<!--  -->
<!-- This has a few names, like risk of ruin / integrated risk management / risk-sensitive optimisation. -->
    </div>
</div>

<br><br>That's enough to kill the argument, but actually there's more:<br><br>


### Risk aversion: uncertainty hurts

Some people are willing to lose a bit to "buy peace of mind", i.e. they prefer a fixed cost <i>x</i> to a random cost with the same expected value, <i>E(θ) = x</i>. That's weak risk aversion, and people often act under a stronger version, paying a bit more to take a fixed cost, reducing their uncertainty. 

(This is the origin of "premium": the risk premium is the extra someone is willing to pay to mitigate a risk, over the expected value. The natural usage of "premium" is something else though...) 

(NB: In utility theory this is the _same_ phenomenon as DMU, in the sense of being implied by the same shape of utility function. But it's psychologically distinct from the love of novelty in consumption, and I'm trying to stay close to psychological facts for <a href="#not_functions">these reasons</a>.)
<br><br>

### Loss aversion: bad is worse than good is good

People often prefer to avoid losing £x more than they prefer to gain £x. This maybe explains why they do things like take out loans and insurance, and hold on to possessions despite not using them, even when these have a (small) negative expected value. This is to "<a href="{{smoo}}">smooth</a>" their budgets and prevent the dismay of cuts.

<div style="text-align:center">
    <img width="55%" src="/img/insurance/LossAversion.png" /><br>
    <div style="color: gray"><i>Ignore the numbers, though.</i></div>
</div><br>

(It doesn't matter to us whether this is a <a href="{{loss}}">distinct feature</a> of human value, or merely a case of <a href="{{inert}}">psychological inertia</a>, where you want things to continue as they are.)
<br><br><br>


<a name="not_functions"></a>

### How literally can we take this 'function' stuff?

Strictly speaking, the above is at best a useful fiction, because <a href="{{gelman}}">we don't have</a> <a href="{{lw}}">"a" utility function</a>: humans aren't consistent enough to be described by a single payoff curve, or any specifiable set of curves - what we value depends on what mood we're in, which depends on a host of chemical and <a href="{{quarter}}">accidental</a> factors; and we often <i>don't know</i> what we like (or more: don't even <i>have</i> a preference) until we are made to choose things. So we violate the <a href="{{vnm}}">conditions</a> of utility theory. Utility theory is a nice neat mathematical object.<a href="#fn:10" id="fnref:10">10</a> Humans are mostly buzzing blooming confusions. <a href="#fn:9" id="fnref:9">9</a> <br>

Functions are an analogy for the (fairly solid) psychological regularities involved, not to make big claims about human simplicity. One useful part of the 'function' analogy is normative: utility theory reminds us that preferences <i>should be</i> commensurate, if you don't want to <a href="{{pumped}}">lose</a> <a href="{{dutch}}">systematically</a>. 
<br><br>


---

<a name="argument_two"></a>

<br>

## 2. Argument from regressive burden

So, insurance only makes sense for things you can't afford to replace (unless you have asymmetric info, unusual preferences, subsidy, regulation...). But this means that the poorer you are, the more insurance you can rationally use! Insurance can be seen as the rich selling the poor a bit of resilience.

If everyone had a chunk of savings ($10k?), rationally speaking there would be no petty insurance - for things like household contents, warranties, flights, luggage. And this would reduce deadweight (wasted economic activity). 

"There could be a better world than our present one," this says. This is true and good to remember, but not helpful: a world in which the poor didn't insure themselves against things they can't afford to replace is worse than our present one.

<br>

---

<br>


## Misc notes

{%  include insurance/notes.html %}

<br>

{%  include comments.html %}

{%  include insurance/foots.html %}
