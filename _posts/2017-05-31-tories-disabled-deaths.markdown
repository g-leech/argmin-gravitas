---
layout: 	math_post
title:  	"How lethal are the Tories? Part 1"
baselink:	/esa-deaths
permalink:	/esa-deaths/
date:   	2018-02-25  <!--site.time-->
author:		Gavin	
img:		/img/daniel.jpg

visible:	1
published: 	true


summary:	Checking the claim that austerity killed thousands of disabled people.
confidence: 80% in the observational argument; agnostic about causation.
warnings: 	bureaucracy, statistical illiteracy, UK-only
categories: stats, social science, data columbo
count: 		1300
---



{%		include killer-tories/links.md				%}

You sometimes [see][jones] [the claim][naive] that the Conservative-led <a href="{{camClegg}}">coalition</a> killed "thousands" of disabled people by spuriously cutting them off from disability benefit. <a href="#fn:3" id="fnref:3">3</a>

This claim is extremely naive, as I'll show. But it is based on something which <i>sounds</i> similar: the fact that between 2011 and 2014, [2,380][kilodeath] people died after being declared "fit for work" (FFW), i.e. after having their main income stopped. 

(For context, if we established causation, and so responsibility, this would make the Tories about as lethal as uterine cancer, at [720 UK deaths][cancer] a year.)

It's not hard to find [cases][sullivan] where causation seems likely. But, by trying to imply causation from the above figure, the general claim implies that the Conservatives are responsible for <i>all</i> mortality during their reign - which, even speaking as a Scotsman, seems a bit strong. Media discussion of this fact was sloppy even by the low standards of public policy discussion.
<br><br>
_Terms_:

* <span style="font-weight:bold">ESA</span> : <a href="{{esa}}">Employment and Support Allowance</a>; the UK's newish main disability benefit. <br><br>
* <span style="font-weight:bold">WCA</span> : <a href="{{wca}}">Work Capacity Assessment</a>. Quasi-medical screening process for ESA. <br>
Introduced by Labour in 2008, made much stricter by the Conservatives in 2011.
<br><br>

The following is just an <a href="{{obs}}">observational</a> argument: it doesn't exonerate or condemn. All I can say for it is that it's less pig-ignorant than parroting the uncontrolled figure.

If you take one thing from this, make it *You cannot infer anything about impact from one number at one point in time without a reference class.* 

In particular, it doesn't make sense without accounting for the number of deaths in this group *before* the WCA reform. (Maybe 2380 is an improvement.) And it doesn't make sense to compare even those numbers without accounting for large known influences on mortality, e.g. seeing if ages and genders differ between the compared groups. What we actually need is not 'deaths' but 'excess' deaths.

It took me <a href="{{2003to13table}}">10 seconds</a> to find age-adjusted data, compared to the general population, before and after <a href="{{wca}}">WCA</a>:


<div align="center">
	<img src="/img/esa-deaths/granular_rate.png" />

	<small>Suggested headline: <i>"Go on the dole to save your life!"</i> <a href="#fn:2" id="fnref:2">2</a></small>
</div><br>



No large changes: people on incapacity benefit have been dying very slightly less (1043 -> 1032), and there's a slight increase (116 to 138) among JSA recipients. Given ~[2.5m][totalEsa] people on <a href="{{esa}}">ESA</a>, this fall works out to about 275 fewer deaths per year. <a href="#fn:5" id="fnref:5">5</a> <a href="#fn:4" id="fnref:4">4</a> 

<!-- 
If I was a journalist or activist, I'd wrap up here: clearly Tory welfare reforms have been miraculous, saving hundreds of lives somehow. Never you mind the big bold writing that correctly says _This information cannot be used as evidence to support a link or otherwise between mortality and benefit receipt._
 -->


But we're interested in the ones who <i>aren't</i> on disability any more; in particular, the ones who were kicked off. (Many <a href="{{transition}}">move onto Jobseeker's Allowance</a> (JSA), which is actually the lowest-mortality group, even after adjusting for the relative youth of people on JSA. Then there's a group who presumably fall off the official stats entirely.)

So, compare the mortality rate of people on ESA (1.032%) with those kicked off it. The [published data for WCA results][wcaResults] only goes up to March 2013 at present; I'll update this when they're out, but for now let's plot a dumb model for the 2013-4 rate:

<div align="center">
	<img src="/img/esa-deaths/ffw.png" />
	<a href="#fn:6" id="fnref:6">6</a> 
</div><br>


* _December 2011 to March 2013_: 238,100 declared fit for work.
* _Extrapolation for April 2013 to February 2014_: <a href="{{ffwGist}}">131,500, if trend continued</a>.
* _Estimated total "fit for work", December 2011 to February 2014_: 369,600. 
* _December 2011 to February 2014_: 2,380 deaths among "fit for work" within 6 months of decision.

* _Death rate among "fit for work"_: 0.64%.
* _Death rate among ESA recipients_: 1.03%.

<br>
So the death rate among those declared fit to work (0.64%) was halfway between the unfit-to-work (1.03%) and the general population rate (0.24%).

<div align="center">
	<img src="/img/esa-deaths/est_rates.png" />
</div>

What does this tell us? That the "fit-for-work" population is probably not the same as the general population - and you could see this as an indictment of <a href="{{wca}}">WCA</a>, since this is exactly what they are treated as being. It's consistent with half of "FFW" people being equally unwell as the "unfit-for-work" are, or with all "FFW"s being half as disabled - or, more likely, with some mixture of these things.

If the "FFW" had the same health as the general population, you'd expect them to suffer roughly 887 deaths a year. <a href="#fn:7" id="fnref:7">7</a>  As it is, there were 1057, or something like `~170` <i>excess</i> deaths a year. <a href="#fn:8" id="fnref:8">8</a>  

"2,380" is thus many times too high, _even if_ it had been stated as a careful observation of the situation and not as the resounding proof of blame it was stated as. 

The above has nothing to say about causation; many other things besides WCA could have and will have borne on these. I don't even have the row-level data to properly establish that FFWs are a different population, let alone enough to isolate WCA's effects on them.

<br>

---

<br>

## How bad are/were work competency assessments?

_This section has several made up numbers._

<br>
The main reasons to be suspicious of the 2011 WCA are: 1) they are sometimes not conducted by medical staff; 2) the private companies that run them are given [narrow norms][norms] that probably result in a de facto quota; 3) they penalise less visible conditions like major depression and chronic pain. 

If we had just a couple of numbers, we could use the awesome machinery of the [confusion matrix][confu] to objectively rate how good WCAs are at their allotted dirty job.

Buckle up, because it's time for some Bayesian inference.

If the WCA is a disability test, then call a fit-for-work judgment a 'negative' result: i.e. the WCA test doesn't think you are disabled enough. Assume that a successful appeal is the same as showing a false negative on the original test (though in fact appeals will have some error rate too). 

<br>

* Base rate for disability `P(H)`: One estimate is <a href="{{popul}}">21%</a> of UK adults.
* False positive `P(E | ~H)`: being flagged unfit-for-work despite not being disabled. Probably low: 10% ?
* False negative `P(~E | H)`: being flagged fit-for-work despite being disabled: FNR = [59%][appeal]
* True positive `P(E | H)`: being flagged unfit-for-work and being disabled: `1 - FNR = 41%`
* True negative `P(~E | ~H)`: being flagged fit-for-work and not being disabled: 1 - FPR = 0.9

<br>

We can use these to guess the conditional probability that someone is disabled given a positive WCA result ("unfit-for-work"):

	P(H | E) =  P(E|H) x P(H)  / P(E|H) x P(H) + P(E|~H) x (1 - P(H))
		 = (0.41 x 0.21) / (0.41 x 0.21 + 0.1 x 0.79)
		 = 52.2%

Slightly better than a coin flip; and the conditional probability that someone is disabled in spite of a negative WCA result ("fit-for-work"):

	P(H | ~E) =  P(~E| H) x P(H)  / P(~E|H) x P(H) + P(~E|~H) x P(~H)
		  = (0.59 x 0.21) / (0.59 x 0.21 + 0.9 x 0.79)
		  = 16%

<br>
i.e. Under these estimates, the test is fairly weak evidence. (Don't rely on this; there are too many assumptions, and of necessity I've used the UK population rather than the test-taking population, which is bound to have a higher base rate.)


<!-- 
Between 2011 and 2013 around 40% of claimants found 'fit for work' appealed to a tribunal and around 40% of those appeals were successful.[44] The total number of external appeals dropped markedly over the course of 2013, although most appellants who reach the tribunal stage now see their 'fit for work' decision overturned.[45][46]

During 2012, Parliament's Office of Science and Technology analysed the WCA's performance and found that "the number of fit-for-work decisions being overturned on appeal has led to questions about the reliability of the assessment process". In the same year, a parliamentary committee heard evidence from welfare advisors that, in nearly two out of three successful appeals to tribunals against fit-for-work decisions, appellants were seeing their points rise from zero in the original assessments — meaning that the original WCA had detected no relevant disabilities at all — to at least 15 points after the tribunals had independently assessed their claims.[47]

A 2012 study of 28,000 tribunal hearings analysed the reasons for overturning the DWP's decisions:

    In almost two-thirds of successful appeals, the tribunals found the appellants' descriptions of their difficulties, given in person on the day of the tribunal, sufficiently convincing for them to be awarded the benefit — known as presenting "cogent oral evidence" in legal jargon. By implication, in these cases, the tribunal found the oral evidence more persuasive than had the assessor who had conducted the original face-to-face assessment
    In nearly a quarter of successful appeals, the tribunals agreed with the DWP on the facts of the case but decided that the DWP had come to the wrong conclusion based on those facts
    In 13% of cases, documentary evidence was provided that had not been available at the initial assessment
    In under 1% of cases, the assessment report was found to contain important technical errors.[45]
 -->

<br>

---

<br>

## Data

People [struggled with the Department for Work and Pensions][struggle] to get these figures published. This is sometimes read as an admission of guilt. But given how naively the 2,380 figure was received, it is hard to blame them for their cowardice.

Although <a href="{{2003to13}}">this official figure</a> is very misleading

<div align="center">
	<img src="/img/esa-deaths/rates.jpg" />
</div><br>

since it hides a recent rise in death rate (2010 - 2013) behind the big drop between 2003 - 2008.

We could reduce our uncertainty if we had data on the cause of death - e.g. hypothermia and suicide being evidence of WCA responsibility, while decompensation of chronic illnesses wouldn't be. But we don't.

I had a look for people who disappear from the system entirely. Let's try and find them among the homeless. The government [doesn't collect this information][noHome] (an oversight I'm inclined to be cynical about), and the charity Crisis haven't updated [their numbers][homeless] since 2009, just outside our analytical window.

This analysis doesn't cover public time lost to <a href="/img/esa-deaths/bureau.png">bureaucracy</a>, nor the [distress][mental] of those who didn't die.

<br>

---

<br>

Part 2 of this series will be about the [stronger academic claim][elder] that austerity caused 30,000 - [45,000][bmj] excess deaths, mostly among the elderly.

Part 3 might be about the [mental health impact of WCA][mental], though maybe not - that report doesn't make very questionable claims.

<br><br>
 
[sullivan]: http://www.newstatesman.com/politics/welfare/2015/09/disabled-man-killed-himself-over-benefit-cut-coroner-rules
[naive]: https://www.theguardian.com/society/2015/aug/27/thousands-died-after-fit-for-work-assessment-dwp-figures
[kilodeath]: https://www.gov.uk/government/statistics/mortality-statistics-esa-ib-and-sda-claimants
[totalEsa]: https://www.gov.uk/performance/dwp-incapacity-benefit-employment-support-allowance-esa-claims-maintained
[public]: https://stat-xplore.dwp.gov.uk/webapi/jsf/login.xhtml
[struggle]: https://ico.org.uk/media/action-weve-taken/decision-notices/2015/1424160/fs_50557638.pdf 
[cancer]: https://www.ons.gov.uk/peoplepopulationandcommunity/healthandsocialcare/conditionsanddiseases/datasets/cancersurvivalratescancersurvivalinenglandadultsdiagnosed
[foundFit]: https://www.gov.uk/government/news/million-new-esa-claimants-found-fit-for-work
[wcaResults]: https://www.gov.uk/government/uploads/system/uploads/attachment_data/file/274091/esa_wca_140122.xls
[noHome]: https://www.ons.gov.uk/aboutus/transparencyandgovernance/freedomofinformationfoi/deathsduetohomelessness
[elder]: https://www.ncbi.nlm.nih.gov/pubmed/28208027
[mental]: http://www.advocard.org.uk/wp-content/uploads/2017/02/2017-02-Heriot-Watt-Mental-Health-Report-on-WCA.pdf
[bmj]: http://bmjopen.bmj.com/content/7/11/e017722
[homeless]: https://www.crisis.org.uk/media/236799/crisis_homelessness_kills_es2012.pdf
[jones]: https://www.theguardian.com/commentisfree/2016/mar/17/disabled-people-government-vulnerable-budget-labour
[norms]: https://www.theguardian.com/commentisfree/2013/dec/09/atos-disabled-people-assessment-fit-work-report
[appeal]: https://www.gov.uk/government/uploads/system/uploads/attachment_data/file/558953/esa-wca-summary-september-2016.pdf#page=9
[confu]: https://en.wikipedia.org/wiki/Sensitivity_and_specificity#Confusion_matrix

{%  include comments.html %}

{%  include killer-tories/foots.html %}