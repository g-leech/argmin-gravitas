<style>
	tr.under>td {
  	padding-bottom: 1em;
	}
</style>

<div class="accordion">
	<h3>Mathematical Glossary</h3>
	<div>
		<h3>Indices</h3>
		<table>

		<tr>
			<td>\( A \) :</td> 
			<td>a speaker.</td>
		</tr>

		<tr>
			<td>\( B \) :</td> 
			<td>another speaker, interlocutor with A.</td>
		</tr>

		<tr>
			<td>\( \theta_A \) :</td> 
			<td>a model of A (that is, a HMM trained on vectors from words by A).</td>
		</tr>

		<tr>
			<td>\( \theta_B \) :</td> 
			<td>model of B, A's interlocutor.</td>
		</tr>


		<tr>
			<td>\( i \) :</td> 
			<td>index of given word</td>
		</tr>

		<tr>
			<td>\( j \) :</td> 
			<td>index of given coefficient</td>
		</tr>

		<tr>
			<td>\( k \) :</td> 
			<td>index of current task</td>
		</tr>

		<tr>
			<td>\( t \) :</td> 
			<td>index of given time</td>
		</tr>


		<tr>
			<td>\( T \) :</td> 
			<td>total number of frames</td>
		</tr>

		<tr>
			<td>\( Ta \) :</td> 
			<td>task; a conversation</td>
		</tr>

		<tr>
			<td>\( n_i \) :</td> 
			<td>no. of frames extracted from word i</td>
		</tr>

		<tr>
			<td>\( N_{(A)}\) :</td> 
			<td>total no. of words spoken by speaker A</td>
		</tr>

		<tr>
			<td>\( N^{(A)}_k\) :</td> 
			<td>total no. of words spoken by A during task k.</td>
		</tr>


		<tr>
			<td>\( BW \) :</td> 
			<td>for Baum-Welch. Denotes training data.</td>
		</tr>

		<tr>
			<td>\( V \) :</td> 
			<td>for Viterbi. Denotes test data.</td>
		</tr>

		<tr>
			<td>\( g \) :</td> 
			<td>index of current Gaussian</td>
		</tr>
		</table>
		
		<br><br>


		<h3>Hyperparameters</h3>
		<table>
			<tr>
				<td>\( D \) :</td> 
			<td>the dimensionality of the vector space of features. (Here, 12.)</td>
			</tr>

			<tr>
				<td>\( G \) :</td> 
			<td>no. of Gaussians in each mixture model. (Here, 10.)</td>
			</tr>
		</table>
		
		<br><br>


		<h3>Data structure</h3>
		<table>
			<tr class="under">
				<td>\( o_t \) :</td> 
				<td>an observation at time \(t\); a 12D vector representing one frame of one word waveform. <br>
				(Also depicted as \( o^{(A,i)}_j\), see below.)<br></td>
			</tr>

			<tr class="under">
				<td>\( O^{(A)}_i = ( o^{(A,i)}_1, o^{(A,i)}_2, ... o^{(A,i)}_n) \) :</td> 
				<td>an utterance (or, the sequence of observations extracted from word \(i\) spoken by \(A\) )<br></td>
			</tr>

			<tr class="under">
				<td>\( O^{(A)} = \{ O^{(A)}_i \} \) :</td> 
				<td>all words spoken by \(A\) (or, the set of sequences of observations taken from words spoken by \(A\) ). <br>
				Also equivalent to \( Ta^{(A)} \), all the tasks of \(A\). <br></td>
			</tr>

			<tr class="under">
				<td>\( o^{(A,i)}_j = o_t \) :</td> 
				<td>a frame (vector \(  \large[ 	o^{(A,i,j)}_1, o^{(A,i,j)}_2, ..., o^{(A,i,j)}_D	\large]   \) of coefficients taken from word spoken by \(A\)<br></td>
			</tr>

			<tr class="under">
				<td>\( Ta^{(A)}_k \) :</td> 
				<td>all words spoken by \(A\) during task \(k\) (the set of sequences of observations extracted)</td>
			</tr>
		</table>
		
		<br><br>


		<h3>Modelling</h3>
		<table>
			<tr>
				<td>\( \theta_A \) :</td> 
				<td>the HMM parameter set. Exhausted by \( \{  N_S,M, Tr^{(A)}, E^{(A)}  \}  \) :</td>
			</tr>

			<tr>
				<td></td>
				<td>
					<ul>
						<li>\( N_S \) : number of states in the HMM</li>
						<li>\( M \) : symbols emitted per transition</li>
						<li>\( Tr^{(A)} \) : a set of transition functions for \( \theta_A \).</li>
						<li>\( E^{(A)} \) : a set of emission functions for \( \theta_A \).</li>
					</ul>
				</td>
			</tr>

			<tr>
				<td>\( E_j  \) :</td> 
				<td>the GMM emission function for state \(j\). Exhausted by \( \{ \pi, \sigma, \mu \} \):</td>
			</tr>


			<tr>
				<td></td>
				<td>
					<ul>
						<li>\( \pi_j \) : a set of mixture coefficients</li>
						<li>\( \sigma_j \): a set of covariance matrices (initial state probabilities)</li>
						<li>\( \mu_j \) : a set of means</li>
					</ul>
				</td>
			</tr>

			<tr>
				<td>\( S^{(A)}_i \) :</td> 
				<td>a state sequence taken by \( \theta_A \) while processing a word \( i \)</td>
			</tr>

			<tr>
				<td>\( S^*_i \) :</td> 
				<td>an optimal state sequence given word \( i \).</td>
			</tr>

			<tr>
				<td>\( s(t) \) :</td> 
				<td>the state of the model at time \(t\)</td>
			</tr>

			<tr>
				<td>\( \alpha_j(t) \) :</td> 
				<td>the Forward probability (sum of being in state \(j\) at time \(t\) ).</td>
			</tr>

			<tr>
				<td>\( \beta_j(t) \) :</td> 
				<td>the Backward probability of being in state \(j\) at time \(t\).</td>
			</tr>

			<tr>
				<td>\( L_j(t) \) :</td> 
				<td>the probability of the HMM occupying state \(j\) at time \(t\).</td>
			</tr>
		</table>
		
		<br><br>


		<h3>Inference</h3>
		<table>
			<tr>
				<td>\(  d^{(A)}_i  = \frac{ p(O_i^{(A)} | \theta_A }{ p(O_i^{(A)} | \theta_B } \)</td> 
				<td>&nbsp;&nbsp;&nbsp;
				speaker-distance: the likelihood ratio of \(A\) making an utterance, relative to &nbsp;&nbsp;&nbsp;\(B\) doing so.</td>
			</tr>

			<tr>
				<td>\(  t_i^{(A)} \) :</td> 
				<td>&nbsp;&nbsp;&nbsp;the time at which speaker \(A\) uttered word \(i\).	</td>
			</tr>

			<tr>
				<td>\(  \rho \large(\,\, 	t_i^{(A)}, 	d^{(A)}_i \large) \)</td> 
				<td>&nbsp;&nbsp;&nbsp;Spearman’s rank correlation between paired time & speaker-distance.</td>
			</tr>
		</table>

		<br><br>

	</div>
</div>


<br><br>

<hr />
<br>