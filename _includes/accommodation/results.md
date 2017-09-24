The first evaluation was somewhat circular: whether my first analysis (a single-state GMM-HMM with MFCC features) obtained comparable results to the model in Stuart-Smith, Smith et al (2015). (The stochastic process involved in estimating model makes precise replication improbable.) 

This is borne out in the replication results below, but should be read as a mere indication of comparability, not bearing on the population.


<center><img src="/img/accommodation/table2.png" width="100%" height="100%" />
	<br>	<br>
Table 2. Overview of effect size, % correlations and the order of binomial test, for each model type.
</center><br><br>


We consistently see rates of significant effects with cumulative binomial probabilities less than $$1 \,\times\, 10^{-14}$$. Overall, we see a large shift in the rate of significance when we move from the heavily simplified single-state models to multi-state models (as well as when changing the feature type to LPC) and a decreasing mean absolute effect size. The observational nature of the study means we must consider two empirically adequate explanations for this:

<ol>
	<li>incorporating temporal information adds noise which reliably obscures some accommodative effects which can be captured by single-state models that use the whole sequence of each utterance.</li><br>

	<li>Or, single-state models generate some spurious accommodative effects; multi-state models generate fewer of these.</li>
</ol>

The falling effect size may be considered weak evidence for explanation (1). This significant difference between model sets N=1 and N>1 is borne out in the conversation length analysis. The MFCC and LPC analyses obtained a very similar number of significant results at the 5% level (a
mean 39.5 for MFCC and 38 for LPC); the distribution varies, with a weak tendency to reverse the sign of the distance correlation, relative to the other feature type.<br><br>

<h4>Individual Speaker trends</h4>

To get a sense of the stability of the results, we can see if characteristic patterns persist for each speaker. Table 3 characterises our speakers across all model sets. Significant effects are seen in all speakers, with some consistency between effects over the model sets:<br><br>



<center><img src="/img/accommodation/table3.png" width="100%" height="100%" />
	<br>	<br>
Table 3. Significance and sign of correlations within, and between, speakers.
</center><br><br>



‘Sign consistency’ is a measure of whether the speaker was a consistent converger or diverger. It is the percentage of all significant speaker-tasks that fall under that speaker’s most common sign (where scores close to 50% thus indicate no strong tendency to either sign) . 

The average is 61%; i.e. there is only a mild tendency for a speaker to accommodate in one direction only. The ‘pair correlations’ are an attempt to see if highly accommodative speakers are correlated with higher accommodation in their interlocutor, by effect size, number of significant tasks and sign. It is a simple ratio of the pair’s condition means; thus values near to 1 indicate closeness between the results of a pair. We see a very strong fit between two (absolute) correlation magnitudes, a strong difference between the number of divergent tasks, and a close fit between the mean number of negative results for (which, again corresponds to convergence), but not much closer than the ratio for each speaker’s overall mean.<br><br>


<h4>Conversation length and task difficulty correlations</h4>

In Table 4, the ‘Average length’ is simply the average time of each task’s final utterance in seconds, plus or minus the sample mean standard error. The final column is based on the overlap of the average SEM between Positive and Negative conditions. The absence of overlap is taken as a binary cut-off for being able to report a true (average) difference between the Positive and Negative conditions.

<br>
<center><img src="/img/accommodation/table3.png" width="100%" height="100%" />
	<br>	<br>
Table 4. Average length in each condition, and overlap of SEM for each model type .
</center><br><br>


Taking non-overlapping standard errors as a criterion for reporting a significant difference between conditions, we can only consider $$h_2$$ (the hypothesised effect between length and convergence) confirmed in the single-state MFCC model. 

This implies, again, that the link between longer task length and convergent accommodation is obscured by including temporal effects (or, that the link is
an artefact arising from considering all the acoustic evidence at once).

<br><br>