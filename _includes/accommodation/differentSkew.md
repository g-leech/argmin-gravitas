
The HMM is a complex concept. So it's useful to illustrate all the different ways we're using it:<br><br><br>

<ul>
	<li><h4>Represented in algebra as a parameter set \(\theta\):</h4>

	$$ \theta_A = \{  N_S,M, Tr^{(A)}, E^{(A)}  \}  \qquad\qquad\text{(E2)} $$

	(Which is to say: a hidden Markov model is specified completely by the number of
	emitting states in the model \(N_S\); the symbols emitted per transition \(M\); a set of initial
	state probabilities \(\pi^{(A)}\); a transition function \(Tr^{(A)}\); and \(E^{(A)}\) a set of emission functions.)<br><br>
	</li><br>

	<li><h4>Or as state diagram:</h4>

	<center>
	<img src="/img/accommodation/3.png" width="85%" height="85%" />


	<br><br><br>
			<i>Fig.3</i> - A simple no-skip Left-Right HMM, showing each state \(S_1, ...S_T\) <br>with a transition probability \( p(S_j | S_j-1)\), and 
				an emission probability distribution<br>
				yielding observation \(x_j \) with \( p(x_j | s_j) \). 
	</center>
	<br><br>


	Figure 3 illustrates the differing emission function of <i>each</i> state: a state is thus a joint probability
	distribution over a prior distribution (for controlling transitions) and a conditional distribution (for
	controlling the emission of observation vectors).<br><br><br></li>


	<li><h4>Or as the joint probability of observation sequence \(O\) given state sequence \(S\):</h4>

	$$
		P(S,O^{(A)} | \theta_A )	= P(O^{(A)} | S,\theta_A) \times P(S | \theta_A)
		\qquad\qquad\text{(E3)}
	$$

	<br>
	This inverse-probability calculation represent what an HMM <i>ends up</i> inferring. But for us the state sequence \(S\) is unknown for each input observation sequence. 

	So, read at face value, (E3) denotes a <i>visible</i> Markov model, not an HMM; we must instead compute the ‘total likelihood’ of our current
	observation sequence, by summing over all possible state sequences of length \(T\).</li><br><br>


	<li><h4>Or as the total likelihood of \(O\) for an unknown state sequence \(S\):</h4>

	That is, we take the product of the probability of entering an initial state, multiplied by the product of all transition
	probabilities, multiplied by the product of all emission probabilities for each successive component
	of the observation sequence \(O\):<br><br>

	$$
		P(O^{(A)} | \theta_A )	= P(S_1 | S_0) \times \prod_{T=2}^T \,\, P(S_t | S_{t-1}) \times \prod_{T=1}^T \,\, P(o_t | S_{t})
		\qquad\qquad\text{(E4)}
	$$

	</li><br><br>


	<li><h4>Or as computable definition:</h4>

	<center>
	<img src="/img/accommodation/4.png" width="95%" height="95%" />

	<br><br>
		<i>Fig.4</i> - The code for a two-state GMM-HMM representing speaker ‘ARA14’
	<br><br><br></center>


	This example has only two ‘real’ states plus two non-emitting start and end states. It is a Left-Right HMM owing to the
	zero transition probabilities in the matrix “transP” for right-to-left transitions. Note the generic means and variances: this is
	before model initialization and re-estimation. (Fuller discussion in section 5.2.)
	</li><br><br>


	<li><h4>Or, finally, by topology:</h4>

	{%	include accommodation/topology.md	%}

	</li>

</ul>


The baseline models used in this paper are Left-Right, self-looping, no-skip, continuous-density,
discrete-state, diagonal-covariance, first-order GMM-HMMs. <a href="#fn:33" id="fnref:33">33</a> This topology is assumed throughout.<br><br><br>

<hr /><br>
