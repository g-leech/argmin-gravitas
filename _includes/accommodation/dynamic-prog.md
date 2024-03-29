<br>  

<h4>3.2.1. Forward-Backward smoothing</h4>

The forward-backward algorithm is used in training the HMMs (to find an input to the
Baum-Welch procedure – the probability of being in a given state at a given time) and in the ‘total
likelihood’ evaluation of each word. It involves the calculation of:<br><br>

<span style="padding-left:30px">a. the forward probability (joint probability of producing $$O$$ and ending in state $$j$$;</span><br>
<span style="padding-left:30px">b. the backward probability (conditional probability of completing sequence $$O$$ given start state $$j$$);</span><br>
<span style="padding-left:30px">c. and smoothing: the product of (a) and (b): a measure of the distribution of states over $$O$$.</span><br>

<br><br>
<center>
<img src="/img/accommodation/7.png" width="100%" height="100%" />

<br><br>	<i>Fig.7</i> - Calculating forward \(\alpha\) and backward \(\beta\) probabilities.</center><br>

The following is repeated for each time step:<br> 
<span style="padding-left:30px">(i) forward: sum probabilities of all the possible transitions to a given state \(i\) at time \(t\);<br></span>
<span style="margin-left:30px">(ii) backward: then sum probabilities of transitions leading <i>from</i> state \(i\) for the remainder of the observation sequence.<br><br></span>
<br><br>




<ol>
	<li><h4>Calculating the forward probability.</h4><br>
	The forward ‘probability’ <a href="#fn:40" id="fnref:40">40</a> - &nbsp;&nbsp; \(\alpha_j^{(A)}(t)\) &nbsp;&nbsp;- is the joint probability of observing the first \(t\) vectors of \(O\) and subsequently ending up in state \(j\) at time \(t\), i.e. \( P (\, o_1, ..., o_t \, , \, s(t) = j \,)\):

	$$
		\alpha_j(t) = P (\, [o_1, ..., o_t] \, , \, [s(t) = j] \,|\, \theta_A)  		\qquad\qquad \text{(E14)}
	$$

	<br>
	In practice a recursive pruning form is used:
	$$
		\alpha_j(t) = \large[\,  	\sum_{i=1}^N\, 
		\alpha_i(t-1) \, \times \, P(S_j | S_i )  			\,\large] \,\,\times \,\, P(o_t \,|\, S_j )
		\qquad\qquad \text{(E15)}
	$$

	where \(N\) is total states in the model.



The recursion in E15 depends on the fact that the probability of being in state \(j\) at time \(t\) and seeing
observation \( o_t \) is equivalent to the sum of forward probabilities for all possible predecessor states i,
weighted by the probability of making each transition from that predecessor,,

E15 has an initial condition:
	$$
		\alpha_j(1) = P(S_j \,|\, S_1 )    \,\,\times \,\,  P(o_1 \,|\, S_j )
		\qquad\qquad \text{(E16)}
	$$


And a final condition:
	$$
		\alpha_N(T) =   	\sum_{i=1}^N\, 
		\alpha_i(T) \, \times \, P(S_N | S_i )  		
		\qquad\qquad \text{(E17)}
	$$


Though generated in passing, \(\alpha_N(T)\) is actually our target variable in the evaluation step: it is the final forward probability, which is also the total likelihood of an observation sequence \(O_i^{(A)}\) given a speaker model \(\theta_A\):
	
	$$
	P(O^{(A)} \,|\, \theta_A ) = \alpha_N(T)
	\qquad\qquad \text{(E18)}
	$$


	</li><br>

	<li><h4>Calculating the backward probability</h4><br>
		The backward probability \(\beta_j(t) \)is the conditional probability of observing the remainder of the observation sequence from now to the end ( \( O_r = o_{t+1}, ..., o_T \) ), given that the model is in state \(j\) at time \(t\):
		$$
			\beta_j(T) = P(\,  O_r \,|\, s(t)=j, \, \theta_A)
			\qquad\qquad \text{(E19)}
		$$

		Again, in practice an efficient recursion is used to compute
		$$
			\beta_j(T) = \sum_{j=1}^N \,\, P(S_j \,|\, S_i) \,\times\, P(o_{t+1} \,|\, S_j ) \,\times\, \beta_j(t+1)
			\qquad\qquad \text{(E20)}
		$$
		Where the computation begins at the final time \(\beta_j(T)\) and ends by computing the backward probability of the first frame \(\beta_j(1)\).
	</li><br><br>


	<li><h4>Calculating the smoothed expectation</h4><br>
		Lastly, the product of \( \alpha \) and \( \beta \) yields us a maximum-likelihood of the sequence \(O\) at an occupied state \(j\):

		$$
			\alpha_j(t) \beta_j(t) = P(O, s(t)=j \,|\, \theta_A)
			\qquad\qquad \text{(E21)}
		$$

	</li>

</ol><br><br>


<h4>3.2.2. Baum-Welch re-estimation</h4>
From the statement of the learning problem, E14, we know that finding optimal parameters for a
HMM speaker model involves the , we need the probabilities of state occupation at each time. The
algorithm is as follows:

<pre><code>
1. For each training sequence:
	1.1. Calculate the forward probability, by E16.
	1.2. Calculate backward probabilities, by E21.
	1.3. Weight the contribution of the current sequence `O` to the 
	     model’s transition function.
	1.4. Weight the contribution of the current sequence `O` to the 
	     model’s emission function.
	1.5. Calculate new model parameters from the weighted average of 
	     [1.3] and [1.4] (that is, the initial state probabilities, 
	     transition probabilities, and emission probabilities.<br>
</code></pre>


We derive 1.3 and 1.4 from the product of the forward and backward probability densities (E22) as
follows. The probability of state occupation \(  L_j(t) \), for the chance of being in state \(j\) at time \(t\), is computed from the product
of the forward and backward probabilities (E21) by the following:

$$
\begin{align*}
	L_j(t) &= P(\, [s(t) = j] \,|\, O^{(A)}, \, \theta_A) \\\\
	&= \frac{ P(\, s(t) = j \,\,|\, \theta_A) } {P(O \,|\, \theta_A )}  \\\\
	&= \frac{ 1 } {P(O \,|\, \theta_A )} \, \times \,   \alpha_j(t) \beta_j(t)
\end{align*}
	\qquad\qquad \text{(E22)}
$$

(That is, the inverse of the total likelihood multiplied by the forward-backward product, E21.)

We can then set the means for each Gaussian of each state: the estimated mean $$ \hat{\mu}_{jg} $$ of the $$g$$th Gaussian of state $$j$$ is a weighted average of the probability of this vector $$o_t$$ given $$L_j(t)$$: 

$$
	\hat{\mu}_{jg} = \frac{ \sum^T_{t=1} {L_j(t)} \,\times\, p(o_t)  } { \sum^T_{t=1} {L_j(t)} }
	\qquad\qquad \text{(E23)}
$$



Given this re-estimated mean, we can find the covariances as the following weighted average: 

$$
	\hat{\Sigma}_{jg} = \frac{ \sum^T_{t=1} {L_j(t)} \,\times\, ( o_t - \hat{\mu}_{jg} )^2 } { \sum^T_{t=1} {L_j(t)} }
	\qquad\qquad \text{(E24)}
$$


For long sequences of vectors, repeatedly multiplying probabilities as in E17 and E20 leads to very small probabilities, and thus a risk of underflow: the probabilities are thus logarithms, and E22 (and E26 below) are calculated in log arithmetic.



<br><br><br>

<hr /><br>
