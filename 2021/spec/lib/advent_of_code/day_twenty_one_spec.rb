RSpec.describe AdventOfCode::DayTwentyOne do
  let(:simple_input) do
    <<~TXT
      Player 1 starting position: 4
      Player 2 starting position: 8
    TXT
  end

  let(:full_input) do
    <<~TXT
      Player 1 starting position: 3
      Player 2 starting position: 5
    TXT
  end

  describe "#part_one" do
    context "with simple input" do
      it "works" do
        expect(described_class.for(input: StringIO.new(simple_input)).part_one).to eq(739785)
      end
    end

    context "with full input" do
      it "works" do
        expect(described_class.for(input: StringIO.new(full_input)).part_one).to eq(720750)
      end
    end
  end

  describe "#part_two" do
    context "with simple input" do
      xit "works" do
        expect(described_class.for(input: StringIO.new(simple_input)).part_two).to eq(444356092776315)
      end
    end

    context "with full input" do
      xit "works" do
        expect(described_class.for(input: StringIO.new(full_input)).part_two).to eq(42)
      end
    end
  end
end
