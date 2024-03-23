#[doc = "Register `OFR4` reader"]
pub type R = crate::R<OFR4rs>;
#[doc = "Register `OFR4` writer"]
pub type W = crate::W<OFR4rs>;
#[doc = "Field `OFFSET4` reader - Data offset 4 for the channel programmed into bits OFFSET4_CH"]
pub type OFFSET4_R = crate::FieldReader<u16>;
#[doc = "Field `OFFSET4` writer - Data offset 4 for the channel programmed into bits OFFSET4_CH"]
pub type OFFSET4_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 12, u16>;
#[doc = "Field `OFFSET4_CH` reader - Channel selection for the Data offset 4"]
pub type OFFSET4_CH_R = crate::FieldReader;
#[doc = "Field `OFFSET4_CH` writer - Channel selection for the Data offset 4"]
pub type OFFSET4_CH_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
#[doc = "Offset 4 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFFSET4_EN {
    #[doc = "0: Offset disabled"]
    Disabled = 0,
    #[doc = "1: Offset enabled"]
    Enabled = 1,
}
impl From<OFFSET4_EN> for bool {
    #[inline(always)]
    fn from(variant: OFFSET4_EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OFFSET4_EN` reader - Offset 4 Enable"]
pub type OFFSET4_EN_R = crate::BitReader<OFFSET4_EN>;
impl OFFSET4_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OFFSET4_EN {
        match self.bits {
            false => OFFSET4_EN::Disabled,
            true => OFFSET4_EN::Enabled,
        }
    }
    #[doc = "Offset disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OFFSET4_EN::Disabled
    }
    #[doc = "Offset enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OFFSET4_EN::Enabled
    }
}
#[doc = "Field `OFFSET4_EN` writer - Offset 4 Enable"]
pub type OFFSET4_EN_W<'a, REG> = crate::BitWriter<'a, REG, OFFSET4_EN>;
impl<'a, REG> OFFSET4_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Offset disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OFFSET4_EN::Disabled)
    }
    #[doc = "Offset enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OFFSET4_EN::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:11 - Data offset 4 for the channel programmed into bits OFFSET4_CH"]
    #[inline(always)]
    pub fn offset4(&self) -> OFFSET4_R {
        OFFSET4_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 26:30 - Channel selection for the Data offset 4"]
    #[inline(always)]
    pub fn offset4_ch(&self) -> OFFSET4_CH_R {
        OFFSET4_CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Offset 4 Enable"]
    #[inline(always)]
    pub fn offset4_en(&self) -> OFFSET4_EN_R {
        OFFSET4_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Data offset 4 for the channel programmed into bits OFFSET4_CH"]
    #[inline(always)]
    #[must_use]
    pub fn offset4(&mut self) -> OFFSET4_W<OFR4rs> {
        OFFSET4_W::new(self, 0)
    }
    #[doc = "Bits 26:30 - Channel selection for the Data offset 4"]
    #[inline(always)]
    #[must_use]
    pub fn offset4_ch(&mut self) -> OFFSET4_CH_W<OFR4rs> {
        OFFSET4_CH_W::new(self, 26)
    }
    #[doc = "Bit 31 - Offset 4 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn offset4_en(&mut self) -> OFFSET4_EN_W<OFR4rs> {
        OFFSET4_EN_W::new(self, 31)
    }
}
#[doc = "ADC offset register4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ofr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ofr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OFR4rs;
impl crate::RegisterSpec for OFR4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ofr4::R`](R) reader structure"]
impl crate::Readable for OFR4rs {}
#[doc = "`write(|w| ..)` method takes [`ofr4::W`](W) writer structure"]
impl crate::Writable for OFR4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OFR4 to value 0"]
impl crate::Resettable for OFR4rs {
    const RESET_VALUE: u32 = 0;
}
