<div class="accordion">
	<h3>Linguistics Glossary</h3>
	<div>
	<ul>

		<li><span class="heav">Utterance</span>: a token of a speaker: here, one word decomposed into a sequence of vectors, i.e \(O^{(A)}_i\). (Used in place of ‘word’, since the latter is ambiguous between the symbol – the type – and the individual waveform – the token.)</li><br>

		<li><span class="heav">Speech processing</span>: The analysis of speech as a digital signal (and thereby as a mathematical object). As opposed to</li><br>

		<li><span class="heav">Natural language processing</span> (NLP): The analysis of speech as symbolic information, as syntax and semantics (and thereby as a mathematical object).</li><br>

		<li><span class="heav">(Automatic) speech recognition (ASR)</span>: NLP for words. The decoding of speech signals into the most likely words (or sentences).</li><br>

		<li><span class="heav">Paralinguistics</span>: The analysis of the ‘supra-segmental’ parts of communication: including rhythm, pitch, volume and intonation. (That is, of those components that extend over multiple units of speech, e.g. phonemes.)</li><br>

		<li><span class="heav">Social signal processing</span> (SSP): The analysis of interaction context (i.e. of properties beyond the literal communication). Involves automated inference from data to phenomena that cannot be directly observed, but only inferred from correlated behaviours.</li><br>

		<li><span class="heav">Speaker verification</span>: Particular task in speech processing: given a speaker model, how likely is it that some new speech sample is by that speaker? The decoding of speech signals to the most likely speaker.</li><br>


		<li><span class="heav">Speaker distance</span>: the likelihood ratio of the model of the actual speaker (\(A\)) of an utterance \(o_t\) generating \(o_t\), relative to the likelihood of their interlocutor model (\(B\)) generating \(o_t\).  </li><br><br>

		<li><span class="heav">Finite state-machine (FSM)</span>: An abstract computer used to generate or test sequences of symbols. It consists in 1) a set of states, 2) a set of links (“transitions”) between these states, and 3) an alphabet of symbols labelling these links. An FSM transitions from its current state, to one of its connected states once per time step, and emits an observation every time a state is entered.</li><br>

		<li><span class="heav">Probabilistic finite-state machine</span> (PFSM): A tool for representing probability distributions over sequences of observations. A family of nondeterministic models that change state and emit signals according to the current emission function (the density function of the current state).</li><br>

		<li><span class="heav">Transition probability</span>: \( p( S_t | S_{t-1} ) \); the chance of the PFSM switching from state \( S_t \) to \( S_{t-1} \)</li><br>

		<li><span class="heav">Emission probability</span>: \( p( o_t | S_{t} ) \); the chance of observing \( o_t \)on entering state \( S_{t} \).</li><br>

		<li><span class="heav">Markov model</span>: A stochastic model, representable as a PFSM, in which the <i>Markov property</i> holds (that is, in which the transitions between each state depend only on the most recent antecedent state).	</li><br>

		<li><span class="heav">Hidden Markov model</span>: A Markov model in which the data-generating process is hidden, but observations dependent on the states are available. Thus, an HMM is a joint probability distribution over outputs and hidden states, \(  p(O|S) \).<br><br>

		The number of states in the model is the number of 'sections' the input sequence is treated as having. State transitions are determined by a transition matrix (i.e. a prior probability distribution); on entering a state, a vector is generated from the new emission function (the distribution associated with the state being entered). This is illustrated in Fig.3.</li><br>

		<li><span class="heav">Learning</span>: Determining optimal model parameters from training data (see section 3.1.2).</li>	<br>
		<li><span class="heav">Decoding</span>: Determining an optimal sequence of states, given an observation sequence.</li> 	<br>
		<li><span class="heav">Evaluation</span>: Determining the total likelihood of an observation sequence, given a trained model and a decoding.</li><br>

		
		<li><span class="heav">Gaussian mixture model</span>: A weighted sum of \(G\) unimodal Gaussian distributions.</li><br>

		<li><span class="heav">Ergodicity</span>. Whether model states can transition to all other states.</li><br>

		<li><span class="heav">Directionality</span>. Whether model states can transition backwards as well as forwards.</li><br>

		<li><span class="heav">Self-looping</span>. Whether states can ‘transition’ to themselves.</li><br>

		<li><span class="heav">Skips</span>. Whether state transitions can bypass the next indexed state.</li><br>

		<li><span class="heav">Forward-backward algorithm</span>: Dynamic programming algorithm used both in training the HMMs (to find an input to the Baum-Welch procedure – the probability of being in a given state at a given time) and in the ‘total likelihood’ evaluation of each utterance.</li> <br>

		<li><span class="heav">Frame</span>: A fixed-length portion of a speech signal, from which to extract a feature.</li><br>
		
		<li><span class="heav">Windowing</span>: The subdivision of a signal into successive frames. (Perhaps overlapping frames, as here.) The resulting windows are multiplied by a ‘window function’ that eliminates the signal lying outwith the present interval.</li><br>

		<li><span class="heav">Extraction rate</span>: how often the processor takes a window from the signal. Here, 10ms.</li><br>
		<li><span class="heav">Window size</span>: the length of each window. Here, it is 25ms long. So we have overlapping windows.</li><br>

		<li><span class="heav">Mel-Frequency Cepstral Coefficients (MFCCs)</span>: Type of acoustic vector. Uses cepstral coefficients from a Fourier transform of the original waveform.</li><br>

		<li><span class="heav">Linear predictive coding (LPC)</span>: early digital speech encoding scheme. Rather than storing all information from a signal, we can model signals by estimating the magnitude of the signal at \(t\) as a linear combination of the previous \(p\) samples. The resulting feature vector is just a sequence of linear coefficients plus a set of errors. </li><br>
	</ul>
	</div>
</div>