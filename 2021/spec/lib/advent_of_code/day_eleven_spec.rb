RSpec.describe AdventOfCode::DayEleven do
  let(:simple_input) do
    <<~TXT
      5483143223
      2745854711
      5264556173
      6141336146
      6357385478
      4167524645
      2176841721
      6882881134
      4846848554
      5283751526
    TXT
  end

  let(:full_input) do
    <<~TXT
    5251578181
    6158452313
    1818578571
    3844615143
    6857251244
    2375817613
    8883514435
    2321265735
    2857275182
    4821156644
    TXT
  end

  describe "#part_one" do
    context "with simple input" do
      it "works" do
        expect(described_class.for(input: StringIO.new(simple_input)).part_one).to eq(1656)
      end
    end

    context "with full input" do
      it "works" do
        expect(described_class.for(input: StringIO.new(full_input)).part_one).to eq(1637)
      end
    end
  end

  describe "#part_two" do
    context "with simple input" do
      it "works" do
        expect(described_class.for(input: StringIO.new(simple_input)).part_two).to eq(195)
      end
    end

    context "with full input" do
      it "works" do
        expect(described_class.for(input: StringIO.new(full_input)).part_two).to eq(242)
      end
    end
  end
end
