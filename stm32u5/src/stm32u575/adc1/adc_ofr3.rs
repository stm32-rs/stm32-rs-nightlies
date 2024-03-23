#[doc = "Register `ADC_OFR3` reader"]
pub type R = crate::R<ADC_OFR3rs>;
#[doc = "Register `ADC_OFR3` writer"]
pub type W = crate::W<ADC_OFR3rs>;
#[doc = "Field `OFFSET` reader - Data offset y for the channel programmed into OFFSETy_CH\\[4:0\\]
bits These bits are written by software to define the offset y to be subtracted from the raw converted data when converting a channel (regular or injected). The channel to which the data offset y applies must be programmed to the OFFSETy_CH\\[4:0\\]
bits. The conversion result can be read from in the ADC_DR (regular conversion) or from in the ADC_JDRyi registers (injected conversion). When OFFSETy\\[21:0\\]
bitfield is reset, the offset compensation is disabled. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). If several offsets (OFFSETy) point to the same channel, only the offset with the lowest y value is considered for the subtraction. For example, if OFFSET1_CH\\[4:0\\]
= 4 and OFFSET2_CH\\[4:0\\]
= 4, this is OFFSET1\\[25:0\\]
that is subtracted when converting channel 4."]
pub type OFFSET_R = crate::FieldReader<u32>;
#[doc = "Field `OFFSET` writer - Data offset y for the channel programmed into OFFSETy_CH\\[4:0\\]
bits These bits are written by software to define the offset y to be subtracted from the raw converted data when converting a channel (regular or injected). The channel to which the data offset y applies must be programmed to the OFFSETy_CH\\[4:0\\]
bits. The conversion result can be read from in the ADC_DR (regular conversion) or from in the ADC_JDRyi registers (injected conversion). When OFFSETy\\[21:0\\]
bitfield is reset, the offset compensation is disabled. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). If several offsets (OFFSETy) point to the same channel, only the offset with the lowest y value is considered for the subtraction. For example, if OFFSET1_CH\\[4:0\\]
= 4 and OFFSET2_CH\\[4:0\\]
= 4, this is OFFSET1\\[25:0\\]
that is subtracted when converting channel 4."]
pub type OFFSET_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `POSOFF` reader - offset sign This bit is set and cleared by software to enable the positive offset. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type POSOFF_R = crate::BitReader;
#[doc = "Field `POSOFF` writer - offset sign This bit is set and cleared by software to enable the positive offset. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type POSOFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USAT` reader - Unsigned saturation enable This bit is written by software to enable or disable the unsigned saturation feature. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type USAT_R = crate::BitReader;
#[doc = "Field `USAT` writer - Unsigned saturation enable This bit is written by software to enable or disable the unsigned saturation feature. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type USAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSAT` reader - Signed saturation enable This bit is written by software to enable or disable the Signed saturation feature. (see OFFSETy_CH, OVSS, LSHIFT, USAT, SSAT) for details). Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type SSAT_R = crate::BitReader;
#[doc = "Field `SSAT` writer - Signed saturation enable This bit is written by software to enable or disable the Signed saturation feature. (see OFFSETy_CH, OVSS, LSHIFT, USAT, SSAT) for details). Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type SSAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OFFSET_CH` reader - Channel selection for the data offset y These bits are written by software to define the channel to which the offset programmed into OFFSETy\\[25:0\\]
bits applies. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). If OFFSETy_EN bit is set, it is not allowed to select the same channel in different ADC_OFRy registers."]
pub type OFFSET_CH_R = crate::FieldReader;
#[doc = "Field `OFFSET_CH` writer - Channel selection for the data offset y These bits are written by software to define the channel to which the offset programmed into OFFSETy\\[25:0\\]
bits applies. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). If OFFSETy_EN bit is set, it is not allowed to select the same channel in different ADC_OFRy registers."]
pub type OFFSET_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:23 - Data offset y for the channel programmed into OFFSETy_CH\\[4:0\\]
bits These bits are written by software to define the offset y to be subtracted from the raw converted data when converting a channel (regular or injected). The channel to which the data offset y applies must be programmed to the OFFSETy_CH\\[4:0\\]
bits. The conversion result can be read from in the ADC_DR (regular conversion) or from in the ADC_JDRyi registers (injected conversion). When OFFSETy\\[21:0\\]
bitfield is reset, the offset compensation is disabled. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). If several offsets (OFFSETy) point to the same channel, only the offset with the lowest y value is considered for the subtraction. For example, if OFFSET1_CH\\[4:0\\]
= 4 and OFFSET2_CH\\[4:0\\]
= 4, this is OFFSET1\\[25:0\\]
that is subtracted when converting channel 4."]
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - offset sign This bit is set and cleared by software to enable the positive offset. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn posoff(&self) -> POSOFF_R {
        POSOFF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Unsigned saturation enable This bit is written by software to enable or disable the unsigned saturation feature. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn usat(&self) -> USAT_R {
        USAT_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Signed saturation enable This bit is written by software to enable or disable the Signed saturation feature. (see OFFSETy_CH, OVSS, LSHIFT, USAT, SSAT) for details). Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn ssat(&self) -> SSAT_R {
        SSAT_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:31 - Channel selection for the data offset y These bits are written by software to define the channel to which the offset programmed into OFFSETy\\[25:0\\]
bits applies. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). If OFFSETy_EN bit is set, it is not allowed to select the same channel in different ADC_OFRy registers."]
    #[inline(always)]
    pub fn offset_ch(&self) -> OFFSET_CH_R {
        OFFSET_CH_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - Data offset y for the channel programmed into OFFSETy_CH\\[4:0\\]
bits These bits are written by software to define the offset y to be subtracted from the raw converted data when converting a channel (regular or injected). The channel to which the data offset y applies must be programmed to the OFFSETy_CH\\[4:0\\]
bits. The conversion result can be read from in the ADC_DR (regular conversion) or from in the ADC_JDRyi registers (injected conversion). When OFFSETy\\[21:0\\]
bitfield is reset, the offset compensation is disabled. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). If several offsets (OFFSETy) point to the same channel, only the offset with the lowest y value is considered for the subtraction. For example, if OFFSET1_CH\\[4:0\\]
= 4 and OFFSET2_CH\\[4:0\\]
= 4, this is OFFSET1\\[25:0\\]
that is subtracted when converting channel 4."]
    #[inline(always)]
    #[must_use]
    pub fn offset(&mut self) -> OFFSET_W<ADC_OFR3rs> {
        OFFSET_W::new(self, 0)
    }
    #[doc = "Bit 24 - offset sign This bit is set and cleared by software to enable the positive offset. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn posoff(&mut self) -> POSOFF_W<ADC_OFR3rs> {
        POSOFF_W::new(self, 24)
    }
    #[doc = "Bit 25 - Unsigned saturation enable This bit is written by software to enable or disable the unsigned saturation feature. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn usat(&mut self) -> USAT_W<ADC_OFR3rs> {
        USAT_W::new(self, 25)
    }
    #[doc = "Bit 26 - Signed saturation enable This bit is written by software to enable or disable the Signed saturation feature. (see OFFSETy_CH, OVSS, LSHIFT, USAT, SSAT) for details). Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn ssat(&mut self) -> SSAT_W<ADC_OFR3rs> {
        SSAT_W::new(self, 26)
    }
    #[doc = "Bits 27:31 - Channel selection for the data offset y These bits are written by software to define the channel to which the offset programmed into OFFSETy\\[25:0\\]
bits applies. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). If OFFSETy_EN bit is set, it is not allowed to select the same channel in different ADC_OFRy registers."]
    #[inline(always)]
    #[must_use]
    pub fn offset_ch(&mut self) -> OFFSET_CH_W<ADC_OFR3rs> {
        OFFSET_CH_W::new(self, 27)
    }
}
#[doc = "ADC offset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_ofr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_ofr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC_OFR3rs;
impl crate::RegisterSpec for ADC_OFR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_ofr3::R`](R) reader structure"]
impl crate::Readable for ADC_OFR3rs {}
#[doc = "`write(|w| ..)` method takes [`adc_ofr3::W`](W) writer structure"]
impl crate::Writable for ADC_OFR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC_OFR3 to value 0"]
impl crate::Resettable for ADC_OFR3rs {
    const RESET_VALUE: u32 = 0;
}
