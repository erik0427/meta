require 'spec_helper'

describe CMAC do
  describe 'sign' do
    test_vectors.each do |name, options|
      it "should match the \"#{name}\" test vector" do
        cmac = CMAC.new(options[:Key])
        output = cmac.sign(options[:Message], options[:Truncate].to_i)
        expect(output).to eq(options[:Tag])
      end
    end

    it 'should give a truncated output if requested' do
      cmac = CMAC.new(TestKey)
      output = cmac.sign('attack at dawn', 12)
      expect(output.length).to eq(12)
    end

    it 'should raise error if truncation request is greater than 16 bytes' do
      cmac = CMAC.new(TestKey)
      expect do
        cmac.sign('attack at dawn', 17)
      end.to raise_error(CMAC::Exception, 'Tag cannot be greater than maximum (16 bytes)')
    end

    it 'should raise error if truncation request is less than 8 bytes' do
      cmac = CMAC.new(TestKey)
      expect do
        cmac.sign('attack at dawn', 7)
      end.to raise_error(CMAC::Exception, 'Tag cannot be less than minimum (8 bytes)')
    end
  end

  describe 'valid_message?' do
    it 'should be true for matching messages' do
      message = 'attack at dawn'
      cmac = CMAC.new(TestKey)
      tag = cmac.sign(message)
      result = cmac.valid_message?(tag, message)
      expect(result).to be_truthy
    end

    it 'should be false for modified messages' do
      cmac = CMAC.new(TestKey)
      tag = cmac.sign('attack at dawn')
      result = cmac.valid_message?(tag, 'attack at dusk')
      expect(result).to be_falsey
    end
  end
end
