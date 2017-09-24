
Feature extraction is the conversion of the continuous speech waveform into an efficient, expressive
lower-dimension representation, for the purposes of increasing the tractability of the data, and
perhaps to narrow the analysis to certain spectral aspects of speech. <a href="#fn:49" id="fnref:49">49</a>


Some definitions:

<ul>
<li><span class="heav">Frame</span>: A fixed-length portion of a speech signal, from which to extract a feature.</li><br>
<li><span class="heav">Windowing</span>: The subdivision of a signal into successive frames. (Perhaps overlapping frames, as here.) The resulting windows are multiplied by a ‘window function’ that eliminates the signal lying outwith the present interval.</li><br>

<li><span class="heav">Extraction rate</span>: how often the processor takes a window from the signal. Here, 10ms.</li><br>
<li><span class="heav">Window size</span>: the length of each window. Here, it is 25ms long. So we have overlapping windows.</li><br>
</ul>


The vector size must strike a balance between being small enough to permit easy
computation (and tuned for relevance, to prevent the capture of spurious variations between tokens
of the same linguistic unit which might impede recognition performance); but still large enough to capture salient features of the signal. In our case a sequence of 12-dimensional real-value vectors is
output by the models at a rate of one observation per 10ms.
The format chosen must also match the distribution assumptions of the models; for example, since
our models use Gaussians with diagonal covariance, the features used have to follow the Gaussian
distribution and get decorrelated as part of the extraction.


<center>
<img src="/img/accommodation/12.png" width="60%" height="60%" />

<br><br>	<i>Fig.12</i> - The power spectral density (or simply ‘spectrum’) of one frame of speech.

<br><br>
</center><br>


The HTK tool ‘HCopy’ performs each feature extraction. This results in a single file ( .mfc, .lpc ) containing all the frames of the source
utterance. In our case we also need to store information about the frame number and speaker for
each vector; we use separate label files for this purpose, one label per feature file, created with the
vector reading tool ‘HList’ (see script 5).

The present work uses two feature extraction techniques to approximate different properties of
speech spectra: Mel-frequency Cepstral Coefficient analysis (MFCC) and Linear Predictive Coding
(LPC). 
<br><br>

<h4>Mel-Frequency Cepstral Coefficients (MFCCs)</h4>

<center>
<img src="/img/accommodation/13.png" width="60%" height="60%" />

<br><br>	<i>Fig.13</i> - The cepstrum of the spectrum frame from Fig.12.

<br><br>
</center>


MFCCs are a standard feature encoding scheme in speech recognition and speaker-verification. For a Fourier transform of a signal $$ F(o_t^{(A,i)})$$, its power cepstrum $$C(z)$$ is:

$$
	C(z) = F\, (\,\log \, | \log F(o_t^{(A,i)})|^2\,)^2
$$


The result vectors themselves are a series of ‘cepstral’ coefficients read off this cepstrum, and are obtained by the following algorithm:

<br><br>
<pre><code>
1. For each frame:
   1.1. Take the Fourier transform of each overlapping window.
   1.2. Summarise this resulting spectrum into fewer energy values, 
	one for each frequency range required (Mel-scale filter bank).
   1.3. Square and take the log of the filterbank energies (F(o_t^{(A,i)}))
   1.4. Decorrelate the result using a discrete cosine transform 
        (i.e. we are plotting the spectrum of the transformed log-spectrum).
        This is the cepstrum.
   1.5. Read off the required number of amplitudes (D) of the cepstrum, 
        ignoring the first and counting the next D most significant values. 
</code></pre><a href="#fn:52" id="fnref:52">52</a>

We thereby trim off less-expressive components of the signal, in passing. This yields a vector of
values representing the energy in each frequency band of the current frame.

<center>
<img src="/img/accommodation/14.png" width="100%" height="100%" />

<br><br>	<i>Fig.14</i> - Plots of the transformations of the MFCC algorithm.

<br><br>
</center>


In concrete terms, MFCCs account for 

> vocal tract properties and phonemes more than for properties that a speaker can control [in order] to imitate, consciously or not, others (e.g., prosody). <a href="#fn:53" id="fnref:53">53</a> 

The mel-frequency scale was developed in order to capture the information processing of the human ear.<a href="#fn:54" id="fnref:54">54</a> An example of MFCCs being used for holistic measurement of a high-level linguistic phenomenon is Huckvale’s ACCDIST measure for accents.<a href="#fn:55" id="fnref:55">55</a>; using MFCCs as features resulted in accent recognition rates of “up to 92%” for his system.<a href="#fn:56" id="fnref:56">56</a>

<br><br>


<h4>Linear predictive coding (LPC)</h4>

LPCs are one of the earliest digital speech encoding schemes, known for lossy compression. However, relative to its complexity, linear predictive coding is actually a remarkably effective encoding, and capture by compressing away. Rather than storing the whole chain of a signal’s formant and pitch information, we can numerically model signals by estimating the magnitude of the signal at time $$t$$ as a linear combination of the previous $$p$$ samples:<br><br>

$$
	x(t) = a_1 \times x(t-1) \,-\, a_2 \times x(t-2) \,-\, ... \,-\, a_p \times x(t-p)
$$

<br>where $$p$$ is the ‘order’ of the LPC predictor, typically set between 12 and 18. The whole resulting feature vector is thus just a sequence of linear coefficients, plus a set of errors (the "prediction residual"). 

The algorithm to extract an LPC vector is incredibly simple: fit a line across each p samples by minimising the size of the prediction residual (for instance by ordinary least squares); then read off the required number of coefficients and residuals.<a href="#fn:58" id="fnref:58">58</a>

To re-synthesise the original speech waveform (or to generate an emission vector during a HMM evaluation) is a simple matter of chaining the stored values of
, which results in an adequate signal, as can be seen approximately in Fig.15:<br><br>


<center>
<img src="/img/accommodation/15.png" width="100%" height="100%" />

<br><br>	<i>Fig.15</i> - (i) shows a speech waveform before LPC,<br> (ii) the waveform after re-synthesis from its LPC vector.<a href="#fn:59" id="fnref:59">59</a>

<br><br>
</center>



The p coefficients collectively represent the changing resonances (“formants”) in the vocal tract. The
spikes in the prediction residual correspond to the glottal pulses of speech (the part of speech called
"voiced speech" including the sibilant and plosive consonants). These occur at a regular rate, (the
‘fundamental frequency’ of speech) and as such can be incorporated to the final LPC by pitch
prediction techniques, without losing much of the format’s compression. <a href="#fn:60" id="fnref:60">60</a>

The advantages of LPCs are the simplicity and efficiency of the extraction process (least squares) and the extremely lightweight storage (far less than 1KB per utterance); this makes them most suitable
for speech processing applications involving real-time transmission (e.g. telephone speaker verification) or lookups to large vocabularies. In our case they serve as a useful contrast with MFCCs, capturing dynamic features like pitch and rhythm rather than the shape of the vocal tract.
