<div class="accordion">

<h3>Classifying ILP Systems</h3>
<div>
	There are <a href="{{sys}}">dozens</a> of ILP systems. We classify ILP systems using the following dimensions:

<br><br>
<ol>
	<li><i>Order of hypotheses</i>: does it allow first-order logic or higher-order logics in the  representation of inputs, intermediates, and outputs?</li><br>
	<li><i>Target language</i>: Almost all ILP systems induce Prolog programs; however recent systems use other target languages, for instance Datalog (less expressive than Prolog) or answer-set programming\citep{ilasp} (more expressive).</li><br>
	<li><i>Search strategy</i>: whether the search is conducted top-down (that is, from general to specific) or bottom-up (starting with an example and generalising it), or whether both are used (as in 'theory revision'). </li><br>
	<li><i>Exact or probabilistic search</i>: does the search include stochastic steps?</li><br>
	<li><i>Noise handling</i>: how are mislabeled examples or other corrupt data-points handled? An implementation detail, this can involve restricting specialisation in top-down search; allowing some negatives to be covered by a clause; or by using a neural network to preprocess data. % metagol is a kind of bootstrap scoring</li><br>
	<li><i>Inference engine</i>: Almost all ILP systems are meta-interpreters running on Prolog. More recent systems attempt to replace symbolic inference with latent embeddings in a structured neural network or some other differentiable structure, for instance the differentiable deduction (\(\partial D\)) of \(\partial\mathrm{ILP}\).</li><br>
	<li><i>Predicate invention</i>: can the system induce new background assumptions during learning?</li><br>
</ol>


ILP systems differ along other dimensions, but the above are informative.<br><br>

<center>
<table style="border-collapse: collapse; border: none; border-spacing: 0px;">	
	<tr>
		<td style="border-top: 1px solid black; border-bottom: 1px solid black; padding-right: 3pt; padding-left: 3pt;">
			<b>System</b>
		</td>
		<td style="border-top: 1px solid black; border-bottom: 1px solid black; padding-right: 3pt; padding-left: 3pt;">
			<b>Order</b>
		</td>
		<td style="border-top: 1px solid black; border-bottom: 1px solid black; padding-right: 3pt; padding-left: 3pt;">
			<b>Target</b>
		</td>
		<td style="border-top: 1px solid black; border-bottom: 1px solid black; padding-right: 3pt; padding-left: 3pt;">
			<b>Search</b>
		</td>
		<td style="border-top: 1px solid black; border-bottom: 1px solid black; padding-right: 3pt; padding-left: 3pt;">
			<b>Exact</b>
		</td>
		<td style="border-top: 1px solid black; border-bottom: 1px solid black; padding-right: 3pt; padding-left: 3pt;">
			<b>Noise</b>
		</td>
		<td style="border-top: 1px solid black; border-bottom: 1px solid black; padding-right: 3pt; padding-left: 3pt;">
			<b>Engine</b>
		</td>
		<td style="border-top: 1px solid black; border-bottom: 1px solid black; padding-right: 3pt; padding-left: 3pt;">
			<b>Invent</b>
		</td>
	</tr>
	<tr>
		<td style="border-bottom: 1px solid black; padding-right: 3pt; padding-left: 3pt;">
			FOIL
		</td>
		<td style="border-bottom: 1px solid black; padding-right: 3pt; padding-left: 3pt;">
			First
		</td>
		<td style="border-bottom: 1px solid black; padding-right: 3pt; padding-left: 3pt;">
			Prolog
		</td>
		<td style="border-bottom: 1px solid black; padding-right: 3pt; padding-left: 3pt;">
			Top-down
		</td>
		<td style="border-bottom: 1px solid black; padding-right: 3pt; padding-left: 3pt;">
			Yes
		</td>
		<td style="border-bottom: 1px solid black; padding-right: 3pt; padding-left: 3pt;">
			None
		</td>
		<td style="border-bottom: 1px solid black; padding-right: 3pt; padding-left: 3pt;">
			Prolog
		</td>
		<td style="border-bottom: 1px solid black; padding-right: 3pt; padding-left: 3pt;">
			No
		</td>
	</tr>
	<tr>
		<td style="border-bottom: 1px solid black; padding-right: 3pt; padding-left: 3pt;">
			Progol
		</td>
		<td style="border-bottom: 1px solid black; padding-right: 3pt; padding-left: 3pt;">
			First
		</td>
		<td style="border-bottom: 1px solid black; padding-right: 3pt; padding-left: 3pt;">
			Prolog
		</td>
		<td style="border-bottom: 1px solid black; padding-right: 3pt; padding-left: 3pt;">
			Top-down
		</td>
		<td style="border-bottom: 1px solid black; padding-right: 3pt; padding-left: 3pt;">
			Yes
		</td>
		<td style="border-bottom: 1px solid black; padding-right: 3pt; padding-left: 3pt;">
			Bounded
		</td>
		<td style="border-bottom: 1px solid black; padding-right: 3pt; padding-left: 3pt;">
			Prolog
		</td>
		<td style="border-bottom: 1px solid black; padding-right: 3pt; padding-left: 3pt;">
			No
		</td>
	</tr>
	<tr>
		<td style="border-bottom: 1px solid black; padding-right: 3pt; padding-left: 3pt;">
			Aleph
		</td>
		<td style="border-bottom: 1px solid black; padding-right: 3pt; padding-left: 3pt;">
			First
		</td>
		<td style="border-bottom: 1px solid black; padding-right: 3pt; padding-left: 3pt;">
			Prolog
		</td>
		<td style="border-bottom: 1px solid black; padding-right: 3pt; padding-left: 3pt;">
			Bottom-up
		</td>
		<td style="border-bottom: 1px solid black; padding-right: 3pt; padding-left: 3pt;">
			Both
		</td>
		<td style="border-bottom: 1px solid black; padding-right: 3pt; padding-left: 3pt;">
			Bounded
		</td>
		<td style="border-bottom: 1px solid black; padding-right: 3pt; padding-left: 3pt;">
			Prolog
		</td>
		<td style="border-bottom: 1px solid black; padding-right: 3pt; padding-left: 3pt;">
			No
		</td>
	</tr>
	<tr>
		<td style="border-bottom: 1px solid black; padding-right: 3pt; padding-left: 3pt;">
			Metagol
		</td>
		<td style="border-bottom: 1px solid black; padding-right: 3pt; padding-left: 3pt;">
			Higher
		</td>
		<td style="border-bottom: 1px solid black; padding-right: 3pt; padding-left: 3pt;">
			Datalog/ASP
		</td>
		<td style="border-bottom: 1px solid black; padding-right: 3pt; padding-left: 3pt;">
			Top-down
		</td>
		<td style="border-bottom: 1px solid black; padding-right: 3pt; padding-left: 3pt;">
			Yes
		</td>
		<td style="border-bottom: 1px solid black; padding-right: 3pt; padding-left: 3pt;">
			Scoring
		</td>
		<td style="border-bottom: 1px solid black; padding-right: 3pt; padding-left: 3pt;">
			Prolog
		</td>
		<td style="border-bottom: 1px solid black; padding-right: 3pt; padding-left: 3pt;">
			Yes
		</td>
	</tr>
	<tr>
		<td style="border-bottom: 1px solid black; padding-right: 3pt; padding-left: 3pt;">
			\(\partial\)ILP
		</td>
		<td style="border-bottom: 1px solid black; padding-right: 3pt; padding-left: 3pt;">
			First
		</td>
		<td style="border-bottom: 1px solid black; padding-right: 3pt; padding-left: 3pt;">
			Datalog
		</td>
		<td style="border-bottom: 1px solid black; padding-right: 3pt; padding-left: 3pt;">
			Top-down
		</td>
		<td style="border-bottom: 1px solid black; padding-right: 3pt; padding-left: 3pt;">
			No
		</td>
		<td style="border-bottom: 1px solid black; padding-right: 3pt; padding-left: 3pt;">
			ANN
		</td>
		<td style="border-bottom: 1px solid black; padding-right: 3pt; padding-left: 3pt;">
			$$\partial D$$
		</td>
		<td style="border-bottom: 1px solid black; padding-right: 3pt; padding-left: 3pt;">
			Yes
		</td>
	</tr>
	<tr>
		<td style="padding-right: 3pt; padding-left: 3pt;">
		</td>
		<td style="padding-right: 3pt; padding-left: 3pt;">
		</td>
		<td style="padding-right: 3pt; padding-left: 3pt;">
		</td>
		<td style="padding-right: 3pt; padding-left: 3pt;">
		</td>
		<td style="padding-right: 3pt; padding-left: 3pt;">
		</td>
		<td style="padding-right: 3pt; padding-left: 3pt;">
		</td>
		<td style="padding-right: 3pt; padding-left: 3pt;">
		</td>
		<td style="padding-right: 3pt; padding-left: 3pt;">
		</td>
	</tr>
</table>
</center>

</div>


</div>

<br><br>