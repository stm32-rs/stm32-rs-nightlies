///Register `OFR%s` reader
pub type R = crate::R<OFRrs>;
///Register `OFR%s` writer
pub type W = crate::W<OFRrs>;
///Field `OFFSET` reader - Data offset y for the channel programmed into bits OFFSET_CH\[4:0\] These bits are written by software to define the offset to be subtracted from the raw converted data when converting a channel (can be regular or injected). The channel to which applies the data offset must be programmed in the bits OFFSET_CH\[4:0\]. The conversion result can be read from in the ADC_DR (regular conversion) or from in the ADC_JDRyi registers (injected conversion). Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). If several offset (OFFSET) point to the same channel, only the offset with the lowest x value is considered for the subtraction. Ex: if OFFSET1_CH\[4:0\] = 4 and OFFSET2_CH\[4:0\] = 4, this is OFFSET1\[11:0\] which is subtracted when converting channel 4.
pub type OFFSET_R = crate::FieldReader<u16>;
///Field `OFFSET` writer - Data offset y for the channel programmed into bits OFFSET_CH\[4:0\] These bits are written by software to define the offset to be subtracted from the raw converted data when converting a channel (can be regular or injected). The channel to which applies the data offset must be programmed in the bits OFFSET_CH\[4:0\]. The conversion result can be read from in the ADC_DR (regular conversion) or from in the ADC_JDRyi registers (injected conversion). Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). If several offset (OFFSET) point to the same channel, only the offset with the lowest x value is considered for the subtraction. Ex: if OFFSET1_CH\[4:0\] = 4 and OFFSET2_CH\[4:0\] = 4, this is OFFSET1\[11:0\] which is subtracted when converting channel 4.
pub type OFFSET_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16, crate::Safe>;
///Field `OFFSETPOS` reader - Positive offset This bit is set and cleared by software to enable the positive offset. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type OFFSETPOS_R = crate::BitReader;
///Field `OFFSETPOS` writer - Positive offset This bit is set and cleared by software to enable the positive offset. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type OFFSETPOS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SATEN` reader - Saturation enable This bit is set and cleared by software to enable the saturation at 0x000 and 0xFFF for the offset function. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type SATEN_R = crate::BitReader;
///Field `SATEN` writer - Saturation enable This bit is set and cleared by software to enable the saturation at 0x000 and 0xFFF for the offset function. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type SATEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OFFSET_CH` reader - Channel selection for the data offset y These bits are written by software to define the channel to which the offset programmed into bits OFFSET\[11:0\] applies. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically and must not be selected for the data offset y. If OFFSET_EN is set, it is not allowed to select the same channel for different ADC_OFRy registers.
pub type OFFSET_CH_R = crate::FieldReader;
///Field `OFFSET_CH` writer - Channel selection for the data offset y These bits are written by software to define the channel to which the offset programmed into bits OFFSET\[11:0\] applies. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically and must not be selected for the data offset y. If OFFSET_EN is set, it is not allowed to select the same channel for different ADC_OFRy registers.
pub type OFFSET_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
/**Offset y enable This bit is written by software to enable or disable the offset programmed into bits OFFSET\[11:0\]. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFFSET_EN {
    ///0: This bit is written by software to enable or disable the offset programmed into bits OFFSETy\[11:0\]
    Disabled = 0,
    ///1: This bit is written by software to enable or disable the offset programmed into bits OFFSETy\[11:0\]
    Enabled = 1,
}
impl From<OFFSET_EN> for bool {
    #[inline(always)]
    fn from(variant: OFFSET_EN) -> Self {
        variant as u8 != 0
    }
}
///Field `OFFSET_EN` reader - Offset y enable This bit is written by software to enable or disable the offset programmed into bits OFFSET\[11:0\]. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type OFFSET_EN_R = crate::BitReader<OFFSET_EN>;
impl OFFSET_EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OFFSET_EN {
        match self.bits {
            false => OFFSET_EN::Disabled,
            true => OFFSET_EN::Enabled,
        }
    }
    ///This bit is written by software to enable or disable the offset programmed into bits OFFSETy\[11:0\]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OFFSET_EN::Disabled
    }
    ///This bit is written by software to enable or disable the offset programmed into bits OFFSETy\[11:0\]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OFFSET_EN::Enabled
    }
}
///Field `OFFSET_EN` writer - Offset y enable This bit is written by software to enable or disable the offset programmed into bits OFFSET\[11:0\]. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
pub type OFFSET_EN_W<'a, REG> = crate::BitWriter<'a, REG, OFFSET_EN>;
impl<'a, REG> OFFSET_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///This bit is written by software to enable or disable the offset programmed into bits OFFSETy\[11:0\]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OFFSET_EN::Disabled)
    }
    ///This bit is written by software to enable or disable the offset programmed into bits OFFSETy\[11:0\]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OFFSET_EN::Enabled)
    }
}
impl R {
    ///Bits 0:11 - Data offset y for the channel programmed into bits OFFSET_CH\[4:0\] These bits are written by software to define the offset to be subtracted from the raw converted data when converting a channel (can be regular or injected). The channel to which applies the data offset must be programmed in the bits OFFSET_CH\[4:0\]. The conversion result can be read from in the ADC_DR (regular conversion) or from in the ADC_JDRyi registers (injected conversion). Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). If several offset (OFFSET) point to the same channel, only the offset with the lowest x value is considered for the subtraction. Ex: if OFFSET1_CH\[4:0\] = 4 and OFFSET2_CH\[4:0\] = 4, this is OFFSET1\[11:0\] which is subtracted when converting channel 4.
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
    ///Bits 26:30 - Channel selection for the data offset y These bits are written by software to define the channel to which the offset programmed into bits OFFSET\[11:0\] applies. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically and must not be selected for the data offset y. If OFFSET_EN is set, it is not allowed to select the same channel for different ADC_OFRy registers.
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
        f.debug_struct("OFR")
            .field("offset", &self.offset())
            .field("offsetpos", &self.offsetpos())
            .field("saten", &self.saten())
            .field("offset_ch", &self.offset_ch())
            .field("offset_en", &self.offset_en())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - Data offset y for the channel programmed into bits OFFSET_CH\[4:0\] These bits are written by software to define the offset to be subtracted from the raw converted data when converting a channel (can be regular or injected). The channel to which applies the data offset must be programmed in the bits OFFSET_CH\[4:0\]. The conversion result can be read from in the ADC_DR (regular conversion) or from in the ADC_JDRyi registers (injected conversion). Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). If several offset (OFFSET) point to the same channel, only the offset with the lowest x value is considered for the subtraction. Ex: if OFFSET1_CH\[4:0\] = 4 and OFFSET2_CH\[4:0\] = 4, this is OFFSET1\[11:0\] which is subtracted when converting channel 4.
    #[inline(always)]
    pub fn offset(&mut self) -> OFFSET_W<'_, OFRrs> {
        OFFSET_W::new(self, 0)
    }
    ///Bit 24 - Positive offset This bit is set and cleared by software to enable the positive offset. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn offsetpos(&mut self) -> OFFSETPOS_W<'_, OFRrs> {
        OFFSETPOS_W::new(self, 24)
    }
    ///Bit 25 - Saturation enable This bit is set and cleared by software to enable the saturation at 0x000 and 0xFFF for the offset function. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn saten(&mut self) -> SATEN_W<'_, OFRrs> {
        SATEN_W::new(self, 25)
    }
    ///Bits 26:30 - Channel selection for the data offset y These bits are written by software to define the channel to which the offset programmed into bits OFFSET\[11:0\] applies. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically and must not be selected for the data offset y. If OFFSET_EN is set, it is not allowed to select the same channel for different ADC_OFRy registers.
    #[inline(always)]
    pub fn offset_ch(&mut self) -> OFFSET_CH_W<'_, OFRrs> {
        OFFSET_CH_W::new(self, 26)
    }
    ///Bit 31 - Offset y enable This bit is written by software to enable or disable the offset programmed into bits OFFSET\[11:0\]. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn offset_en(&mut self) -> OFFSET_EN_W<'_, OFRrs> {
        OFFSET_EN_W::new(self, 31)
    }
}
/**ADC offset %s register

You can [`read`](crate::Reg::read) this register and get [`ofr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ofr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#ADC1:OFR[1])*/
pub struct OFRrs;
impl crate::RegisterSpec for OFRrs {
    type Ux = u32;
}
///`read()` method returns [`ofr::R`](R) reader structure
impl crate::Readable for OFRrs {}
///`write(|w| ..)` method takes [`ofr::W`](W) writer structure
impl crate::Writable for OFRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OFR%s to value 0
impl crate::Resettable for OFRrs {}
