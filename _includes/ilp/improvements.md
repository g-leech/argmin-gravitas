As discussed in Section \ref{limitationsILP}, there are shortcomings in common to all extant ILP systems. We list aspects of ILP that could mitigate these:
\begin{enumerate}
    \item Shrinking the target theory
%         e.g. higher-order programs (push into expressive language)<br>
%         e.g. curry metarules + abstractions require an extra clause in $h$ <br>
%         e.g. Task decomposition (learn several small programs) % Nandi: Nice, I hadn't verbally realized that task decomposition falls under this heading
    \item Reducing the background size
%     e.g. with a relevance prediction module / throwing away unnecessary info <br>
%     e.g. with dimensionality reduction<br>
%     e.g. with active learning queries   % this is non-obvious to me
    \item Data compression: Unification of examples or literals
% e.g. Semantic duplicates: Exploiting background knowledge
    \item New search algorithms
%     e.g. Linear programming for proof search<br>
%     e.g. GANs for hypothesis generation
    \item Handling noise and ambiguity (Response noise or attribute noise)
%     e.g. with DL preprocessing.<br>
%     e.g. with First-Order Bayesian Networks \citep{fobn}
\end{enumerate}
% Nandi: I think the e.g.'s are valuable to mention
% In particular if they happen to have a nice taxonomy, but even if they don't, then still I think it is fitting to give a few examples.
% I think we do need to make sure that we put equal or more emphasis on the aim of the improvement than on the method of improvement (examples)

The first three approaches all relate to the fact that ILP takes longer to run if it has more data to parse.
% The above sentence may not be exactly correct
The order of finding a hypothesis $h$ with background $B$ is in $O\left(\, (|h|\cdot|B|)^{|h|} \,\right)$. Thus if we shrink the target theory $|h|$ or reduce the background $|B|$, we drastically speed up the algorithm.
In general, data compression leads to ... <br>

Reducing the target theory is an active line of research [cite cropper and more ?]. 
In \citep{cropper}, the target theory is reduced through ...
Other ways in which it could be reduced include ... <br>

Examples of ways in which the background could be reduced. <br>

Just as an exponential increase in computation power enabled the boom in large neural networks, so too might inductive proof search benefit from similar algorithms to exploit this increase. Another route is to import general improvements from other areas of ML, for instance: parallelism in computation \citep{srini} and modelling \citep{microsoft-zero}, population-based training \citep{popul}, ensembles of weak learners \citep{jiang}, curriculum learning \citep{conn}, % TODO

