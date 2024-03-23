#[doc = "Register `RCC_MC_RSTSCLRR` reader"]
pub type R = crate::R<RCC_MC_RSTSCLRRrs>;
#[doc = "Register `RCC_MC_RSTSCLRR` writer"]
pub type W = crate::W<RCC_MC_RSTSCLRRrs>;
#[doc = "Field `PORRSTF` reader - PORRSTF"]
pub type PORRSTF_R = crate::BitReader;
#[doc = "Field `PORRSTF` writer - PORRSTF"]
pub type PORRSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BORRSTF` reader - BORRSTF"]
pub type BORRSTF_R = crate::BitReader;
#[doc = "Field `BORRSTF` writer - BORRSTF"]
pub type BORRSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADRSTF` reader - PADRSTF"]
pub type PADRSTF_R = crate::BitReader;
#[doc = "Field `PADRSTF` writer - PADRSTF"]
pub type PADRSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCSSRSTF` reader - HCSSRSTF"]
pub type HCSSRSTF_R = crate::BitReader;
#[doc = "Field `HCSSRSTF` writer - HCSSRSTF"]
pub type HCSSRSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VCORERSTF` reader - VCORERSTF"]
pub type VCORERSTF_R = crate::BitReader;
#[doc = "Field `VCORERSTF` writer - VCORERSTF"]
pub type VCORERSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCURSTF` reader - MCURSTF"]
pub type MCURSTF_R = crate::BitReader;
#[doc = "Field `MCURSTF` writer - MCURSTF"]
pub type MCURSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPSYSRSTF` reader - MPSYSRSTF"]
pub type MPSYSRSTF_R = crate::BitReader;
#[doc = "Field `MPSYSRSTF` writer - MPSYSRSTF"]
pub type MPSYSRSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCSYSRSTF` reader - MCSYSRSTF"]
pub type MCSYSRSTF_R = crate::BitReader;
#[doc = "Field `MCSYSRSTF` writer - MCSYSRSTF"]
pub type MCSYSRSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWDG1RSTF` reader - IWDG1RSTF"]
pub type IWDG1RSTF_R = crate::BitReader;
#[doc = "Field `IWDG1RSTF` writer - IWDG1RSTF"]
pub type IWDG1RSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWDG2RSTF` reader - IWDG2RSTF"]
pub type IWDG2RSTF_R = crate::BitReader;
#[doc = "Field `IWDG2RSTF` writer - IWDG2RSTF"]
pub type IWDG2RSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDG1RSTF` reader - WWDG1RSTF"]
pub type WWDG1RSTF_R = crate::BitReader;
#[doc = "Field `WWDG1RSTF` writer - WWDG1RSTF"]
pub type WWDG1RSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PORRSTF"]
    #[inline(always)]
    pub fn porrstf(&self) -> PORRSTF_R {
        PORRSTF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BORRSTF"]
    #[inline(always)]
    pub fn borrstf(&self) -> BORRSTF_R {
        BORRSTF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PADRSTF"]
    #[inline(always)]
    pub fn padrstf(&self) -> PADRSTF_R {
        PADRSTF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HCSSRSTF"]
    #[inline(always)]
    pub fn hcssrstf(&self) -> HCSSRSTF_R {
        HCSSRSTF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VCORERSTF"]
    #[inline(always)]
    pub fn vcorerstf(&self) -> VCORERSTF_R {
        VCORERSTF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MCURSTF"]
    #[inline(always)]
    pub fn mcurstf(&self) -> MCURSTF_R {
        MCURSTF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MPSYSRSTF"]
    #[inline(always)]
    pub fn mpsysrstf(&self) -> MPSYSRSTF_R {
        MPSYSRSTF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MCSYSRSTF"]
    #[inline(always)]
    pub fn mcsysrstf(&self) -> MCSYSRSTF_R {
        MCSYSRSTF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - IWDG1RSTF"]
    #[inline(always)]
    pub fn iwdg1rstf(&self) -> IWDG1RSTF_R {
        IWDG1RSTF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - IWDG2RSTF"]
    #[inline(always)]
    pub fn iwdg2rstf(&self) -> IWDG2RSTF_R {
        IWDG2RSTF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - WWDG1RSTF"]
    #[inline(always)]
    pub fn wwdg1rstf(&self) -> WWDG1RSTF_R {
        WWDG1RSTF_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PORRSTF"]
    #[inline(always)]
    #[must_use]
    pub fn porrstf(&mut self) -> PORRSTF_W<RCC_MC_RSTSCLRRrs> {
        PORRSTF_W::new(self, 0)
    }
    #[doc = "Bit 1 - BORRSTF"]
    #[inline(always)]
    #[must_use]
    pub fn borrstf(&mut self) -> BORRSTF_W<RCC_MC_RSTSCLRRrs> {
        BORRSTF_W::new(self, 1)
    }
    #[doc = "Bit 2 - PADRSTF"]
    #[inline(always)]
    #[must_use]
    pub fn padrstf(&mut self) -> PADRSTF_W<RCC_MC_RSTSCLRRrs> {
        PADRSTF_W::new(self, 2)
    }
    #[doc = "Bit 3 - HCSSRSTF"]
    #[inline(always)]
    #[must_use]
    pub fn hcssrstf(&mut self) -> HCSSRSTF_W<RCC_MC_RSTSCLRRrs> {
        HCSSRSTF_W::new(self, 3)
    }
    #[doc = "Bit 4 - VCORERSTF"]
    #[inline(always)]
    #[must_use]
    pub fn vcorerstf(&mut self) -> VCORERSTF_W<RCC_MC_RSTSCLRRrs> {
        VCORERSTF_W::new(self, 4)
    }
    #[doc = "Bit 5 - MCURSTF"]
    #[inline(always)]
    #[must_use]
    pub fn mcurstf(&mut self) -> MCURSTF_W<RCC_MC_RSTSCLRRrs> {
        MCURSTF_W::new(self, 5)
    }
    #[doc = "Bit 6 - MPSYSRSTF"]
    #[inline(always)]
    #[must_use]
    pub fn mpsysrstf(&mut self) -> MPSYSRSTF_W<RCC_MC_RSTSCLRRrs> {
        MPSYSRSTF_W::new(self, 6)
    }
    #[doc = "Bit 7 - MCSYSRSTF"]
    #[inline(always)]
    #[must_use]
    pub fn mcsysrstf(&mut self) -> MCSYSRSTF_W<RCC_MC_RSTSCLRRrs> {
        MCSYSRSTF_W::new(self, 7)
    }
    #[doc = "Bit 8 - IWDG1RSTF"]
    #[inline(always)]
    #[must_use]
    pub fn iwdg1rstf(&mut self) -> IWDG1RSTF_W<RCC_MC_RSTSCLRRrs> {
        IWDG1RSTF_W::new(self, 8)
    }
    #[doc = "Bit 9 - IWDG2RSTF"]
    #[inline(always)]
    #[must_use]
    pub fn iwdg2rstf(&mut self) -> IWDG2RSTF_W<RCC_MC_RSTSCLRRrs> {
        IWDG2RSTF_W::new(self, 9)
    }
    #[doc = "Bit 10 - WWDG1RSTF"]
    #[inline(always)]
    #[must_use]
    pub fn wwdg1rstf(&mut self) -> WWDG1RSTF_W<RCC_MC_RSTSCLRRrs> {
        WWDG1RSTF_W::new(self, 10)
    }
}
#[doc = "This register is used by the MCU to check the reset source.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_rstsclrr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_rstsclrr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_MC_RSTSCLRRrs;
impl crate::RegisterSpec for RCC_MC_RSTSCLRRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_mc_rstsclrr::R`](R) reader structure"]
impl crate::Readable for RCC_MC_RSTSCLRRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_mc_rstsclrr::W`](W) writer structure"]
impl crate::Writable for RCC_MC_RSTSCLRRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_MC_RSTSCLRR to value 0x15"]
impl crate::Resettable for RCC_MC_RSTSCLRRrs {
    const RESET_VALUE: u32 = 0x15;
}
