The first evaluation was somewhat circular: whether my first analysis (a single-state GMM-HMM with MFCC features) obtained comparable results to the model in Stuart-Smith, Smith et al (2015). (The stochastic process involved in estimating model makes precise replication improbable.) This is borne out in the replication results below, and was more an indication of the basic comparability of the current work, than validation of any inference to population effects.

Significance-testing correlations
Model set Mean
absolute
correlation % of tasks
significant
at 5% Cumulative
binomial P of
spurious
result (5%)<a href="#fn:65" id="fnref:65">65</a>% of tasks
significant
at 1% Cumulative
binomial P of
spurious result
(1%)

Stuart-Smith et al. 0.11 40.2 % 1 x<a href="#fn:10" id="fnref:10">10</a>-15 22.0 % 1 x<a href="#fn:10" id="fnref:10">10</a>-17

Replication

(Single MFCC) 0.067 38.6% 1 x<a href="#fn:10" id="fnref:10">10</a>-15 24.2 % 1 x<a href="#fn:10" id="fnref:10">10</a>-17
Single-state LPC 0.075 40.2 % 1 x<a href="#fn:10" id="fnref:10">10</a>-15 32.6 % 1 x<a href="#fn:10" id="fnref:10">10</a>-17
Two-state MFCC 0.06 28.8 % 1 x<a href="#fn:10" id="fnref:10">10</a>-15 18.2% 1 x<a href="#fn:10" id="fnref:10">10</a>-17
Two-state LPC 0.05 25.8 % 1 x<a href="#fn:10" id="fnref:10">10</a>-15 12.9% 1 x<a href="#fn:10" id="fnref:10">10</a>-15
Three-state MFCC 0.052<a href="#fn:25" id="fnref:25">25</a>% 1 x<a href="#fn:10" id="fnref:10">10</a>-14 17.4% 1 x<a href="#fn:10" id="fnref:10">10</a>-17
Three-state LPC 0.051 24.2 % 1 x<a href="#fn:10" id="fnref:10">10</a>-14 12.1% 1 x<a href="#fn:10" id="fnref:10">10</a>-13
Four-state MFCC 0.052 27.3 % 1 x<a href="#fn:10" id="fnref:10">10</a>-15 15.2% 1 x<a href="#fn:10" id="fnref:10">10</a>-17
Four-state LPC 0.049<a href="#fn:25" id="fnref:25">25</a>% 1 x<a href="#fn:10" id="fnref:10">10</a>-14 12.1% 1 x<a href="#fn:10" id="fnref:10">10</a>-13
Table 2. Overview of effect size, % correlations and the order of binomial test, for each model type.
We consistently see numbers of significant accommodative effects with cumulative binomial
probabilities less than 1 x<a href="#fn:10" id="fnref:10">10</a>-14 . Overall, we see a large shift in the rate of significance when we move
from the heavily simplified single-state models to multi-state models (as well as when changing the
feature type to LPC) and a decreasing mean absolute effect size. The observational nature of the
study means we must consider two empirically adequate explanations for this:
1) incorporating temporal information adds noise which reliably obscures some
accommodative effects which can be captured by single-state models that use the whole
sequence of each utterance.<br><br>

The binomial probability of a spurious result is the cumulative result of a two-tailed binomial test a t each s ignificance
l evel: i t is the probability of obtaining at l east this many s ignificant tasks by cha nce.
362) Or, single-state models generate some spurious accommodative effects; multi-state
models generate fewer of these.

The falling effect size may be considered weak evidence for explanation (1). This significant
difference between model sets N=1 and N>1 is borne out in the conversation length analysis.
The MFCC and LPC analyses obtained a very similar number of significant results at the 5% level (a
mean 39.5 for MFCC and<a href="#fn:38" id="fnref:38">38</a>for LPC); the distribution varies, with a weak tendency to reverse the
sign of the distance correlation, relative to the other feature type.<br><br>

<h4>Individual Speaker trends</h4>

To get a sense of the stability of the results, we can see if characteristic patterns persist for each
speaker. Table 3 characterises our speakers across all model sets. Significant effects are seen in all
speakers, with some consistency between effects over the model sets:
<br><br>
Mean significant tasks
Speaker
Average
effect
size
Overall
+ve
Pair correlation
-ve Sign
consist-
ency
Effect
size Overall
+ve -ve
“ARA14” 0.048 2.6 1.4 1.3 52% 0.731 0.58 0.85 0.43
“GJN14” 0.070 4.0 2.5 1.5 63% 1.556 1.60 2.50 1.00
“HLH30” 0.045 2.5 1.0 1.5 60% 0.643 0.63 0.40 1.00
“JSE11” 0.073 4.8 2.4 2.4 50% 1.115 1.31 1.90 1.00
“JTN20” 0.065 4.5 1.6 2.9 64% 1.368 1.71 1.18 2.30
“JYN22” 0.045 1.4 1.0 0.4 73% 1.125 2.20 8.00 0.75
“KBN30” 0.068 2.6 0.9 1.8 67% 0.818 0.57 0.37 0.78
“SCA01” 0.083 4.6 2.4 2.3 51% 1.222 1.76 2.71 1.29
“SHA13” 0.088 4.0 2.0 2.0 50% 1.522 1.28 1.07 1.60
“SKN03” 0.058 3.1 1.9 1.3 60% 0.657 0.78 0.94 0.63
“TMY30” 0.065 3.6 1.3 2.4 66% 0.897 0.76 0.53 1.00
“ZSE07” 0.040 0.6 0.1 0.5 80% 0.889 0.45 0.13 1.33
Mean 0.062 3.2 1.5 1.7 61% 1.045 1.14 1.71 1.09

<br><br>
Table 3. Significance and sign of correlations within, and between, speakers.
<br><br>

‘Sign consistency’ is a measure of whether the speaker was a consistent converger or diverger. It is
the percentage of all significant speaker-tasks that fall under that speaker’s most common sign
37(where scores close to 50% thus indicate no strong tendency to either sign) . The average value is
61%; i.e. there is only a mild tendency for a speaker to accommodate in one direction only.
The ‘pair correlations’ are an attempt to see if highly accommodative speakers are correlated with
higher accommodation in their interlocutor, by effect size, number of significant tasks and sign. It is
a simple ratio of the pair’s condition means; thus values near to 1 indicate closeness between the
results of a pair. We see a very strong fit between two (absolute) correlation magnitudes, a strong
difference between the number of divergent tasks, and a close fit between the mean number of
negative results for (which, again corresponds to convergence), but not much closer than the ratio
for each speaker’s overall mean.


Conversation length and task difficulty correlations


In Table 4 below, the ‘Average length’ is simply the average time of each task’s final utterance in
seconds, plus or minus the standard error of the sample mean. The final column is based on the
overlap of the average SEM between Positive and Negative conditions. The absence of overlap is
taken as a binary cut-off for being able to report a true (average) difference between the Positive
and Negative conditions.
Model set
Average length in each condition
Significant distinction
between conditions?

Positive Negative Null Stuart-Smith et al. 585 ±51  427 ±43  430 ±33  Significant
Replication (MFCCs) 595 ±51  465 ±36  502 ±27  Significant
Single-state LPCs 572 ±48  577 ±32  440 ±34  Not significant
Two-state MFCCs 562 ±54  508 ±50  505 ±28  Not significant
Two-state LPCs 607 ±60  552 ±50  477 ±26  Not significant
Three-state MFCCs 619 ±53  517 ±51  475 ±26  Not significant
Three-state LPCs 688 ±74  595 ±44  446 ±21  Not significant
Four-state MFCCs 583 ±53  536 ±49  487 ±28  Not significant
Four-state LPCs 619 ±50  594 ±65  460 ±23  Not significant

Table 4. Average length in each condition, and overlap of SEM for each model type .


Taking non-overlapping standard errors as a criterion for reporting a significant difference between
conditions, we can only consider h 2 (the hypothesised effect between length and convergence)
confirmed in the single-state MFCC model. This implies, again, that the link between longer task
length and convergent accommodation is obscured by including temporal effects (or, that the link is
an artefact arising from considering all the acoustic evidence at once).
<br><br>