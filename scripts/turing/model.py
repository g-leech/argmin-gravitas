"""
Bayesian model for gleech.org/turing -- "Did Turing commit suicide?"

Two hypotheses, conditioning on the fact that he died of cyanide poisoning:
    S = suicide
    A = accident (inhaled vapour off the electrolysis rig, or hand-to-mouth)

Everything is in odds form. Prior odds are set to 1:1 and the reasons why
are in the post: the population base rates cannot give us the prior, because
the population accident rate is generated almost entirely by people with no
cyanide in the house, and Turing is not one of them.

    python3 model.py published    # as currently live
    python3 model.py proposed     # forensic row honest; see the post's conflict blocks
"""

import math
import random
import statistics as st
import sys

random.seed(11)


# ---------------------------------------------------------------- hazards
# Suicide hazard for Turing, per year:
#   base rate for a British man in his early 40s, 1954   = 12 / 100,000
#   x3 for being gay (soft: 1990s US *attempts*, see fn 2)
#   xm for conviction + public disgrace + chemical castration
BASE_S = 12 / 100_000
GAY = 3
C_METHOD = 0.5  # P(cyanide is the method | Turing suicides). He had it to hand.


def suicide_hazard(m):
    return BASE_S * GAY * m


def indifference_accident_hazard(m):
    """What would Turing's accidental-cyanide-death hazard have to be for the
    two hypotheses to tie? This is the number the post asks you to judge."""
    return C_METHOD * suicide_hazard(m)


# ------------------------------------------------------- likelihood ratios
# (name, central, lo, hi)  as LR(S:A).
# lo = end of the range most favourable to accident, hi = most favourable to suicide.
#
# Two variants, because the post currently carries both pending resolution:
#
#   published -- as first posted. The forensic row asserts that swallowed cyanide
#                kills in seconds and inhalation is slower. I made that up.
#   proposed  -- the forensic row set to 1:1 and allowed to range over BOTH signs,
#                because the toxicology does not identify route from speed:
#                ATSDR has oral onset faster than inhalation at equal doses,
#                StatPearls has the reverse, and both routes convulse at lethal
#                doses. Speed tracks dose, not route.
#
# NB the seven non-forensic rows multiply to exactly 1.0 (= 0 decibans), so in
# both variants LR_total is identically the forensic row. That is the finding.

BUNDLES_PUBLISHED = [
    ("forensic",       0.2,  0.10, 0.5),   # slow death, no contortion
    ("stomach",        1.0,  0.70, 1.2),   # 4oz bitter fluid; inhalation does this too
    ("no-prep",        1 / 3, 0.20, 0.5),  # no note, papers a mess, forward commitments
    ("no-disclosure",  0.5,  0.33, 0.8),   # told neither psychiatrist nor confidants
    ("will",           2.0,  1.50, 3.0),   # new will, 11 Feb 1954
    ("1930s scheme",   1.5,  1.20, 2.0),   # apple + electrical wiring, told to Atkins
    ("apple",          1.0,  0.90, 1.2),   # he ate one nightly; never assayed
    ("gypsy",          2.0,  1.50, 4.0),   # the Gypsy Queen, mid-May 1954
]

BUNDLES_PROPOSED = [
    ("forensic",       1.0,  0.20, 3.0),   # NOT IDENTIFIABLE: spans both signs
    ("stomach",        1.0,  0.70, 3.0),   # strong smell constrains presence, not dose (fn 11)
    ("no-prep",        1 / 3, 0.20, 0.5),
    ("no-disclosure",  0.5,  0.33, 0.8),
    ("will",           2.0,  1.50, 3.0),
    ("1930s scheme",   1.5,  1.20, 2.0),
    ("apple",          1.0,  0.90, 1.2),
    ("gypsy",          2.0,  1.50, 4.0),
]

# selected in main() from argv; defaults to what is currently live
BUNDLES = BUNDLES_PUBLISHED

PRIOR_ODDS = 1.0  # agnostic; see post


def posterior(lr, prior=PRIOR_ODDS):
    odds = prior * lr
    return odds / (1 + odds)


def decibans(lr):
    """Turing's unit of weight of evidence: a ban is log10 of a likelihood
    ratio, a deciban a tenth of one. Banburismus, c.1940. Taking logs turns
    the product over bundles into a sum, which is the whole point."""
    return 10 * math.log10(lr)


def deciban_table():
    print("\nThe same table in Turing's units (Banburismus):\n")
    total = 0.0
    for name, c, _, _ in BUNDLES:
        db = decibans(c)
        total += db
        print(f"  {name:<15s} LR {c:>5.2f}   {db:+6.2f} db")
    print(f"  {'':<15s} {'':>8s}   {'-'*6}")
    print(f"  {'TOTAL':<15s} {'':>8s}   {total:+6.2f} db")
    print(f"\n  10^({total/10:.3f}) = {10**(total/10):.3f}  -- recovers the LR.")
    if abs(total) < 0.005:
        print("  The entire case is ZERO decibans. Not a faint signal: no signal.")
        print("  The seven non-forensic rows cancel to 0 db, and the eighth was the one I invented.")
    else:
        print(f"  The entire case is {abs(total):.0f} decibans, {abs(total)/10:.2f} of a ban:")
        print("  about the smallest weight of evidence a human can notice.")


def main():
    global BUNDLES
    variant = (sys.argv[1] if len(sys.argv) > 1 else "published").lower()
    if variant not in ("published", "proposed"):
        sys.exit("usage: model.py [published|proposed]")
    BUNDLES = BUNDLES_PUBLISHED if variant == "published" else BUNDLES_PROPOSED
    print(f"=== variant: {variant.upper()} ===")

    print("\nIndifference: how clumsy would he have to be?\n")
    for m in (3, 10, 30):
        lS = suicide_hazard(m)
        lA = indifference_accident_hazard(m)
        print(f"  m={m:<3d} lambda_S = {lS:.2e} (1 in {round(1/lS):,})"
              f"   ->  lambda_A* = {lA:.2e} (1 in {round(1/lA):,} per year)")

    central = math.prod(c for _, c, _, _ in BUNDLES)
    lo = math.prod(l for _, _, l, _ in BUNDLES)
    hi = math.prod(h for _, _, _, h in BUNDLES)

    if central < 1:
        direction = f"1:{1/central:.1f} against suicide"
    elif central > 1:
        direction = f"{central:.1f}:1 for suicide"
    else:
        direction = "dead level -- no evidence either way"
    print(f"\nCentral LR = {central:.3f}  ({direction})")
    print(f"  -> P(S) = {posterior(central):.1%}")
    print(f"All-corners LR: {lo:.4f} .. {hi:.2f}  ->  P(S) {posterior(lo):.1%} .. {posterior(hi):.1%}")

    deciban_table()

    print("\nOne-at-a-time leverage (swing one bundle, hold the rest central):")
    for name, c, l, h in BUNDLES:
        others = central / c
        print(f"  {name:<15s} x{h/l:<4.1f}  P(S) {posterior(others*l):>5.1%} .. {posterior(others*h):>5.1%}")

    # Monte Carlo: log-uniform inside each range. Log-uniform because these are
    # ratios and I have no more information than the endpoints.
    def draw():
        return math.prod(
            math.exp(random.uniform(math.log(l), math.log(h)))
            for _, _, l, h in BUNDLES
        )

    sims = sorted(posterior(draw()) for _ in range(200_000))
    q = lambda p: sims[int(p * len(sims))]
    print(f"\nMonte Carlo P(S): median {st.median(sims):.1%}"
          f"   80% CI [{q(.10):.1%}, {q(.90):.1%}]"
          f"   95% CI [{q(.025):.1%}, {q(.975):.1%}]")
    print(f"  P(evidence favours suicide at all) = {sum(x > .5 for x in sims)/len(sims):.1%}")
    print(f"  P(reaching 'obvious', >90%)        = {sum(x > .9 for x in sims)/len(sims):.2%}")


if __name__ == "__main__":
    main()
