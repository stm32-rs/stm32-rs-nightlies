///Register `OFR3` reader
pub type R = crate::R<OFR3rs>;
///Register `OFR3` writer
pub type W = crate::W<OFR3rs>;
///Field `OFFSET3` reader - Data offset 3 for the channel programmed into bits OFFSET3_CH
pub type OFFSET3_R = crate::FieldReader<u16>;
///Field `OFFSET3` writer - Data offset 3 for the channel programmed into bits OFFSET3_CH
pub type OFFSET3_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16, crate::Safe>;
///Field `OFFSETPOS` reader - Positive offset
pub type OFFSETPOS_R = crate::BitReader;
///Field `OFFSETPOS` writer - Positive offset
pub type OFFSETPOS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SATEN` reader - Saturation enable
pub type SATEN_R = crate::BitReader;
///Field `SATEN` writer - Saturation enable
pub type SATEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OFFSET3_CH` reader - Channel selection for the data offset 3
pub type OFFSET3_CH_R = crate::FieldReader;
///Field `OFFSET3_CH` writer - Channel selection for the data offset 3
pub type OFFSET3_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
/**Offset 3 Enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFFSET3_EN {
    ///0: Offset disabled
    Disabled = 0,
    ///1: Offset enabled
    Enabled = 1,
}
impl From<OFFSET3_EN> for bool {
    #[inline(always)]
    fn from(variant: OFFSET3_EN) -> Self {
        variant as u8 != 0
    }
}
///Field `OFFSET3_EN` reader - Offset 3 Enable
pub type OFFSET3_EN_R = crate::BitReader<OFFSET3_EN>;
impl OFFSET3_EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OFFSET3_EN {
        match self.bits {
            false => OFFSET3_EN::Disabled,
            true => OFFSET3_EN::Enabled,
        }
    }
    ///Offset disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OFFSET3_EN::Disabled
    }
    ///Offset enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OFFSET3_EN::Enabled
    }
}
///Field `OFFSET3_EN` writer - Offset 3 Enable
pub type OFFSET3_EN_W<'a, REG> = crate::BitWriter<'a, REG, OFFSET3_EN>;
impl<'a, REG> OFFSET3_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Offset disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OFFSET3_EN::Disabled)
    }
    ///Offset enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OFFSET3_EN::Enabled)
    }
}
impl R {
    ///Bits 0:11 - Data offset 3 for the channel programmed into bits OFFSET3_CH
    #[inline(always)]
    pub fn offset3(&self) -> OFFSET3_R {
        OFFSET3_R::new((self.bits & 0x0fff) as u16)
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
    ///Bits 26:30 - Channel selection for the data offset 3
    #[inline(always)]
    pub fn offset3_ch(&self) -> OFFSET3_CH_R {
        OFFSET3_CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    ///Bit 31 - Offset 3 Enable
    #[inline(always)]
    pub fn offset3_en(&self) -> OFFSET3_EN_R {
        OFFSET3_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OFR3")
            .field("offset3_en", &self.offset3_en())
            .field("offset3_ch", &self.offset3_ch())
            .field("saten", &self.saten())
            .field("offsetpos", &self.offsetpos())
            .field("offset3", &self.offset3())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - Data offset 3 for the channel programmed into bits OFFSET3_CH
    #[inline(always)]
    pub fn offset3(&mut self) -> OFFSET3_W<OFR3rs> {
        OFFSET3_W::new(self, 0)
    }
    ///Bit 24 - Positive offset
    #[inline(always)]
    pub fn offsetpos(&mut self) -> OFFSETPOS_W<OFR3rs> {
        OFFSETPOS_W::new(self, 24)
    }
    ///Bit 25 - Saturation enable
    #[inline(always)]
    pub fn saten(&mut self) -> SATEN_W<OFR3rs> {
        SATEN_W::new(self, 25)
    }
    ///Bits 26:30 - Channel selection for the data offset 3
    #[inline(always)]
    pub fn offset3_ch(&mut self) -> OFFSET3_CH_W<OFR3rs> {
        OFFSET3_CH_W::new(self, 26)
    }
    ///Bit 31 - Offset 3 Enable
    #[inline(always)]
    pub fn offset3_en(&mut self) -> OFFSET3_EN_W<OFR3rs> {
        OFFSET3_EN_W::new(self, 31)
    }
}
/**offset register 3

You can [`read`](crate::Reg::read) this register and get [`ofr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ofr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G471xx.html#ADC3:OFR3)*/
pub struct OFR3rs;
impl crate::RegisterSpec for OFR3rs {
    type Ux = u32;
}
///`read()` method returns [`ofr3::R`](R) reader structure
impl crate::Readable for OFR3rs {}
///`write(|w| ..)` method takes [`ofr3::W`](W) writer structure
impl crate::Writable for OFR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OFR3 to value 0
impl crate::Resettable for OFR3rs {
    const RESET_VALUE: u32 = 0;
}
