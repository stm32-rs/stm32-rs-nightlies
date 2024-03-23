#[doc = "Register `CICR` writer"]
pub type W = crate::W<CICRrs>;
#[doc = "Field `LSIRDYC` writer - LSI ready interrupt clear This bit is set by software to clear the LSIRDYF flag."]
pub type LSIRDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSERDYC` writer - LSE ready interrupt clear This bit is set by software to clear the LSERDYF flag."]
pub type LSERDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIRDYC` writer - HSI16 ready interrupt clear This bit is set software to clear the HSIRDYF flag."]
pub type HSIRDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSERDYC` writer - HSE ready interrupt clear This bit is set by software to clear the HSERDYF flag."]
pub type HSERDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSSC` writer - Clock security system interrupt clear This bit is set by software to clear the HSECSSF flag."]
pub type CSSC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSECSSC` writer - LSE Clock security system interrupt clear This bit is set by software to clear the LSECSSF flag."]
pub type LSECSSC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - LSI ready interrupt clear This bit is set by software to clear the LSIRDYF flag."]
    #[inline(always)]
    #[must_use]
    pub fn lsirdyc(&mut self) -> LSIRDYC_W<CICRrs> {
        LSIRDYC_W::new(self, 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt clear This bit is set by software to clear the LSERDYF flag."]
    #[inline(always)]
    #[must_use]
    pub fn lserdyc(&mut self) -> LSERDYC_W<CICRrs> {
        LSERDYC_W::new(self, 1)
    }
    #[doc = "Bit 3 - HSI16 ready interrupt clear This bit is set software to clear the HSIRDYF flag."]
    #[inline(always)]
    #[must_use]
    pub fn hsirdyc(&mut self) -> HSIRDYC_W<CICRrs> {
        HSIRDYC_W::new(self, 3)
    }
    #[doc = "Bit 4 - HSE ready interrupt clear This bit is set by software to clear the HSERDYF flag."]
    #[inline(always)]
    #[must_use]
    pub fn hserdyc(&mut self) -> HSERDYC_W<CICRrs> {
        HSERDYC_W::new(self, 4)
    }
    #[doc = "Bit 8 - Clock security system interrupt clear This bit is set by software to clear the HSECSSF flag."]
    #[inline(always)]
    #[must_use]
    pub fn cssc(&mut self) -> CSSC_W<CICRrs> {
        CSSC_W::new(self, 8)
    }
    #[doc = "Bit 9 - LSE Clock security system interrupt clear This bit is set by software to clear the LSECSSF flag."]
    #[inline(always)]
    #[must_use]
    pub fn lsecssc(&mut self) -> LSECSSC_W<CICRrs> {
        LSECSSC_W::new(self, 9)
    }
}
#[doc = "RCC clock interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cicr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
