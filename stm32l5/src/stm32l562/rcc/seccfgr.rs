#[doc = "Register `SECCFGR` reader"]
pub type R = crate::R<SECCFGRrs>;
#[doc = "Register `SECCFGR` writer"]
pub type W = crate::W<SECCFGRrs>;
#[doc = "Field `HSISEC` reader - HSISEC"]
pub type HSISEC_R = crate::BitReader;
#[doc = "Field `HSISEC` writer - HSISEC"]
pub type HSISEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSESEC` reader - HSESEC"]
pub type HSESEC_R = crate::BitReader;
#[doc = "Field `HSESEC` writer - HSESEC"]
pub type HSESEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSISEC` reader - MSISEC"]
pub type MSISEC_R = crate::BitReader;
#[doc = "Field `MSISEC` writer - MSISEC"]
pub type MSISEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSISEC` reader - LSISEC"]
pub type LSISEC_R = crate::BitReader;
#[doc = "Field `LSISEC` writer - LSISEC"]
pub type LSISEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSESEC` reader - LSESEC"]
pub type LSESEC_R = crate::BitReader;
#[doc = "Field `LSESEC` writer - LSESEC"]
pub type LSESEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSCLKSEC` reader - SYSCLKSEC"]
pub type SYSCLKSEC_R = crate::BitReader;
#[doc = "Field `SYSCLKSEC` writer - SYSCLKSEC"]
pub type SYSCLKSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESCSEC` reader - PRESCSEC"]
pub type PRESCSEC_R = crate::BitReader;
#[doc = "Field `PRESCSEC` writer - PRESCSEC"]
pub type PRESCSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLSEC` reader - PLLSEC"]
pub type PLLSEC_R = crate::BitReader;
#[doc = "Field `PLLSEC` writer - PLLSEC"]
pub type PLLSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLSAI1SEC` reader - PLLSAI1SEC"]
pub type PLLSAI1SEC_R = crate::BitReader;
#[doc = "Field `PLLSAI1SEC` writer - PLLSAI1SEC"]
pub type PLLSAI1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLSAI2SEC` reader - PLLSAI2SEC"]
pub type PLLSAI2SEC_R = crate::BitReader;
#[doc = "Field `PLLSAI2SEC` writer - PLLSAI2SEC"]
pub type PLLSAI2SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK48MSEC` reader - CLK48MSEC"]
pub type CLK48MSEC_R = crate::BitReader;
#[doc = "Field `CLK48MSEC` writer - CLK48MSEC"]
pub type CLK48MSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSI48SEC` reader - HSI48SEC"]
pub type HSI48SEC_R = crate::BitReader;
#[doc = "Field `HSI48SEC` writer - HSI48SEC"]
pub type HSI48SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RMVFSEC` reader - RMVFSEC"]
pub type RMVFSEC_R = crate::BitReader;
#[doc = "Field `RMVFSEC` writer - RMVFSEC"]
pub type RMVFSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - HSISEC"]
    #[inline(always)]
    pub fn hsisec(&self) -> HSISEC_R {
        HSISEC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HSESEC"]
    #[inline(always)]
    pub fn hsesec(&self) -> HSESEC_R {
        HSESEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MSISEC"]
    #[inline(always)]
    pub fn msisec(&self) -> MSISEC_R {
        MSISEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LSISEC"]
    #[inline(always)]
    pub fn lsisec(&self) -> LSISEC_R {
        LSISEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LSESEC"]
    #[inline(always)]
    pub fn lsesec(&self) -> LSESEC_R {
        LSESEC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SYSCLKSEC"]
    #[inline(always)]
    pub fn sysclksec(&self) -> SYSCLKSEC_R {
        SYSCLKSEC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PRESCSEC"]
    #[inline(always)]
    pub fn prescsec(&self) -> PRESCSEC_R {
        PRESCSEC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PLLSEC"]
    #[inline(always)]
    pub fn pllsec(&self) -> PLLSEC_R {
        PLLSEC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PLLSAI1SEC"]
    #[inline(always)]
    pub fn pllsai1sec(&self) -> PLLSAI1SEC_R {
        PLLSAI1SEC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PLLSAI2SEC"]
    #[inline(always)]
    pub fn pllsai2sec(&self) -> PLLSAI2SEC_R {
        PLLSAI2SEC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CLK48MSEC"]
    #[inline(always)]
    pub fn clk48msec(&self) -> CLK48MSEC_R {
        CLK48MSEC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HSI48SEC"]
    #[inline(always)]
    pub fn hsi48sec(&self) -> HSI48SEC_R {
        HSI48SEC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RMVFSEC"]
    #[inline(always)]
    pub fn rmvfsec(&self) -> RMVFSEC_R {
        RMVFSEC_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HSISEC"]
    #[inline(always)]
    #[must_use]
    pub fn hsisec(&mut self) -> HSISEC_W<SECCFGRrs> {
        HSISEC_W::new(self, 0)
    }
    #[doc = "Bit 1 - HSESEC"]
    #[inline(always)]
    #[must_use]
    pub fn hsesec(&mut self) -> HSESEC_W<SECCFGRrs> {
        HSESEC_W::new(self, 1)
    }
    #[doc = "Bit 2 - MSISEC"]
    #[inline(always)]
    #[must_use]
    pub fn msisec(&mut self) -> MSISEC_W<SECCFGRrs> {
        MSISEC_W::new(self, 2)
    }
    #[doc = "Bit 3 - LSISEC"]
    #[inline(always)]
    #[must_use]
    pub fn lsisec(&mut self) -> LSISEC_W<SECCFGRrs> {
        LSISEC_W::new(self, 3)
    }
    #[doc = "Bit 4 - LSESEC"]
    #[inline(always)]
    #[must_use]
    pub fn lsesec(&mut self) -> LSESEC_W<SECCFGRrs> {
        LSESEC_W::new(self, 4)
    }
    #[doc = "Bit 5 - SYSCLKSEC"]
    #[inline(always)]
    #[must_use]
    pub fn sysclksec(&mut self) -> SYSCLKSEC_W<SECCFGRrs> {
        SYSCLKSEC_W::new(self, 5)
    }
    #[doc = "Bit 6 - PRESCSEC"]
    #[inline(always)]
    #[must_use]
    pub fn prescsec(&mut self) -> PRESCSEC_W<SECCFGRrs> {
        PRESCSEC_W::new(self, 6)
    }
    #[doc = "Bit 7 - PLLSEC"]
    #[inline(always)]
    #[must_use]
    pub fn pllsec(&mut self) -> PLLSEC_W<SECCFGRrs> {
        PLLSEC_W::new(self, 7)
    }
    #[doc = "Bit 8 - PLLSAI1SEC"]
    #[inline(always)]
    #[must_use]
    pub fn pllsai1sec(&mut self) -> PLLSAI1SEC_W<SECCFGRrs> {
        PLLSAI1SEC_W::new(self, 8)
    }
    #[doc = "Bit 9 - PLLSAI2SEC"]
    #[inline(always)]
    #[must_use]
    pub fn pllsai2sec(&mut self) -> PLLSAI2SEC_W<SECCFGRrs> {
        PLLSAI2SEC_W::new(self, 9)
    }
    #[doc = "Bit 10 - CLK48MSEC"]
    #[inline(always)]
    #[must_use]
    pub fn clk48msec(&mut self) -> CLK48MSEC_W<SECCFGRrs> {
        CLK48MSEC_W::new(self, 10)
    }
    #[doc = "Bit 11 - HSI48SEC"]
    #[inline(always)]
    #[must_use]
    pub fn hsi48sec(&mut self) -> HSI48SEC_W<SECCFGRrs> {
        HSI48SEC_W::new(self, 11)
    }
    #[doc = "Bit 12 - RMVFSEC"]
    #[inline(always)]
    #[must_use]
    pub fn rmvfsec(&mut self) -> RMVFSEC_W<SECCFGRrs> {
        RMVFSEC_W::new(self, 12)
    }
}
#[doc = "RCC secure configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seccfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seccfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SECCFGRrs;
impl crate::RegisterSpec for SECCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seccfgr::R`](R) reader structure"]
impl crate::Readable for SECCFGRrs {}
#[doc = "`write(|w| ..)` method takes [`seccfgr::W`](W) writer structure"]
impl crate::Writable for SECCFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECCFGR to value 0"]
impl crate::Resettable for SECCFGRrs {
    const RESET_VALUE: u32 = 0;
}
