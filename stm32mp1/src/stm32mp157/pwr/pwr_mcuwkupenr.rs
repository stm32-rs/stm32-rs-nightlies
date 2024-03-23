#[doc = "Register `PWR_MCUWKUPENR` reader"]
pub type R = crate::R<PWR_MCUWKUPENRrs>;
#[doc = "Register `PWR_MCUWKUPENR` writer"]
pub type W = crate::W<PWR_MCUWKUPENRrs>;
#[doc = "Field `WKUPEN1` reader - WKUPEN1"]
pub type WKUPEN1_R = crate::BitReader;
#[doc = "Field `WKUPEN1` writer - WKUPEN1"]
pub type WKUPEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPEN2` reader - WKUPEN2"]
pub type WKUPEN2_R = crate::BitReader;
#[doc = "Field `WKUPEN2` writer - WKUPEN2"]
pub type WKUPEN2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPEN3` reader - WKUPEN3"]
pub type WKUPEN3_R = crate::BitReader;
#[doc = "Field `WKUPEN3` writer - WKUPEN3"]
pub type WKUPEN3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPEN4` reader - WKUPEN4"]
pub type WKUPEN4_R = crate::BitReader;
#[doc = "Field `WKUPEN4` writer - WKUPEN4"]
pub type WKUPEN4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPEN5` reader - WKUPEN5"]
pub type WKUPEN5_R = crate::BitReader;
#[doc = "Field `WKUPEN5` writer - WKUPEN5"]
pub type WKUPEN5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPEN6` reader - WKUPEN6"]
pub type WKUPEN6_R = crate::BitReader;
#[doc = "Field `WKUPEN6` writer - WKUPEN6"]
pub type WKUPEN6_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - WKUPEN1"]
    #[inline(always)]
    pub fn wkupen1(&self) -> WKUPEN1_R {
        WKUPEN1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WKUPEN2"]
    #[inline(always)]
    pub fn wkupen2(&self) -> WKUPEN2_R {
        WKUPEN2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WKUPEN3"]
    #[inline(always)]
    pub fn wkupen3(&self) -> WKUPEN3_R {
        WKUPEN3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WKUPEN4"]
    #[inline(always)]
    pub fn wkupen4(&self) -> WKUPEN4_R {
        WKUPEN4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - WKUPEN5"]
    #[inline(always)]
    pub fn wkupen5(&self) -> WKUPEN5_R {
        WKUPEN5_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - WKUPEN6"]
    #[inline(always)]
    pub fn wkupen6(&self) -> WKUPEN6_R {
        WKUPEN6_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WKUPEN1"]
    #[inline(always)]
    #[must_use]
    pub fn wkupen1(&mut self) -> WKUPEN1_W<PWR_MCUWKUPENRrs> {
        WKUPEN1_W::new(self, 0)
    }
    #[doc = "Bit 1 - WKUPEN2"]
    #[inline(always)]
    #[must_use]
    pub fn wkupen2(&mut self) -> WKUPEN2_W<PWR_MCUWKUPENRrs> {
        WKUPEN2_W::new(self, 1)
    }
    #[doc = "Bit 2 - WKUPEN3"]
    #[inline(always)]
    #[must_use]
    pub fn wkupen3(&mut self) -> WKUPEN3_W<PWR_MCUWKUPENRrs> {
        WKUPEN3_W::new(self, 2)
    }
    #[doc = "Bit 3 - WKUPEN4"]
    #[inline(always)]
    #[must_use]
    pub fn wkupen4(&mut self) -> WKUPEN4_W<PWR_MCUWKUPENRrs> {
        WKUPEN4_W::new(self, 3)
    }
    #[doc = "Bit 4 - WKUPEN5"]
    #[inline(always)]
    #[must_use]
    pub fn wkupen5(&mut self) -> WKUPEN5_W<PWR_MCUWKUPENRrs> {
        WKUPEN5_W::new(self, 4)
    }
    #[doc = "Bit 5 - WKUPEN6"]
    #[inline(always)]
    #[must_use]
    pub fn wkupen6(&mut self) -> WKUPEN6_W<PWR_MCUWKUPENRrs> {
        WKUPEN6_W::new(self, 5)
    }
}
#[doc = "Not reset by wakeup from Standby mode but by any Application reset (NRST, IWDG, ...) Access 6 wait states when writing this register. When a system reset occurs during the register write cycle the written data is not guaranteed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_mcuwkupenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_mcuwkupenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWR_MCUWKUPENRrs;
impl crate::RegisterSpec for PWR_MCUWKUPENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_mcuwkupenr::R`](R) reader structure"]
impl crate::Readable for PWR_MCUWKUPENRrs {}
#[doc = "`write(|w| ..)` method takes [`pwr_mcuwkupenr::W`](W) writer structure"]
impl crate::Writable for PWR_MCUWKUPENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWR_MCUWKUPENR to value 0"]
impl crate::Resettable for PWR_MCUWKUPENRrs {
    const RESET_VALUE: u32 = 0;
}
