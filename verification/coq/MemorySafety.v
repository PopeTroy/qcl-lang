Theorem tmr_voting_safety : forall (a b c : nat),
  a = b -> (if Nat.eqb a b then a else c) = a.
Proof.
  intros a b c H.
  rewrite H.
  rewrite Nat.eqb_refl.
  reflexivity.
Qed.
