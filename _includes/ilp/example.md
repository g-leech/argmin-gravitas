ILP problems are an extension of the simple 'concept learning' case, which lacks background knowledge, so let's begin there. Consider the training data $$E$$ representing a number of animal species\footnote[3]{\,a: human, b: platypus, c: sparrow, d: naked mole rat, e: salmon.}:

\begin{table}[!hbtp]
\begin{tabular}{|l|l|l|l|l|l|}
\hline
\textbf{} & \textbf{Gestation} & \textbf{Fur coverage} & \textbf{Mammaries} & \textbf{Beak} & \textbf{Label} <br> \hline
a         & Viviparous         & 0.2                   & Y                  & N             & 1              <br> \hline
b         & Oviparous          & 0.9                   & Y                  & Y             & 1              <br> \hline
c         & Oviparous          & 0                     & N                  & Y             & 0              <br> \hline
d         & Viviparous         & 0                     & Y                  & N             & 1              <br> \hline
e         & Oviparous          & 0                     & N                  & N             & 0              <br> \hline
\end{tabular}
\caption{\label{tab:example} Certain distinctive features of animal species}
\end{table}

Converted into conjunctive form, we obtain for instance
$$a: \,\, (G=V) \land (F=0.2) \land (M=Y) \land (B=N) $$

% Simple solution to make the hypothesis more complex: delete the mammaries column

\textcolor{purple}{and background knowledge}.<br> %TODO Javi: replace naked mole rat by kangoroo, B = {mammal(human), mammal(platypus), mammal(kangoroo)}, h=

We want to learn the definition of the rule 'mammal(X)'. If we tackle this with top-down search (FOIL), then we start with the empty body (that is, the empty antecedent): $$\emptyset \to$$ mammal$(X)$.  We iteratively add clauses $$c$$ to the body when $$c$$ satisfies positive examples and satisfies no negative examples (or, for noise handling, when the ratio of positive\,:\,negative coverage is sufficiently good). $$c$$ can be a predicate from $$B$$ or its negation, or an equality, or inequality, between two bound variables.<br>


One solution to the above is Mammaries$=Y$.<br>  %TODO: too simple, cmon
% TODO: is this the only solution? how would each of the previous algorithms go about finding it?

% This illustrates the power of background knowledge: without supplying (or inventing) the predicates \texttt{pred(X,Y)} and \url{pred2(X):-}, the problem could not be solved. <br> %TODO

But ILP is not limited to tabular 'single-table single-tuple' problems like this. A better demonstration of the method's core capability are the Bongard problems: visual, relational concept learning tasks. <br>
% TODO: same as above: is this the only solution? how would each of the previous algorithms go about finding it?

 \begin{figure}[hbtp!]
 \centering
 \includegraphics[scale=0.45]{bongard_71}
 \caption{Bongard problem \#71 \citep{foundalis}}
 (The left panel are the positive examples, to the right the negatives.)
 \label{fig:bong}
 \end{figure}

To infer the concept involved \textendash\, here, <i>Involves second-order nesting of shapes}", $$\exists x \,\, n(n(x))$$ \textendash\, the learner must account for relations between elements. 

An example of background to supply to an ILP system solving \#71 might be

%TODO
\texttt{nested(X):- }


One early ILP system solved 41\% of the Bongard problems, after handcoding the examples' relations into first-order logic \citep{saito}.<br>