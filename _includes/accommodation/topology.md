<div>
	Finally, we can categorise HMMs by key parameters:<br><br>

	<ul>
		<li>\(N\). (Number of states per model.) Single-state HMMs do not take into account the temporal information in the sequence; they can thus be usefully considered a separate class, the 'GMM' (see Section 3.1.1 below).</li><br>

		<li>\(S\). (Continuous or discrete states.) In the following, only HMMs with discrete state spaces are considered; extensions of the models to continuous hidden variables exist, but were not applied.</li><br>

		<li>\(M\). (The alphabet size.) In a sense, the alphabet of each of the present HMMs is of size 1; the models only seek to recognise one speaker at a time, and simply report a probability of this being the case. In another sense, we have a meta-alphabet of 12 (i.e. one symbol for each different speaker.)</li><br>

		<li>\(E\). The nature of the emission function. In each model below, the emission functions are a mixture of ten Gaussians, making the models Gaussian-mixture-model hidden Markov models (GMM-HMMs).</li><br>

		<li>\(MO x\). The order of the Markov property; i.e. how many prior states the transitions are conditionalised upon. The present models are all first-order Markov models, as defined in equation (E1).</li><br>

		<li>\(T\). The model topology, as embedded in the transition function:<br><br>
		
		<ul>	
			<li><i>Ergodicity</i>. That is, whether all states can transition to all other states. The present models are non-ergodic.</li><br>

			<li><i>Directionality</i>. Whether states can transition backwards as well as forwards. The models here developed are ‘Left-Right’ models, allowing only transitions of nondecreasing index, as in (E5):
			$$ \forall \, ij \,\, | \,\, i \lt j, \quad  p(s_i | s_j) = 0. \qquad\qquad \text{(E5)} $$
			</li><br>

			<li><i>Self-looping</i>. Whether states can ‘transition’ to themselves at a time step. All states in the present models allow self-looping; this  property is:
			$$
				\forall \, i \,\,\, p(s_i | s_i) \neq 0 \qquad\qquad \text{(E6)}
			$$
			</li><br>

			<li><i>Skips</i>. Whether state transitions can bypass the next indexed state. The present models disallow this; the no-skip property is defined in (E7):
			$$ \forall \, ij \,\,\, |\, (s_i \rightarrow s_j)\,  \& \,\, i \lt j, \quad  (j - i) = 1. \qquad\qquad \text{(E7)} $$
			</li>
		</ul>
		</li><br>

	</ul>
</div>
