#[doc = "Register `CICR` reader"]
pub type R = crate::R<CICRrs>;
#[doc = "Register `CICR` writer"]
pub type W = crate::W<CICRrs>;
#[doc = "LSI ready interrupt clear Set by software to clear LSIRDYF. Reset by hardware when clear done.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYC {
    #[doc = "1: Clear interrupt flag"]
    Clear = 1,
}
impl From<LSIRDYC> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSIRDYC` reader - LSI ready interrupt clear Set by software to clear LSIRDYF. Reset by hardware when clear done."]
pub type LSIRDYC_R = crate::BitReader<LSIRDYC>;
impl LSIRDYC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LSIRDYC> {
        match self.bits {
            true => Some(LSIRDYC::Clear),
            _ => None,
        }
    }
    #[doc = "Clear interrupt flag"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == LSIRDYC::Clear
    }
}
#[doc = "Field `LSIRDYC` writer - LSI ready interrupt clear Set by software to clear LSIRDYF. Reset by hardware when clear done."]
pub type LSIRDYC_W<'a, REG> = crate::BitWriter<'a, REG, LSIRDYC>;
impl<'a, REG> LSIRDYC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear interrupt flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(LSIRDYC::Clear)
    }
}
#[doc = "Field `LSERDYC` reader - LSE ready interrupt clear Set by software to clear LSERDYF. Reset by hardware when clear done."]
pub use LSIRDYC_R as LSERDYC_R;
#[doc = "Field `CSIRDYC` reader - HSI ready interrupt clear Set by software to clear CSIRDYF. Reset by hardware when clear done."]
pub use LSIRDYC_R as CSIRDYC_R;
#[doc = "Field `HSIRDYC` reader - HSI ready interrupt clear Set by software to clear HSIRDYF. Reset by hardware when clear done."]
pub use LSIRDYC_R as HSIRDYC_R;
#[doc = "Field `HSERDYC` reader - HSE ready interrupt clear Set by software to clear HSERDYF. Reset by hardware when clear done."]
pub use LSIRDYC_R as HSERDYC_R;
#[doc = "Field `HSI48RDYC` reader - HSI48 ready interrupt clear Set by software to clear HSI48RDYF. Reset by hardware when clear done."]
pub use LSIRDYC_R as HSI48RDYC_R;
#[doc = "Field `PLL1RDYC` reader - PLL1 ready interrupt clear Set by software to clear PLL1RDYF. Reset by hardware when clear done."]
pub use LSIRDYC_R as PLL1RDYC_R;
#[doc = "Field `PLL2RDYC` reader - PLL2 ready interrupt clear Set by software to clear PLL2RDYF. Reset by hardware when clear done."]
pub use LSIRDYC_R as PLL2RDYC_R;
#[doc = "Field `HSECSSC` reader - HSE clock security system interrupt clear Set by software to clear HSECSSF. Reset by hardware when clear done."]
pub use LSIRDYC_R as HSECSSC_R;
#[doc = "Field `LSERDYC` writer - LSE ready interrupt clear Set by software to clear LSERDYF. Reset by hardware when clear done."]
pub use LSIRDYC_W as LSERDYC_W;
#[doc = "Field `CSIRDYC` writer - HSI ready interrupt clear Set by software to clear CSIRDYF. Reset by hardware when clear done."]
pub use LSIRDYC_W as CSIRDYC_W;
#[doc = "Field `HSIRDYC` writer - HSI ready interrupt clear Set by software to clear HSIRDYF. Reset by hardware when clear done."]
pub use LSIRDYC_W as HSIRDYC_W;
#[doc = "Field `HSERDYC` writer - HSE ready interrupt clear Set by software to clear HSERDYF. Reset by hardware when clear done."]
pub use LSIRDYC_W as HSERDYC_W;
#[doc = "Field `HSI48RDYC` writer - HSI48 ready interrupt clear Set by software to clear HSI48RDYF. Reset by hardware when clear done."]
pub use LSIRDYC_W as HSI48RDYC_W;
#[doc = "Field `PLL1RDYC` writer - PLL1 ready interrupt clear Set by software to clear PLL1RDYF. Reset by hardware when clear done."]
pub use LSIRDYC_W as PLL1RDYC_W;
#[doc = "Field `PLL2RDYC` writer - PLL2 ready interrupt clear Set by software to clear PLL2RDYF. Reset by hardware when clear done."]
pub use LSIRDYC_W as PLL2RDYC_W;
#[doc = "Field `HSECSSC` writer - HSE clock security system interrupt clear Set by software to clear HSECSSF. Reset by hardware when clear done."]
pub use LSIRDYC_W as HSECSSC_W;
impl R {
    #[doc = "Bit 0 - LSI ready interrupt clear Set by software to clear LSIRDYF. Reset by hardware when clear done."]
    #[inline(always)]
    pub fn lsirdyc(&self) -> LSIRDYC_R {
        LSIRDYC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt clear Set by software to clear LSERDYF. Reset by hardware when clear done."]
    #[inline(always)]
    pub fn lserdyc(&self) -> LSERDYC_R {
        LSERDYC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSI ready interrupt clear Set by software to clear CSIRDYF. Reset by hardware when clear done."]
    #[inline(always)]
    pub fn csirdyc(&self) -> CSIRDYC_R {
        CSIRDYC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSI ready interrupt clear Set by software to clear HSIRDYF. Reset by hardware when clear done."]
    #[inline(always)]
    pub fn hsirdyc(&self) -> HSIRDYC_R {
        HSIRDYC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HSE ready interrupt clear Set by software to clear HSERDYF. Reset by hardware when clear done."]
    #[inline(always)]
    pub fn hserdyc(&self) -> HSERDYC_R {
        HSERDYC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HSI48 ready interrupt clear Set by software to clear HSI48RDYF. Reset by hardware when clear done."]
    #[inline(always)]
    pub fn hsi48rdyc(&self) -> HSI48RDYC_R {
        HSI48RDYC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PLL1 ready interrupt clear Set by software to clear PLL1RDYF. Reset by hardware when clear done."]
    #[inline(always)]
    pub fn pll1rdyc(&self) -> PLL1RDYC_R {
        PLL1RDYC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PLL2 ready interrupt clear Set by software to clear PLL2RDYF. Reset by hardware when clear done."]
    #[inline(always)]
    pub fn pll2rdyc(&self) -> PLL2RDYC_R {
        PLL2RDYC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - HSE clock security system interrupt clear Set by software to clear HSECSSF. Reset by hardware when clear done."]
    #[inline(always)]
    pub fn hsecssc(&self) -> HSECSSC_R {
        HSECSSC_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSI ready interrupt clear Set by software to clear LSIRDYF. Reset by hardware when clear done."]
    #[inline(always)]
    #[must_use]
    pub fn lsirdyc(&mut self) -> LSIRDYC_W<CICRrs> {
        LSIRDYC_W::new(self, 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt clear Set by software to clear LSERDYF. Reset by hardware when clear done."]
    #[inline(always)]
    #[must_use]
    pub fn lserdyc(&mut self) -> LSERDYC_W<CICRrs> {
        LSERDYC_W::new(self, 1)
    }
    #[doc = "Bit 2 - HSI ready interrupt clear Set by software to clear CSIRDYF. Reset by hardware when clear done."]
    #[inline(always)]
    #[must_use]
    pub fn csirdyc(&mut self) -> CSIRDYC_W<CICRrs> {
        CSIRDYC_W::new(self, 2)
    }
    #[doc = "Bit 3 - HSI ready interrupt clear Set by software to clear HSIRDYF. Reset by hardware when clear done."]
    #[inline(always)]
    #[must_use]
    pub fn hsirdyc(&mut self) -> HSIRDYC_W<CICRrs> {
        HSIRDYC_W::new(self, 3)
    }
    #[doc = "Bit 4 - HSE ready interrupt clear Set by software to clear HSERDYF. Reset by hardware when clear done."]
    #[inline(always)]
    #[must_use]
    pub fn hserdyc(&mut self) -> HSERDYC_W<CICRrs> {
        HSERDYC_W::new(self, 4)
    }
    #[doc = "Bit 5 - HSI48 ready interrupt clear Set by software to clear HSI48RDYF. Reset by hardware when clear done."]
    #[inline(always)]
    #[must_use]
    pub fn hsi48rdyc(&mut self) -> HSI48RDYC_W<CICRrs> {
        HSI48RDYC_W::new(self, 5)
    }
    #[doc = "Bit 6 - PLL1 ready interrupt clear Set by software to clear PLL1RDYF. Reset by hardware when clear done."]
    #[inline(always)]
    #[must_use]
    pub fn pll1rdyc(&mut self) -> PLL1RDYC_W<CICRrs> {
        PLL1RDYC_W::new(self, 6)
    }
    #[doc = "Bit 7 - PLL2 ready interrupt clear Set by software to clear PLL2RDYF. Reset by hardware when clear done."]
    #[inline(always)]
    #[must_use]
    pub fn pll2rdyc(&mut self) -> PLL2RDYC_W<CICRrs> {
        PLL2RDYC_W::new(self, 7)
    }
    #[doc = "Bit 10 - HSE clock security system interrupt clear Set by software to clear HSECSSF. Reset by hardware when clear done."]
    #[inline(always)]
    #[must_use]
    pub fn hsecssc(&mut self) -> HSECSSC_W<CICRrs> {
        HSECSSC_W::new(self, 10)
    }
}
#[doc = "RCC clock source interrupt clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cicr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cicr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CICRrs;
impl crate::RegisterSpec for CICRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cicr::R`](R) reader structure"]
impl crate::Readable for CICRrs {}
#[doc = "`write(|w| ..)` method takes [`cicr::W`](W) writer structure"]
impl crate::Writable for CICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CICR to value 0"]
impl crate::Resettable for CICRrs {
    const RESET_VALUE: u32 = 0;
}
