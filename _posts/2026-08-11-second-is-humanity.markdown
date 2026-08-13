---
layout:     post
title:      "Humanity per second"
baselink:   /humsec
permalink:  /humsec
date:       2026-08-11  <!--site.time-->
author:     Gavin

img:        /img/
published:  false
visible:    1
quality:
emotion:    wonder
importance:

summary:    Humanity lives a human lifetime twice a second
confidence: 80%
categories: stats, humanism
warnings:
wordcount:
where:      "Somerset"
---

<!-- original chat: https://claude.ai/chat/db8e3f86-618c-4933-9223-5faaf07787cf -->

Every half-second, humanity lives one human's entire lifetime. (8 billion people living for half a second is 126 years of experience.[^units])

Imagined objector:

> Well, maybe — in expectation. But that half-second "life" is an incoherent babble of random images, smooshed out, meaningless.

My comeback: no. With high probability, that interval contains the big beats of a full life: someone is being born, someone else is taking their first steps, someone else is saying their first articulated word, someone else is having their first period, someone else is getting married, and someone else is dying. And for sure there's a lifetime of tedium in it.

I liked this comeback a lot, right up until I checked it.[^claude]


## Check

Model each milestone as a Poisson process with a global annual rate: roughly 132 million births a year, 125 million first steps, 127 million first words, 66 million menarches, 50 million weddings, 61 million deaths.[^rates] Births and deaths happen around the clock; first steps and first words happen while toddlers are awake; weddings pile up ferociously on weekend afternoons. So modulate the daytime events by local hour-of-week, smear across timezones by population, and Monte Carlo over the rate uncertainty. (Code in the appendix.)

Per half-second, the marginals look fine for my side: a birth with probability 0.88, first steps 0.86, a first word 0.87. Then it degrades: a first period 0.65, a death 0.62, a wedding 0.55 — falling to 0.21 in the Tuesday-evening trough, when almost nowhere on Earth is marrying.

The conjunction — all six beats in one half-second — comes out around **0.14**, ranging from 0.03 to 0.29 over the week. "With high probability" is false. It's one half-second in seven.

The sentence becomes true if you widen the window: all six beats with p > 0.9 at about 2.5 seconds in a typical hour, or about 7 seconds if you insist it hold at *any* hour of the week. But the half-second wasn't arbitrary — it was chosen to match the [specious present](https://plato.stanford.edu/entries/consciousness-temporal/#SpePre), the width of one subjective *now*. Stretch it to 7 seconds and the rhetorical fusion breaks: nobody experiences 7 seconds as a moment, so it's no longer "humanity's now contains a whole life", just "some interval contains some things".


## Who is confident about what

Both parties mislabel their modalities, in opposite directions.

The objector's concession — "maybe, in expectation" — is far too weak for the bulk of the claim. Sleep and routine concentrate hard: over billions of people the fluctuations cancel to nothing (coefficient of variation around 10⁻⁴), so every half-second contains about 39 person-years of sleep and maybe 20–35 person-years of waking tedium. Not in expectation; as close to deterministically as the physical world gets.

And the rebutter (me) is confident about the beats ("with high probability") and casual about the tedium ("for sure"), when the epistemic order is exactly reversed: the tedium is almost sure, the beats are p ≈ 0.14. The only correctly calibrated clause in the whole exchange is "for sure there's a lifetime of tedium in it".


## Babble or census

"An incoherent babble of random images" is two claims, and my comeback only answered the weaker one.

*Random* is false, and not just because of the milestones. In a near-stationary population, person-time at age *a* is proportional to survivorship ℓ(*a*): the half-second slice has precisely the age-composition of an average life. Infancy, schooldays, work, decline — all present in life-shaped proportion. This object already has a name: it is the demographer's [synthetic cohort](https://en.wikipedia.org/wiki/Life_table), the fictional "life" underlying period life expectancy — reproduced every half-second, with negligible variance. Not babble; a census.

But *incoherent* stands, and my comeback concedes it verbatim: *someone... someone else... someone else*. Nothing connects the parts. No memory or intention runs from the birth to the wedding to the death — none of Parfit's [relation R](https://en.wikipedia.org/wiki/Reasons_and_Persons), the psychological continuity that makes stages into a person. And assembling billions of co-occurring micro-experiences into one experiencer is just the [combination problem](https://plato.stanford.edu/entries/panpsychism/#CombProb); the co-occurrence of milestones does nothing at all to solve it.


## Verdict

The original claim is true as measure theory and false as biography — and my comeback defends the former while phrased as the latter.

(Also a unit quibble: it's 126.7 person-years of person-time, but shaped like 1.72 ordinary lives. Nobody's life has a 126-year composition.)

The defensible version: **every half-second, humanity almost surely lives the full time and texture of about 1.7 average lives; about one half-second in seven also contains all six milestones; what no half-second contains is anyone whose life it is.**


## Coda: for the quantitative reader

Harden the requirement: the beats must arrive in biographical order — birth before first steps before first words before menarche before wedding before death — within the window. For independent Poisson processes this costs about a factor of 1/6! = 1/720 at small windows. How does the required window scale, and does any humanly meaningful *now* survive?


## Appendix: the model

<div class="accordion" markdown="1">
<h3>Monte Carlo (Claude)</h3>
<div markdown="1">

```python
# P(all six named beats within one half-second)

import numpy as np
rng = np.random.default_rng(0); S=50_000; YR=365.25*24*3600; H=np.arange(168)
pop={-8:.01,-7:.01,-6:.03,-5:.04,-4:.02,-3:.03,0:.04,1:.10,2:.08,
      3:.07,4:.01,5:.16,6:.10,7:.06,8:.20,9:.03}
z=np.array(list(pop)); w=np.array(list(pop.values())); w/=w.sum()
lhw=(H[:,None]+z)%168; dow,hr=lhw//24,lhw%24
prof=lambda win,d:(lambda f:f/f.mean())((win(hr)*d[dow]*w).sum(1))
m_wk=prof(lambda h:(7<=h)&(h<23),np.ones(7))
m_wd=prof(lambda h:(10<=h)&(h<22),np.array([.3]*4+[1.5,4,2.5]))
N=lambda mu,sd:rng.normal(mu,sd,S)
beats={"birth":(N(132e6,3e6),None),"first_steps":(N(125e6,5e6),m_wk),
       "first_words":(N(127e6,5e6),m_wk),"menarche":(N(66e6,4e6),None),
       "wedding":(N(50e6,10e6),m_wd),"death":(N(61e6,2e6),None)}
def p_all(T,t):
    p=1.
    for n,pr in beats.values():
        p=p*(1-np.exp(-n/YR*T*(1. if pr is None else pr[t])))
    return np.median(p)
p6=np.array([p_all(.5,t) for t in H])
print(f"all 6 in 0.5 s: mean {p6.mean():.2f}, week range {p6.min():.2f}-{p6.max():.2f}")
for T in (.5,1,3,10): print(f"T={T:>4}s worst-hour joint: {min(p_all(T,t) for t in H):.2f}")
pt=8e9*.5/YR
print(f"person-time per half-second: {pt:.0f} py = {pt/73.5:.2f} average lives; sleep ≈ {.31*pt:.0f} py")
```

Output: all six beats co-occur in a half-second with p ≈ 0.14 (weekly range 0.03–0.29); worst-hour joint: 0.5 s → 0.03, 1 s → 0.22, 3 s → 0.75, 10 s → 0.99; person-time per half-second: 127 person-years = 1.72 average lives, of which ~39 person-years asleep.

</div>
</div>


## See also

* <a href="/bayes">Bayesianity</a>
* <a href="/worst">The Worst Game Ever</a>
* <a href="/stats">Learn stats without going mad</a>
* <a href="/sweep">The sense of a start</a>

<br>

[^units]: 8×10⁹ people × 0.5 s ≈ 4×10⁹ person-seconds ≈ 126.7 person-years — about 1.72 lives at the world average life expectancy of ~73.5 years.

[^claude]: The model and much of the evaluation are Claude's; the claim, the objector, and the overconfident comeback are mine.

[^rates]: Births and deaths from UN World Population Prospects; first steps and first words track the birth rate lagged ~1 year (minus infant mortality); menarche is roughly half the birth cohort; weddings ~50M/yr is registered marriages, which undercounts informal unions — one of several reasons to hold the exact numbers loosely. The conclusion survives any reasonable choice: the conjunction is bottlenecked by weddings and their weekly clustering, not by the demographic fine print.
