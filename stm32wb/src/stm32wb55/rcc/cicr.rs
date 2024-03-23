#[doc = "Register `CICR` writer"]
pub type W = crate::W<CICRrs>;
#[doc = "Field `LSI1RDYC` writer - LSI1 ready interrupt clear"]
pub type LSI1RDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSERDYC` writer - LSE ready interrupt clear"]
pub type LSERDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSIRDYC` writer - MSI ready interrupt clear"]
pub type MSIRDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIRDYC` writer - HSI ready interrupt clear"]
pub type HSIRDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSERDYC` writer - HSE ready interrupt clear"]
pub type HSERDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLRDYC` writer - PLL ready interrupt clear"]
pub type PLLRDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLSAI1RDYC` writer - PLLSAI1 ready interrupt clear"]
pub type PLLSAI1RDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSECSSC` writer - HSE Clock security system interrupt clear"]
pub type HSECSSC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSECSSC` writer - LSE Clock security system interrupt clear"]
pub type LSECSSC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSI48RDYC` writer - HSI48 ready interrupt clear"]
pub type HSI48RDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSI2RDYC` writer - LSI2 ready interrupt clear"]
pub type LSI2RDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - LSI1 ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn lsi1rdyc(&mut self) -> LSI1RDYC_W<CICRrs> {
        LSI1RDYC_W::new(self, 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn lserdyc(&mut self) -> LSERDYC_W<CICRrs> {
        LSERDYC_W::new(self, 1)
    }
    #[doc = "Bit 2 - MSI ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn msirdyc(&mut self) -> MSIRDYC_W<CICRrs> {
        MSIRDYC_W::new(self, 2)
    }
    #[doc = "Bit 3 - HSI ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn hsirdyc(&mut self) -> HSIRDYC_W<CICRrs> {
        HSIRDYC_W::new(self, 3)
    }
    #[doc = "Bit 4 - HSE ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn hserdyc(&mut self) -> HSERDYC_W<CICRrs> {
        HSERDYC_W::new(self, 4)
    }
    #[doc = "Bit 5 - PLL ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn pllrdyc(&mut self) -> PLLRDYC_W<CICRrs> {
        PLLRDYC_W::new(self, 5)
    }
    #[doc = "Bit 6 - PLLSAI1 ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn pllsai1rdyc(&mut self) -> PLLSAI1RDYC_W<CICRrs> {
        PLLSAI1RDYC_W::new(self, 6)
    }
    #[doc = "Bit 8 - HSE Clock security system interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn hsecssc(&mut self) -> HSECSSC_W<CICRrs> {
        HSECSSC_W::new(self, 8)
    }
    #[doc = "Bit 9 - LSE Clock security system interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn lsecssc(&mut self) -> LSECSSC_W<CICRrs> {
        LSECSSC_W::new(self, 9)
    }
    #[doc = "Bit 10 - HSI48 ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn hsi48rdyc(&mut self) -> HSI48RDYC_W<CICRrs> {
        HSI48RDYC_W::new(self, 10)
    }
    #[doc = "Bit 11 - LSI2 ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn lsi2rdyc(&mut self) -> LSI2RDYC_W<CICRrs> {
        LSI2RDYC_W::new(self, 11)
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
