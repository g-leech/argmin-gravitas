---
layout:     math_post
title:      "prospective science please"
baselink:   /prospect
permalink:  /prospect
date:       2025-10-28
author:     Gavin
img:        /img/janus.jpg

visible:    1
published:  true
quality:    4

summary:    On a titanic social failure and doing better next time
confidence: 70%
importance: 7
wordcount:  
categories: science, epistemology
where:      "Williamsburg"
---


> Time... burns without leaving ashes
<center>— Elsa Triolet</center>


## Case study: whither Google Search?

For now, Google is the premier information infrastructure on Earth. It is the most-visited website. It handles [14 billion](https://www.semrush.com/blog/google-search-statistics/) search queries a day, nearly two for everyone on earth. <a href="#fn:1" id="fnref:1">1</a> As of writing it's more than the traffic of [ChatGPT](https://techcrunch.com/2025/07/21/chatgpt-users-send-2-5-billion-prompts-a-day/), Claude, Wikipedia, Bing, and Duckduckgo put together. 1 / 500 of your waking life is spent on this site, probably, and its decisions determine how you spend much more time than that. <a href="#fn:3" id="fnref:3">3</a>

> Google Search effectively controls the epistemology of the world — <a href="https://x.com/zetalyrae/status/1794590304101957645">@atomgardner</a>

<br>

So... how's it doing? Is search quality changing? Opinions [differ](https://danluu.com/seo-spam/) but it does _[seem](https://x.com/danluu/status/1730705885037801686)_ to have gotten much worse over the last decade. 

<img src="https://www.gleech.org/img/goog/danluu.png" />
<br><br>

OK unc, but surely we have actual quantitative studies of something this important? 

Broadly: no. In the last 25 years, I could only find _one_ major empirical study measuring search _quality_ ([Bevendorff et al. 2024](https://link.springer.com/chapter/10.1007/978-3-031-56063-7_4)). It's fine, and they deserve huge credit for doing what no one did, but it doesn't really answer the question and only covers 2022-3, well after the subjective decline. We had two decades to notice that this was really important and to collect data about it ahead of the decline. We didn't. 

I [complain](https://www.gleech.org/psych) [about](https://pmc.ncbi.nlm.nih.gov/articles/PMC12397490/) [academia](https://arxiv.org/abs/2407.12220) a lot, but this one failure dwarfs almost all others. We have better longitudinal data on bird migration than on the modal channel for belief formation. Academia and civil society had 25 years to run this and didn't, and our chance to measure it is gone. (Google could maybe still do this study given the logs, but why would they?) 

<br>

<div class="accordion">
	<h3>In fairness</h3>
	<div>
		There's a huge attribution problem (was Google degrading or were they accurately representing <i>the web itself</i> degrading?<br><br>
		Google's quality is constantly under attack by the SEO industry and the bots. This would be enough to cause the observed decline without Google doing anything wrong per se. They invest large amounts into countermeasures. It doesn’t work (see this identical effort from 2022).<br><br>
	The defence against the first (non-AI) wave of SEO spam already cost us <a href="https://www.benlandautaylor.com/p/the-ddos-attack-of-academic-bullshit">a lot</a>:
	<blockquote>in 1999… When you looked for information about how to tell if your bread is rising correctly, or about South Korean cement manufacturing, or the musical influences on Igor Stravinsky, or whatever weird thing, Google would pull up high-quality reference material, or blogs and BBS arguments among disagreeable weirdos who specialized in the subject… A cottage industry arose of finding some search term and churning out low-cost copy on the subject in order to serve ads to people trying to find real information. Specialists in “search engine optimization” popularized their techniques as consultants to big companies, and before long this became standard practice. In their efforts to keep these problems from getting totally out of hand, Google and other search engines weighted search results towards a whitelist of standard lowest-common-denominator websites. The long tail of the internet could no longer be found from a simple search.</blockquote><br>
	Then there's the Web 2.0 turn to <a href="https://en.wikipedia.org/wiki/Closed_platform#Examples">walled gardens</a>, platforms which search engines can't really index. That's mostly not Google's doing.
	</div>
	<h3>In unfairness</h3>
	<div>
		It looks like Google itself systematically degraded search by:
		<ul>
			<li><a href="https://web.archive.org/web/20250206165606/https://searchengineland.com/search-ad-labeling-history-google-bing-254332">Making ads</a> look identical to organic results (2011-2020)</li>
			<li>Zero-click answers that keep users in Google's walled garden</li>
			<li>Prioritizing engagement over quality (the "code yellow" pivot of 2019)</li>
			<li>Soft whitelisting of large publishers</li>
			<li>Decreasingly terrible forced AI content</li>
		</ul>
		As the noted scholars Brin and Page <a href="http://infolab.stanford.edu/pub/papers/google.pdf#page=18">said in 1998</a>:
		<blockquote>The goals of the advertising business model do not always correspond to providing quality search to users. For example, in our prototype search engine one of the top results for cellular phone is "The Effect of Cellular Phone Use Upon Driver Attention", a study which explains in great detail the distractions and risk associated with conversing on a cell phone while driving. This search result came up first because of its high importance as judged by the PageRank algorithm, an approximation of citation importance on the web [Page, 98]. It is clear that a search engine which was taking money for showing cellular phone ads would have difficulty justifying the page that our system returned to its paying advertisers. For this type of reason and historical experience with other media [Bagdikian 83], we expect that advertising funded search engines will be inherently biased towards the advertisers and away from the needs of the Consumers.<br><br>... we believe the issue of advertising causes enough mixed incentives that it is crucial to have a competitive search engine that is transparent and in the academic realm.</blockquote>
	</div>
</div>

### The evidence we have

<div class="accordion">
	<!--  -->
	<h3>Our one actual datum: Bevendorff et al. 2024</h3>
	<div>
		That one study, "<a href="https://link.springer.com/chapter/10.1007/978-3-031-56063-7_4">Is Google Getting Worse?</a> <a href="https://link.springer.com/chapter/10.1007/978-3-031-56063-7_4">A Longitudinal Investigation of SEO</a> <a href="https://link.springer.com/chapter/10.1007/978-3-031-56063-7_4">Spam in Search Engines</a>", monitored 7392 queries for particular product reviews on Google (proxied using Startpage), Bing, and DuckDuckGo over one year (2022-3).<br><br>
		It's fine but not amazing evidence: 
		<ul>
			<li>Just one year, and a post-decline year at that</li>
			<li>Just product searches</li>
			<li>Just a couple of thousand products</li>
			<li>Focussed on spam alone</li>
			<li>Using a <a href="https://www.sketchengine.eu/glossary/type-token-ratio-ttr/">dumb</a> mechanical proxy measure for diversity, itself as a proxy for quality.</li>
		</ul>
		<br>
		Still: 
		<ul>
			<li>29% of frontpage Google results had affiliate links compared to the random-page baseline of 2%. (The other two search engines were even worse.)</li>
			<li>That's not totally damning - Dwarkesh has affiliate links - but it is a warning sign: pages with more affiliate links were worse on a mechanical measure of quality. </li>
			<li>23%% of frontpage Google results were outright spam/review farms compared to the random-page baseline of 13%. (The other two search engines were even worse.) This is based on manual annotation, god bless them.</li>
			<li>The buried conclusion is that the very worst spam on Google actually <b>decreased</b> over the study period. The 95th percentile of affiliate links per page fell from 50(!) to 35(!).</li>
			<li>But improvements are short-lived. They found that spam is cyclical (what they call "breathing patterns", like the rise and fall of your chest when you breathe): spam gets into results, the engines update their algorithms to squash them, spam returns in new forms.</li>
			<li>"Text quality" was decreasing in all search engines.</li>
			<li>Overall this study is just too short to measure a nonlinear phenom.</li>
		</ul>
		<br><br>
<!--  -->
<!--  -->
<!--  -->
	</div>
	<h3>What's hard about it?</h3>
	<div>
		Measuring search quality properly requires:<br><br>
<ol>
	<li>Access to search results at scale. Google doesn't provide any examples, so you need to actively scrape it over time yourself.</li>
	<li>Longitudinal data (because one-off snapshots don't give you effects and miss adversarial dynamics)</li>
	<li>sampling representative queries. We should hit common queries and types.</li>
	<li>Baselines: you need to hit multiple search engines (to see if they're also struggling) or to randomly sample webpages (to separate "Google getting worse" from "internet getting worse")</li>
	<li>Quality is not a hard endpoint: Quality is subjective, and there's a higher bar in academia for handling such things. The valid reasons to worry are massive measurement error, low test-rest reliability, cross-cultural and intra-cultural heterogeneity...</li>
	</ol>
	But many of these things are now hundreds of times easier with LLMs!
	</div>
	<h3>Leontiadis et al 2014</h3>
	<div>
		<a href="https://dl.acm.org/doi/pdf/10.1145/2660267.2660332">This</a> is just a study of one particularly obvious and aggressive kind of spam: "search poisoning" redirection attacks where the spammers buy a previously legit URL and send you to their shit.<br><br>
		They tracked a set of pharmaceutical and product queries for 3.5 years (2010-3).<br><br>
		The rate of the frontpage having one or more of these particular attacks jumped from 30% to 60% of results. But it seems to have gotten a lot better after the EEAT update.
	</div>
	<h3>Philipp et al 2014</h3>
	<div>
		<a href="https://arxiv.org/pdf/1712.03622">Analyses</a> logs from 190,000 users over a 6 month period (late 2012 - early 2013) but only for health queries. This is one way that Google obviously got "better" over the last decade, though in a crank-minimising way rather than an <a href="https://www.astralcodexten.com/p/webmd-and-the-tragedy-of-legible">info-maximising way</a>.
	</div>
	<h3>Proxies: Zero-click searches</h3>
	<div>
		Possibly-unrepresentative tracking data from <a href="https://sparktoro.com/blog/2024-zero-click-search-study-for-every-1000-us-google-searches-only-374-clicks-go-to-the-open-web-in-the-eu-its-360/">Datos</a> shows a huge rise in "zero-click" searches, queries that never leave Google.
		<br><br>
		<table>
			<tr>
				<th></th>
				<th>Zero-click rate</th>
			</tr>
			<tr>
				<td>2016</td>
				<td>44%</td>
			</tr>
			<tr>
				<td>2020</td>
				<td>65%</td>
			</tr>
			<tr>
				<td>2025</td>
				<td>68%</td>
			</tr>
		</table>
		<br>
		But this is a bundle measure: it mixes up the results being obviously too crap to bother with and Google sucking up the information from third-parties and so diverting the traffic to themselves. (Google Flights, Google Maps, featured snippets have gotten better; the Google Knowledge Graph is actually often useful; and the AI Overviews are sometimes useful and even more often falsely appear useful.)
	</div>
	<h3>Proxies: UI changes as tells of motivation </h3>
	<div>
		This excellent chart from <a href="https://web.archive.org/web/20250206165606/https://searchengineland.com/search-ad-labeling-history-google-bing-254332">SearchEngineLand</a> shows how, starting in 2011, Google Ads have been steadily made more stealthy and resultlike. <a href="/img/GoogleAds_Timeline_FNL2.001.png">Click it for full-size</a>:
		<br><br>
		<center>
			<a target="_blank" width="100%" href="/img/GoogleAds_Timeline_FNL2.001.png"><img src="/img/GoogleAds_Timeline_FNL2.001.png" /></a>
		</center>
	</div>
	<h3>Proxies: Journalism</h3>
	<div>
		We're reduced from science to journalism. To weakly understand what happened we have to read dreadful people like Ed Zitron, who <a href="https://www.wheresyoured.at/requiem-for-raghavan/">blames</a> one guy, Prabhakar Raghavan (as if the org didn't give him the incentive). They did indeed <a href="https://news.bloomberglaw.com/antitrust/googles-2019-code-yellow-blurred-line-between-search-ads">panic</a> and prioritise ads over actual content, and the timing (2019) does correlate with some of the subjective decline. But that's all we can say from this.
	</div>
	<h3>Proxies: Straw polls from memory</h3>
	<div>
		<img src="/img/goog/danluu.png" />
	</div>
	<h3>Terrible Proxies: User engagement</h3>
	<div>
		Despite all of this, Google's volume metrics are <a href="https://sparktoro.com/blog/new-research-google-search-grew-20-in-2024-receives-373x-more-searches-than-chatgpt">up 20% YoY</a>. Searches per searcher at historic highs.<br><br>
		If search was getting worse, why would people be using it more?:
		<br><br>
		1. Normalisation of deviance: maybe users lowered their expectations<br>
		2. Query inflation: maybe it takes more searches to find useful results <br>
		3. Mobile growth: maybe spending more time on mobile means you just increase your total screentime and so googling time.
	</div>
	<h3>Tiny study: Bloggers</h3>
	<div>
		Dan Luu also <a href="https://danluu.com/seo-spam/">did a manual test</a> on six tricky queries and six search engines. Google was second-worst and didn't get any of them. On two of them no search engine worked at all. On some axes this is better than Bevendorff.
	</div>
</div>





## Sum total

Overall it sure looks like it got worse but we don't really know how much.

What can we do about it?


### Project TODO: Archive reconstruction

You could probably do some limited good by finding common queries which <a href="https://web.archive.org/web/20250000000000*/https://www.google.com/search?q=shoes">happened</a> to get tracked on the Internet Archive:

<img src="/img/ia_shoes.jpg" />


### Project TODO: The second-best time is now

You can in fact start pinging Google every day with the same query and saving the frontpages they give you.

1. **Public search archives**: Either Google preserves and makes available historical search results for vetted researchers or we build it ourselves.

2. **Crawling infrastructure**: Sustained funding for researchers to continuously monitor search quality across engines and domains. Relying on sending it to the Internet Archive is one cheap way but fairly buggy.

<!-- 3. **Algorithm transparency**: Not trade secrets, but basic disclosure about ranking signals, when they change, and what problems they're designed to solve. -->

<!-- 4. **Red team access**: Independent researchers with API access to test hypotheses about search quality, spam, and algorithmic failures. -->

Some possible research questions on Google:

- How has search quality changed for non-commercial queries?
- How does quality vary by domain (health, finance, news, technical)?
- What fraction of queries return primarily SEO spam vs. "authoritative" sources?
- How much of this is caused by Google? 
- How effective are Google's (announced) algorithm updates at *actually improving* quality (vs. temporarily shuffling spam)?
- How strong is the soft whitelist? How far down the ranking have blogs slipped now?
- Has the rollout of AI Overviews (2024) improved or degraded quality?
- How much of the problem is Google's choices vs. the degrading web?
- Are they picking up on publicised test queries and doing whack-a-mole on them?

<br>

And maybe you want to start logging ChatGPT now before they add ads.

## Conclusion: We need prospective science 

That latter project makes me realise that a whole kind of science is missing: watching the world and collecting data before things happen. <a href="#fn:4" id="fnref:4">4</a>

Most academic studies look backwards. (RCTs look forwards but at enormous expense on tiny questions.) But scraping is now hundreds of times cheaper than it used to be. Dumb first-pass [classification and scoring](https://huggingface.co/learn/cookbook/en/llm_judge) is now thousands of times cheaper. Let's just start collecting.

<br>

1. Look around you. Think of the most important digital services and phenomena in the world.
2. Come up with hypotheses about what could happen to them. Put them on [OSF](https://www.cos.io/initiatives/prereg) to tie your hands.
3. Think really hard about the design - this bit is irreversible if you want consistent, backwards-compatible results. Consider interventions as well as constant baselines.
3. Start scraping.
4. Check back every year. 
5. Iterate on the design if you must.


## See also

* [Google-fu](https://www.gleech.org/google)
* [Google as Epistemic Tool](https://philarchive.org/archive/DEVRGA)


<div class="footnotes">

<ol>
    <!-- 1 -->
    <li class="footnote" id="fn:1">
		I'm focussing on consumer queries.<br>
		ChatGPT = 5.4B<br>
		Wiki = 4.4B<br>
		Bing = 2.2B<br>
		Duckduckgo = 2.1B<br>
		Claude ~= 0.4B<br><br>
		Twitter and Reddit are harder to compare, since people usually don't put queries to them, but Google is maybe twice as big as both put together.<br>
		Reddit = 4.6B but not so analogous<br>
		Twitter = 3.6B but not so analogous<br>
	</li>
<!-- 	<li class="footnote" id="fn:2">
		One reason to do <a href="https://themultiplicity.ai/">multimodel queries</a> every time is basic: the three big models all use different search indices (Gemini - Google, GPT - Bing, Anthropic - Brave). I presume Kimi is some cool Chinese one with sick amounts of IP in it.
	</li> -->
	<li class="footnote" id="fn:3">
		Very rough estimate: <br><br>
		Queries per person per day = 14bn / 8bn = 1.75 <br>
		Say 1 min per query<br>
		1.75 min/day / 1440 min/day = 0.12% = 1/833<br>
		waking life is two-thirds = (1/833) * 3 / 2 = 1/555
	</li>
	<li class="footnote" id="fn:4">
		The "<a href="https://en.wikipedia.org/wiki/Credibility_revolution">Credibility Revolution</a>" in economics is sort of similar, but even they don't seem to actively collect things. Instead they just cleverly notice when existing datasets have a convenient randomish discontinuity.
	</li>
</ol>
</div>



<br><br>
<div class="accordion">
	<h3>Comments</h3>
	<div>
		<b>Tomas</b> commented on 18 November 2025:
		<blockquote>
			Collecting the evolution/history of answers of LLMs to a given set of problems is brilliant, and it is a shame if it is not happening already.<br><br>
			Note that some (most?) of it needs to happen in secret (i.e. not published yearly, using unaffiliated API keys, etc) - not to be easily targeted by providers, whether intentionally or not (e.g. future LLMs training on it.) Then release some of it after 3 years, some after 5, ..<br><br>
			* How would you operationalize "the internet itself getting worse"?<br>
			* What questions do we ask the LLMs and monitor for change? What questions do we ask google and others, and monitor for change? (Setting this up as an ~automated process that only needs a yearly minor code update and $100/y in credit does not sound that hard otherwise!)<br>
			* How do we support preservation of the Internet Archive data over long horizons? You can donate to IA, but I would (also) give money to an independent org mirroring (almost) all its data reliably & long-term. (There were a few mirrors, quick search tells me they are gone, and there are partial/small mirrors of selected IA data.)
		</blockquote>	
	</div>
</div>



