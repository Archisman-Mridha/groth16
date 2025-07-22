use {
  ark_ff::PrimeField,
  ark_poly::{EvaluationDomain, Radix2EvaluationDomain},
  ark_relations::r1cs::{ConstraintSystemRef, SynthesisError},
  std::marker::PhantomData,
};

/// You write down multiple constraints using R1CS (Rank 1 Constraint System).
/// They are then reduced to a single constraint, as a QAP (Quadratic Arithmetic Program).
pub struct QAP<FF: PrimeField> {
  _phantomData: PhantomData<FF>,
}

pub struct QAPGenerationOutput<FF: PrimeField> {
  pub qapVariableCount: usize,

  pub evaluationDomainSize: u64,

  pub AAtTau: Vec<FF>,
  pub BAtTau: Vec<FF>,
  pub CAtTau: Vec<FF>,
  pub ZAtTau: FF,
}

impl<FF: PrimeField> QAP<FF> {
  /// Generates a QAP, from the given R1CS.
  ///
  /// **Parameters** :
  ///
  /// - `FF` : the prime (finite) field, on which, the R1CS's addition and multiplication
  ///   operations are defined.
  ///
  /// - `r1cs` : the Rank1 Constraint System (R1CS).
  ///
  /// - `tau` (part of the toxic waste) : random element chosen from FF, to perform the KZG
  ///   polynomial commitment scheme based trusted setup ceremony.
  pub fn generateFromR1CS(r1cs: ConstraintSystemRef<FF>, tau: &FF) -> QAPGenerationOutput<FF> {
    /*
      The R1CS is reduced to a single constraint, which is initially in matrix form :

        (A*w) * (B*w) = C*w

      Here, each matrix is of dimension : |w| x m, where,
      w = witness vector.
      m = constraint count.
    */
    // Get the matrices.
    let matrices = r1cs.to_matrices().unwrap();

    // The witness vector contains public inputs, followed by the variables created in the
    // intermediate steps of the R1CS.
    let witnessVectorSize = r1cs.num_instance_variables() + r1cs.num_witness_variables();

    // By convention, the first public input is always 1.
    // Excluding that conventional element from the witness vector, leaves us with what we call :
    // QAP variables.
    let qapVariableCount = witnessVectorSize - 1;

    /*
      The matrix form is then converted to polynomial form :

        A(X)*B(X) - C(X) = Z(X)*H(X)

      A(X), B(X) and C(X), each is of degree (m-1).
      Z(X)       and H(X), each is of degree m.

      Suppose, a₀ = [a₀₀,....,a₍ₘ₋₁₎₀] is the first column of the matrix A.
      Using Langrange interpolation, we construct the polynomial A₀(X), which passes through
      a₀₀,....,a₍ₘ₋₁₎₀ at X = 1,2,....,m.
      Similarly, A₁(X), ...., A|w|₋₁(X) are constructed.
      We then use A₀(X), ...., A|w|₋₁(X) and the witness vector to construct the polynomial A(X).

      Similar procedure is followed to construct B(X) and C(X).

      Z(X) is the vanishing polynomial on evaluation domain D.

      And lastly, we need to determine the polynomial H(X).
      Computing atleast (m+1) values of H(X), will reveal H(X) in its value representation.
      This requires us to compute (m+1) values of A(X), B(X), C(X) and Z(X).
      And taking the bruteforce approach, computational Time Complexity will be O(n²).
      We'll instead use Fast Fourier Transform (FFT), which cuts down the Time Complexity to
      O(n.log(n)).
    */

    // Construct the Evaluation Domain D, which will be used to do the FFTs.
    // We'll use the Radix2 evaluation domain, which implies that the roots of unity will be our
    // evaluation points.
    let evaluationDomain = Self::getEvaluationDomain(&r1cs);

    // Evaluate A(τ), B(τ), C(τ) and Z(τ).
    // These will be used for the KZG Polynomial Commitment Scheme based trusted setup ceremony.

    // Evaluate Z(τ).
    let ZAtTau = evaluationDomain.evaluate_vanishing_polynomial(*tau);

    // To determine A(τ), B(τ) and C(τ) :
    // NOTE : Each of the above polynomial is of degree (m-1).

    // Evaluation points will be chosen from the constructed Radix2 Evaluation Domain. And each
    // evaluation point will have a corresponding Lagrange base.
    // Lets evaluate those Lagrange bases at τ.
    let evaluatedLagrangeBases = evaluationDomain.evaluate_all_lagrange_coefficients(*tau);

    // Then, evaluate individual terms at τ, for each polynomial.

    let mut AAtTau = vec![FF::zero(); witnessVectorSize]; // Stores Aⱼ(X=τ) for 0 <= j <= (|w|-1).
    let mut BAtTau = vec![FF::zero(); witnessVectorSize];
    let mut CAtTau = vec![FF::zero(); witnessVectorSize];
    {
      // Regarding public inputs.
      #[allow(clippy::identity_op)]
      AAtTau[0..r1cs.num_instance_variables()].copy_from_slice(
        &evaluatedLagrangeBases
          [(0 + r1cs.num_constraints())..(r1cs.num_instance_variables() + r1cs.num_constraints())],
      );

      // Regarding variables created in the intermediate steps of the R1CS.
      for (i, evaluatedLagrangeBase) in evaluatedLagrangeBases
        .iter()
        .enumerate()
        .take(r1cs.num_constraints())
      {
        // For each Aⱼ(X) in A(X),
        // add the ith term.
        for &(ref ithCoefficient, j) in &matrices.a[i] {
          AAtTau[j] += (*ithCoefficient) * (*evaluatedLagrangeBase);
        }

        // Similarly,

        for &(ref ithCoefficient, j) in &matrices.b[i] {
          BAtTau[j] += (*ithCoefficient) * (*evaluatedLagrangeBase);
        }

        for &(ref ithCoefficient, j) in &matrices.c[i] {
          CAtTau[j] += (*ithCoefficient) * (*evaluatedLagrangeBase);
        }
      }
    }

    QAPGenerationOutput {
      qapVariableCount,

      evaluationDomainSize: evaluationDomain.size,

      AAtTau,
      BAtTau,
      CAtTau,
      ZAtTau,
    }
  }

  /// Constructs Radix2 Evaluation Domain, which will be used to do the FFTs.
  /// This implies that that roots of unity will be the evaluation points.
  pub(crate) fn getEvaluationDomain(r1cs: &ConstraintSystemRef<FF>) -> Radix2EvaluationDomain<FF> {
    // TODO : Understand, why that value is used as the minimum evaluation domain size.
    //        The highest degree polynomial in the QAP is A(X)*B(X), with degree 2(m-1),
    //        m being the number of constraints in R1CS.
    let minEvaluationDomainSize = r1cs.num_constraints() + r1cs.num_instance_variables();

    // NOTE : For FFT, the evaluation point count must be a power of 2.
    //        Radix2EvaluationDomain::new( ) takes care of that.
    Radix2EvaluationDomain::<FF>::new(minEvaluationDomainSize)
      .ok_or(SynthesisError::PolynomialDegreeTooLarge)
      .unwrap()
  }
}
