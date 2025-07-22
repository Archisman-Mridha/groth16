use {
  crate::qap::QAP,
  ark_ec::pairing::Pairing,
  ark_ff::{PrimeField, UniformRand},
  ark_poly::EvaluationDomain,
  ark_relations::r1cs::{
    ConstraintSynthesizer, ConstraintSystem, ConstraintSystemRef, OptimizationGoal, SynthesisMode,
  },
  ark_std::rand::Rng,
};

pub struct KZGPolynomialCommitmentScheme<P: Pairing> {
  _phantomData: P,
}

struct ToxicWaste<P: Pairing> {
  // The prover commits polynomials A(X), B(X), C(X) at X=τ.
  pub tau: P::ScalarField,

  /*
    The verifier can learn some information about the QAP, by doing :
    e(A(τ), B(τ)) = e(A(τ).B(τ), 1).

    We'll use α and β to shift the polynomials A(τ), B(τ) and C(τ), so the verifier can learn
    nothing about them :

      [A] = α + A(τ) + rδ
      [B] = β + B(τ) + sδ

    [A] and [B] are called commitments to A(τ) and B(τ) respectively.
  */
  pub alpha: P::ScalarField,
  pub beta: P::ScalarField,

  // TODO : Understand what is melliability.

  // Used to prevent the prover from manipulating the public inputs.
  pub gamma: P::ScalarField,

  // Used to prevent the prover from manipulating remaining values in the witness vector.
  pub delta: P::ScalarField,
}

pub struct ProvingKey {}

pub struct VerifyingKey {}

impl<P: Pairing> KZGPolynomialCommitmentScheme<P> {
  pub fn performTrustedSetupCeremony<C>(circuit: C, rng: &mut impl Rng) -> ProvingKey
  where
    C: ConstraintSynthesizer<P::ScalarField>,
  {
    // For the Bilinear Pairing friendly Cyclic Groups G1 and G2,
    // fix corresponding generators g1 and g2, for usage.
    let g1 = P::G1::rand(rng);
    let g2 = P::G2::rand(rng);

    // Construct the R1CS, from the given arithmetic circuit.
    let r1cs = generateR1CSFromCircuit(circuit);

    // Generate toxic waste.
    let toxicWaste = Self::generateToxicWaste(&r1cs, rng);

    // Reduce the R1CS to a QAP.
    let qapGenerationOutput = QAP::generateFromR1CS(r1cs, &toxicWaste.tau);

    unimplemented!()
  }

  fn generateToxicWaste(
    r1cs: &ConstraintSystemRef<P::ScalarField>,
    rng: &mut impl Rng,
  ) -> ToxicWaste<P> {
    let alpha = P::ScalarField::rand(rng);
    let beta = P::ScalarField::rand(rng);
    let gamma = P::ScalarField::rand(rng);
    let delta = P::ScalarField::rand(rng);

    // We need to be careful when generating τ.
    // While evaluating H(τ) = (A(τ)*B(τ) - C(τ))/Z(τ), if the vanishing polynomial Z(τ) becomes
    // 0, then the division is not possible.
    // This is why, we'll first exclude the Evaluation Domain D from the scalar field,
    // and then choose τ randomly.
    let tau = QAP::getEvaluationDomain(r1cs).sample_element_outside_domain(rng);

    ToxicWaste {
      alpha,
      beta,
      gamma,
      delta,
      tau,
    }
  }
}

// Constructs R1CS from the given arithmetic circuit.
// The R1CS is then returned.
fn generateR1CSFromCircuit<FF, C>(circuit: C) -> ConstraintSystemRef<FF>
where
  FF: PrimeField,
  C: ConstraintSynthesizer<FF>,
{
  let r1cs = ConstraintSystem::new_ref();

  // Try optimizing the number of constraints.
  r1cs.set_optimization_goal(OptimizationGoal::Constraints);

  // Only generate the matrices A(X), B(X) and C(X).
  r1cs.set_mode(SynthesisMode::Setup);

  circuit.generate_constraints(r1cs.clone()).unwrap();
  r1cs.finalize(); // Perform optimizations following the optimization goal, and finalize the
                   // circuit.

  r1cs
}
