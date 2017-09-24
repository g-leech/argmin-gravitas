<center>
<img src="/img/accommodation/fsms.png" width="55%" height="55%" />

<br><br>	<i>Fig.2</i>. The relation between key classes of automata, as nested sets.<br><br>
<br><br>
</center>

In speech, as with many real-world signals, the observation sequence is of too high dimensionality to directly estimate its joint conditional probability $$p(o_1, o_2, ...o_n | \theta)$$
from examples of words (or, for us, from examples of words betokening a speaker). 

To reduce the problem from estimating the class-conditional densities over arbitrary dimensions, we assume that the speaker is a parametric model (that is, has a fixed number of parameters). 

This limits a priori the distributions our model can use, but estimating these parameters is a much simpler and tractable problem.
The most common parametric model in linguistics is the Markov model:
<br><br>

<ul>
	<li><span class="heav">Markov model</span>: A stochastic model, representable as a PFSM, in which the <i>Markov property</i> holds (that is, in which the transitions between each state depend only on the most recent antecedent state).

	<center>
		$$  p(S_1, ... S_n) = \prod_{i=1}^n \,\, p(S_i | S_{i-1}) 		\qquad\qquad (E1)$$
	</center>
	</li>
</ul>	

In cases where the signal-generating process (the state sequence) is not itself observable, but must
be inferred from the emitted observations, call the Markov model a <i>hidden Markov model</i>. These
models are doubly stochastic: the underlying stochastic process (the state sequence) is inferred via
another set of stochastic processes (the observation sequence).<br><br>


<ul>
	<li><span class="heav">Hidden Markov model</span>: A Markov model in which the data-generating process is hidden, but observations dependent on the states are available. Thus, an HMM is a joint probability distribution over outputs and hidden states, \(  p(O|S) \).<br><br>

	The number of states in the model is the number of 'sections' the input sequence is treated as having. State transitions are determined by a transition matrix (i.e. a prior probability distribution); on entering a state, a vector is generated from the new emission function (the distribution associated with the state being entered). This is illustrated in Fig.3.<br><br>

	An HMM is built and used in three stages:<br><br>

	<ol>
		<li><span class="heav">Learning</span>: Determining optimal parameters from training data (see section 3.1.2).</li>		<br>
		<li><span class="heav">Decoding</span>: Determining an optimal sequence of states, given an observation sequence.</li> 	<br>
		<li><span class="heav">Evaluation</span>: Determining the total likelihood of an observation sequence, given a trained model and a decoding.</li><br>
	</ol>

	</li>
</ul>

Though based on Andrey Markov's century-old theory, the HMM itself was first formalised in the 60s<a href="#fn:31" id="fnref:31">31</a> and the applied
method has flourished only in the last twenty years.<a href="#fn:32" id="fnref:32">32</a>

{%	include accommodation/properties.html	%}



<h4>3.1.1. Representing HMMs</h4>

{%	include accommodation/differentSkew.md	%}



<h4>3.1.2. Gaussian mixture models (GMMs)</h4>

{%	include accommodation/mixtures.md	%}



<h4>3.1.3. Designing HMMs</h4>

In his essential introduction to HMMs<a href="#fn:37" id="fnref:37">37</a>, Lawrence Rabiner lays out the three basic problems of
designing HMMs and the standard algorithms used to overcome them:<br><br>

<ol>

<hr /><br>

	<li><h4><i>Conditional evaluation</i></h4>: given a model \(\theta_A\), what’s the probability of a given observation sequence \( o_t\), or set thereof \( O^{(A)}\)?<br><br>

		: We use the Forward-Backward algorithm to find this: it is the sum of probabilities of generating \(O\), given all different state sequences it arises in (see equation E3, above or): 
		$$
			p(O^{(A)} | \theta_A) = \sum_{s \in S} \,\, P(S, O^{(A)} \, | \, \theta_A )  		\qquad\qquad \text{(E10)}
		$$
		
		Or thirdly in terms of the forward probability \(\alpha_i \) at time \(t\): 
		$$
			p(O^{(A)} | \theta_A) = \sum_{j = 1}^{N} \,\, \alpha_i(t) \, \times \, P( S_N | S_i )  		\qquad\qquad \text{(E11)}
		$$
	</li><br>

<hr /><br>

	<li><h4><i>Best decoding</i></h4>: how to choose an optimal state sequence \(S*\) through \(\theta_A\)? <br><br>

		: We use the Viterbi algorithm to find the likeliest state sequence, such that: 
		$$
			S^* = arg \,max \,\, P(S \,|\, O^{(A)}, \theta_A)  		\qquad\qquad \text{(E12)}
		$$

		<br>(NB: For our purposes – isolated ‘word’ [speaker] recognition – the Viterbi algorithm is actually unnecessary, since the forward probability calculation provides us with a total likelihood.<a href="#fn:38" id="fnref:38">38</a>)
	</li><br><br>

<hr /><br>

	<li><h4><i>Model learning</i></h4>: How do we choose model parameters that maximise the likelihood of the training data, \( P(O_{BW} \,|\, \theta_A) \)?<br><br>

	We use Baum-Welch re-estimation to find the optimal parameters given a training set:
	$$
		\theta_A^* = arg \,max \,\, P( O_{BW} \,|\, \theta_A)  		\qquad\qquad \text{(E13)}
	$$

	This makes use of the Forward-Backward algorithm, to determine the total likelihood of a sequence over all state sequences that could generate it. (We go on to describe this vital machinery.)
	</li>
</ol><br><br>
