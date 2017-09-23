This section describes the SSSV experiment and elaborates the pipeline used to build and evaluate our HMMs, and thereby infer accommodative effects.
<br><br>

<h3>5.1. Experiment design</h3>

The experimental work was prior to the present project, but we characterise the setup to clarify the data. 

Twelve female volunteers, all native to Glasgow, and previously unacquainted, were assigned into pairs. (These variables were controlled because gender and accent have been shown to have strong effect on accommodation, and because prior acquaintance is a direct confounder of the desired effect.)

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
	<li>i. At time correlation, the unit is the speaker-task: speaker A’s accommodative trend over the course of conducting one Diapix task.</li>

	<li>ii. When evaluating model likelihoods, the unit is the utterance considered as a single sequence;</li>

	<li>iii. At feature extraction (and for each time step of forward-backward evaluation), the unit is the observation (a vector of coefficients representing one frame within an utterance);</li>

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


That is: Level (i) is the speaker-task level; it shows speaker A’s whole sequence of words, separated by task. Level (ii) denotes the set of words spoken during task 4, Level (iii) shows a word split into its T constituent frames or observations. Level (iv) shows one frame, a vector of D coefficients extracted from that frame of the signal.

<center>
<img src="/img/accommodation/11.png" width="100%" height="100%" />

<br><br>	<i>Fig.11</i> - Results from the SSSV experiment: rows indicate each speaker pair, columns indicate the task.
Blanks indicate tasks used as training data. Bubbles scale to the size of the speaker -task correlation value,
with the positive condition in red, negative in blue, and asterisks for significance, * = 5% and ** = 1%.

<br><br>
</center>


Finally, for the final analysis, speaker-tasks are paired up according to the original conversational
partners, in order to categorise the disjunctive presence of accommodation for each task, i.e. its
presence in either speaker (see section 5.2.i for the conditioning used). This final analysis is
illustrated in Fig.11.

<br><br>


<h4>Feature extraction and labelling</h4>

{%		include accommodation/features.md 	%}


e. Model definition and training.

Topology: The first step in modelling an HMM is to define the topology and input type. The optimal
number of states is not known and cannot be readily learned from the data; this then is one of our
design variables.


<center>
<img src="/img/accommodation/16.png" width="100%" height="100%" />

<br><br>	<i>Fig.11b</i> -  State diagrams illustrating two HMMs N=3, one with, and one without state ‘skips’.

<br><br>
</center>

<br><br>

(where denotes the transition probability .)

We take the dimensionality of the input vectors D and set further hyperparameters: the number of
Gaussians, G = 10, the alphabet size M = 1. We constrain the transitions to be forward only (by E5)
and sequential (by E6). Again, we are creating a set of Left-Right, no-skip, continuous-density,
discrete-state, diagonal-covariance, first-order GMM-HMMs, distinguished by varying the number of
states and input vector type.

These set, we can define a prototype HMM for each speaker; an example of one such prototype
definition file can be seen in Fig.4, listing the dimensionality "\<VecSize\>" and the vector type used for the model,
a (presently) arbitary mean and covariance vector for each state, and a transition matrix for the whole model (in which each floating point number at row i and column j is ) . 

(Note that this file requires us to list N+2 states, in order to represent the terminal non-emitting states used by HTK.) In total we define and evaluate 8 sets of 12</a>speaker models, for N=1..4 and using both MFCC and LPC features.



<center>
<img src="/img/accommodation/16actually.png" width="100%" height="100%" />

<br><br>	<i>Fig.16</i> - The N=2 HMM file from Fig.4, after re-estimation.

<br><br>
</center>

<br><br>


Initialisation: In order to obtain optimal parameters from the Baum-Welch algorithm, we first
require good initial estimates for the values of the mean and covariance vectors of each state, in
order that the estimates may convergence more precisely to the local optima. We obtain this simply
by assigning the global mean and covariance of the training data to the emission distributions and
setting the transition probabilities to be equal, a ‘flat-start model’. This is performed with the HTK
tool ‘HCompV’ (see script 6).

Training : We perform10</a>iterations of simultaneous training of all models, or ‘embedded re-
estimation’ by use of the HTK tool ‘HERest’. This uses the Baum-Welch procedure of section 3.2.2,
but modified for parallel training as follows:

<br><br>
<pre><code>
1. Allocate an accumulator variable for each parameter of each HMM.
2. For each training utterance:
	2.1. Construct a composite HMM by joining up the HMMs in sequence.
	2.2. For this composite HMM, calculate forward probabilities for all states j & times t.
	2.3. Calculate the backward probabilities for all states j & times t.
	2.4. Use 2.2 & 2.3 to compute the probabilities of state occupation at each frame
	2.5. Combining these with the current utterance, by E23.
	2.6. Update the accumulators of each parameter by weighting the existing values by the new utterance, E24 and E25.
3. Use the final value in each accumulator as parameter estimates for each HMM.
</code></pre>

<br><br>
Fig.17 - Algorithm for embedded Baum-Welch re-estimation
<br><br>

Model testing,
Finding distance ratios,
Correlating distance over time
Parsing a task grammar into a word lattice


The default mode of HTK is continuous speech recognition; we are instead using the software in a
special case of ‘isolated word recognition’ (where there is no specific task grammar constraining
sequences of recognitions as there is in sentence-level ASR). Even so, the parameters of the relevant
HTK shell ‘HVite’ require us to define a ‘syntax’ (an acceptable sequence of symbols to recognise)
and a ‘dictionary’ (where the ‘words’ to recognise are defined in terms of models).
Our syntax is generated (by the tool ‘HParse’) from the simplest possible Backus-Naur Form
grammar: just one variable in a loop, the speaker code. The dictionary is simply a mapping from the
speaker code to the name of the model representing that speaker, where we include one instance of
this mapping per state in the HMM. The grammar, though non-functional in our case, is anyway
expanded into a graph of possible sequences, a ‘word lattice’ (a set of nodes which represent time
steps, and a set of transitions which represent symbol hypotheses).

<br><br>

<center>
<img src="/img/accommodation/18.png" width="100%" height="100%" />

<br><br>	<i>Fig.18</i> - Parsing the Backus-Naur grammar into a word network for the HVite recognizer.

<br><br>
</center>

<br><br>


<h4>Recognition and evaluation</h4>
At last we can produce likelihoods for each word of each speaker.<a href="#fn:62" id="fnref:62">62</a>

The implementation of the forward-backward algorithm (particularly E16) is handled indirectly by the HTK tool ‘HRec’. Two complete passes are made for each utterance of each speaker ( i.e. two iterations over the set ), evaluating each utterance conditional on both (“self” mode) and on (“target”
mode). This process produces two ‘recognition’ (.rec) files for each utterance, which include its log likelihood and the time it was uttered. The two log likelihoods are then simply subtracted in order to obtain a point speaker-distance estimate for each utterance. (In the present method, this is compared with B.)
, with indicating superior explanatory power of A

Checking a trained model against new data goes by ‘testing’, ‘scoring’, ‘evaluation’ discussed in the abstract, or ‘recognition’ or ‘transcription’ when applied to speech.
<br><br>


<h4>Correlating the distances</h4>

These sets of ratios are paired with the time the utterance began, forming a set of tuples
We use Spearman’s rank correlation coefficient (E28) as a measure of the ordinal trend of these
tuples. was chosen for its robustness to possible outlier values (by comparison to the linear
Pearson’s r). SciPy takes tuples as arrays.
<br><br>

<h4>Inferring accommodation</h4>

On obtaining speaker-task values for , we pair the speakers up and assign each of the resulting pair-
tasks one of three ‘conditions’ by the algorithm in Fig.19:<br><br>

For each pair-task:

<pre><code>
	1. If both correlations are not significant at 5%
	1.1. Terminate: pair are Null condition for this task.
	2. If both correlations are significant at 5% and have the same sign, S:
	2.1. Terminate: pair are in the condition opposite of S.
	3. If only one correlation is significant at 5%
	3.1. Terminate: pair are in the condition indicated by the opposite of the significant
	correlation's sign
	4. If both are significant at 5% and the signs are mixed:
	4.1. Terminate: pair are in the condition indicated by the opposite of the sign of the
	correlation with the largest absolute value.
</code></pre>

<br><br>
Fig.19 - Algorithm for conditioning each pair of speaker-tasks, for inference purposes. (We take the opposite correlation sign following the terms in the SSSV experiment.)
<br><br>


That is, the positive condition is the case in which the largest significant correlation is positive; the negative condition is the corresponding case for largest negative; and null if neither correlation is significant. These condition categorisations are our final, high-level results for each model set. We
first want to detect accommodation at either tail: whether either speaker is either converging or diverging reliably over the course of a task. We then attempt to distinguish convergence (the Positive condition) and divergence (the Negative condition) by cross-referencing the detected accommodation against the task’s length (which is a potential proxy for the pair experiencing difficulty). The hypotheses under test for each pair-task:

<br><br>
\( h_1 \) = Accommodation occurs (statistically significant at 5% or 1% level).<a href="#fn:64" id="fnref:64">64</a><br>
\( h_2 \) = Convergence is found more on tasks which take longer.
<br><br>

But are we actually measuring the tendency of speakers to paralinguistically react to each other, or
are the models generating spurious patterns? One option to validate a novel computational measure
would of course be comparing the results of subjective detecti on for the conversations; however,
the small effect size and concomitant expert linguistic attention was beyond this project’s resources.
The lack of developed objective holistic measures for accommodation (q.v.) makes it difficult to
validate new approaches directly.

We thus investigate and (somewhat) validate the results by two indirect means: 1) the binomial
probability of obtaining the n significant results that we did is computed by a cumulative binomial
test, giving a sense of the probability or improbability of the models generating the patterns by
(pure) chance; 2) looking for significant differences in the average lengths of the tasks in each
condition. The latter two offer some validation because of the constant complexity of each Diapix
task: if the pair reliably accommodate more when the task is taking longer, this further regularity in
the data speaks against mere-chance detection of accommodation.

The analysis is two-tailed: what is measured is the convergence or divergence of paired speaker models over ti me.
<br><br>

<hr /><br>
