#[doc = "Register `AHBRSTR` reader"]
pub type R = crate::R<AHBRSTRrs>;
#[doc = "Register `AHBRSTR` writer"]
pub type W = crate::W<AHBRSTRrs>;
#[doc = "I/O port A reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOPARST {
    #[doc = "1: Reset the selected module"]
    Reset = 1,
}
impl From<IOPARST> for bool {
    #[inline(always)]
    fn from(variant: IOPARST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IOPARST` reader - I/O port A reset"]
pub type IOPARST_R = crate::BitReader<IOPARST>;
impl IOPARST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<IOPARST> {
        match self.bits {
            true => Some(IOPARST::Reset),
            _ => None,
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == IOPARST::Reset
    }
}
#[doc = "Field `IOPARST` writer - I/O port A reset"]
pub type IOPARST_W<'a, REG> = crate::BitWriter<'a, REG, IOPARST>;
impl<'a, REG> IOPARST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(IOPARST::Reset)
    }
}
#[doc = "Field `IOPBRST` reader - I/O port B reset"]
pub use IOPARST_R as IOPBRST_R;
#[doc = "Field `IOPCRST` reader - I/O port C reset"]
pub use IOPARST_R as IOPCRST_R;
#[doc = "Field `IOPDRST` reader - I/O port D reset"]
pub use IOPARST_R as IOPDRST_R;
#[doc = "Field `IOPFRST` reader - I/O port F reset"]
pub use IOPARST_R as IOPFRST_R;
#[doc = "Field `TSCRST` reader - Touch sensing controller reset"]
pub use IOPARST_R as TSCRST_R;
#[doc = "Field `ADC1RST` reader - ADC1 reset"]
pub use IOPARST_R as ADC1RST_R;
#[doc = "Field `IOPBRST` writer - I/O port B reset"]
pub use IOPARST_W as IOPBRST_W;
#[doc = "Field `IOPCRST` writer - I/O port C reset"]
pub use IOPARST_W as IOPCRST_W;
#[doc = "Field `IOPDRST` writer - I/O port D reset"]
pub use IOPARST_W as IOPDRST_W;
#[doc = "Field `IOPFRST` writer - I/O port F reset"]
pub use IOPARST_W as IOPFRST_W;
#[doc = "Field `TSCRST` writer - Touch sensing controller reset"]
pub use IOPARST_W as TSCRST_W;
#[doc = "Field `ADC1RST` writer - ADC1 reset"]
pub use IOPARST_W as ADC1RST_W;
impl R {
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
    #[doc = "Bit 22 - I/O port F reset"]
    #[inline(always)]
    pub fn iopfrst(&self) -> IOPFRST_R {
        IOPFRST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Touch sensing controller reset"]
    #[inline(always)]
    pub fn tscrst(&self) -> TSCRST_R {
        TSCRST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - ADC1 reset"]
    #[inline(always)]
    pub fn adc1rst(&self) -> ADC1RST_R {
        ADC1RST_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
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
    #[doc = "Bit 22 - I/O port F reset"]
    #[inline(always)]
    #[must_use]
    pub fn iopfrst(&mut self) -> IOPFRST_W<AHBRSTRrs> {
        IOPFRST_W::new(self, 22)
    }
    #[doc = "Bit 24 - Touch sensing controller reset"]
    #[inline(always)]
    #[must_use]
    pub fn tscrst(&mut self) -> TSCRST_W<AHBRSTRrs> {
        TSCRST_W::new(self, 24)
    }
    #[doc = "Bit 28 - ADC1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn adc1rst(&mut self) -> ADC1RST_W<AHBRSTRrs> {
        ADC1RST_W::new(self, 28)
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
