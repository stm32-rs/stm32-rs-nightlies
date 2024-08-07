///Register `OFR1` reader
pub type R = crate::R<OFR1rs>;
///Register `OFR1` writer
pub type W = crate::W<OFR1rs>;
///Field `OFFSET1` reader - Data offset 1 for the channel programmed into bits OFFSET1_CH
pub type OFFSET1_R = crate::FieldReader<u16>;
///Field `OFFSET1` writer - Data offset 1 for the channel programmed into bits OFFSET1_CH
pub type OFFSET1_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16, crate::Safe>;
///Field `OFFSETPOS` reader - Positive offset
pub type OFFSETPOS_R = crate::BitReader;
///Field `OFFSETPOS` writer - Positive offset
pub type OFFSETPOS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SATEN` reader - Saturation enable
pub type SATEN_R = crate::BitReader;
///Field `SATEN` writer - Saturation enable
pub type SATEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OFFSET1_CH` reader - Channel selection for the data offset 1
pub type OFFSET1_CH_R = crate::FieldReader;
///Field `OFFSET1_CH` writer - Channel selection for the data offset 1
pub type OFFSET1_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
/**Offset 1 Enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFFSET1_EN {
    ///0: Offset disabled
    Disabled = 0,
    ///1: Offset enabled
    Enabled = 1,
}
impl From<OFFSET1_EN> for bool {
    #[inline(always)]
    fn from(variant: OFFSET1_EN) -> Self {
        variant as u8 != 0
    }
}
///Field `OFFSET1_EN` reader - Offset 1 Enable
pub type OFFSET1_EN_R = crate::BitReader<OFFSET1_EN>;
impl OFFSET1_EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OFFSET1_EN {
        match self.bits {
            false => OFFSET1_EN::Disabled,
            true => OFFSET1_EN::Enabled,
        }
    }
    ///Offset disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OFFSET1_EN::Disabled
    }
    ///Offset enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OFFSET1_EN::Enabled
    }
}
///Field `OFFSET1_EN` writer - Offset 1 Enable
pub type OFFSET1_EN_W<'a, REG> = crate::BitWriter<'a, REG, OFFSET1_EN>;
impl<'a, REG> OFFSET1_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Offset disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OFFSET1_EN::Disabled)
    }
    ///Offset enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OFFSET1_EN::Enabled)
    }
}
impl R {
    ///Bits 0:11 - Data offset 1 for the channel programmed into bits OFFSET1_CH
    #[inline(always)]
    pub fn offset1(&self) -> OFFSET1_R {
        OFFSET1_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bit 24 - Positive offset
    #[inline(always)]
    pub fn offsetpos(&self) -> OFFSETPOS_R {
        OFFSETPOS_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Saturation enable
    #[inline(always)]
    pub fn saten(&self) -> SATEN_R {
        SATEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bits 26:30 - Channel selection for the data offset 1
    #[inline(always)]
    pub fn offset1_ch(&self) -> OFFSET1_CH_R {
        OFFSET1_CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    ///Bit 31 - Offset 1 Enable
    #[inline(always)]
    pub fn offset1_en(&self) -> OFFSET1_EN_R {
        OFFSET1_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OFR1")
            .field("offset1_en", &self.offset1_en())
            .field("offset1_ch", &self.offset1_ch())
            .field("saten", &self.saten())
            .field("offsetpos", &self.offsetpos())
            .field("offset1", &self.offset1())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - Data offset 1 for the channel programmed into bits OFFSET1_CH
    #[inline(always)]
    #[must_use]
    pub fn offset1(&mut self) -> OFFSET1_W<OFR1rs> {
        OFFSET1_W::new(self, 0)
    }
    ///Bit 24 - Positive offset
    #[inline(always)]
    #[must_use]
    pub fn offsetpos(&mut self) -> OFFSETPOS_W<OFR1rs> {
        OFFSETPOS_W::new(self, 24)
    }
    ///Bit 25 - Saturation enable
    #[inline(always)]
    #[must_use]
    pub fn saten(&mut self) -> SATEN_W<OFR1rs> {
        SATEN_W::new(self, 25)
    }
    ///Bits 26:30 - Channel selection for the data offset 1
    #[inline(always)]
    #[must_use]
    pub fn offset1_ch(&mut self) -> OFFSET1_CH_W<OFR1rs> {
        OFFSET1_CH_W::new(self, 26)
    }
    ///Bit 31 - Offset 1 Enable
    #[inline(always)]
    #[must_use]
    pub fn offset1_en(&mut self) -> OFFSET1_EN_W<OFR1rs> {
        OFFSET1_EN_W::new(self, 31)
    }
}
/**offset register 1

You can [`read`](crate::Reg::read) this register and get [`ofr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ofr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474xx.html#ADC1:OFR1)*/
pub struct OFR1rs;
impl crate::RegisterSpec for OFR1rs {
    type Ux = u32;
}
///`read()` method returns [`ofr1::R`](R) reader structure
impl crate::Readable for OFR1rs {}
///`write(|w| ..)` method takes [`ofr1::W`](W) writer structure
impl crate::Writable for OFR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OFR1 to value 0
impl crate::Resettable for OFR1rs {
    const RESET_VALUE: u32 = 0;
}
