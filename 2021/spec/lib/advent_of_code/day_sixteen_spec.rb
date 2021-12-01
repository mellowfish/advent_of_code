RSpec.describe AdventOfCode::DaySixteen do
  let(:simple_input) do
    <<~TXT
      a
      b
      c
    TXT
  end

  let(:full_input) do
    <<~TXT
      a
      b
      c
      d
      e
      f
    TXT
  end

  describe "#part_one" do
    context "with simple input" do
      it "works" do
        expect(described_class.for(input: StringIO.new(simple_input)).part_one).to eq(1)
      end
    end

    context "with full input" do
      it "works" do
        expect(described_class.for(input: StringIO.new(full_input)).part_one).to eq(42)
      end
    end
  end

  describe "#part_two" do
    context "with simple input" do
      it "works" do
        expect(described_class.for(input: StringIO.new(simple_input)).part_two).to eq(1)
      end
    end

    context "with full input" do
      it "works" do
        expect(described_class.for(input: StringIO.new(full_input)).part_two).to eq(42)
      end
    end
  end
end
