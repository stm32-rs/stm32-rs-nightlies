#[doc = "Register `AHB2RSTR` reader"]
pub type R = crate::R<AHB2RSTRrs>;
#[doc = "Register `AHB2RSTR` writer"]
pub type W = crate::W<AHB2RSTRrs>;
#[doc = "AES module reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AESRST {
    #[doc = "1: Reset the selected module"]
    Reset = 1,
}
impl From<AESRST> for bool {
    #[inline(always)]
    fn from(variant: AESRST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AESRST` reader - AES module reset"]
pub type AESRST_R = crate::BitReader<AESRST>;
impl AESRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<AESRST> {
        match self.bits {
            true => Some(AESRST::Reset),
            _ => None,
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == AESRST::Reset
    }
}
#[doc = "Field `AESRST` writer - AES module reset"]
pub type AESRST_W<'a, REG> = crate::BitWriter<'a, REG, AESRST>;
impl<'a, REG> AESRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(AESRST::Reset)
    }
}
#[doc = "Field `RNGRST` reader - Random number generator module reset"]
pub use AESRST_R as RNGRST_R;
#[doc = "Field `OTGFSRST` reader - USB OTG FS module reset"]
pub use AESRST_R as OTGFSRST_R;
#[doc = "Field `RNGRST` writer - Random number generator module reset"]
pub use AESRST_W as RNGRST_W;
#[doc = "Field `OTGFSRST` writer - USB OTG FS module reset"]
pub use AESRST_W as OTGFSRST_W;
impl R {
    #[doc = "Bit 4 - AES module reset"]
    #[inline(always)]
    pub fn aesrst(&self) -> AESRST_R {
        AESRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Random number generator module reset"]
    #[inline(always)]
    pub fn rngrst(&self) -> RNGRST_R {
        RNGRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USB OTG FS module reset"]
    #[inline(always)]
    pub fn otgfsrst(&self) -> OTGFSRST_R {
        OTGFSRST_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - AES module reset"]
    #[inline(always)]
    #[must_use]
    pub fn aesrst(&mut self) -> AESRST_W<AHB2RSTRrs> {
        AESRST_W::new(self, 4)
    }
    #[doc = "Bit 6 - Random number generator module reset"]
    #[inline(always)]
    #[must_use]
    pub fn rngrst(&mut self) -> RNGRST_W<AHB2RSTRrs> {
        RNGRST_W::new(self, 6)
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
