RSpec.describe AdventOfCode::DaySixteen do
  let(:full_input) do
    <<~TXT
      420D74C3088043390499ED709E6EB49A5CC4A3A3898B7E0F44011C4CC48AC0119D049B0C500265EB8F615900180910C88129B2F0007C61C4B7F74ED7396B20020A44A4C014D005E5A72E274B4E5C4B96CC3793410078C01D82F1DA08180351661AC1920042A3CC578BA6008F802138D93352B9CFCEF61D3009A7D2268D254925569C02A92D62BF108D52C1B3E4B257B57FAE5C54400A84840267880311D23245F1007A35C79848200C4288FF0E8C01194A4E625E00A4EFEF5F5996486C400C5002800BFA402D3D00A9C4027B98093D602231C00F001D38C009500258057E601324C00D3003D400C7003DC00A20053A6F1DBDE2D4600A6802B37C4B9E872B0E44CA5FF0BFB116C3004740119895E6F7312BCDE25EF077700725B9F2B8F131F333005740169A7F92EFEB3BC8A21998027400D2CDF30F927880B4C62D6CDFFD88EB0068D2BF019A8DAAF3245B39C9CFA1D2DF9C3DB9D3E50A0164BE2A3339436993894EC41A0D10020B329334C62016C8E7A5F27C97D0663982D8EB23C5282529CDD271E8F100AE1401AA80021119E3A4511006E1E47689323585F3AEBF900AEB2B6942BD91EE8028000874238AB0C00010B8D913220A004A73D789C4D54E24816301802538E940198880371AE15C1D1007638C43856C00954C25CD595A471FE9D90056D60094CEA61933A9854E9F3801F2BBC6131001F792F6796ACB40D036605C80348C005F64F5AC374888CA42FD99A98025319EB950025713656F202200B767AB6A30E802D278F81CBA89004CD286360094FC03A7E01640245CED5A3C010100660FC578B60008641C8B105CC017F004E597E596E633BA5AB78B9C8F840C029917C9E389B439179927A3004F003511006610C658A200084C2989D0AE67BD07000606154B70E66DC0C01E99649545950B8AB34C8401A5CDA050043D319F31CB7EBCEE14
    TXT
  end

  describe AdventOfCode::DaySixteen::Packets do
    describe ".parse" do
      context "with a literal packet" do
        it "contains a number" do
          packet = AdventOfCode::DaySixteen::Packets.parse("D2FE28")
          expect(packet).to be_a(AdventOfCode::DaySixteen::Packets::NumberLiteral)
          expect(packet.number).to eq(2021)
        end
      end

      context "with a operator and bit size sub-packets" do
        it "contains two numbers" do
          packet = AdventOfCode::DaySixteen::Packets.parse("38006F45291200")
          expect(packet).to be_a(AdventOfCode::DaySixteen::Packets::Operator)
          expect(packet.sub_packets.map(&:number)).to eq([10, 20])
        end
      end

      context "with a operator and countable sub-packets" do
        it "contains two numbers" do
          packet = AdventOfCode::DaySixteen::Packets.parse("EE00D40C823060")
          expect(packet).to be_a(AdventOfCode::DaySixteen::Packets::Operator)
          expect(packet.sub_packets.map(&:number)).to eq([1, 2, 3])
        end
      end
    end
  end

  describe "#part_one" do
    context "with simple input" do
      it "works" do
        expect(described_class.for(input: StringIO.new("8A004A801A8002F478")).part_one).to eq(16)
        expect(described_class.for(input: StringIO.new("620080001611562C8802118E34")).part_one).to eq(12)
        expect(described_class.for(input: StringIO.new("C0015000016115A2E0802F182340")).part_one).to eq(23)
        expect(described_class.for(input: StringIO.new("A0016C880162017C3686B18A3D4780")).part_one).to eq(31)
      end
    end

    context "with full input" do
      it "works" do
        expect(described_class.for(input: StringIO.new(full_input)).part_one).to eq(895)
      end
    end
  end

  describe "#part_two" do
    context "with simple input" do
      it "works" do
        expect(described_class.for(input: StringIO.new("C200B40A82")).part_two).to eq(3)
        expect(described_class.for(input: StringIO.new("04005AC33890")).part_two).to eq(54)
        expect(described_class.for(input: StringIO.new("880086C3E88112")).part_two).to eq(7)
        expect(described_class.for(input: StringIO.new("CE00C43D881120")).part_two).to eq(9)
        expect(described_class.for(input: StringIO.new("D8005AC2A8F0")).part_two).to eq(1)
        expect(described_class.for(input: StringIO.new("F600BC2D8F")).part_two).to eq(0)
        expect(described_class.for(input: StringIO.new("9C005AC2F8F0")).part_two).to eq(0)
        expect(described_class.for(input: StringIO.new("9C0141080250320F1802104A08")).part_two).to eq(1)
      end
    end

    context "with full input" do
      it "works" do
        expect(described_class.for(input: StringIO.new(full_input)).part_two).to eq(1148595959144)
      end
    end
  end
end
