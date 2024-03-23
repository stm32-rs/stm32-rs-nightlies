#[doc = "Register `CICR` writer"]
pub type W = crate::W<CICRrs>;
#[doc = "LSI ready Interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYCW {
    #[doc = "1: Clear interrupt flag"]
    Clear = 1,
}
impl From<LSIRDYCW> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYCW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSIRDYC` writer - LSI ready Interrupt clear"]
pub type LSIRDYC_W<'a, REG> = crate::BitWriter<'a, REG, LSIRDYCW>;
impl<'a, REG> LSIRDYC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear interrupt flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(LSIRDYCW::Clear)
    }
}
#[doc = "Field `LSERDYC` writer - LSE ready Interrupt clear"]
pub use LSIRDYC_W as LSERDYC_W;
#[doc = "Field `HSI16RDYC` writer - HSI16 ready Interrupt clear"]
pub use LSIRDYC_W as HSI16RDYC_W;
#[doc = "Field `HSERDYC` writer - HSE ready Interrupt clear"]
pub use LSIRDYC_W as HSERDYC_W;
#[doc = "Field `PLLRDYC` writer - PLL ready Interrupt clear"]
pub use LSIRDYC_W as PLLRDYC_W;
#[doc = "Field `MSIRDYC` writer - MSI ready Interrupt clear"]
pub use LSIRDYC_W as MSIRDYC_W;
#[doc = "Field `HSI48RDYC` writer - HSI48 ready Interrupt clear"]
pub use LSIRDYC_W as HSI48RDYC_W;
#[doc = "Field `CSSLSEC` writer - LSE Clock Security System Interrupt clear"]
pub use LSIRDYC_W as CSSLSEC_W;
#[doc = "Field `CSSHSEC` writer - Clock Security System Interrupt clear"]
pub use LSIRDYC_W as CSSHSEC_W;
impl W {
    #[doc = "Bit 0 - LSI ready Interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn lsirdyc(&mut self) -> LSIRDYC_W<CICRrs> {
        LSIRDYC_W::new(self, 0)
    }
    #[doc = "Bit 1 - LSE ready Interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn lserdyc(&mut self) -> LSERDYC_W<CICRrs> {
        LSERDYC_W::new(self, 1)
    }
    #[doc = "Bit 2 - HSI16 ready Interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn hsi16rdyc(&mut self) -> HSI16RDYC_W<CICRrs> {
        HSI16RDYC_W::new(self, 2)
    }
    #[doc = "Bit 3 - HSE ready Interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn hserdyc(&mut self) -> HSERDYC_W<CICRrs> {
        HSERDYC_W::new(self, 3)
    }
    #[doc = "Bit 4 - PLL ready Interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn pllrdyc(&mut self) -> PLLRDYC_W<CICRrs> {
        PLLRDYC_W::new(self, 4)
    }
    #[doc = "Bit 5 - MSI ready Interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn msirdyc(&mut self) -> MSIRDYC_W<CICRrs> {
        MSIRDYC_W::new(self, 5)
    }
    #[doc = "Bit 6 - HSI48 ready Interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn hsi48rdyc(&mut self) -> HSI48RDYC_W<CICRrs> {
        HSI48RDYC_W::new(self, 6)
    }
    #[doc = "Bit 7 - LSE Clock Security System Interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn csslsec(&mut self) -> CSSLSEC_W<CICRrs> {
        CSSLSEC_W::new(self, 7)
    }
    #[doc = "Bit 8 - Clock Security System Interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn csshsec(&mut self) -> CSSHSEC_W<CICRrs> {
        CSSHSEC_W::new(self, 8)
    }
}
#[doc = "Clock interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cicr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CICRrs;
impl crate::RegisterSpec for CICRrs {
    type Ux = u32;
}
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
