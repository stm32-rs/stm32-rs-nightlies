#[doc = "Register `CIER` reader"]
pub type R = crate::R<CIERrs>;
#[doc = "Register `CIER` writer"]
pub type W = crate::W<CIERrs>;
#[doc = "LSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the LSI oscillator stabilization.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYIE {
    #[doc = "0: Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt enabled"]
    Enabled = 1,
}
impl From<LSIRDYIE> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSIRDYIE` reader - LSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the LSI oscillator stabilization."]
pub type LSIRDYIE_R = crate::BitReader<LSIRDYIE>;
impl LSIRDYIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSIRDYIE {
        match self.bits {
            false => LSIRDYIE::Disabled,
            true => LSIRDYIE::Enabled,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSIRDYIE::Disabled
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSIRDYIE::Enabled
    }
}
#[doc = "Field `LSIRDYIE` writer - LSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the LSI oscillator stabilization."]
pub type LSIRDYIE_W<'a, REG> = crate::BitWriter<'a, REG, LSIRDYIE>;
impl<'a, REG> LSIRDYIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSIRDYIE::Disabled)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSIRDYIE::Enabled)
    }
}
#[doc = "Field `LSERDYIE` reader - LSE ready interrupt enable Set and reset by software to enable/disable interrupt caused by the LSE oscillator stabilization."]
pub use LSIRDYIE_R as LSERDYIE_R;
#[doc = "Field `CSIRDYIE` reader - CSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the CSI oscillator stabilization."]
pub use LSIRDYIE_R as CSIRDYIE_R;
#[doc = "Field `HSIRDYIE` reader - HSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSI oscillator stabilization."]
pub use LSIRDYIE_R as HSIRDYIE_R;
#[doc = "Field `HSERDYIE` reader - HSE ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSE oscillator stabilization."]
pub use LSIRDYIE_R as HSERDYIE_R;
#[doc = "Field `HSI48RDYIE` reader - HSI48 ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSI48 oscillator stabilization."]
pub use LSIRDYIE_R as HSI48RDYIE_R;
#[doc = "Field `PLL1RDYIE` reader - PLL1 ready interrupt enable Set and reset by software to enable/disable interrupt caused by PLL1 lock."]
pub use LSIRDYIE_R as PLL1RDYIE_R;
#[doc = "Field `PLL2RDYIE` reader - PLL2 ready interrupt enable Set and reset by software to enable/disable interrupt caused by PLL2 lock."]
pub use LSIRDYIE_R as PLL2RDYIE_R;
#[doc = "Field `LSERDYIE` writer - LSE ready interrupt enable Set and reset by software to enable/disable interrupt caused by the LSE oscillator stabilization."]
pub use LSIRDYIE_W as LSERDYIE_W;
#[doc = "Field `CSIRDYIE` writer - CSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the CSI oscillator stabilization."]
pub use LSIRDYIE_W as CSIRDYIE_W;
#[doc = "Field `HSIRDYIE` writer - HSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSI oscillator stabilization."]
pub use LSIRDYIE_W as HSIRDYIE_W;
#[doc = "Field `HSERDYIE` writer - HSE ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSE oscillator stabilization."]
pub use LSIRDYIE_W as HSERDYIE_W;
#[doc = "Field `HSI48RDYIE` writer - HSI48 ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSI48 oscillator stabilization."]
pub use LSIRDYIE_W as HSI48RDYIE_W;
#[doc = "Field `PLL1RDYIE` writer - PLL1 ready interrupt enable Set and reset by software to enable/disable interrupt caused by PLL1 lock."]
pub use LSIRDYIE_W as PLL1RDYIE_W;
#[doc = "Field `PLL2RDYIE` writer - PLL2 ready interrupt enable Set and reset by software to enable/disable interrupt caused by PLL2 lock."]
pub use LSIRDYIE_W as PLL2RDYIE_W;
impl R {
    #[doc = "Bit 0 - LSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the LSI oscillator stabilization."]
    #[inline(always)]
    pub fn lsirdyie(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt enable Set and reset by software to enable/disable interrupt caused by the LSE oscillator stabilization."]
    #[inline(always)]
    pub fn lserdyie(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the CSI oscillator stabilization."]
    #[inline(always)]
    pub fn csirdyie(&self) -> CSIRDYIE_R {
        CSIRDYIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSI oscillator stabilization."]
    #[inline(always)]
    pub fn hsirdyie(&self) -> HSIRDYIE_R {
        HSIRDYIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HSE ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSE oscillator stabilization."]
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HSI48 ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSI48 oscillator stabilization."]
    #[inline(always)]
    pub fn hsi48rdyie(&self) -> HSI48RDYIE_R {
        HSI48RDYIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PLL1 ready interrupt enable Set and reset by software to enable/disable interrupt caused by PLL1 lock."]
    #[inline(always)]
    pub fn pll1rdyie(&self) -> PLL1RDYIE_R {
        PLL1RDYIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PLL2 ready interrupt enable Set and reset by software to enable/disable interrupt caused by PLL2 lock."]
    #[inline(always)]
    pub fn pll2rdyie(&self) -> PLL2RDYIE_R {
        PLL2RDYIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the LSI oscillator stabilization."]
    #[inline(always)]
    #[must_use]
    pub fn lsirdyie(&mut self) -> LSIRDYIE_W<CIERrs> {
        LSIRDYIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt enable Set and reset by software to enable/disable interrupt caused by the LSE oscillator stabilization."]
    #[inline(always)]
    #[must_use]
    pub fn lserdyie(&mut self) -> LSERDYIE_W<CIERrs> {
        LSERDYIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - CSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the CSI oscillator stabilization."]
    #[inline(always)]
    #[must_use]
    pub fn csirdyie(&mut self) -> CSIRDYIE_W<CIERrs> {
        CSIRDYIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - HSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSI oscillator stabilization."]
    #[inline(always)]
    #[must_use]
    pub fn hsirdyie(&mut self) -> HSIRDYIE_W<CIERrs> {
        HSIRDYIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - HSE ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSE oscillator stabilization."]
    #[inline(always)]
    #[must_use]
    pub fn hserdyie(&mut self) -> HSERDYIE_W<CIERrs> {
        HSERDYIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - HSI48 ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSI48 oscillator stabilization."]
    #[inline(always)]
    #[must_use]
    pub fn hsi48rdyie(&mut self) -> HSI48RDYIE_W<CIERrs> {
        HSI48RDYIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - PLL1 ready interrupt enable Set and reset by software to enable/disable interrupt caused by PLL1 lock."]
    #[inline(always)]
    #[must_use]
    pub fn pll1rdyie(&mut self) -> PLL1RDYIE_W<CIERrs> {
        PLL1RDYIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - PLL2 ready interrupt enable Set and reset by software to enable/disable interrupt caused by PLL2 lock."]
    #[inline(always)]
    #[must_use]
    pub fn pll2rdyie(&mut self) -> PLL2RDYIE_W<CIERrs> {
        PLL2RDYIE_W::new(self, 7)
    }
}
#[doc = "RCC clock source interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIERrs;
impl crate::RegisterSpec for CIERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cier::R`](R) reader structure"]
impl crate::Readable for CIERrs {}
#[doc = "`write(|w| ..)` method takes [`cier::W`](W) writer structure"]
impl crate::Writable for CIERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CIER to value 0"]
impl crate::Resettable for CIERrs {
    const RESET_VALUE: u32 = 0;
}
