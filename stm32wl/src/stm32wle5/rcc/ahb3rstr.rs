#[doc = "Register `AHB3RSTR` reader"]
pub type R = crate::R<AHB3RSTRrs>;
#[doc = "Register `AHB3RSTR` writer"]
pub type W = crate::W<AHB3RSTRrs>;
#[doc = "PKARST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PKARST {
    #[doc = "0: No effect"]
    NoReset = 0,
    #[doc = "1: Reset peripheral"]
    Reset = 1,
}
impl From<PKARST> for bool {
    #[inline(always)]
    fn from(variant: PKARST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PKARST` reader - PKARST"]
pub type PKARST_R = crate::BitReader<PKARST>;
impl PKARST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PKARST {
        match self.bits {
            false => PKARST::NoReset,
            true => PKARST::Reset,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == PKARST::NoReset
    }
    #[doc = "Reset peripheral"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == PKARST::Reset
    }
}
#[doc = "Field `PKARST` writer - PKARST"]
pub type PKARST_W<'a, REG> = crate::BitWriter<'a, REG, PKARST>;
impl<'a, REG> PKARST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut crate::W<REG> {
        self.variant(PKARST::NoReset)
    }
    #[doc = "Reset peripheral"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(PKARST::Reset)
    }
}
#[doc = "Field `AESRST` reader - AESRST"]
pub use PKARST_R as AESRST_R;
#[doc = "Field `RNGRST` reader - RNGRST"]
pub use PKARST_R as RNGRST_R;
#[doc = "Field `HSEMRST` reader - HSEMRST"]
pub use PKARST_R as HSEMRST_R;
#[doc = "Field `FLASHRST` reader - Flash interface reset"]
pub use PKARST_R as FLASHRST_R;
#[doc = "Field `AESRST` writer - AESRST"]
pub use PKARST_W as AESRST_W;
#[doc = "Field `RNGRST` writer - RNGRST"]
pub use PKARST_W as RNGRST_W;
#[doc = "Field `HSEMRST` writer - HSEMRST"]
pub use PKARST_W as HSEMRST_W;
#[doc = "Field `FLASHRST` writer - Flash interface reset"]
pub use PKARST_W as FLASHRST_W;
impl R {
    #[doc = "Bit 16 - PKARST"]
    #[inline(always)]
    pub fn pkarst(&self) -> PKARST_R {
        PKARST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - AESRST"]
    #[inline(always)]
    pub fn aesrst(&self) -> AESRST_R {
        AESRST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - RNGRST"]
    #[inline(always)]
    pub fn rngrst(&self) -> RNGRST_R {
        RNGRST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - HSEMRST"]
    #[inline(always)]
    pub fn hsemrst(&self) -> HSEMRST_R {
        HSEMRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 25 - Flash interface reset"]
    #[inline(always)]
    pub fn flashrst(&self) -> FLASHRST_R {
        FLASHRST_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - PKARST"]
    #[inline(always)]
    #[must_use]
    pub fn pkarst(&mut self) -> PKARST_W<AHB3RSTRrs> {
        PKARST_W::new(self, 16)
    }
    #[doc = "Bit 17 - AESRST"]
    #[inline(always)]
    #[must_use]
    pub fn aesrst(&mut self) -> AESRST_W<AHB3RSTRrs> {
        AESRST_W::new(self, 17)
    }
    #[doc = "Bit 18 - RNGRST"]
    #[inline(always)]
    #[must_use]
    pub fn rngrst(&mut self) -> RNGRST_W<AHB3RSTRrs> {
        RNGRST_W::new(self, 18)
    }
    #[doc = "Bit 19 - HSEMRST"]
    #[inline(always)]
    #[must_use]
    pub fn hsemrst(&mut self) -> HSEMRST_W<AHB3RSTRrs> {
        HSEMRST_W::new(self, 19)
    }
    #[doc = "Bit 25 - Flash interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn flashrst(&mut self) -> FLASHRST_W<AHB3RSTRrs> {
        FLASHRST_W::new(self, 25)
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
