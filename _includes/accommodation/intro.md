To help locate us, let's define some subfields of computational linguistics:

- **Speech processing**: The analysis of speech as a digital signal (and thereby as a mathematical object). As opposed to...

- **Natural language processing** (NLP): The analysis of speech as symbolic information, as syntax and semantics (and thereby as a mathematical object).

- **Paralinguistics**: The analysis of the ‘supra-segmental’ parts of communication: including rhythm, pitch, volume and intonation. (That is, of those components that extend over multiple units of speech, e.g. phonemes.)

- **Social signal processing** (SSP): The analysis of interaction context (i.e. of properties beyond the literal communication). Involves automated inference from data to phenomena that cannot be directly observed, but only inferred from correlated behaviours.


We use techniques from each of the above: from speech processing, the vectorization of speech waves for tractable model inputs; from NLP, we adapt a mechanism (the general 'recogniser') initially developed for automatic word recognition; we incorporate paralinguistic content in considering more than the symbolic or phonemic content of speech; and in line with SSP we are attempting to make inferences about a social process indirectly.

The goal is an objective, holistic, and automatic approach to linguistic accommodation. The speakers are represented using Hidden Markov models (HMMs), long the dominant tool for acoustic modelling. <a href="#fn:5" id="fnref:5">5</a> 

To very briefly introduce the analytical method: We begin with no models of the speakers, no model parameters, and no known general laws of accommodation to apply. What we do have are twelve sets of time-annotated speech signals (one set per speaker, with each signal representing a task completed by that speaker) and a fully general signal-modelling framework (i.e. HMMs). 

The tractability and performance of HMMs depends on three powerful dynamic programming algorithms: Baum-Welch re-estimation (for learning model parameters from training data); Viterbi decoding (for inferring an optimal processing for each given observation sequence); and Forward-backward smoothing (for inferring the probability of an observation sequence given a state sequence and a model).

Our unit of analysis is the ‘speaker-task’ (that is, one of the conversational prompts completed by each of 12 speakers of the dataset). The sub-units are the words spoken by each speaker during this task, with frames within each word serving as important sub-sub-units (see Section 5.2 for a schematic view of the data).


Each speaker is modelled as an HMM; these models are used to estimate the likelihood ratio (a measure of the relative extent to which each model explains or predicts each word) for each word in the test set. For each task completed by each speaker, these ratios are combined into correlations of the pair’s ‘distance’ over the time of an eight-minute conversation. The sign and significance of the time correlation between these ratios is used to infer the presence of accommodation, and to estimate effect sizes. (See Fig.9 for a schematic view of this calculation.)

The use of likelihood-ratio correlations to measure the relative distance between speakers was first developed for authenticating speaker identity. <br><br>


<h3>Overview of this dissertation:</h3>

The pipeline is given in Table 1. In the following, Stages (a) through (c) are referred to as ‘the SSSV experiment’. The rest of the introduction gives definitions and methodology for accommodation research. Section 2 defines the formalism used.

Section 3 lays out the abstract theory of modelling sequences with finite-state machines, and then of
Markov and Hidden Markov models. 

Section 4 applies the HMM framework to the study of speech, particularly speaker verification and accommodation. 

Section 5 explains the experimental setup and the implementation of our HMMs; section 6 lays out key results from evaluating them.

Section 7 concludes with discussion, limitations, and potential connections to future work.


<br><br>
<center>
{%  include accommodation/table.html %}

<br>Table 1. The project
</center>
<br><br><br>


<h3>1.1. What is linguistic accommodation?</h3>

Accommodation is 

> the tendency of people involved in an interaction to converge towards [or diverge from] common behavioural patterns. <a href="#fn:8" id="fnref:8">8</a>

Thus, linguistic accommodation is where speakers come to talk more (or less) similarly as they interact. It is fundamental to human communication, closely correlated with key aspects of interactions including empathy, status, co-ordination, and team effectiveness (especially in task-oriented interactions). <a href="#fn:9" id="fnref:9">9</a> <a href="#fn:10" id="fnref:10">10</a>

A note on navigation: Depending on the particular behaviour being emphasised, accommodation is sometimes known as ‘<i>interpersonal adaptation</i>’, ‘<i>synchrony</i>’, ‘<i>convergence</i>’, ‘<i>(nonconscious) mimicry</i>’ or ‘<i>mimesis</i>’, ‘<i>alignment</i>’, ‘<i>spontaneous imitation</i>’, and sometimes ‘<i>entrainment</i>’ (after the phenomenon of coupled oscillators in physics).

Accommodation has been experimentally detected in many aspects of speech including: speakers’ fundamental frequency <a href="#fn:11" id="fnref:11">11</a>; phonemes <a href="#fn:12" id="fnref:12">12</a> <a href="#fn:13" id="fnref:13">13</a> ; speech rate <a href="#fn:14" id="fnref:14">14</a> and rhythm <a href="#fn:15" id="fnref:15">15</a> ; as well as higher-level aspects of language like accent <a href="#fn:16" id="fnref:16">16</a> and vocabulary. <a href="#fn:17" id="fnref:17">17</a> While pervasive, accommodation may not be too dramatic in effect size. The holistic analysis of the SSSV experiment found $$R^2 = 0.11 \pm 0.04$$; my analysis found effect sizes half this strong. <a href="#fn:18" id="fnref:18">18</a>

It is usually treated as the <i>unconscious</i> tendency to converge, independent of
intentional flattery; certainly the effect has been seen to occur in cases without conscious intention
to mimic (for instance in the experiments of Chartrand and Bargh (1999), in which experimenters’
mannerisms were accommodated by participants without their being aware of doing so). <a href="#fn:19" id="fnref:19">19</a> 

The effect has been seen to vary by the degree of mutual liking between interlocutors <a href="#fn:20" id="fnref:20">20</a> , differences in
social status<a href="#fn:21" id="fnref:21">21</a>, and by gender matching (that is, people accommodate more to interlocutors of their
own gender).<a href="#fn:22" id="fnref:22">22</a>

Various evolutionary rationales have been presented to explain the pervasiveness and apparent
power of the phenomenon in human life: in terms of the recency effect in episodic memory (by
which one’s language depends strongly on the language most recently heard), of displaying one’s
group affiliation, or of approval-seeking.<a href="#fn:23" id="fnref:23">23</a> 

Some variant of them is bound to be true; but these go
well beyond the scope of the present work, which is focussed on existence and effect size, rather
than causal direction or ancestral origins.<br><br><br>



<h3>1.2. Measuring accommodation</h3>

We can divide studies of accommodation by methodology: 

<ul>
	<li>whether the measurements made are objective, or subjective (and quantitative or qualitative);</li>
	<li>whether the acoustic evaluation is holistic, or reductive (that is, whether the signal is taken as one variable, or analysed only in terms of particular spectral or syntactic components);</li>
 	<li>the temporal scope of the analysis (whether the phenomenon is studied at the conversation level or by short ‘reduced stimuli’);</li>
 	<li>whether the interactions analysed are guided by experimenters, or are free-form.</li>
</ul>
<br>

The present work aims for an objective, holistic measure of accommodation, with data analysed at
the conversation level, and the conversations guided by task prompts. Precedents for this specific
approach are relatively scarce in the literature, but Lelong and Bailly (2011) provide a good
example of the approach, in speech recognition. <a href="#fn:24" id="fnref:24">24</a> 

We attempt to use all the acoustic evidence – to capture the signal interactions within a conversation that may
produce accommodation – whilst retaining the analytical power to detect the relatively subtle
phenomenon. (Holistic processing also has the advantage of reflecting the manner in which humans
perceive speech signals: as a whole, rather than a logical composite of parts.)

Most accommodation research does not take the conversation as the unit of analysis, even
though accommodative effects are consistently found across supra-segmental features of speech
(where ‘supra-segmental’ denotes effects that are longer-term than any given unit, e.g. phoneme,
word or sentence). 

The general approach is to measure effects using ‘reduced stimuli’, marginal
units: the reasoning being that, if accommodation can be detected to occur given only minimal
acoustic cues, then the findings can be called robust. (The popularity of this method can be seen in
the papers listed in this footnote <a href="#fn:25" id="fnref:25">25</a>, comprising a substantial fraction of all accommodation experiments found.) 

Setting the scope of analysis at the conversation level should allow accommodation to manifest, and fills a gap left by the proliferation of careful segmental reductionist research.
<br><br><br>


<hr /><br>
