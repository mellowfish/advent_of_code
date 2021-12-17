RSpec.describe AdventOfCode::DaySeventeen do
  let(:simple_input) do
    <<~TXT
      target area: x=20..30, y=-10..-5
    TXT
  end

  let(:full_input) do
    <<~TXT
      target area: x=144..178, y=-100..-76
    TXT
  end

  describe "#part_one" do
    context "with simple input" do
      it "works" do
        expect(described_class.for(input: StringIO.new(simple_input)).part_one).to eq(45)
      end
    end

    context "with full input" do
      it "works" do
        expect(described_class.for(input: StringIO.new(full_input)).part_one).to eq(4950)
      end
    end
  end

  describe "#part_two" do
    context "with simple input" do
      it "works" do
        expect(described_class.for(input: StringIO.new(simple_input)).part_two).to eq(112)
      end
    end

    context "with full input" do
      it "works" do
        expect(described_class.for(input: StringIO.new(full_input)).part_two).to eq(1477)
      end
    end
  end
end
