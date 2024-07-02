///Register `ADC_OFR1` reader
pub type R = crate::R<ADC_OFR1rs>;
///Register `ADC_OFR1` writer
pub type W = crate::W<ADC_OFR1rs>;
/**Field `OFFSET` reader - Data offset y for the channel programmed into OFFSETy_CH\[4:0\]
bits These bits are written by software to define the offset y to be subtracted from the raw converted data when converting a channel (regular or injected). The channel to which the data offset y applies must be programmed to the OFFSETy_CH\[4:0\]
bits. The conversion result can be read from in the ADC_DR (regular conversion) or from in the ADC_JDRyi registers (injected conversion). When OFFSETy\[21:0\]
bitfield is reset, the offset compensation is disabled. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). If several offsets (OFFSETy) point to the same channel, only the offset with the lowest y value is considered for the subtraction. For example, if OFFSET1_CH\[4:0\]
= 4 and OFFSET2_CH\[4:0\]
= 4, this is OFFSET1\[25:0\]
that is subtracted when converting channel 4.*/
pub type OFFSET_R = crate::FieldReader<u32>;
/**Field `OFFSET` writer - Data offset y for the channel programmed into OFFSETy_CH\[4:0\]
bits These bits are written by software to define the offset y to be subtracted from the raw converted data when converting a channel (regular or injected). The channel to which the data offset y applies must be programmed to the OFFSETy_CH\[4:0\]
bits. The conversion result can be read from in the ADC_DR (regular conversion) or from in the ADC_JDRyi registers (injected conversion). When OFFSETy\[21:0\]
bitfield is reset, the offset compensation is disabled. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). If several offsets (OFFSETy) point to the same channel, only the offset with the lowest y value is considered for the subtraction. For example, if OFFSET1_CH\[4:0\]
= 4 and OFFSET2_CH\[4:0\]
= 4, this is OFFSET1\[25:0\]
that is subtracted when converting channel 4.*/
pub type OFFSET_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
///Field `POSOFF` reader - offset sign This bit is set and cleared by software to enable the positive offset. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type POSOFF_R = crate::BitReader;
///Field `POSOFF` writer - offset sign This bit is set and cleared by software to enable the positive offset. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type POSOFF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USAT` reader - Unsigned saturation enable This bit is written by software to enable or disable the unsigned saturation feature. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type USAT_R = crate::BitReader;
///Field `USAT` writer - Unsigned saturation enable This bit is written by software to enable or disable the unsigned saturation feature. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type USAT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SSAT` reader - Signed saturation enable This bit is written by software to enable or disable the Signed saturation feature. (see OFFSETy_CH, OVSS, LSHIFT, USAT, SSAT) for details). Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type SSAT_R = crate::BitReader;
///Field `SSAT` writer - Signed saturation enable This bit is written by software to enable or disable the Signed saturation feature. (see OFFSETy_CH, OVSS, LSHIFT, USAT, SSAT) for details). Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type SSAT_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `OFFSET_CH` reader - Channel selection for the data offset y These bits are written by software to define the channel to which the offset programmed into OFFSETy\[25:0\]
bits applies. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). If OFFSETy_EN bit is set, it is not allowed to select the same channel in different ADC_OFRy registers.*/
pub type OFFSET_CH_R = crate::FieldReader;
/**Field `OFFSET_CH` writer - Channel selection for the data offset y These bits are written by software to define the channel to which the offset programmed into OFFSETy\[25:0\]
bits applies. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). If OFFSETy_EN bit is set, it is not allowed to select the same channel in different ADC_OFRy registers.*/
pub type OFFSET_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    /**Bits 0:23 - Data offset y for the channel programmed into OFFSETy_CH\[4:0\]
    bits These bits are written by software to define the offset y to be subtracted from the raw converted data when converting a channel (regular or injected). The channel to which the data offset y applies must be programmed to the OFFSETy_CH\[4:0\]
    bits. The conversion result can be read from in the ADC_DR (regular conversion) or from in the ADC_JDRyi registers (injected conversion). When OFFSETy\[21:0\]
    bitfield is reset, the offset compensation is disabled. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). If several offsets (OFFSETy) point to the same channel, only the offset with the lowest y value is considered for the subtraction. For example, if OFFSET1_CH\[4:0\]
    = 4 and OFFSET2_CH\[4:0\]
    = 4, this is OFFSET1\[25:0\]
    that is subtracted when converting channel 4.*/
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new(self.bits & 0x00ff_ffff)
    }
    ///Bit 24 - offset sign This bit is set and cleared by software to enable the positive offset. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn posoff(&self) -> POSOFF_R {
        POSOFF_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Unsigned saturation enable This bit is written by software to enable or disable the unsigned saturation feature. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn usat(&self) -> USAT_R {
        USAT_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Signed saturation enable This bit is written by software to enable or disable the Signed saturation feature. (see OFFSETy_CH, OVSS, LSHIFT, USAT, SSAT) for details). Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn ssat(&self) -> SSAT_R {
        SSAT_R::new(((self.bits >> 26) & 1) != 0)
    }
    /**Bits 27:31 - Channel selection for the data offset y These bits are written by software to define the channel to which the offset programmed into OFFSETy\[25:0\]
    bits applies. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). If OFFSETy_EN bit is set, it is not allowed to select the same channel in different ADC_OFRy registers.*/
    #[inline(always)]
    pub fn offset_ch(&self) -> OFFSET_CH_R {
        OFFSET_CH_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_OFR1")
            .field("offset", &self.offset())
            .field("posoff", &self.posoff())
            .field("usat", &self.usat())
            .field("ssat", &self.ssat())
            .field("offset_ch", &self.offset_ch())
            .finish()
    }
}
impl W {
    /**Bits 0:23 - Data offset y for the channel programmed into OFFSETy_CH\[4:0\]
    bits These bits are written by software to define the offset y to be subtracted from the raw converted data when converting a channel (regular or injected). The channel to which the data offset y applies must be programmed to the OFFSETy_CH\[4:0\]
    bits. The conversion result can be read from in the ADC_DR (regular conversion) or from in the ADC_JDRyi registers (injected conversion). When OFFSETy\[21:0\]
    bitfield is reset, the offset compensation is disabled. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). If several offsets (OFFSETy) point to the same channel, only the offset with the lowest y value is considered for the subtraction. For example, if OFFSET1_CH\[4:0\]
    = 4 and OFFSET2_CH\[4:0\]
    = 4, this is OFFSET1\[25:0\]
    that is subtracted when converting channel 4.*/
    #[inline(always)]
    #[must_use]
    pub fn offset(&mut self) -> OFFSET_W<ADC_OFR1rs> {
        OFFSET_W::new(self, 0)
    }
    ///Bit 24 - offset sign This bit is set and cleared by software to enable the positive offset. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn posoff(&mut self) -> POSOFF_W<ADC_OFR1rs> {
        POSOFF_W::new(self, 24)
    }
    ///Bit 25 - Unsigned saturation enable This bit is written by software to enable or disable the unsigned saturation feature. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn usat(&mut self) -> USAT_W<ADC_OFR1rs> {
        USAT_W::new(self, 25)
    }
    ///Bit 26 - Signed saturation enable This bit is written by software to enable or disable the Signed saturation feature. (see OFFSETy_CH, OVSS, LSHIFT, USAT, SSAT) for details). Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn ssat(&mut self) -> SSAT_W<ADC_OFR1rs> {
        SSAT_W::new(self, 26)
    }
    /**Bits 27:31 - Channel selection for the data offset y These bits are written by software to define the channel to which the offset programmed into OFFSETy\[25:0\]
    bits applies. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). If OFFSETy_EN bit is set, it is not allowed to select the same channel in different ADC_OFRy registers.*/
    #[inline(always)]
    #[must_use]
    pub fn offset_ch(&mut self) -> OFFSET_CH_W<ADC_OFR1rs> {
        OFFSET_CH_W::new(self, 27)
    }
}
/**ADC offset register

You can [`read`](crate::Reg::read) this register and get [`adc_ofr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_ofr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#ADC1:ADC_OFR1)*/
pub struct ADC_OFR1rs;
impl crate::RegisterSpec for ADC_OFR1rs {
    type Ux = u32;
}
///`read()` method returns [`adc_ofr1::R`](R) reader structure
impl crate::Readable for ADC_OFR1rs {}
///`write(|w| ..)` method takes [`adc_ofr1::W`](W) writer structure
impl crate::Writable for ADC_OFR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ADC_OFR1 to value 0
impl crate::Resettable for ADC_OFR1rs {
    const RESET_VALUE: u32 = 0;
}
