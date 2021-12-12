RSpec.describe AdventOfCode::DayTwelve do
  let(:simple_input) do
    <<~TXT
      fs-end
      he-DX
      fs-he
      start-DX
      pj-DX
      end-zg
      zg-sl
      zg-pj
      pj-he
      RW-he
      fs-DX
      pj-RW
      zg-RW
      start-pj
      he-WI
      zg-he
      pj-fs
      start-RW
    TXT
  end

  let(:full_input) do
    <<~TXT
      yw-MN
      wn-XB
      DG-dc
      MN-wn
      yw-DG
      start-dc
      start-ah
      MN-start
      fi-yw
      XB-fi
      wn-ah
      MN-ah
      MN-dc
      end-yw
      fi-end
      th-fi
      end-XB
      dc-XB
      yw-XN
      wn-yw
      dc-ah
      MN-fi
      wn-DG
    TXT
  end

  describe "#part_one" do
    context "with simple input" do
      it "works" do
        expect(described_class.for(input: StringIO.new(simple_input)).part_one).to eq(226)
      end
    end

    context "with full input" do
      it "works" do
        expect(described_class.for(input: StringIO.new(full_input)).part_one).to eq(4241)
      end
    end
  end

  describe "#part_two" do
    context "with simple input" do
      it "works" do
        expect(described_class.for(input: StringIO.new(simple_input)).part_two).to eq(3509)
      end
    end

    context "with full input" do
      it "works" do
        expect(described_class.for(input: StringIO.new(full_input)).part_two).to eq(122134)
      end
    end
  end
end
