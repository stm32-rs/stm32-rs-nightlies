#[doc = "Register `AHBRSTR` reader"]
pub type R = crate::R<AHBRSTRrs>;
#[doc = "Register `AHBRSTR` writer"]
pub type W = crate::W<AHBRSTRrs>;
#[doc = "FMC reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMCRST {
    #[doc = "1: Reset the selected module"]
    Reset = 1,
}
impl From<FMCRST> for bool {
    #[inline(always)]
    fn from(variant: FMCRST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FMCRST` reader - FMC reset"]
pub type FMCRST_R = crate::BitReader<FMCRST>;
impl FMCRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<FMCRST> {
        match self.bits {
            true => Some(FMCRST::Reset),
            _ => None,
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == FMCRST::Reset
    }
}
#[doc = "Field `FMCRST` writer - FMC reset"]
pub type FMCRST_W<'a, REG> = crate::BitWriter<'a, REG, FMCRST>;
impl<'a, REG> FMCRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(FMCRST::Reset)
    }
}
#[doc = "Field `IOPHRST` reader - I/O port H reset"]
pub use FMCRST_R as IOPHRST_R;
#[doc = "Field `IOPARST` reader - I/O port A reset"]
pub use FMCRST_R as IOPARST_R;
#[doc = "Field `IOPBRST` reader - I/O port B reset"]
pub use FMCRST_R as IOPBRST_R;
#[doc = "Field `IOPCRST` reader - I/O port C reset"]
pub use FMCRST_R as IOPCRST_R;
#[doc = "Field `IOPDRST` reader - I/O port D reset"]
pub use FMCRST_R as IOPDRST_R;
#[doc = "Field `IOPERST` reader - I/O port E reset"]
pub use FMCRST_R as IOPERST_R;
#[doc = "Field `IOPFRST` reader - I/O port F reset"]
pub use FMCRST_R as IOPFRST_R;
#[doc = "Field `IOPGRST` reader - Touch sensing controller reset"]
pub use FMCRST_R as IOPGRST_R;
#[doc = "Field `TSCRST` reader - Touch sensing controller reset"]
pub use FMCRST_R as TSCRST_R;
#[doc = "Field `ADC12RST` reader - ADC1 and ADC2 reset"]
pub use FMCRST_R as ADC12RST_R;
#[doc = "Field `ADC34RST` reader - ADC3 and ADC4 reset"]
pub use FMCRST_R as ADC34RST_R;
#[doc = "Field `IOPHRST` writer - I/O port H reset"]
pub use FMCRST_W as IOPHRST_W;
#[doc = "Field `IOPARST` writer - I/O port A reset"]
pub use FMCRST_W as IOPARST_W;
#[doc = "Field `IOPBRST` writer - I/O port B reset"]
pub use FMCRST_W as IOPBRST_W;
#[doc = "Field `IOPCRST` writer - I/O port C reset"]
pub use FMCRST_W as IOPCRST_W;
#[doc = "Field `IOPDRST` writer - I/O port D reset"]
pub use FMCRST_W as IOPDRST_W;
#[doc = "Field `IOPERST` writer - I/O port E reset"]
pub use FMCRST_W as IOPERST_W;
#[doc = "Field `IOPFRST` writer - I/O port F reset"]
pub use FMCRST_W as IOPFRST_W;
#[doc = "Field `IOPGRST` writer - Touch sensing controller reset"]
pub use FMCRST_W as IOPGRST_W;
#[doc = "Field `TSCRST` writer - Touch sensing controller reset"]
pub use FMCRST_W as TSCRST_W;
#[doc = "Field `ADC12RST` writer - ADC1 and ADC2 reset"]
pub use FMCRST_W as ADC12RST_W;
#[doc = "Field `ADC34RST` writer - ADC3 and ADC4 reset"]
pub use FMCRST_W as ADC34RST_W;
impl R {
    #[doc = "Bit 5 - FMC reset"]
    #[inline(always)]
    pub fn fmcrst(&self) -> FMCRST_R {
        FMCRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - I/O port H reset"]
    #[inline(always)]
    pub fn iophrst(&self) -> IOPHRST_R {
        IOPHRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - I/O port A reset"]
    #[inline(always)]
    pub fn ioparst(&self) -> IOPARST_R {
        IOPARST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - I/O port B reset"]
    #[inline(always)]
    pub fn iopbrst(&self) -> IOPBRST_R {
        IOPBRST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - I/O port C reset"]
    #[inline(always)]
    pub fn iopcrst(&self) -> IOPCRST_R {
        IOPCRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - I/O port D reset"]
    #[inline(always)]
    pub fn iopdrst(&self) -> IOPDRST_R {
        IOPDRST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I/O port E reset"]
    #[inline(always)]
    pub fn ioperst(&self) -> IOPERST_R {
        IOPERST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I/O port F reset"]
    #[inline(always)]
    pub fn iopfrst(&self) -> IOPFRST_R {
        IOPFRST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Touch sensing controller reset"]
    #[inline(always)]
    pub fn iopgrst(&self) -> IOPGRST_R {
        IOPGRST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Touch sensing controller reset"]
    #[inline(always)]
    pub fn tscrst(&self) -> TSCRST_R {
        TSCRST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - ADC1 and ADC2 reset"]
    #[inline(always)]
    pub fn adc12rst(&self) -> ADC12RST_R {
        ADC12RST_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - ADC3 and ADC4 reset"]
    #[inline(always)]
    pub fn adc34rst(&self) -> ADC34RST_R {
        ADC34RST_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - FMC reset"]
    #[inline(always)]
    #[must_use]
    pub fn fmcrst(&mut self) -> FMCRST_W<AHBRSTRrs> {
        FMCRST_W::new(self, 5)
    }
    #[doc = "Bit 16 - I/O port H reset"]
    #[inline(always)]
    #[must_use]
    pub fn iophrst(&mut self) -> IOPHRST_W<AHBRSTRrs> {
        IOPHRST_W::new(self, 16)
    }
    #[doc = "Bit 17 - I/O port A reset"]
    #[inline(always)]
    #[must_use]
    pub fn ioparst(&mut self) -> IOPARST_W<AHBRSTRrs> {
        IOPARST_W::new(self, 17)
    }
    #[doc = "Bit 18 - I/O port B reset"]
    #[inline(always)]
    #[must_use]
    pub fn iopbrst(&mut self) -> IOPBRST_W<AHBRSTRrs> {
        IOPBRST_W::new(self, 18)
    }
    #[doc = "Bit 19 - I/O port C reset"]
    #[inline(always)]
    #[must_use]
    pub fn iopcrst(&mut self) -> IOPCRST_W<AHBRSTRrs> {
        IOPCRST_W::new(self, 19)
    }
    #[doc = "Bit 20 - I/O port D reset"]
    #[inline(always)]
    #[must_use]
    pub fn iopdrst(&mut self) -> IOPDRST_W<AHBRSTRrs> {
        IOPDRST_W::new(self, 20)
    }
    #[doc = "Bit 21 - I/O port E reset"]
    #[inline(always)]
    #[must_use]
    pub fn ioperst(&mut self) -> IOPERST_W<AHBRSTRrs> {
        IOPERST_W::new(self, 21)
    }
    #[doc = "Bit 22 - I/O port F reset"]
    #[inline(always)]
    #[must_use]
    pub fn iopfrst(&mut self) -> IOPFRST_W<AHBRSTRrs> {
        IOPFRST_W::new(self, 22)
    }
    #[doc = "Bit 23 - Touch sensing controller reset"]
    #[inline(always)]
    #[must_use]
    pub fn iopgrst(&mut self) -> IOPGRST_W<AHBRSTRrs> {
        IOPGRST_W::new(self, 23)
    }
    #[doc = "Bit 24 - Touch sensing controller reset"]
    #[inline(always)]
    #[must_use]
    pub fn tscrst(&mut self) -> TSCRST_W<AHBRSTRrs> {
        TSCRST_W::new(self, 24)
    }
    #[doc = "Bit 28 - ADC1 and ADC2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn adc12rst(&mut self) -> ADC12RST_W<AHBRSTRrs> {
        ADC12RST_W::new(self, 28)
    }
    #[doc = "Bit 29 - ADC3 and ADC4 reset"]
    #[inline(always)]
    #[must_use]
    pub fn adc34rst(&mut self) -> ADC34RST_W<AHBRSTRrs> {
        ADC34RST_W::new(self, 29)
    }
}
#[doc = "AHB peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbrstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbrstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBRSTRrs;
impl crate::RegisterSpec for AHBRSTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbrstr::R`](R) reader structure"]
impl crate::Readable for AHBRSTRrs {}
#[doc = "`write(|w| ..)` method takes [`ahbrstr::W`](W) writer structure"]
impl crate::Writable for AHBRSTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBRSTR to value 0"]
impl crate::Resettable for AHBRSTRrs {
    const RESET_VALUE: u32 = 0;
}
