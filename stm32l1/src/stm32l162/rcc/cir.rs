#[doc = "Register `CIR` reader"]
pub type R = crate::R<CIRrs>;
#[doc = "Register `CIR` writer"]
pub type W = crate::W<CIRrs>;
#[doc = "Field `LSIRDYF` reader - LSI ready interrupt flag"]
pub type LSIRDYF_R = crate::BitReader;
#[doc = "Field `LSERDYF` reader - LSE ready interrupt flag"]
pub type LSERDYF_R = crate::BitReader;
#[doc = "Field `HSIRDYF` reader - HSI ready interrupt flag"]
pub type HSIRDYF_R = crate::BitReader;
#[doc = "Field `HSERDYF` reader - HSE ready interrupt flag"]
pub type HSERDYF_R = crate::BitReader;
#[doc = "Field `PLLRDYF` reader - PLL ready interrupt flag"]
pub type PLLRDYF_R = crate::BitReader;
#[doc = "Field `MSIRDYF` reader - MSI ready interrupt flag"]
pub type MSIRDYF_R = crate::BitReader;
#[doc = "Field `CSSF` reader - Clock security system interrupt flag"]
pub type CSSF_R = crate::BitReader;
#[doc = "Field `LSIRDYIE` reader - LSI ready interrupt enable"]
pub type LSIRDYIE_R = crate::BitReader;
#[doc = "Field `LSIRDYIE` writer - LSI ready interrupt enable"]
pub type LSIRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSERDYIE` reader - LSE ready interrupt enable"]
pub type LSERDYIE_R = crate::BitReader;
#[doc = "Field `LSERDYIE` writer - LSE ready interrupt enable"]
pub type LSERDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIRDYIE` reader - HSI ready interrupt enable"]
pub type HSIRDYIE_R = crate::BitReader;
#[doc = "Field `HSIRDYIE` writer - HSI ready interrupt enable"]
pub type HSIRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSERDYIE` reader - HSE ready interrupt enable"]
pub type HSERDYIE_R = crate::BitReader;
#[doc = "Field `HSERDYIE` writer - HSE ready interrupt enable"]
pub type HSERDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLRDYIE` reader - PLL ready interrupt enable"]
pub type PLLRDYIE_R = crate::BitReader;
#[doc = "Field `PLLRDYIE` writer - PLL ready interrupt enable"]
pub type PLLRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSIRDYIE` reader - MSI ready interrupt enable"]
pub type MSIRDYIE_R = crate::BitReader;
#[doc = "Field `MSIRDYIE` writer - MSI ready interrupt enable"]
pub type MSIRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSIRDYC` writer - LSI ready interrupt clear"]
pub type LSIRDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSERDYC` writer - LSE ready interrupt clear"]
pub type LSERDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIRDYC` writer - HSI ready interrupt clear"]
pub type HSIRDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSERDYC` writer - HSE ready interrupt clear"]
pub type HSERDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLRDYC` writer - PLL ready interrupt clear"]
pub type PLLRDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSIRDYC` writer - MSI ready interrupt clear"]
pub type MSIRDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSSC` writer - Clock security system interrupt clear"]
pub type CSSC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LSI ready interrupt flag"]
    #[inline(always)]
    pub fn lsirdyf(&self) -> LSIRDYF_R {
        LSIRDYF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt flag"]
    #[inline(always)]
    pub fn lserdyf(&self) -> LSERDYF_R {
        LSERDYF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSI ready interrupt flag"]
    #[inline(always)]
    pub fn hsirdyf(&self) -> HSIRDYF_R {
        HSIRDYF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSE ready interrupt flag"]
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PLL ready interrupt flag"]
    #[inline(always)]
    pub fn pllrdyf(&self) -> PLLRDYF_R {
        PLLRDYF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MSI ready interrupt flag"]
    #[inline(always)]
    pub fn msirdyf(&self) -> MSIRDYF_R {
        MSIRDYF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Clock security system interrupt flag"]
    #[inline(always)]
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LSI ready interrupt enable"]
    #[inline(always)]
    pub fn lsirdyie(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LSE ready interrupt enable"]
    #[inline(always)]
    pub fn lserdyie(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HSI ready interrupt enable"]
    #[inline(always)]
    pub fn hsirdyie(&self) -> HSIRDYIE_R {
        HSIRDYIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HSE ready interrupt enable"]
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PLL ready interrupt enable"]
    #[inline(always)]
    pub fn pllrdyie(&self) -> PLLRDYIE_R {
        PLLRDYIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MSI ready interrupt enable"]
    #[inline(always)]
    pub fn msirdyie(&self) -> MSIRDYIE_R {
        MSIRDYIE_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - LSI ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn lsirdyie(&mut self) -> LSIRDYIE_W<CIRrs> {
        LSIRDYIE_W::new(self, 8)
    }
    #[doc = "Bit 9 - LSE ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn lserdyie(&mut self) -> LSERDYIE_W<CIRrs> {
        LSERDYIE_W::new(self, 9)
    }
    #[doc = "Bit 10 - HSI ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsirdyie(&mut self) -> HSIRDYIE_W<CIRrs> {
        HSIRDYIE_W::new(self, 10)
    }
    #[doc = "Bit 11 - HSE ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn hserdyie(&mut self) -> HSERDYIE_W<CIRrs> {
        HSERDYIE_W::new(self, 11)
    }
    #[doc = "Bit 12 - PLL ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllrdyie(&mut self) -> PLLRDYIE_W<CIRrs> {
        PLLRDYIE_W::new(self, 12)
    }
    #[doc = "Bit 13 - MSI ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn msirdyie(&mut self) -> MSIRDYIE_W<CIRrs> {
        MSIRDYIE_W::new(self, 13)
    }
    #[doc = "Bit 16 - LSI ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn lsirdyc(&mut self) -> LSIRDYC_W<CIRrs> {
        LSIRDYC_W::new(self, 16)
    }
    #[doc = "Bit 17 - LSE ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn lserdyc(&mut self) -> LSERDYC_W<CIRrs> {
        LSERDYC_W::new(self, 17)
    }
    #[doc = "Bit 18 - HSI ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn hsirdyc(&mut self) -> HSIRDYC_W<CIRrs> {
        HSIRDYC_W::new(self, 18)
    }
    #[doc = "Bit 19 - HSE ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn hserdyc(&mut self) -> HSERDYC_W<CIRrs> {
        HSERDYC_W::new(self, 19)
    }
    #[doc = "Bit 20 - PLL ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn pllrdyc(&mut self) -> PLLRDYC_W<CIRrs> {
        PLLRDYC_W::new(self, 20)
    }
    #[doc = "Bit 21 - MSI ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn msirdyc(&mut self) -> MSIRDYC_W<CIRrs> {
        MSIRDYC_W::new(self, 21)
    }
    #[doc = "Bit 23 - Clock security system interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn cssc(&mut self) -> CSSC_W<CIRrs> {
        CSSC_W::new(self, 23)
    }
}
#[doc = "Clock interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIRrs;
impl crate::RegisterSpec for CIRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cir::R`](R) reader structure"]
impl crate::Readable for CIRrs {}
#[doc = "`write(|w| ..)` method takes [`cir::W`](W) writer structure"]
impl crate::Writable for CIRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CIR to value 0"]
impl crate::Resettable for CIRrs {
    const RESET_VALUE: u32 = 0;
}
