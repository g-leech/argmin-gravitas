To reiterate: the emission function of each state of each HMM is implemented as a Gaussian mixture
model (GMM), a further nested stochastic process for the system:<br><br>

<ul>
	<li><span class="heav">Gaussian mixture model</span>: A weighted sum of \(G\) unimodal Gaussian distributions. The parameters for an GMM-HMM’s emission function are simply: 
	$$
		E^{(A)} =  \{ \pi^{(A)}, \sigma^{(A)}, \mu^{(A)} \}
	$$

	(Where \(\pi^{(A)}\) is a set of \(G\) sets of mixture coefficients (one set per state);  \(\sigma^{(A)}\) is a set of \(G\) covariance matrices; and \(\mu^{(A)}\) a set of \(G\) means. Note that in the present model set only diagonal covariances are used. The mixture weights are probabilities, and so must follow ∑ and .)</li>
</ul><br>

<center>
<img src="/img/accommodation/5.png" width="55%" height="55%" />

<br><br>	<i>Fig.5.</i> Each individual state of a HMM is itself a set of \(G\) sub-states, <br>the Gaussian components \(j_g\).
<br><br>
</center><br>

The probability of generating an observation from a GMM is the {sum of the probability of selecting each Gaussian component ( $$ \pi_G $$ )} multiplied by the {probability of generating $$ o_t $$ from that Gaussian}:

$$
	 P(O^{(A)}_t \, | \,S_j) = \sum_{g=1}^G \,\, \pi_g \,\times\, \aleph\, \large( 	o_t, \mu_{jg}, \sigma_{jg}	\large) 				\qquad\qquad \text{(E8)} 
$$

Where $$ \aleph $$ denotes the normal distribution. The product in (E8) can be understood by saying each state of an HMM possesses $$G$$ sub-states, the choice of which is a discrete latent variable, illustrated in figures 5 and 6.


HMMs are actually a generalisation of GMMs in which the mixture weights that select the emission
function for each observation are dependent on a Markov process (i.e. the multiple states and their
conditional transitions), rather than being independent constant probabilities, as in figure 6. That is,
a GMM is a single-state HMM.

As such we can generalize (E8) for HMMs: the probability of an HMM generating an observation sequence $$O^{(A)}_t$$ is: 

$$
	 P(O^{(A)}_t \, | \,\theta_A) = \prod_{j=1}^n \,\, \large(\,\, \sum_{g=1}^G \,\, \pi_g \,\times\, \aleph\, \large( 	o_j^{(A,i)}, \mu_{g}, \sigma_{g}	\large) \,\,\large)				\qquad\qquad \text{(E9)} 
$$

Where $$j$$ is the present frame of word $$i$$ and $$n$$ is the number of frames taken from $$i$$. i.e. When evaluating a given sequence of observations, the GMM-HMM takes the product of the probability sums of (E8). <br><br>



Finally, consider some useful properties of GMMs: 

<ol>
	<li><i>Flexibility</i>. GMMs are able to model asymmetrical input vectors, as well as multiple modes in the data distribution.</li> <br>
	
	<li><i>Multiple modes</i>. The characteristic spectral signature of different speakers can manifest as multiple modes. Some indication of how this happens is given in the discussion of MFCC features in Section 5.2.d).</li><br>

	<li><i>Tractable parameter estimation</i>. The GMM’s simple parameters, particularly the use of high-order (but diagonal) covariances, serve to preserve the tractability of the parameter algorithms for HMMs.</li>
</ol>

<br><br>
<center>
<img src="/img/accommodation/6.png" width="100%" height="100%" />

<br><br>	<i>Fig.6</i> - A HMM considered as a nested state diagram: a series of sets of Gaussians.
<br><br>
</center><br>


<hr /><br>