RSpec.describe AdventOfCode::DayFourteen do
  let(:simple_input) do
    <<~TXT
      NNCB
      
      CH -> B
      HH -> N
      CB -> H
      NH -> C
      HB -> C
      HC -> B
      HN -> C
      NN -> C
      BH -> H
      NC -> B
      NB -> B
      BN -> B
      BB -> N
      BC -> B
      CC -> N
      CN -> C
    TXT
  end
  #  2: N4                                              N4
  #  3: N3                      C3                      N3
  #  5: N2          B2          C2          C2          N2
  #  9: N1    B1    B1    B1    C1    N1    C1    C1    N1
  # 17: N0 B0 B0 N0 B0 N0 B0 B0 C0 C0 N0 B0 C0 N0 C0 C0 N0
  #     0  1  2  3  4  5  6  7  8  9  10 11 12 13 14 15 16
  # size (2): 2^depth + 1
  # size (n): (n-1) * 2^(depth - n) + 1

  #  4: NNCB
  #  7: NCNBCHB
  # 13: NBCCNBBBCBHCB
  # 25: NBBBCNCCNBBNBNBBCHBHHBCHB
  # 49: NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB
  let(:full_input) do
    <<~TXT
      BNBBNCFHHKOSCHBKKSHN
      
      CH -> S
      KK -> V
      FS -> V
      CN -> P
      VC -> N
      CB -> V
      VK -> H
      CF -> N
      PO -> O
      KC -> S
      HC -> P
      PP -> B
      KO -> B
      BK -> P
      BH -> N
      CC -> N
      PC -> O
      FK -> N
      KF -> F
      FH -> S
      SS -> V
      ON -> K
      OV -> K
      NK -> H
      BO -> C
      VP -> O
      CS -> V
      KS -> K
      SK -> B
      OP -> S
      PK -> S
      HF -> P
      SV -> P
      SB -> C
      BC -> C
      FP -> H
      FC -> P
      PB -> N
      NV -> F
      VO -> F
      VH -> P
      BB -> N
      SF -> F
      NB -> K
      KB -> S
      VV -> S
      NP -> N
      SO -> O
      PN -> B
      BP -> H
      BV -> V
      OB -> C
      HV -> N
      PF -> B
      SP -> N
      HN -> N
      CV -> H
      BN -> V
      PS -> V
      CO -> S
      BS -> N
      VB -> H
      PV -> P
      NN -> P
      HS -> C
      OS -> P
      FB -> S
      HO -> C
      KH -> H
      HB -> K
      VF -> S
      CK -> K
      FF -> H
      FN -> P
      OK -> F
      SC -> B
      HH -> N
      OH -> O
      VS -> N
      FO -> N
      OC -> H
      NF -> F
      PH -> S
      HK -> K
      NH -> H
      FV -> S
      OF -> V
      NC -> O
      HP -> O
      KP -> B
      BF -> N
      NO -> S
      CP -> C
      NS -> N
      VN -> K
      KV -> N
      OO -> V
      SN -> O
      KN -> C
      SH -> F
    TXT
  end

  describe "#part_one" do
    context "with simple input" do
      it "works" do
        expect(described_class.for(input: StringIO.new(simple_input)).part_one).to eq(1588)
      end
    end

    context "with full input" do
      it "works" do
        expect(described_class.for(input: StringIO.new(full_input)).part_one).to eq(3009)
      end
    end
  end

  describe "#part_two" do
    context "with simple input" do
      it "works" do
        expect(described_class.for(input: StringIO.new(simple_input)).part_two).to eq(2188189693529)
      end
    end

    context "with full input" do
      it "works" do
        expect(described_class.for(input: StringIO.new(full_input)).part_two).to eq(3459822539451)
      end
    end
  end
end
