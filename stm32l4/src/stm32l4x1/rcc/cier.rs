#[doc = "Register `CIER` reader"]
pub type R = crate::R<CIERrs>;
#[doc = "Register `CIER` writer"]
pub type W = crate::W<CIERrs>;
#[doc = "Field `LSIRDYIE` reader - LSI ready interrupt enable"]
pub type LSIRDYIE_R = crate::BitReader;
#[doc = "Field `LSIRDYIE` writer - LSI ready interrupt enable"]
pub type LSIRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSERDYIE` reader - LSE ready interrupt enable"]
pub type LSERDYIE_R = crate::BitReader;
#[doc = "Field `LSERDYIE` writer - LSE ready interrupt enable"]
pub type LSERDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSIRDYIE` reader - MSI ready interrupt enable"]
pub type MSIRDYIE_R = crate::BitReader;
#[doc = "Field `MSIRDYIE` writer - MSI ready interrupt enable"]
pub type MSIRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `PLLSAI1RDYIE` reader - PLLSAI1 ready interrupt enable"]
pub type PLLSAI1RDYIE_R = crate::BitReader;
#[doc = "Field `PLLSAI1RDYIE` writer - PLLSAI1 ready interrupt enable"]
pub type PLLSAI1RDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSECSSIE` reader - LSE clock security system interrupt enable"]
pub type LSECSSIE_R = crate::BitReader;
#[doc = "Field `LSECSSIE` writer - LSE clock security system interrupt enable"]
pub type LSECSSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSI48RDYIE` reader - HSI48 ready interrupt enable"]
pub type HSI48RDYIE_R = crate::BitReader;
#[doc = "Field `HSI48RDYIE` writer - HSI48 ready interrupt enable"]
pub type HSI48RDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LSI ready interrupt enable"]
    #[inline(always)]
    pub fn lsirdyie(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt enable"]
    #[inline(always)]
    pub fn lserdyie(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MSI ready interrupt enable"]
    #[inline(always)]
    pub fn msirdyie(&self) -> MSIRDYIE_R {
        MSIRDYIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSI ready interrupt enable"]
    #[inline(always)]
    pub fn hsirdyie(&self) -> HSIRDYIE_R {
        HSIRDYIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HSE ready interrupt enable"]
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PLL ready interrupt enable"]
    #[inline(always)]
    pub fn pllrdyie(&self) -> PLLRDYIE_R {
        PLLRDYIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PLLSAI1 ready interrupt enable"]
    #[inline(always)]
    pub fn pllsai1rdyie(&self) -> PLLSAI1RDYIE_R {
        PLLSAI1RDYIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - LSE clock security system interrupt enable"]
    #[inline(always)]
    pub fn lsecssie(&self) -> LSECSSIE_R {
        LSECSSIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HSI48 ready interrupt enable"]
    #[inline(always)]
    pub fn hsi48rdyie(&self) -> HSI48RDYIE_R {
        HSI48RDYIE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSI ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn lsirdyie(&mut self) -> LSIRDYIE_W<CIERrs> {
        LSIRDYIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn lserdyie(&mut self) -> LSERDYIE_W<CIERrs> {
        LSERDYIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - MSI ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn msirdyie(&mut self) -> MSIRDYIE_W<CIERrs> {
        MSIRDYIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - HSI ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsirdyie(&mut self) -> HSIRDYIE_W<CIERrs> {
        HSIRDYIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - HSE ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn hserdyie(&mut self) -> HSERDYIE_W<CIERrs> {
        HSERDYIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - PLL ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllrdyie(&mut self) -> PLLRDYIE_W<CIERrs> {
        PLLRDYIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - PLLSAI1 ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllsai1rdyie(&mut self) -> PLLSAI1RDYIE_W<CIERrs> {
        PLLSAI1RDYIE_W::new(self, 6)
    }
    #[doc = "Bit 9 - LSE clock security system interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn lsecssie(&mut self) -> LSECSSIE_W<CIERrs> {
        LSECSSIE_W::new(self, 9)
    }
    #[doc = "Bit 10 - HSI48 ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsi48rdyie(&mut self) -> HSI48RDYIE_W<CIERrs> {
        HSI48RDYIE_W::new(self, 10)
    }
}
#[doc = "Clock interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
