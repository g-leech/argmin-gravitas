---
layout:     post
title:      "Mistakes in data science homework"
baselink:   /homework
permalink:  /homework
date:       2019-04-08
author:     Gavin

img:        /img/
published:  false
visible:    1

summary:    .
categories: 
confidence: 
importance: 
wordcount:      
---


(Takehome exercises)

    Forcing everything into classification, particularly binary classification.
    No robustness checks - e.g. of nondeterm algos which narrowly beat
    Non-random train/test split
    Code in a pdf / images
    No code reuse
    More than twice the error of random guessing
    Not normalising for linear models
    Forking paths - multiple comparisons
    Doing PCA but not reducing the damn dimensions
    Interpreting regression leaf nodes as classes
    No library import statements

    hings I have seen marking machine learning take-homes:

- No test split. 

- Test split but forgot to use it.

- An exquisite description of the mathematical and practical motivation behind PCA, followed by them taking as many components as there were original columns.

- Tree depths of 50 on data with 1000 rows (this is saturated, with one row per perverse leaf, after 10 splits).

- Absolutely everyone uses classifiers on ordinal/interval responses. I seriously want to know why.


- Excellent performance in the Notebook, which disappears once I rerun it (with the same seed). This was probably cheeky .ipynb file hacking. 

- Most people forget to normalise the data for linear reg, making everything else in the analysis nonsense.

- No code reuse ever.

- Blithe recommendation of multiple comparisons (cheating).

- Editing the response data, instead of binning or rescaling.

<!-- %  include mistake/foots.html %} -->
{%  include comments.html %}