---
layout: 	post
title: 		"Robots, Games, Life, Markets"
baselink:	/games-of-life
permalink:	/games-of-life
date:		2020-04-14
author:		Gavin   
img:		/img/fitness.jpg

visible:	1
published:	true
best: 		1

summary: 	Understanding game theory, reinforcement learning, evolutionary dynamics, & economic calculation, with each other.
confidence:	80%. Couple figurative bits.
importance:	7
wordcount:	1000
---

{%	assign jh = "https://www.journals.uchicago.edu/doi/abs/10.1086/257416?journalCode=jpe"	%}
{%	assign gt = "http://www.cs.cmu.edu/~mmv/papers/00TR-mike.pdf" 	%}
{%	assign egt = "https://www.sciencedirect.com/science/article/pii/S002205319792319X" 	%}
{%	assign crm = "https://arxiv.org/abs/1811.00164"	%}
{%	assign kael = "https://dl.acm.org/doi/10.5555/2074158.2074203"	%}
{%	assign lipton = "https://dl.acm.org/doi/10.1145/779928.779933"	%}
{%	assign ppf = "https://en.wikipedia.org/wiki/Production%E2%80%93possibility_frontier"		%}
{%	assign cosma = "http://webcache.googleusercontent.com/search?q=cache:GsO3yWjNuHwJ:crookedtimber.org/2012/05/30/in-soviet-union-optimization-problem-solves-you/&hl=en&gl=de&strip=1&vwsrc=0"	%}
{%	assign thicc = "https://en.wikipedia.org/wiki/Thick_concept"		%}


<style type="text/css">
	.tg {
		border-collapse:collapse;
		border-spacing: 50px 0;
	}
	td, th {
  		padding: 6px;
  		padding-bottom: 24px;
	}
	th {
		border-bottom: 1px solid; 

		text-align: left;
	}
	table th + th, td + td { 
		border-left: 1px solid; 
	}


</style>
<center>
<table class="tg">
  <tr>
    <th>Reinforcement<br>learning</th>
    <th>Game<br>theory</th>
    <th>Evolutionary dynamics</th>
    <th>Market<br>calculation</th>
  </tr>
<!--  -->
	<tr>
		<td>agent</td>
		<td>player</td>
		<td>population</td>
		<td>actor</td>
	</tr>
	<tr>
		<td>action </td>
		<td>move</td>
		<td>subspecies</td>
		<td><a href="{{ppf}}">PPF/CPF bundle</a></td>
	</tr>
	<tr>
		<td>policy </td>
		<td>strategy</td>
		<td>subspecies distribution</td>
		<td>product lines</td>
	</tr>
	<tr>
		<td>Total reward </td>
		<td>payoff</td>
		<td>fitness</td>
		<td> profit </td>
	</tr>
	<tr>
		<td>multi-agent Markov<br> decision process</td>
		<td>game</td>
		<td>game (Competition)</td>
		<td>market</td>
	</tr>
	<tr>
		<td>environment</td>
		<td>noncompetitive <br>second player</td>
		<td>niche</td>
		<td>niche  </td>
	</tr>
	<tr>
		<td>environment dynamics</td>
		<td>move by nature</td>
		<td>move by Nature</td>
		<td>exogenous shocks  </td>
	</tr>
	<tr>
		<td>MDP</td>
		<td>
			State-based infinite game 
			<a href="#fn:2" id="fnref:2">2</a>
		</td>
		<td>ecology</td>
		<td>industry  </td>
	</tr>
	<tr>
		<td>episode</td>
		<td>iteration</td>
		<td>generation</td>
		<td>timeless?<br> (for complete markets)  </td>
	</tr>
	<tr>
		<td>multi-agent multi-armed bandit</td>
		<td>Matrix game</td>
		<td>Matrix game</td>
		<td>exchange</td>
	</tr>
	<tr>
		<td>Bellman optimality</td>
		<td>equilibria</td>
		<td>stable strategies /<br>Liapunov stable states</td>
		<td>general equilibrium  </td>
	</tr>
	<tr>
		<td>optimal substructure</td>
		<td>subgame perfect <br>equilibrium</td>
		<td>subgame perfect <br>equilibrium</td>
		<td>partial equilibrium  </td>
	</tr>
	<tr>
		<td>known dynamics & rewards</td>
		<td>common knowledge</td>
		<td>given fitness function</td>
		<td>perfect information  </td>
	</tr>
	<tr> <!-- N =|S|  and M = |A| -->
		<td>
			MDP: P-complete
		</td>
		<!--  -->
		<td>Nash eq: PPAD-complete</td>
		<!--  -->
		<td>ESS: Σ^𝑃_2 complete (NP^SAT)  </td>
		<!--  -->
		<td>Arrow-Debreu: PPAD  </td>
	</tr>
	<tr>
		<td>
			<a href="{{kael}}">Value iteration</a>: O(|A| |S|^2) per iteration
		</td>
		<td>
			<a href="{{lipton}}">Approx</a>: at most <br>O(n^{log n/e^2})  
		</td>
		<td>?</td>
		<td><a href="{{cosma}}">O(n^2 log(1/h)</a> for lateral exchange  </td>
	</tr>
	<tr>
		<td>Dynamic Bellman learning</td>
		<td>No learning 
			<a href="#fn:1" id="fnref:1">1</a>
		</td>
		<td>Replicator dynamics as learning</td>
		<td>Lateral exchange pricing </td>
	</tr>
	<tr>
		<td>agent focussed <br>(process; planning;<br> computational learning)</td>
		<td>game focussed <br>(equilibria; perfect rationality)</td>
		<td>dynamics focussed <br>(process; replication;<br> change in mix)</td>
		<td>game focussed <br>(equilibria; perfect rationality)</td>
	</tr>
	<tr>
		<td>Engineering</td>
		<td>Normative</td>
		<td>Descriptive</td>
		<td><a href="{{thicc}}">Thick</a></td>
	</tr>

</table>
</center>
<br><br>

(For making the correspondence really nice, you could frame evolution from the perspective of a single actor like the others - a hypothetical organism behind a <a href="{{jh}}">veil of ignorance</a>, maximising their expected fitness by selecting which subspecies to join. The subspecies distribution is then their chance of switching to a given subspecies.)
<br>

What to call the topic in common? 'Distributed optimisation'?

<br><br>


## See also


* <a href="/conversion">Mapping metaphysics, mathematics, and programming</a>
* _<a href="{{gt}}">An Analysis of Stochastic Game Theory for Multiagent
Reinforcement Learning</a>_ (2000)
* _<a href="{{egt}}">Learning Through Reinforcement and Replicator Dynamics</a>_ (1997)
* _<a href="{{cosma}}">In Soviet Russia, Optimisation Problem Solves You</a>_ (2012)

<br><br>

{%	include comments.html	%}


<div class="footnotes">

<ol>
    <!-- 1 -->
    <li class="footnote" id="fn:1">
    	Though there are new forms which do learn, including important relaxations like <a href="{{crm}}">Counterfactual Regret Minimization</a>. Thanks to Misha Yagudin for this point.
    </li>
<!--  -->
	<li class="footnote" id="fn:2">
    	often single-player, stochastic, discrete action, imperfect information
	</li>
</ol>

</div>


<br><br>