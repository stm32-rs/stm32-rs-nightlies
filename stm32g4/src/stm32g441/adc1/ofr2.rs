///Register `OFR2` reader
pub type R = crate::R<OFR2rs>;
///Register `OFR2` writer
pub type W = crate::W<OFR2rs>;
///Field `OFFSET2` reader - Data offset 2 for the channel programmed into bits OFFSET2_CH
pub type OFFSET2_R = crate::FieldReader<u16>;
///Field `OFFSET2` writer - Data offset 2 for the channel programmed into bits OFFSET2_CH
pub type OFFSET2_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16, crate::Safe>;
///Field `OFFSETPOS` reader - Positive offset
pub type OFFSETPOS_R = crate::BitReader;
///Field `OFFSETPOS` writer - Positive offset
pub type OFFSETPOS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SATEN` reader - Saturation enable
pub type SATEN_R = crate::BitReader;
///Field `SATEN` writer - Saturation enable
pub type SATEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OFFSET2_CH` reader - Channel selection for the data offset 2
pub type OFFSET2_CH_R = crate::FieldReader;
///Field `OFFSET2_CH` writer - Channel selection for the data offset 2
pub type OFFSET2_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
/**Offset 2 Enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFFSET2_EN {
    ///0: Offset disabled
    Disabled = 0,
    ///1: Offset enabled
    Enabled = 1,
}
impl From<OFFSET2_EN> for bool {
    #[inline(always)]
    fn from(variant: OFFSET2_EN) -> Self {
        variant as u8 != 0
    }
}
///Field `OFFSET2_EN` reader - Offset 2 Enable
pub type OFFSET2_EN_R = crate::BitReader<OFFSET2_EN>;
impl OFFSET2_EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OFFSET2_EN {
        match self.bits {
            false => OFFSET2_EN::Disabled,
            true => OFFSET2_EN::Enabled,
        }
    }
    ///Offset disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OFFSET2_EN::Disabled
    }
    ///Offset enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OFFSET2_EN::Enabled
    }
}
///Field `OFFSET2_EN` writer - Offset 2 Enable
pub type OFFSET2_EN_W<'a, REG> = crate::BitWriter<'a, REG, OFFSET2_EN>;
impl<'a, REG> OFFSET2_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Offset disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OFFSET2_EN::Disabled)
    }
    ///Offset enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OFFSET2_EN::Enabled)
    }
}
impl R {
    ///Bits 0:11 - Data offset 2 for the channel programmed into bits OFFSET2_CH
    #[inline(always)]
    pub fn offset2(&self) -> OFFSET2_R {
        OFFSET2_R::new((self.bits & 0x0fff) as u16)
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
    ///Bits 26:30 - Channel selection for the data offset 2
    #[inline(always)]
    pub fn offset2_ch(&self) -> OFFSET2_CH_R {
        OFFSET2_CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    ///Bit 31 - Offset 2 Enable
    #[inline(always)]
    pub fn offset2_en(&self) -> OFFSET2_EN_R {
        OFFSET2_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OFR2")
            .field("offset2_en", &self.offset2_en())
            .field("offset2_ch", &self.offset2_ch())
            .field("saten", &self.saten())
            .field("offsetpos", &self.offsetpos())
            .field("offset2", &self.offset2())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - Data offset 2 for the channel programmed into bits OFFSET2_CH
    #[inline(always)]
    pub fn offset2(&mut self) -> OFFSET2_W<OFR2rs> {
        OFFSET2_W::new(self, 0)
    }
    ///Bit 24 - Positive offset
    #[inline(always)]
    pub fn offsetpos(&mut self) -> OFFSETPOS_W<OFR2rs> {
        OFFSETPOS_W::new(self, 24)
    }
    ///Bit 25 - Saturation enable
    #[inline(always)]
    pub fn saten(&mut self) -> SATEN_W<OFR2rs> {
        SATEN_W::new(self, 25)
    }
    ///Bits 26:30 - Channel selection for the data offset 2
    #[inline(always)]
    pub fn offset2_ch(&mut self) -> OFFSET2_CH_W<OFR2rs> {
        OFFSET2_CH_W::new(self, 26)
    }
    ///Bit 31 - Offset 2 Enable
    #[inline(always)]
    pub fn offset2_en(&mut self) -> OFFSET2_EN_W<OFR2rs> {
        OFFSET2_EN_W::new(self, 31)
    }
}
/**offset register 2

You can [`read`](crate::Reg::read) this register and get [`ofr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ofr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G441.html#ADC1:OFR2)*/
pub struct OFR2rs;
impl crate::RegisterSpec for OFR2rs {
    type Ux = u32;
}
///`read()` method returns [`ofr2::R`](R) reader structure
impl crate::Readable for OFR2rs {}
///`write(|w| ..)` method takes [`ofr2::W`](W) writer structure
impl crate::Writable for OFR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OFR2 to value 0
impl crate::Resettable for OFR2rs {
    const RESET_VALUE: u32 = 0;
}
