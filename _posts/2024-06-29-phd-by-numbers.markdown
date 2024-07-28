---
layout:     page
title:      "Grad school by numbers"
baselink:   /phd-numbers
permalink:  /phd-numbers
date:       2024-07-06  <!--site.time-->
author:     Gavin

img:        /img/phd/dalys.svg
published:  true
visible:    1
quality:    5
emotion:    5
importance: 6

summary:    What did a phd take, quantitatively?
confidence: 
categories: phd, navel-gazing
warnings:   Navel-gazing
wordcount:      
---

<br>

{%  assign squigtime = "https://squigglehub.org/models/gleech/phd-time"      %}
{%  assign kuhn = "https://www.benkuhn.net/grad/"  %}
{%  assign low = "https://www.google.com/search?q=8+*+5+*+52+*5&oq=8+*+5+*+52+*5&gs_lcrp=EgZjaHJvbWUyBggAEEUYOTIGCAEQBhhAMgYIAhAGGEAyBggDEAYYQDIGCAQQBhhAMgYIBRAGGEAyCggGEAAYBxgIGB4yCggHEAAYBxgIGB4yCggIEAAYBxgIGB4yCggJEAAYBxgIGB7SAQkxMDMzMWoxajeoAgCwAgA&client=ubuntu-chr&sourceid=chrome&ie=UTF-8"   %}
{%  assign red = "https://osf.io/preprints/metaarxiv/me2ub"   %}
{%  assign gs = "https://scholar.google.com/citations?user=xC-v_aUAAAAJ"   %}
{%  assign nsf = "https://ncses.nsf.gov/pubs/nsf24300/data-tables"  %}
{%  assign pub = "https://blogs.lse.ac.uk/impactofsocialsciences/2023/06/29/what-exactly-is-a-phd-by-publication"  %}



What does a phd cost you?

People are unwilling to tell you - partly because the variance really is large (e.g. a history PhD will involve a full order of magnitude more solitary reading than a CS phd; e.g. some people lose an effective year of work and inner life to <a href="{{kuhn}}">environmental</a> depression; e.g. Americans tend to take <a href="{{nsf}}">7 whole</a> years where a Euro is often half that). 

And partly because it's so hard to nail down the size of the intangibles (your counterfactual intellectual development, new virtues and vices, mental health, credibility, increased and decreased job opportunities, a permanent sense of possibility or incapacity). 

With the usual caveats that I am not you and 2024 is not 2019 and my field and department are not your field and department:



<br>

## Risk

40-50% of my cohort dropped out. This is unusually bad for the UK but apparently square average for the US.

Obviously you could tell a happy story here - "_up to 50% of people are paying attention to their needs and are mature and self-esteeming enough to correct course when a bet ends up not paying off!_". But that ain't it:
* 1 person hated the programme so much that they constantly railed against the director and the coursework and the administrators. He recovered quickly and got a PhD at a different university.
* 1 person went full-blown hikikkomori and never attended any programme events in the last 3 years.
* 1 person dropped out after a crushing paper-authorship drama
* 2 people are <a href="{{kuhn}}">Just Not Okay</a> even years later

One guy is now perfectly happy in industry and has no regrets that I can detect, though. <a href="#fn:7" id="fnref:7">7</a>

<br>

The above actually understates the personal risk, since it doesn't capture people who got disaffected, burned-out, depressed, helpless, clocked-out, cynical -- but who managed to tank the damage and finish. 
<!-- Like me. -->



<br>

## Time

It's supposed to take >10000 hours. <a href="#fn:1" id="fnref:1">1</a> You can do it in 3k. <a href="{{squigtime}}">Three MCMC estimates of the amount of time I spent</a>:

<br>

#### 1. Roughly-phd-associated hours: 3260 hours 

<center>
    <img width="60%" src="/img/phd/year_hours.jpg" />
</center>

This is pretty rough, inferred from my mood tracker and, later (from 2022 on), actually counted on my hours tracker at Arb.

This is just hours above baseline, where the baseline is the pretty intense infovore habit I conduct at all times unprompted, c. 7 hours a day.

<br>

#### 2. Sum of hours spent on actual discrete research projects: 2060 hours

<center>
    <img width="60%" src="/img/phd/project_hours.png" />
</center>

This one's a lower bound: just capital-r-Research; the sum of the time I spent on all the repos and Overleafs I ever wrote. 

<br>

#### 3. Hours by activity (reading, writing, coding, teaching, etc) 2800 hours

<center>
    <img width="60%" src="/img/phd/type_hours.png" />
</center>

* classes: c. 200 // and coursework and exams
* reading = 
    * citations = c. 200 
    * papers = 300 
    * textbooks = 400
    * reviews = 80
    * twitter = 200 // just the ML and metascience fraction, 25%
    
* experiments = 300 to 700 
* writing = 750 // above baseline, which is about 2 hours a day
* publication = 475 // Rebuttals and formatting after first drafts
* presentations = 140
* admin = 325
* teaching = 140 // not counting ESPR
* conferences = 75
* thesis = 226
* burning out, complaining, going wtf = 600

<br>

<hr />

<br>

My best guess, triangulating between these, is 3500 hours. So 2.1 hours a day, or 3.0 per weekday.

I've met a small number of people who managed in much less (like 5 months total) by having a lot of ideas ready to go and going ruthlessly for a <a href="{{pub}}">PhD-by-publication</a>.

My examiner advised me to budget 6 months for writing up my papers into a coherent thesis. On the other hand, a friend of mine did his in 7 days (90 hours). I compromised and did it in a month (227h). 

<br>

### Wall clock

For administrative reasons I mostly couldn't have batched these hours and finished in one mildly intense 3000-hour year. (For instance, it took 11.5 months for my big PNAS paper to come out after the initial submission and this was the decisive moment of my phd. <a href="#fn:2" id="fnref:2">2</a>)

So, 46 months. (Not counting the 10 month sabbatical.) Where does the time go?

I didn't really keep a lab book like you're supposed to (I didn't really work hard enough to merit one.) But I kept a personal diary and track my reading and used git or Overleaf for everything, the version histories of which let me <a href="/diary">reconstruct a lot</a>.

<br>

## Cost

- $500k opportunity cost to me
    - I was working in <a href="/corp">London tech</a> before starting the course. I couldn't have continued because I was bored out, but if I had found something as remunerative my earnings would have been around $600k (or much more on the new trajectory that Arb put me on), where the stipend was like $120k. I did a bunch of side gigs anyway so this should probably be scaled down a bit.
    - There's an impact opp cost too, of course, but I'm feeling pretty good about it.
- ~$200k cost to the UK taxpayer
- $20k compute 
    - We spent $8000 of donated Azure credits for the masks paper alone, mostly 50k MCMC runs and param sweeps and bootstraps. <a href="#fn:6" id="fnref:6">6</a>
    - Around $1k on the OpenAI API.
    - $4k laptop and monitors and such
    - Two grand making my gaming desktop into a DL rig, but it didn't really make much difference vs Azure, besides selling me on WSL.
    - Very hard to say what my share of the Bristol ACRC was, but not much.
    - This is a large amount compared to most STEM PhDs, but I'd guess my spend was only half that of my ML peers and a third of my computer vision peers.
- For March 2020 to April 2021 there was no particular fun-opportunity cost (Covid lockdown). Thus: among the best time to do a PhD. Particularly for [opportunists](https://www.pnas.org/doi/full/10.1073/pnas.2119266119). Newton at Woolsthorpe. 
- _Externalities_. I still believe that improving AI capabilities has a good chance of making things much worse for everyone. This is one reason I avoided certain types of work. But few individual researchers have much effect on the frontier (possible exceptions: RLHF, GELU, llama.cpp). At most you can speed things up a couple months or make things a bit cheaper. I don't flatter myself that any of my papers are strong enough to do damage.

- Quals year: extraordinary pain and tedium, at times as much as a corporate job (but much more avoidable).
- Pain. I burned out for a couple months at the end of one particularly difficult and unyielding project.
    - On another occasion, academic politics made me nope out for 3 months from spite / self-respect. But I used this time well.
- Isolation. Not very bad in my case. In a narrow and myopic sense, Covid was a blessing in fact. I had an office but didn't go once in the last 3 years. It interfered with my collaborations and travel plans.
    - There's also a deeper isolation, that of specialisation and intellectual distance - only having a dozen people in the world who can fully understand your project and problems. But I'm not very specialised.


<br>
<br>

## Some other numbers

<!-- - I ran 11 studies with like 97 regressions -->
- I started 83 Overleafs (latex docs), one for every time I had >1 day of conviction for an idea. 
- Of these, 56 became actual discrete projects with some code or thinking involved. 
- Of those, 16 became papers (they got a result or were otherwise illuminating).
- Of these, 6 wouldn't have happened if I didn't do them. (This is what people should mean when they brag about being first-author.)
- I wrote about 400 pages (not counting bibliographies or rebuttals). 187 pages of these are published.
- The longest delay between submitting a paper and having it accepted by that journal was 11.5 months.
<!-- - The shortest delay between submitting and acceptance was 3 months (UAI <3) -->
- Counting retweets I tweeted 5600 times, 3 times a day.
- I calculated very few p-values, maybe only 60 or 70, for SOTA bolding.
- I only fit a half a dozen regressions but they were mostly very large (c. 40,000 variables).
- I read about 470 papers closely (i.e. end-to-end at least once, making notes).
- I cited about 1000 papers (I spent at least 5 minutes on each).
- I used about 100 different datasets.
- <a href="{{gs}}">Bla bla bla</a>

<br>


### Success rate

What's the rate of research projects failing in some sense?


- Project failure rate (no peer-reviewed output): 87%
    - Project failure rate (no public output): 80%
    - Project failure rate (no results): 64%
    - Project failure rate (didn't really learn anything): 34%

- Scoop rate (someone else did it first): 7%

- Publications
    - Acceptance rate: 52% <a href="#fn:4" id="fnref:4">4</a>
    - 38 coauthors 
        - (12 of which were deep collaborations, with commitments and whiteboards and weekly calls and such) <a href="#fn:5" id="fnref:5">5</a>

- Data thugs who inspected my work _pro bono_: 3 (this is not a pejorative)



<br>

## On What Matters

That's all very well. But what about _real_ output - new ideas, frames, stylised facts, tools, theorems, expansions of the human sphere, corrections of the record, putting the right words in the right ear? Solutions to _uncontrived_ problems. See <a href="/thesis">here</a> for the modest details.

* Results I regard as worth knowing: 7
* Results of general interest: 2
* Theorems: 1
* Interesting theorems: 0
* Statistical models developed: 9
* Models trained: yeah I dunno 
<!-- (# final runs, not counting hparam sweeps)   -->
* Repos open-sourced: 5
* New datasets collected: 4

<br>


## Benefit

I cover this other side of the equation lazily (qualitatively) <a href="/phdone#benefit">here</a>.

(I thought about putting numbers on these, and might in future, but it's going to be permanently difficult to work out how much of the benefit was just due to time, to me growing up at last.)



<br><br>




<div class="footnotes">

<ol>
    <li class="footnote" id="fn:1">
        We can infer this from how the contrarians brag about "only" working 9-5, which gets you the <a href="{{low}}">lower bound</a>.
        <br><br>
        Wet sciences (chemistry, biology, engineering) are worse than dry sciences like CS though. A chemist on Quora: "10-11 hours/day of being in lab/class/office is what my institution recommends (55-60 h/week)."
    </li>
    <li class="footnote" id="fn:2">
        Incidentally, PNAS lie about this in the paper's front matter; they count the date they "received" it as the date of our <i>re</i>submission, i.e. 4.5 months after the initial submission. This is because journals look bad if they take years to accept things.
    </li>
    <li class="footnote" id="fn:4">
        not counting arxiv or medrxiv, not counting later-successful R&Rs as rejects
    </li>
    <li class="footnote" id="fn:5">
        not counting giant collaborations like the 190 authors on <a href="{{red}}">ReD</a>
    </li>
    <li class="footnote" id="fn:6">
        Obviously you can't sum these costs for each coauthor, so I'll just totally discount the compute used on the other Covid papers to crudely compensate for this.
    </li>
    <li class="footnote" id="fn:7">
        Another "risk" is that of not becoming a professor. Depending on your field this could be 98% likely. (It seems like 80% of my set got out of academia.) But this is only a hazard if you go into it naively or can't cope with the outside world.
    </li>
</ol>

</div>