#[doc = "Register `OFR2` reader"]
pub type R = crate::R<OFR2rs>;
#[doc = "Register `OFR2` writer"]
pub type W = crate::W<OFR2rs>;
#[doc = "Field `OFFSET2` reader - Data offset 2 for the channel programmed into bits OFFSET2_CH"]
pub type OFFSET2_R = crate::FieldReader<u16>;
#[doc = "Field `OFFSET2` writer - Data offset 2 for the channel programmed into bits OFFSET2_CH"]
pub type OFFSET2_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 12, u16>;
#[doc = "Field `OFFSETPOS` reader - Positive offset"]
pub type OFFSETPOS_R = crate::BitReader;
#[doc = "Field `OFFSETPOS` writer - Positive offset"]
pub type OFFSETPOS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SATEN` reader - Saturation enable"]
pub type SATEN_R = crate::BitReader;
#[doc = "Field `SATEN` writer - Saturation enable"]
pub type SATEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OFFSET2_CH` reader - Channel selection for the data offset 2"]
pub type OFFSET2_CH_R = crate::FieldReader;
#[doc = "Field `OFFSET2_CH` writer - Channel selection for the data offset 2"]
pub type OFFSET2_CH_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
#[doc = "Offset 2 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFFSET2_EN {
    #[doc = "0: Offset disabled"]
    Disabled = 0,
    #[doc = "1: Offset enabled"]
    Enabled = 1,
}
impl From<OFFSET2_EN> for bool {
    #[inline(always)]
    fn from(variant: OFFSET2_EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OFFSET2_EN` reader - Offset 2 Enable"]
pub type OFFSET2_EN_R = crate::BitReader<OFFSET2_EN>;
impl OFFSET2_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OFFSET2_EN {
        match self.bits {
            false => OFFSET2_EN::Disabled,
            true => OFFSET2_EN::Enabled,
        }
    }
    #[doc = "Offset disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OFFSET2_EN::Disabled
    }
    #[doc = "Offset enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OFFSET2_EN::Enabled
    }
}
#[doc = "Field `OFFSET2_EN` writer - Offset 2 Enable"]
pub type OFFSET2_EN_W<'a, REG> = crate::BitWriter<'a, REG, OFFSET2_EN>;
impl<'a, REG> OFFSET2_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Offset disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OFFSET2_EN::Disabled)
    }
    #[doc = "Offset enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OFFSET2_EN::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:11 - Data offset 2 for the channel programmed into bits OFFSET2_CH"]
    #[inline(always)]
    pub fn offset2(&self) -> OFFSET2_R {
        OFFSET2_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 24 - Positive offset"]
    #[inline(always)]
    pub fn offsetpos(&self) -> OFFSETPOS_R {
        OFFSETPOS_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Saturation enable"]
    #[inline(always)]
    pub fn saten(&self) -> SATEN_R {
        SATEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:30 - Channel selection for the data offset 2"]
    #[inline(always)]
    pub fn offset2_ch(&self) -> OFFSET2_CH_R {
        OFFSET2_CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Offset 2 Enable"]
    #[inline(always)]
    pub fn offset2_en(&self) -> OFFSET2_EN_R {
        OFFSET2_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Data offset 2 for the channel programmed into bits OFFSET2_CH"]
    #[inline(always)]
    #[must_use]
    pub fn offset2(&mut self) -> OFFSET2_W<OFR2rs> {
        OFFSET2_W::new(self, 0)
    }
    #[doc = "Bit 24 - Positive offset"]
    #[inline(always)]
    #[must_use]
    pub fn offsetpos(&mut self) -> OFFSETPOS_W<OFR2rs> {
        OFFSETPOS_W::new(self, 24)
    }
    #[doc = "Bit 25 - Saturation enable"]
    #[inline(always)]
    #[must_use]
    pub fn saten(&mut self) -> SATEN_W<OFR2rs> {
        SATEN_W::new(self, 25)
    }
    #[doc = "Bits 26:30 - Channel selection for the data offset 2"]
    #[inline(always)]
    #[must_use]
    pub fn offset2_ch(&mut self) -> OFFSET2_CH_W<OFR2rs> {
        OFFSET2_CH_W::new(self, 26)
    }
    #[doc = "Bit 31 - Offset 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn offset2_en(&mut self) -> OFFSET2_EN_W<OFR2rs> {
        OFFSET2_EN_W::new(self, 31)
    }
}
#[doc = "offset register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ofr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ofr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OFR2rs;
impl crate::RegisterSpec for OFR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ofr2::R`](R) reader structure"]
impl crate::Readable for OFR2rs {}
#[doc = "`write(|w| ..)` method takes [`ofr2::W`](W) writer structure"]
impl crate::Writable for OFR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OFR2 to value 0"]
impl crate::Resettable for OFR2rs {
    const RESET_VALUE: u32 = 0;
}
