---
layout: 	post
title: 		"Robots, Games, Evolution"
baselink:	/games-of-life
permalink:	/games-of-life
date:		2020-04-14
author:		Gavin   
img:		/img/fitness.jpg

visible:	1
published:	true

summary: 	Understanding game theory, reinforcement learning, and evolutionary dynamics with each other.
confidence:	80%. Couple figurative bits.
importance:	6
wordcount:	200
---

{%	assign jh = "https://www.journals.uchicago.edu/doi/abs/10.1086/257416?journalCode=jpe"	%}
{%	assign gt ="http://www.cs.cmu.edu/~mmv/papers/00TR-mike.pdf" 	%}
{%	assign egt ="https://www.sciencedirect.com/science/article/pii/S002205319792319X" 	%}


<style type="text/css">
	.tg {
		border-collapse:collapse;
		border-spacing: 50px 0;
	}
	td, th {
  		padding: 10px;
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
    <th>Reinforcement learning</th>
    <th>Game theory</th>
    <th>Evolutionary dynamics</th>
  </tr>
<!--  -->
	<tr>
		<td>agent</td>
		<td>player</td>
		<td>population</td>
	</tr>
	<tr>
		<td>action </td>
		<td>move</td>
		<td>subspecies</td>
	</tr>
	<tr>
		<td>policy </td>
		<td>strategy</td>
		<td>subspecies distribution</td>
	</tr>
	<tr>
		<td>Total reward </td>
		<td>payoff</td>
		<td>fitness</td>
	</tr>
	<tr>
		<td>multi-agent Markov<br> decision process</td>
		<td>game</td>
		<td>game (Competition)</td>
	</tr>
	<tr>
		<td>environment</td>
		<td>nonoptimising, noncompetitive <br>second player</td>
		<td>niche</td>
	</tr>
	<tr>
		<td>environment dynamics</td>
		<td>move by nature</td>
		<td>move by Nature</td>
	</tr>
	<tr>
		<td>MDP</td>
		<td>State-based infinite game -<br>
		often single-player, <br>stochastic, 
		discrete action, <br>imperfect information</td>
		<td>ecology</td>
	</tr>
	<tr>
		<td>episode</td>
		<td>iteration</td>
		<td>generation</td>
	</tr>
	<tr>
		<td>multi-agent multi-armed bandit</td>
		<td>Matrix game</td>
		<td>Matrix game</td>
	</tr>
	<tr>
		<td>Bellman optimality</td>
		<td>equilibria</td>
		<td>stable strategies /<br>Liapunov stable states</td>
	</tr>
	<tr>
		<td>optimal substructure</td>
		<td>subgame perfect equilibrium</td>
		<td>subgame perfect equilibrium</td>
	</tr>
	<tr>
		<td>Dynamic Bellman learning</td>
		<td>No learning</td>
		<td>Replicator dynamics as learning</td>
	</tr>
	<tr>
		<td>agent focussed <br>(process; planning;<br> computational learning)</td>
		<td>game focussed <br>(equilibria; perfect rationality)</td>
		<td>dynamics focussed <br>(process; replication;<br> change in population mix)</td>
	</tr>
	<tr>
		<td>Engineering</td>
		<td>Normative</td>
		<td>Descriptive</td>
	</tr>

</table>
</center>
<br><br>

(For making the correspondence really nice, you could frame evolution from the perspective of a single actor like the others - a hypothetical organism behind a <a href="{{jh}}">veil of ignorance</a>, maximising their expected fitness by selecting which subspecies to join. The subspecies distribution is then their chance of switching to a given subspecies.)

<br><br>

## See also


* <a href="/conversion">Mapping metaphysics, mathematics, and programming</a>
* _<a href="{{gt}}">An Analysis of Stochastic Game Theory for Multiagent
Reinforcement Learning</a>_ (2000)
* _<a href="{{egt}}">Learning Through Reinforcement and Replicator Dynamics</a>_ (1997)

<br><br>

{%	include comments.html	%}

<br><br>