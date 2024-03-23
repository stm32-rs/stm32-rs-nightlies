#[doc = "Register `AHB2RSTR` reader"]
pub type R = crate::R<AHB2RSTRrs>;
#[doc = "Register `AHB2RSTR` writer"]
pub type W = crate::W<AHB2RSTRrs>;
#[doc = "Camera interface reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCMIRST {
    #[doc = "1: Reset the selected module"]
    Reset = 1,
}
impl From<DCMIRST> for bool {
    #[inline(always)]
    fn from(variant: DCMIRST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCMIRST` reader - Camera interface reset"]
pub type DCMIRST_R = crate::BitReader<DCMIRST>;
impl DCMIRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DCMIRST> {
        match self.bits {
            true => Some(DCMIRST::Reset),
            _ => None,
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == DCMIRST::Reset
    }
}
#[doc = "Field `DCMIRST` writer - Camera interface reset"]
pub type DCMIRST_W<'a, REG> = crate::BitWriter<'a, REG, DCMIRST>;
impl<'a, REG> DCMIRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(DCMIRST::Reset)
    }
}
#[doc = "Field `OTGFSRST` reader - USB OTG FS module reset"]
pub use DCMIRST_R as OTGFSRST_R;
#[doc = "Field `OTGFSRST` writer - USB OTG FS module reset"]
pub use DCMIRST_W as OTGFSRST_W;
impl R {
    #[doc = "Bit 0 - Camera interface reset"]
    #[inline(always)]
    pub fn dcmirst(&self) -> DCMIRST_R {
        DCMIRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - USB OTG FS module reset"]
    #[inline(always)]
    pub fn otgfsrst(&self) -> OTGFSRST_R {
        OTGFSRST_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Camera interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn dcmirst(&mut self) -> DCMIRST_W<AHB2RSTRrs> {
        DCMIRST_W::new(self, 0)
    }
    #[doc = "Bit 7 - USB OTG FS module reset"]
    #[inline(always)]
    #[must_use]
    pub fn otgfsrst(&mut self) -> OTGFSRST_W<AHB2RSTRrs> {
        OTGFSRST_W::new(self, 7)
    }
}
#[doc = "AHB2 peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb2rstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb2rstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB2RSTRrs;
impl crate::RegisterSpec for AHB2RSTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb2rstr::R`](R) reader structure"]
impl crate::Readable for AHB2RSTRrs {}
#[doc = "`write(|w| ..)` method takes [`ahb2rstr::W`](W) writer structure"]
impl crate::Writable for AHB2RSTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB2RSTR to value 0"]
impl crate::Resettable for AHB2RSTRrs {
    const RESET_VALUE: u32 = 0;
}
