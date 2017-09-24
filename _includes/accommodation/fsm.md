Abstractly, we can consider any speech signal as a statistical sequence of observations . For a multi-level phenomenon like speech, the terms of $$O$$ might be words, phonemes, or lower-level features like vectors of spectral coefficients extracted from the signal. The signal is treated statistically in order to make the analysis tractable, to handle the massive variability found between tokens of linguistic symbols, and because of the automation it
offers: the models we need can be optimised given just data and some initial guesses. <a href="#fn:27" id="fnref:27">27</a>

This consideration made, we can define the pattern recognition (or sequence analysis) problem as: “<i>How do we get a probability density function for the space of these sequences</i>?” 

There are two rough approaches: 

<i>Generative</i> models, which first describe the underlying data-generating process for $$O$$ as a model $$ \theta $$, then categorises tokens of that process by computing the probability of $$O$$ conditional on the model, i.e. 
$$
p(O | \theta)
$$ 


Or <i>discriminative</i> models, which don't model the original process, but instead directly categorise tokens via taking a conditional probability distribution over the signal, i.e. $$ p( \theta \,|\, O ) $$. <a href="#fn:28" id="fnref:28">28</a>
<br>

In linguistics, we generally want to map $$O$$ to a series of symbols, ‘decoding’ the signal into one or other familiar categories, as in:<br><br>

<ul>
	<li><span class="heav">(Automatic) speech recognition (ASR)</span>: The decoding of speech signals into the most likely words (or sentences).</li><br>
	<li><span class="heav">Speaker verification</span>: The decoding of speech signals to the most likely speaker.</li><br>
</ul>


“<i>The most important computational technique in spoken language processing</i>” is a type of generative model called the <i>finite-state machine</i>. <a href="#fn:29" id="fnref:29">29</a> <br><br>


<ul>
	<li><span class="heav">Finite state-machine (FSM)</span>: An abstract computer used to generate or test sequences of symbols. It consists in 1) a set of states, 2) a set of links (“transitions”) between these states, and 3) an alphabet of symbols labelling these links. <br>

	An FSM transitions from its current state, to one of its connected states once per time step, and emits an observation every time a state is entered.</li><br>
</ul>

Given a topology defining the structure of acceptable sequences, FSMs can be used to generate valid sequences of symbols incrementally or, equivalently, to check input sequences for validity.<br><br>


<center>
<img src="/img/accommodation/NFSM.jpg" />

<br><br>
	<i>Fig.1</i> - A finite state machine for checking if phoneme strings are English-like.<br>(Circles are states, lines are transitions.)
<br><br><br>
</center>

Since our object of study is neither a deterministic grammar nor a simple non-deterministic set of
acceptable strings, we need the sub-family of FSMs that can handle stochastic <i>distributions</i> of observations:<br><br>


<ul>
	<li><span class="heav">Probabilistic finite-state machine</span> (PFSM): A tool for representing probability distributions over sequences of observations. A family of nondeterministic models that change state and emit signals according to the current emission function (the density function of the current state).</li><br>

	<li><span class="heav">Transition probability</span>: \( p( S_t | S_{t-1} ) \); the chance of the PFSM switching from state \( S_t \) to \( S_{t-1} \)</li><br>

	<li><span class="heav">Emission probability</span>: \( p( o_t | S_{t} ) \); the chance of observing \( o_t \)on entering state \( S_{t} \).</li><br>
</ul>
Each state has an emission function $$ p(o_t | \theta) $$, where $$ \theta $$ is a model / parameter set learned from training data (observations labelled by correct symbols); the model overall has a transition matrix which determines the probabilities of moving from each state to each state. 

(This can include remaining in the same state, or “self-looping”.)
<br><br><br>



<h3>3.1. Markov and Hidden Markov models</h3><br>

{%		include accommodation/markov.md 		%}



<h3>3.2. Dynamic programming algorithms for HMMs</h3>

{%		include accommodation/dynamic-prog.md 		%}
