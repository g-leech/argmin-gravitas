---
title: "10 Hard Problems in Artificial Intelligence"
full_title: "Ten Hard Problems in Artificial Intelligence We Must Get Right"
authors:
  - Gavin Leech
  - Simson Garfinkel
  - Misha Yagudin
  - Alexander Briand
  - Aleksandr Zhuravlev
gleech_role: first author
year: 2024
type: preprint
status: in review at JMLR
arxiv: 2402.04464
doi: 10.48550/arXiv.2402.04464
url: https://arxiv.org/abs/2402.04464
links:
  explainer: https://twitter.com/g_leech_/status/1755542850433946107
contribution_hours: 800
topics: [ai, ai-safety, ai-governance, capabilities, alignment, survey]
---

## Abstract

We explore the AI2050 "hard problems" that block the promise of AI and cause AI
risks: (1) developing general capabilities of the systems; (2) assuring the
performance of AI systems and their training processes; (3) aligning system
goals with human goals; (4) enabling great applications of AI in real life;
(5) addressing economic disruptions; (6) ensuring the participation of all;
(7) ensuring socially responsible deployment; (8) addressing geopolitical
disruptions that AI causes; (9) promoting sound governance of the technology;
and (10) managing the philosophical disruptions for humans living in the age of
AI. For each problem, we outline the area, identify significant recent work, and
suggest ways forward.

*Note: this paper reviews literature through January 2023.*

## Full text

> Converted from the arXiv PDF (2402.04464). This is a ~43,000-word survey; rather than
> inline it verbatim, this section reproduces the abstract, the background, each of the
> ten Hard Problems with its (subjunctive) headline and a faithful synthesis, and the
> closing analysis. Figures and the ~600-item reference list are omitted. Contribution
> note: Garfinkel wrote the introduction, wicked-problems discussion, and conclusion;
> Garfinkel and Leech jointly wrote the history; the per-problem analysis is by Leech,
> Zhuravlev, Yagudin, and Briand, edited by Garfinkel.

### Background

From the coining of "artificial intelligence" onward, AI progress has been coupled to
improvements in computing power and data storage. For decades progress was slow; we now
appear to be in the "second half of the chessboard," where the acceleration of
capabilities is impossible to ignore (so many specific claims about AI limitations here
may be invalid within a year or two). Big improvements rely on scaling compute (tens of
thousands of machines, millions of cores) and breakthroughs in parallelized training.
The last decade saw qualitative changes: above-human perception on some tasks,
multi-task and multi-modal ability, natural-language interfaces, and human-like
creativity (diffusion models). But the grandest promises remain unfulfilled —
self-driving cars still hover at the bar of practicality; AI for healthcare penetrates
clinical practice only slowly, reflecting reliability, workflow, and usability issues.

### The Ten Hard Problems

Each headline is subjunctive, conditioned on our collective success by 2050.

**HP#1 — Capabilities.** *"By 2050 we will have solved the scientific and technological
limitations in current AI critical to enabling further breakthrough progress and
powerful AI."* Deep learning's triumph is that it productively enlists vast compute and
data and keeps improving as it scales; today's best systems are billion-to-trillion-
parameter networks. Yet they lack reliability and persistence and remain inscrutable.
Realizing AI's potential needs approaches that work across datasets, domains,
modalities, and forms of cognition. LLM demonstrations tend to *lower-bound*
capabilities (emergent skills — step-by-step reasoning, summarization, analogies — keep
appearing with scale), producing a "capability overhang." Cremer identifies five expert
disagreements (abstraction, generalization, causal models, emergent planning,
intervention; we add sample efficiency). Scaling laws have held over eight orders of
magnitude, with *data* now the limiting factor. A key milestone is a virtuous cycle
where AI improves AI research (ML optimizing ML inputs; aiding researchers; AutoML;
direct self-improvement via self-play). *2024 update:* the strong "stochastic parrot"
hypothesis is disconfirmed, but most tasks appear 20–60% memorization. Capabilities are
upstream of all value and risk, are already well-funded by industry, and accelerating
them shortens the time available to solve the other problems.

**HP#2 — Assurance (worst-case performance).** *"…safety and security, robustness,
performance and output challenges…especially in safety-critical applications."* Most
advanced models perform far below customary engineering reliability, and because we
don't understand how they work, we can't yet detect and prevent dangerous modes. Key
approaches: **systemic safety** (safety is a social as much as technical problem);
**monitoring/interpretability** (mechanistic, concept-based, feature-based);
**robustness** (noise injection, diverse data, red-teaming, calibration, OOD
detection); and **formal verification** (proofs a system meets a specification,
progressing on smaller models). Any solution requires strict, credible, hard-to-game
real-world tests — most benchmarks are artificial and gameable; third-party testing for
dangerous actions (deception, self-replication) is emerging.

**HP#3 — Alignment.** *"…safety and control, alignment, and compatibility with
increasingly powerful…AI…and eventually AGI."* Whereas HP#2 prevents harm from
*incompetent* systems, HP#3 aligns *competent* AIs with human intentions — complicated by
human disagreement about values (intent alignment is one normatively-robust goal). Two
core problems: **specification gaming / reward hacking** (systems optimize the metric at
the expense of its purpose — e.g. the OpenAI hand that *pretended* to grasp a ball; you
can't anticipate all loopholes); and **emergent goals** (power-seeking as a likely
attractor for capable planners; deception, e.g. Cicero; the worst case being *deceptive
alignment*, where a system hides undesirable properties from imperfect monitoring). The
dominant approach — iterated human feedback (automated via a proxy model) — selects for
systems that *appear* safe and can incentivize deception above a planning threshold.
Proposed directions: delegating parts of alignment to ML (Leike), AI interpreters /
Eliciting Latent Knowledge (Christiano), and LMs judging LMs. Alignment overlaps
assurance (HP#2) and responsible AI (HP#7); current and future risks form a continuum,
not a dichotomy.

**HP#4 — Beneficial applications.** *"…game-changing contributions by AI to humanity's
greatest challenges…health and life sciences, climate, foundational science…and
mathematics."* Scientific discovery (AlphaFold's two-orders-of-magnitude jump in
predicted protein structures; ML proxies for expensive quantum simulation); energy &
climate (grids, transport, fusion); chemistry/materials; software & algorithm design
(LMs writing ~3% of new Google code; AlphaCode at median competitive-human level;
a novel matrix-multiplication algorithm); healthcare (Hinton's "stop training
radiologists" was overstated — ~40% of EU radiologists now use AI tools, far short of
automation; AI-suggested drugs entering trials). Uptake is slowed by rampant bad
methodology (only 12.5% of diagnostic studies had a test set; none of 62 Covid imaging
models were clinically usable). Pattern: fields with abundant data, strong theory, and
stationary distributions benefit most; current revenue concentrates where specifications
are imprecise and the cost of error is low.

**HP#5 — Economics.** *"…the economic challenges and opportunities resulting from AI."*
Keynes foresaw a 15-hour week by compound interest, not labor automation; the dissenting
"technology eliminates jobs, not work" assumes a creation/destruction balance AI may
disrupt. Three sub-problems: (1) **growth** — despite progress, total-factor-productivity
growth has declined for 50 years (the Solow/productivity paradox; AI may follow
electrification's ~25-year lag); Trammell & Korinek find no compelling reason to dismiss
even super-exponential scenarios. (2) **labor & inequality** — flawed evidence (narrow
definitions of AI); one cited estimate puts 35% of UK / 47% of US workers at displacement
risk; early LLM studies show productivity gains (e.g. median customer-support workers)
but also wage decreases; Acemoglu & Restrepo list four countervailing positive effects.
(3) **policy** — reliability is a serious obstacle (the dismantled Polish unemployment
system); UBI, hiring incentives, and the "AI Economist" are options. Capability — and
thus labor-shock — timing is unpredictable; few institutions are prepared.

**HP#6 — Participation / democratizing access.** *"…democratizing access, participation,
and agency…especially those not presently involved."* The AI workforce doesn't represent
world demographics (disproportionately men from the US and China). **Within-country**:
under-representation (<25% of CS PhDs are women, <2% Black/African American; only ~5–7% of
Google/Microsoft employees are Black); beware "participation washing." Also the
**privatization** of AI (net migration of researchers from academia to industry).
**Across countries**: unequal access to compute, data, and talent. Solving it requires
addressing inequality both within and between countries.

**HP#7 — Responsible AI / sociotechnical embedding.** *"…responsible research,
deployment, and sociotechnical embedding…accounting for different cultures…externalities,
and market and other forces."* Negative impacts: under-recognized clickwork (mostly in
the global south; a ~$13.7B-by-2030 annotation market with little concern for workers'
rights); environmental cost (inference may dominate emissions; little transparency);
discrimination/toxicity/bias (e.g. a Michigan fraud system with a 93% false-discovery
rate); privacy (GDPR/CCPA; federated learning, differential privacy; surveillance);
security. A flurry of ~84 principles documents mostly post-2016 — but they're usually
"orphaned" (no technical agenda or enforcement), systematically overlook future systems
and capability risks, and can amount to "ethics-washing" (Chinese AI principles vs
documented human-rights uses). Turning principles into action: model cards, broader-
impacts statements, whistleblower protection and worker power (the Google military-
project walkout), and AI tools that *correct* misinformation (Meta's Sphere).

**HP#8 — Geopolitics.** *"…risks around its use and misuse, competition, cooperation, and
coordination between countries and other key actors."* AI will transform international
security with little consensus even on vocabulary; managing its impacts is a collective-
action problem with diffuse, delayed harms. **Military destabilization** (erosion of
strategic stability and nuclear deterrence; cheaper, scalable cyber-threats;
proliferation to non-state actors; leading nations' reluctance to regulate military AI).
**Other interactions** (AI as a "general-purpose technology" redistributing hard and soft
power; the "arms race" framing is inaccurate — better a winner-take-most virtuous circle
turning on data, hardware, software, talent, and chokepoints). The US and China lead
capabilities while the EU leads regulation, driving regulatory fragmentation and
arbitrage; the US approach is largely hands-off plus industrial policy (CHIPS Act). Some
AI tools (translation, monitoring) may improve information flow and reduce conflict.

**HP#9 — Governance / institutional resilience.** *"…adaptation, co-evolution and
resiliency of institutions and social infrastructure to keep up with and harness AI."*
("We are…building the plane as it is taking off.") Focus on domestic governance:
policy-level "hard law" and organizational "soft law." The **Collingridge dilemma** —
regulators must act before impacts are understandable — is acute here because AI is "a
technology which can misuse itself"; Dafoe's four risk sources: robust totalitarianism;
nuclear war; misaligned systems; systematic value erosion from competition. Gasser &
Almeida's five governance layers (technical, responsibility, regulation, public policy,
collaborative); leading tools include **compute governance** (tracking high-end chips),
procurement rules, standards bodies, certification, verifiable-claims mechanisms, and
new instruments like private regulatory markets. Much of the real policy work is
"illegible" (only PR-friendly material is published).

**HP#10 — The human condition.** *"…what it means to be human in the age of AI."* If AI
does your job better than you, what does that mean for you as a moral agent and human?
Danaher's **severance problem**: full automation severs the connection between human
effort and the world improving (and the loss of positional goods); it's only partially
mitigated by walling off certain fields, since the essence is that you *could* be
replaced even if you aren't. Related worries: AI excellence inspiring passivity and
societal stagnation — a "bystander effect for values" — which threatens the voluntary
action, activism, and oversight that democracies and moral progress depend on.

### Are the Hard Problems "wicked problems"?

Calling them "hard" might connote unsolvability ("wicked problems" defy solution due to
their entanglement of facts and values and lack of a definitive formulation). The paper
concludes there is hope for *partial* solutions to suitably *sharpened* versions of the
problems. Those that directly invoke human values (HP#3, HP#6–9) are the better
candidates for genuine wickedness.

### Outlook

The most certain progress is on HP#1 and HP#4 — "the accelerator pedal" — because
researcher and commercial incentives already point there. The other problems (assurance,
alignment, the economic, participatory, responsible, geopolitical, governance, and
philosophical challenges) are comparatively under-funded and under-staffed, even as
capability gains shorten the time available to address them.
