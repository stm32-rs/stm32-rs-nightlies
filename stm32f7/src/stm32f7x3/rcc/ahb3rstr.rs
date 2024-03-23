#[doc = "Register `AHB3RSTR` reader"]
pub type R = crate::R<AHB3RSTRrs>;
#[doc = "Register `AHB3RSTR` writer"]
pub type W = crate::W<AHB3RSTRrs>;
#[doc = "Flexible memory controller module reset\n\nValue on reset: 0"]
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
#[doc = "Field `FMCRST` reader - Flexible memory controller module reset"]
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
#[doc = "Field `FMCRST` writer - Flexible memory controller module reset"]
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
#[doc = "Field `QSPIRST` reader - Quad SPI memory controller reset"]
pub use FMCRST_R as QSPIRST_R;
#[doc = "Field `QSPIRST` writer - Quad SPI memory controller reset"]
pub use FMCRST_W as QSPIRST_W;
impl R {
    #[doc = "Bit 0 - Flexible memory controller module reset"]
    #[inline(always)]
    pub fn fmcrst(&self) -> FMCRST_R {
        FMCRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Quad SPI memory controller reset"]
    #[inline(always)]
    pub fn qspirst(&self) -> QSPIRST_R {
        QSPIRST_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flexible memory controller module reset"]
    #[inline(always)]
    #[must_use]
    pub fn fmcrst(&mut self) -> FMCRST_W<AHB3RSTRrs> {
        FMCRST_W::new(self, 0)
    }
    #[doc = "Bit 1 - Quad SPI memory controller reset"]
    #[inline(always)]
    #[must_use]
    pub fn qspirst(&mut self) -> QSPIRST_W<AHB3RSTRrs> {
        QSPIRST_W::new(self, 1)
    }
}
#[doc = "AHB3 peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb3rstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb3rstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB3RSTRrs;
impl crate::RegisterSpec for AHB3RSTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb3rstr::R`](R) reader structure"]
impl crate::Readable for AHB3RSTRrs {}
#[doc = "`write(|w| ..)` method takes [`ahb3rstr::W`](W) writer structure"]
impl crate::Writable for AHB3RSTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB3RSTR to value 0"]
impl crate::Resettable for AHB3RSTRrs {
    const RESET_VALUE: u32 = 0;
}
