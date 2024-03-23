#[doc = "Register `APB3RSTR` reader"]
pub type R = crate::R<APB3RSTRrs>;
#[doc = "Register `APB3RSTR` writer"]
pub type W = crate::W<APB3RSTRrs>;
#[doc = "SBS block reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBSRST {
    #[doc = "1: Reset the selected module"]
    Reset = 1,
}
impl From<SBSRST> for bool {
    #[inline(always)]
    fn from(variant: SBSRST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBSRST` reader - SBS block reset Set and reset by software."]
pub type SBSRST_R = crate::BitReader<SBSRST>;
impl SBSRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SBSRST> {
        match self.bits {
            true => Some(SBSRST::Reset),
            _ => None,
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SBSRST::Reset
    }
}
#[doc = "Field `SBSRST` writer - SBS block reset Set and reset by software."]
pub type SBSRST_W<'a, REG> = crate::BitWriter<'a, REG, SBSRST>;
impl<'a, REG> SBSRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(SBSRST::Reset)
    }
}
#[doc = "Field `LPUART1RST` reader - LPUART1 block reset Set and reset by software."]
pub use SBSRST_R as LPUART1RST_R;
#[doc = "Field `I3C2RST` reader - I3C2RST block reset Set and reset by software."]
pub use SBSRST_R as I3C2RST_R;
#[doc = "Field `LPTIM1RST` reader - LPTIM1 block reset Set and reset by software."]
pub use SBSRST_R as LPTIM1RST_R;
#[doc = "Field `VREFRST` reader - VREF block reset Set and reset by software."]
pub use SBSRST_R as VREFRST_R;
#[doc = "Field `LPUART1RST` writer - LPUART1 block reset Set and reset by software."]
pub use SBSRST_W as LPUART1RST_W;
#[doc = "Field `I3C2RST` writer - I3C2RST block reset Set and reset by software."]
pub use SBSRST_W as I3C2RST_W;
#[doc = "Field `LPTIM1RST` writer - LPTIM1 block reset Set and reset by software."]
pub use SBSRST_W as LPTIM1RST_W;
#[doc = "Field `VREFRST` writer - VREF block reset Set and reset by software."]
pub use SBSRST_W as VREFRST_W;
impl R {
    #[doc = "Bit 1 - SBS block reset Set and reset by software."]
    #[inline(always)]
    pub fn sbsrst(&self) -> SBSRST_R {
        SBSRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - LPUART1 block reset Set and reset by software."]
    #[inline(always)]
    pub fn lpuart1rst(&self) -> LPUART1RST_R {
        LPUART1RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - I3C2RST block reset Set and reset by software."]
    #[inline(always)]
    pub fn i3c2rst(&self) -> I3C2RST_R {
        I3C2RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - LPTIM1 block reset Set and reset by software."]
    #[inline(always)]
    pub fn lptim1rst(&self) -> LPTIM1RST_R {
        LPTIM1RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 20 - VREF block reset Set and reset by software."]
    #[inline(always)]
    pub fn vrefrst(&self) -> VREFRST_R {
        VREFRST_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SBS block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn sbsrst(&mut self) -> SBSRST_W<APB3RSTRrs> {
        SBSRST_W::new(self, 1)
    }
    #[doc = "Bit 6 - LPUART1 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1rst(&mut self) -> LPUART1RST_W<APB3RSTRrs> {
        LPUART1RST_W::new(self, 6)
    }
    #[doc = "Bit 9 - I3C2RST block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn i3c2rst(&mut self) -> I3C2RST_W<APB3RSTRrs> {
        I3C2RST_W::new(self, 9)
    }
    #[doc = "Bit 11 - LPTIM1 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim1rst(&mut self) -> LPTIM1RST_W<APB3RSTRrs> {
        LPTIM1RST_W::new(self, 11)
    }
    #[doc = "Bit 20 - VREF block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn vrefrst(&mut self) -> VREFRST_W<APB3RSTRrs> {
        VREFRST_W::new(self, 20)
    }
}
#[doc = "RCC APB3 peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb3rstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb3rstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB3RSTRrs;
impl crate::RegisterSpec for APB3RSTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb3rstr::R`](R) reader structure"]
impl crate::Readable for APB3RSTRrs {}
#[doc = "`write(|w| ..)` method takes [`apb3rstr::W`](W) writer structure"]
impl crate::Writable for APB3RSTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB3RSTR to value 0"]
impl crate::Resettable for APB3RSTRrs {
    const RESET_VALUE: u32 = 0;
}
