---
layout: 	math_post
title:  	"How lethal are the Tories?"
baselink:	/esa-deaths
permalink:	/esa-deaths/
date:   	2017-09-20  <!--site.time-->
author:		Gavin	
img:		/img/daniel.jpg

visible:	1
published: 	true


summary:	Actuarial check of the claim that welfare reforms killed thousands of people.
confidence: 80%
warnings: 	bureaucracy, statistical illiteracy, UK-only
categories: stats, social science
count: 		
---


You often see the claim that the Tories have killed "thousands" of disabled people by spuriously cutting them off from disability benefit. ([ESA][esa] is probably the only government program in the world with [a Palme d'Or winning film][blake] about how shit it is.)

This claim is a [naive reading][naive] of mortality statistics: between 2011 and 2014, [2,380][kilodeath] people died after being declared "fit to work" (and so after having their main income removed). Media and blogosphere discussions of this fact are sloppy, even by the standards of public policy debate.

If this were true, the Tories would be about as lethal as uterine cancer ([720 UK deaths][cancer] per year ).

In particular: **You cannot infer anything about a program's impact from a single number, without a reference class.** It took me exactly 10 seconds to find age-adjusted data on deaths in this subpopulation, compared to the general population, before _and_ after the imposition of WCA <a href="#fn:1" id="fnref:1">1</a>:

<br><br>

<!-- REDO IN PLOTLY BARS -->

<div align="center">
	<img src="/img/killer-tories/rates.jpg" />

	<br><br><small>Age-standardised mortality rates (deaths per 100,000) in various subpopulations .</small>
</div><br>


There. Overall, British disabled people have been dying less since 2010. Given a population of 2m, this fall works out to about 2000 fewer deaths per year. If I was a journalist or an activist, I'd wrap up here: clearly Tory welfare reforms have been miraculous, having saved hundreds of disabled people's lives on net. Somehow.

The following is just a statistical argument. All I can say for it is that it is less pig-ignorant than going around parroting the uncontrolled, uncontextualised figure.

Deaths after "fit for work" WCA: 				2,380 
Deaths after unsuccessful DWP appeal:			1,360
Deaths among ESA recipients, 2016:				50,580


https://www.gov.uk/government/uploads/system/uploads/attachment_data/file/459106/mortality-statistics-esa-ib-sda.pdf

## Age distribution of disabled people

We're looking for the following subsets:

	1. people on ESA  U  people with a disability
	2. people with a disability  U  declared fit to work (negative WCA)
	3. people without a disability  U  declared fit to work (negative WCA)

and then comparing this to the people who kept their ESA.

Age-standardised mortality rates (ASMR) 


## How bad are work competency assessments?

The main reasons to be suspicious of WCA are: 1) they are mostly not conducted by medical staff; 2) in the worst years the private companies contracted to run them had a quota of people to kick off benefit; 3) they ignore less visible conditions like major depression and chronic pain. 

I'm calling 'fit to work' a 'negative' result: the WCA test does not think you are disabled enough. 

False positive (disabled | not disabled): 20% ?
False negative ( not disabled | disabled ): 20% ?


Successful appeal = 40%


We can use these to get the conditional probability of being disabled given a negative ("not disabled") WCA result.


## Comorbidity

We could reduce the uncertainty of this, given data on the cause of death - hypothermia and suicide being evidence of WCA killing, and decompensation of chronic illnesses being null evidence. 


## Known deaths

[Michael O'Sullivan][sullivan], AKA "Mr A".


## Natural experiment: Scotland



## Note on data

People [struggled with the Department for Work and Pensions][struggle] to get these figures published. This is sometimes read as an admission of guilt. But given how naively the 2,380 figure was received, it is hard to blame them for their cowardly reluctance.

This doesn't cover the burden of life lost to admin and the distress caused to those who didn't die after it.



## Blaming mortals for mortality

That sockpuppet axeman, Iain Duncan Smith.


* 

[sullivan]: http://www.newstatesman.com/politics/welfare/2015/09/disabled-man-killed-himself-over-benefit-cut-coroner-rules
[esa]: https://en.wikipedia.org/wiki/Employment_and_Support_Allowance
[naive]: https://www.theguardian.com/society/2015/aug/27/thousands-died-after-fit-for-work-assessment-dwp-figures
[kilodeath]: https://www.gov.uk/government/statistics/mortality-statistics-esa-ib-and-sda-claimants
[blake]: https://en.wikipedia.org/wiki/I,_Daniel_Blake
[public]: https://stat-xplore.dwp.gov.uk/webapi/jsf/login.xhtml
[struggle]: https://ico.org.uk/media/action-weve-taken/decision-notices/2015/1424160/fs_50557638.pdf 
[cancer]: https://www.ons.gov.uk/peoplepopulationandcommunity/healthandsocialcare/conditionsanddiseases/datasets/cancersurvivalratescancersurvivalinenglandadultsdiagnosed


{%  include comments.html %}

{%  include killer-tories/foots.html %}