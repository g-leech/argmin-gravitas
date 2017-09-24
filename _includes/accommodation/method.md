This section describes the SSSV experiment and elaborates the pipeline used to build and evaluate our HMMs, and thereby infer accommodative effects.
<br><br>

<h3>5.1. Experiment design</h3>

The experimental work was prior to the present project, but we characterise the setup to clarify the data. 

Twelve female volunteers, all native to Glasgow, and previously unacquainted, were assigned into pairs. (These variables were controlled because gender and accent have been shown to have strong effect on accommodation, and because prior acquaintance is a direct confounder of the desired effect. <a href="#fn:45" id="fnref:45">45</a>  <a href="#fn:46" id="fnref:46">46</a> )

Conversations were conducted behind screens to minimise the effect of nonverbal cues and visual influences. The data is anonymised with speaker codes (e.g. “ARA14”) which serve to identify the speaker models in the following analyses.

Each pair completed the same twelve tasks from a set of standardised dialogue elicitation tasks, ‘Diapix’, which involve spotting the differences in pairs of pictures of the same scene .<a href="#fn:47" id="fnref:47">47</a> The order of completion was randomised to prevent any confounding influence from the progression of tasks. 

The recordings were manually segmented into 109,730 words by all speakers (9,516 for training and 100,214 for test). Twelve speakers conducted twelve tasks each, so we obtained 144 analysis units. Twelve of these, one task per speaker, were used as training data; thus we end up with 132 ‘speaker-task’ units.

The numbering below refers to the stages defined in Table 1. Stages (a,b) and (c) are the Diapix experiment and the professional manual annotation of the conversations, respectively. The computer segmentation of the conversation files into word files (a trivial operation involving the application of pre-processed annotations) requires no further explanation. The following section elaborates on the implementation of stages (d) through (i).
<br><br><br>


<h3>5.2. Analytical method</h3>
For the inference stage, the unit of analysis is the pair-task: that is,
accommodative effects are measured for either speaker of a pair, for each of the 12 tasks they
completed. But begin by clarifying the way the data is handled on the way to this inference. It involves a hierarchy of four orders:

<ul>
	<li>i. At time correlation, the unit is the speaker-task: speaker A’s accommodative trend over the course of conducting one Diapix task.</li><br>

	<li>ii. When evaluating model likelihoods, the unit is the utterance \(O^{(A)}_i\) considered as a single sequence;</li><br>

	<li>iii. At feature extraction (and for each time step of forward-backward evaluation), the unit is the observation (a vector of coefficients representing one frame within an utterance);</li><br>

	<li>iv. And the foundational unit is the single coefficient of an observation vector. (This level is not directly manipulated.)</li>
</ul><br>




Figure 10 illustrates these four levels by expanding one instance of each unit of analysis into subunits:
<br><br>

<center>
[TODO: UML]<br><br>
	(i) Speaker (1) --> Tasks (M)<br>
	(ii) Task (1) --> Words (M)<br>
	(iii) Word (1) --> Frames (M)<br>
	(iv) Frame (1) --> Coefficients (M)
</ol>

<br><br>
Fig.10 - Different representations of the same data from one speaker.
<br><br><br>

</center>


That is: Level (i) is the speaker-task level; it shows speaker A’s whole sequence of words, separated by task. Level (ii) denotes the set of words spoken during task 4, Level (iii) shows a word split into its T constituent frames or observations. Level (iv) shows one frame, a vector of D coefficients extracted from that frame of the signal.<br>

<center>
<img src="/img/accommodation/11.png" width="100%" height="100%" />

<br><br>	<i>Fig.11</i> - SSSV experimental results: rows indicate each speaker pair, columns indicate the task.
</center><br>

Blanks indicate tasks used as training data. Bubbles scale to the size of the speaker -task correlation value,
with the positive condition in red, negative in blue, and asterisks for significance, * = 5% and ** = 1%.

<br>
Finally, for the final analysis, speaker-tasks are paired up according to the original conversational
partners, in order to categorise the disjunctive presence of accommodation for each task, i.e. its
presence in either speaker (see below for the conditioning used). This is Fig.11.

<br><br>
<hr /><br>


<h4>5.2.1. Feature extraction and labelling</h4>

{%		include accommodation/features.md 	%}

<br><br>
<hr /><br>


<h4>5.2.2. Model definition and training.</h4>

{%		include accommodation/training.md 	%}

<br><br>
<hr /><br>


<h4>5.2.3. Recognition and evaluation</h4>
At last we can produce likelihoods for each word of each speaker. <a href="#fn:62" id="fnref:62">62</a> The implementation of the forward-backward algorithm (E15) is handled indirectly by the HTK tool ‘HRec’. 

Two complete passes are made for each utterance of each speaker ( i.e. two iterations over the set $$O^{(A)}$$ ), evaluating each utterance $$O_i^{(A)}$$, conditional on both $$\theta_A$$ (“self” mode) and $$\theta_B$$ (“target” mode). This produces two ‘recognition’ (.rec) files for each utterance, which include its log likelihood and the time it was uttered. 

The two log likelihoods are then simply subtracted in order to obtain a point speaker-distance estimate for each utterance. 

(In the present method, this is 
$$log \, P(O^{(A)} \,|\, \theta_A) - log \, P(O^{(A)} \,|\, \theta_B) $$ with $$d_i > 1$$ indicating superior explanatory power of $$A$$ compared with $$B$$.)

<br><br>
<hr /><br>


<h4>5.2.4. Correlating the distances</h4>

These sets of ratios are paired with the time the utterance began, forming a set of tuples
We use [SciPy] Spearman’s rank correlation coefficient (E27) as a measure of the ordinal trend of these
tuples. $$\rho$$ was chosen for robustness to outliers (by comparison to Pearson’s $$r$$). 
<br><br>
<hr /><br>


<h4>5.2.5. Inferring accommodation</h4>

On obtaining speaker-task values for , we pair the speakers up and assign each of the resulting pair-
tasks one of three ‘conditions’ by the algorithm in Fig.19:<br><br>



<pre><code>
For each pair-task:

1. If both correlations are not significant at 5% :
   1.1. Terminate: Null condition for this pair task.
2. If both correlations are significant at 5% and have the same sign, S :
   2.1. Terminate: pair are the condition opposite of S.
3. If only one correlation is significant at 5% :
   3.1. Call the significant correlation's sign `S`.
   3.2. Terminate: pair are the condition indicated by the opposite of `S`.
4. If both are significant at 5% and the signs are mixed :
   4.1. Call the correlation with the largest absolute value `C1`.
   4.2. Call `S` the sign of `C1` 
   4.3. Terminate: pair are the condition opposite to `S`.<br>
</code></pre>

<center>
<i>Fig.19</i> - Algorithm for conditioning each pair of speaker-tasks, for inference purposes. (We take the opposite correlation sign following the terms in the SSSV experiment.)
</center>
<br><br>


That is, the 'positive' condition is where the largest significant correlation is positive; the 'negative' condition is where it is negative; and 'null' is where neither correlation is significant. These categories are our final, high-level results for each model set. 

We want to detect accommodation at either tail, if either speaker is either converging or diverging reliably over the course of a task. We then attempt to distinguish convergence (the Positive condition) and divergence (the Negative condition) by controlling the detected accommodation for the task’s length (which is a proxy for the pair experiencing difficulty). The hypotheses under test for each pair-task:

<br>
$$ h_1 $$ = Accommodation occurs (statistically significant at 5% or 1% level).<a href="#fn:64" id="fnref:64">64</a><br>
$$ h_2 $$ = Convergence is found more on tasks which take longer.
<br><br>

But are we actually measuring the tendency of speakers to paralinguistically react to each other, or are the models generating spurious patterns? One option to validate a novel computational measure would of course be comparing the results of subjective detecti on for the conversations; however, the small effect size and concomitant expert linguistic attention was beyond this project’s resources. The lack of developed objective holistic measures for accommodation (q.v.) makes it difficult to validate new approaches directly.

We thus investigate and (somewhat) validate the results by two indirect means: 

&nbsp;&nbsp;&nbsp;&nbsp;
1) the binomial probability of obtaining the n significant results that we did is computed by a cumulative binomial
test, giving a sense of the probability or improbability of the models generating the patterns by (pure) chance; 

&nbsp;&nbsp;&nbsp;&nbsp;
2) looking for significant differences in the average lengths of the tasks in each condition. The latter two offer some validation because of the constant complexity of each Diapix task: if the pair reliably accommodate more when the task is taking longer, this further regularity in the data speaks against mere-chance detection of accommodation.

The analysis is two-tailed: what is measured is the convergence or divergence of paired speaker models over time.
<br><br>

<hr /><br>
