<br>
Topology: The first step in modelling an HMM is to define the topology and input type. The optimal
number of states is not known and cannot be readily learned from the data; this then is one of our
design variables.


<center>
<img src="/img/accommodation/16.png" width="85%" height="85%" />

<br><br>	<i>Fig.x</i> -  State diagrams illustrating two HMMs N=3, one with, and one without state ‘skips’.

<br><br>
</center>


(where $$a_{23}$$ denotes the transition probability 
$$p(s_3 \,|\, s_2)$$.)

We take the dimensionality of the input vectors D and set further hyperparameters: the number of
Gaussians, $$G = 10$$, the alphabet size $$M = 1$$. We constrain the transitions to be forward only (by E5)
and sequential (by E6). 

With these set, we can define a prototype HMM for each speaker; an example of one such prototype
definition file can be seen in Fig.4, listing the dimensionality "\<VecSize\>" and the vector type used for the model,
a (presently) arbitary mean and covariance vector for each state, and a transition matrix for the whole model (in which each floating point number at row $$i$$ and column $$j$$ is ) . 

In total we define and evaluate 8 sets of 12 speaker models: one for $$N=1.. 4$$ and using MFCC and LPC features.<br><br>



<center>
<img src="/img/accommodation/16actually.png" width="100%" height="100%" />

<br><br>	<i>Fig.16</i> - The N=2 HMM file from Fig.4, after re-estimation.

<br><br>
</center>
(Note that this file requires us to list $$N+2$$ states, in order to represent the terminal non-emitting states used by HTK.)
<br><br>


Initialisation: In order to obtain optimal parameters from the Baum-Welch algorithm, we first
require good initial estimates for the values of the mean and covariance vectors of each state, in
order that the estimates may convergence more precisely to the local optima. We obtain this simply
by assigning the global mean and covariance of the training data to the emission distributions and
setting the transition probabilities to be equal, a ‘flat-start model’. This is performed with the HTK
tool ‘HCompV’ (see script 6).

Training : We perform 10  simultaneous training iterations (‘embedded re-estimation’) with the HTK tool ‘HERest’. This uses the Baum-Welch procedure of section 3.2.2, modified for parallel training as follows:
<br>
<pre><code>
1. Allocate an accumulator variable for each parameter of each HMM.
2. For each training utterance:
	2.1. Construct a composite HMM by joining up the HMMs in sequence.
	2.2. For this composite HMM, calculate forward probabilities for all 
	    states j & times t.
	2.3. Calculate the backward probabilities for all states j & times t.
	2.4. Use 2.2 & 2.3 to compute the probabilities of state occupation 
	    at each frame
	2.5. Combining these with the current utterance, by E23.
	2.6. Update the accumulators of each parameter by weighting the 
	    existing values by the new utterance, E24 and E25.
3. Use the final value in each accumulator as parameter estimates for each HMM.<br>
</code></pre>

<center>
<i>Fig.17</i> - Algorithm for embedded Baum-Welch re-estimation
<br><br>
</center>


<h4>Parsing a task grammar into a word lattice</h4>

The default mode of HTK is continuous speech recognition; we are instead using the software in a
special case of ‘isolated word recognition’ (where no specific task grammar constrains sequences of recognitions, as there is in sentence-level ASR). 

Even so, the parameters of the relevant HTK shell ‘HVite’ require a formal syntax (an acceptable sequence of symbols) and a ‘dictionary’ (where the ‘words’ to recognise are defined in terms of models). Our syntax is generated (by ‘HParse’) from the simplest possible Backus-Naur grammar: just one variable in a loop, the speaker code. The dictionary is simply a mapping from the speaker code to the name of the model representing that speaker, where we include one instance of this mapping per state in the HMM. The grammar, though non-functional in our case, is anyway expanded into a graph of possible sequences, a ‘word lattice’ (a set of nodes which represent time steps, and a set of transitions which represent symbol hypotheses).


<center>
<img src="/img/accommodation/18.png" width="60%" height="60%" />

<br><br>	<i>Fig.18</i> - Parsing the Backus-Naur grammar into a word network for the HVite recognizer.

<br><br>
</center>