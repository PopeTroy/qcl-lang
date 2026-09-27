def LandauerLimit : Float := 2.87e-21

theorem energy_above_landauer (op_energy : Float) (h : op_energy >= LandauerLimit) :
  op_energy >= LandauerLimit := by
  exact h
