#[doc = "Register `SECSR` reader"]
pub type R = crate::R<SECSRrs>;
#[doc = "Register `SECSR` writer"]
pub type W = crate::W<SECSRrs>;
#[doc = "Field `HSISECF` reader - HSISECF"]
pub type HSISECF_R = crate::BitReader;
#[doc = "Field `HSISECF` writer - HSISECF"]
pub type HSISECF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSESECF` reader - HSESECF"]
pub type HSESECF_R = crate::BitReader;
#[doc = "Field `HSESECF` writer - HSESECF"]
pub type HSESECF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSISECF` reader - MSISECF"]
pub type MSISECF_R = crate::BitReader;
#[doc = "Field `MSISECF` writer - MSISECF"]
pub type MSISECF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSISECF` reader - LSISECF"]
pub type LSISECF_R = crate::BitReader;
#[doc = "Field `LSISECF` writer - LSISECF"]
pub type LSISECF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSESECF` reader - LSESECF"]
pub type LSESECF_R = crate::BitReader;
#[doc = "Field `LSESECF` writer - LSESECF"]
pub type LSESECF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSCLKSECF` reader - SYSCLKSECF"]
pub type SYSCLKSECF_R = crate::BitReader;
#[doc = "Field `SYSCLKSECF` writer - SYSCLKSECF"]
pub type SYSCLKSECF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESCSECF` reader - PRESCSECF"]
pub type PRESCSECF_R = crate::BitReader;
#[doc = "Field `PRESCSECF` writer - PRESCSECF"]
pub type PRESCSECF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLSECF` reader - PLLSECF"]
pub type PLLSECF_R = crate::BitReader;
#[doc = "Field `PLLSECF` writer - PLLSECF"]
pub type PLLSECF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLSAI1SECF` reader - PLLSAI1SECF"]
pub type PLLSAI1SECF_R = crate::BitReader;
#[doc = "Field `PLLSAI1SECF` writer - PLLSAI1SECF"]
pub type PLLSAI1SECF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLSAI2SECF` reader - PLLSAI2SECF"]
pub type PLLSAI2SECF_R = crate::BitReader;
#[doc = "Field `PLLSAI2SECF` writer - PLLSAI2SECF"]
pub type PLLSAI2SECF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK48MSECF` reader - CLK48MSECF"]
pub type CLK48MSECF_R = crate::BitReader;
#[doc = "Field `CLK48MSECF` writer - CLK48MSECF"]
pub type CLK48MSECF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSI48SECF` reader - HSI48SECF"]
pub type HSI48SECF_R = crate::BitReader;
#[doc = "Field `HSI48SECF` writer - HSI48SECF"]
pub type HSI48SECF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RMVFSECF` reader - RMVFSECF"]
pub type RMVFSECF_R = crate::BitReader;
#[doc = "Field `RMVFSECF` writer - RMVFSECF"]
pub type RMVFSECF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - HSISECF"]
    #[inline(always)]
    pub fn hsisecf(&self) -> HSISECF_R {
        HSISECF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HSESECF"]
    #[inline(always)]
    pub fn hsesecf(&self) -> HSESECF_R {
        HSESECF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MSISECF"]
    #[inline(always)]
    pub fn msisecf(&self) -> MSISECF_R {
        MSISECF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LSISECF"]
    #[inline(always)]
    pub fn lsisecf(&self) -> LSISECF_R {
        LSISECF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LSESECF"]
    #[inline(always)]
    pub fn lsesecf(&self) -> LSESECF_R {
        LSESECF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SYSCLKSECF"]
    #[inline(always)]
    pub fn sysclksecf(&self) -> SYSCLKSECF_R {
        SYSCLKSECF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PRESCSECF"]
    #[inline(always)]
    pub fn prescsecf(&self) -> PRESCSECF_R {
        PRESCSECF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PLLSECF"]
    #[inline(always)]
    pub fn pllsecf(&self) -> PLLSECF_R {
        PLLSECF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PLLSAI1SECF"]
    #[inline(always)]
    pub fn pllsai1secf(&self) -> PLLSAI1SECF_R {
        PLLSAI1SECF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PLLSAI2SECF"]
    #[inline(always)]
    pub fn pllsai2secf(&self) -> PLLSAI2SECF_R {
        PLLSAI2SECF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CLK48MSECF"]
    #[inline(always)]
    pub fn clk48msecf(&self) -> CLK48MSECF_R {
        CLK48MSECF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HSI48SECF"]
    #[inline(always)]
    pub fn hsi48secf(&self) -> HSI48SECF_R {
        HSI48SECF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RMVFSECF"]
    #[inline(always)]
    pub fn rmvfsecf(&self) -> RMVFSECF_R {
        RMVFSECF_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HSISECF"]
    #[inline(always)]
    #[must_use]
    pub fn hsisecf(&mut self) -> HSISECF_W<SECSRrs> {
        HSISECF_W::new(self, 0)
    }
    #[doc = "Bit 1 - HSESECF"]
    #[inline(always)]
    #[must_use]
    pub fn hsesecf(&mut self) -> HSESECF_W<SECSRrs> {
        HSESECF_W::new(self, 1)
    }
    #[doc = "Bit 2 - MSISECF"]
    #[inline(always)]
    #[must_use]
    pub fn msisecf(&mut self) -> MSISECF_W<SECSRrs> {
        MSISECF_W::new(self, 2)
    }
    #[doc = "Bit 3 - LSISECF"]
    #[inline(always)]
    #[must_use]
    pub fn lsisecf(&mut self) -> LSISECF_W<SECSRrs> {
        LSISECF_W::new(self, 3)
    }
    #[doc = "Bit 4 - LSESECF"]
    #[inline(always)]
    #[must_use]
    pub fn lsesecf(&mut self) -> LSESECF_W<SECSRrs> {
        LSESECF_W::new(self, 4)
    }
    #[doc = "Bit 5 - SYSCLKSECF"]
    #[inline(always)]
    #[must_use]
    pub fn sysclksecf(&mut self) -> SYSCLKSECF_W<SECSRrs> {
        SYSCLKSECF_W::new(self, 5)
    }
    #[doc = "Bit 6 - PRESCSECF"]
    #[inline(always)]
    #[must_use]
    pub fn prescsecf(&mut self) -> PRESCSECF_W<SECSRrs> {
        PRESCSECF_W::new(self, 6)
    }
    #[doc = "Bit 7 - PLLSECF"]
    #[inline(always)]
    #[must_use]
    pub fn pllsecf(&mut self) -> PLLSECF_W<SECSRrs> {
        PLLSECF_W::new(self, 7)
    }
    #[doc = "Bit 8 - PLLSAI1SECF"]
    #[inline(always)]
    #[must_use]
    pub fn pllsai1secf(&mut self) -> PLLSAI1SECF_W<SECSRrs> {
        PLLSAI1SECF_W::new(self, 8)
    }
    #[doc = "Bit 9 - PLLSAI2SECF"]
    #[inline(always)]
    #[must_use]
    pub fn pllsai2secf(&mut self) -> PLLSAI2SECF_W<SECSRrs> {
        PLLSAI2SECF_W::new(self, 9)
    }
    #[doc = "Bit 10 - CLK48MSECF"]
    #[inline(always)]
    #[must_use]
    pub fn clk48msecf(&mut self) -> CLK48MSECF_W<SECSRrs> {
        CLK48MSECF_W::new(self, 10)
    }
    #[doc = "Bit 11 - HSI48SECF"]
    #[inline(always)]
    #[must_use]
    pub fn hsi48secf(&mut self) -> HSI48SECF_W<SECSRrs> {
        HSI48SECF_W::new(self, 11)
    }
    #[doc = "Bit 12 - RMVFSECF"]
    #[inline(always)]
    #[must_use]
    pub fn rmvfsecf(&mut self) -> RMVFSECF_W<SECSRrs> {
        RMVFSECF_W::new(self, 12)
    }
}
#[doc = "RCC secure status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`secsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`secsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SECSRrs;
impl crate::RegisterSpec for SECSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secsr::R`](R) reader structure"]
impl crate::Readable for SECSRrs {}
#[doc = "`write(|w| ..)` method takes [`secsr::W`](W) writer structure"]
impl crate::Writable for SECSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECSR to value 0"]
impl crate::Resettable for SECSRrs {
    const RESET_VALUE: u32 = 0;
}
