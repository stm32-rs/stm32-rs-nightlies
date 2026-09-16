///Register `OFR2` reader
pub type R = crate::R<OFR2rs>;
///Register `OFR2` writer
pub type W = crate::W<OFR2rs>;
///Field `OFFSET` reader - Data offset y for the channel programmed into bits OFFSET_CH\[4:0\] These bits are written by software to define the offset to be subtracted from the raw converted data when converting a channel (can be regular or injected). The channel to which applies the data offset must be programmed in the bits OFFSET_CH\[4:0\]. The conversion result can be read from in the ADC_DR (regular conversion) or from in the ADC_JDRyi registers (injected conversion). Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). If several offset (OFFSET) point to the same channel, only the offset with the lowest x value is considered for the subtraction. Note: Ex: if OFFSET1_CH\[4:0\] = 4 and OFFSET2_CH\[4:0\] = 4, this is OFFSET1\[11:0\] which is subtracted when converting channel 4.
pub type OFFSET_R = crate::FieldReader<u16>;
///Field `OFFSET` writer - Data offset y for the channel programmed into bits OFFSET_CH\[4:0\] These bits are written by software to define the offset to be subtracted from the raw converted data when converting a channel (can be regular or injected). The channel to which applies the data offset must be programmed in the bits OFFSET_CH\[4:0\]. The conversion result can be read from in the ADC_DR (regular conversion) or from in the ADC_JDRyi registers (injected conversion). Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). If several offset (OFFSET) point to the same channel, only the offset with the lowest x value is considered for the subtraction. Note: Ex: if OFFSET1_CH\[4:0\] = 4 and OFFSET2_CH\[4:0\] = 4, this is OFFSET1\[11:0\] which is subtracted when converting channel 4.
pub type OFFSET_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `OFFSETPOS` reader - Positive offset This bit is set and cleared by software to enable the positive offset. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type OFFSETPOS_R = crate::BitReader;
///Field `OFFSETPOS` writer - Positive offset This bit is set and cleared by software to enable the positive offset. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type OFFSETPOS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SATEN` reader - Saturation enable This bit is set and cleared by software to enable the saturation at 0x000 and 0xFFF for the offset function. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type SATEN_R = crate::BitReader;
///Field `SATEN` writer - Saturation enable This bit is set and cleared by software to enable the saturation at 0x000 and 0xFFF for the offset function. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type SATEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OFFSET_CH` reader - Channel selection for the data offset y These bits are written by software to define the channel to which the offset programmed into bits OFFSET\[11:0\] applies. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Note: Some channels are not connected physically and must not be selected for the data offset y. Note: If OFFSET_EN is set, it is not allowed to select the same channel for different ADC_OFRy registers.
pub type OFFSET_CH_R = crate::FieldReader;
///Field `OFFSET_CH` writer - Channel selection for the data offset y These bits are written by software to define the channel to which the offset programmed into bits OFFSET\[11:0\] applies. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Note: Some channels are not connected physically and must not be selected for the data offset y. Note: If OFFSET_EN is set, it is not allowed to select the same channel for different ADC_OFRy registers.
pub type OFFSET_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `OFFSET_EN` reader - Offset y enable This bit is written by software to enable or disable the offset programmed into bits OFFSET\[11:0\]. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type OFFSET_EN_R = crate::BitReader;
///Field `OFFSET_EN` writer - Offset y enable This bit is written by software to enable or disable the offset programmed into bits OFFSET\[11:0\]. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type OFFSET_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:11 - Data offset y for the channel programmed into bits OFFSET_CH\[4:0\] These bits are written by software to define the offset to be subtracted from the raw converted data when converting a channel (can be regular or injected). The channel to which applies the data offset must be programmed in the bits OFFSET_CH\[4:0\]. The conversion result can be read from in the ADC_DR (regular conversion) or from in the ADC_JDRyi registers (injected conversion). Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). If several offset (OFFSET) point to the same channel, only the offset with the lowest x value is considered for the subtraction. Note: Ex: if OFFSET1_CH\[4:0\] = 4 and OFFSET2_CH\[4:0\] = 4, this is OFFSET1\[11:0\] which is subtracted when converting channel 4.
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bit 24 - Positive offset This bit is set and cleared by software to enable the positive offset. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn offsetpos(&self) -> OFFSETPOS_R {
        OFFSETPOS_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Saturation enable This bit is set and cleared by software to enable the saturation at 0x000 and 0xFFF for the offset function. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn saten(&self) -> SATEN_R {
        SATEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bits 26:30 - Channel selection for the data offset y These bits are written by software to define the channel to which the offset programmed into bits OFFSET\[11:0\] applies. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Note: Some channels are not connected physically and must not be selected for the data offset y. Note: If OFFSET_EN is set, it is not allowed to select the same channel for different ADC_OFRy registers.
    #[inline(always)]
    pub fn offset_ch(&self) -> OFFSET_CH_R {
        OFFSET_CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    ///Bit 31 - Offset y enable This bit is written by software to enable or disable the offset programmed into bits OFFSET\[11:0\]. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn offset_en(&self) -> OFFSET_EN_R {
        OFFSET_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OFR2")
            .field("offset", &self.offset())
            .field("offsetpos", &self.offsetpos())
            .field("saten", &self.saten())
            .field("offset_ch", &self.offset_ch())
            .field("offset_en", &self.offset_en())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - Data offset y for the channel programmed into bits OFFSET_CH\[4:0\] These bits are written by software to define the offset to be subtracted from the raw converted data when converting a channel (can be regular or injected). The channel to which applies the data offset must be programmed in the bits OFFSET_CH\[4:0\]. The conversion result can be read from in the ADC_DR (regular conversion) or from in the ADC_JDRyi registers (injected conversion). Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). If several offset (OFFSET) point to the same channel, only the offset with the lowest x value is considered for the subtraction. Note: Ex: if OFFSET1_CH\[4:0\] = 4 and OFFSET2_CH\[4:0\] = 4, this is OFFSET1\[11:0\] which is subtracted when converting channel 4.
    #[inline(always)]
    pub fn offset(&mut self) -> OFFSET_W<'_, OFR2rs> {
        OFFSET_W::new(self, 0)
    }
    ///Bit 24 - Positive offset This bit is set and cleared by software to enable the positive offset. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn offsetpos(&mut self) -> OFFSETPOS_W<'_, OFR2rs> {
        OFFSETPOS_W::new(self, 24)
    }
    ///Bit 25 - Saturation enable This bit is set and cleared by software to enable the saturation at 0x000 and 0xFFF for the offset function. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn saten(&mut self) -> SATEN_W<'_, OFR2rs> {
        SATEN_W::new(self, 25)
    }
    ///Bits 26:30 - Channel selection for the data offset y These bits are written by software to define the channel to which the offset programmed into bits OFFSET\[11:0\] applies. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Note: Some channels are not connected physically and must not be selected for the data offset y. Note: If OFFSET_EN is set, it is not allowed to select the same channel for different ADC_OFRy registers.
    #[inline(always)]
    pub fn offset_ch(&mut self) -> OFFSET_CH_W<'_, OFR2rs> {
        OFFSET_CH_W::new(self, 26)
    }
    ///Bit 31 - Offset y enable This bit is written by software to enable or disable the offset programmed into bits OFFSET\[11:0\]. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn offset_en(&mut self) -> OFFSET_EN_W<'_, OFR2rs> {
        OFFSET_EN_W::new(self, 31)
    }
}
/**ADC offset 2 register

You can [`read`](crate::Reg::read) this register and get [`ofr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ofr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#ADC1:OFR2)*/
pub struct OFR2rs;
impl crate::RegisterSpec for OFR2rs {
    type Ux = u32;
}
///`read()` method returns [`ofr2::R`](R) reader structure
impl crate::Readable for OFR2rs {}
///`write(|w| ..)` method takes [`ofr2::W`](W) writer structure
impl crate::Writable for OFR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OFR2 to value 0
impl crate::Resettable for OFR2rs {}
