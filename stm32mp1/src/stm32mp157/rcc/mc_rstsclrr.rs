///Register `MC_RSTSCLRR` reader
pub type R = crate::R<MC_RSTSCLRRrs>;
///Register `MC_RSTSCLRR` writer
pub type W = crate::W<MC_RSTSCLRRrs>;
///Field `PORRSTF` reader - PORRSTF
pub type PORRSTF_R = crate::BitReader;
///Field `PORRSTF` writer - PORRSTF
pub type PORRSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BORRSTF` reader - BORRSTF
pub type BORRSTF_R = crate::BitReader;
///Field `BORRSTF` writer - BORRSTF
pub type BORRSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PADRSTF` reader - PADRSTF
pub type PADRSTF_R = crate::BitReader;
///Field `PADRSTF` writer - PADRSTF
pub type PADRSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HCSSRSTF` reader - HCSSRSTF
pub type HCSSRSTF_R = crate::BitReader;
///Field `HCSSRSTF` writer - HCSSRSTF
pub type HCSSRSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VCORERSTF` reader - VCORERSTF
pub type VCORERSTF_R = crate::BitReader;
///Field `VCORERSTF` writer - VCORERSTF
pub type VCORERSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCURSTF` reader - MCURSTF
pub type MCURSTF_R = crate::BitReader;
///Field `MCURSTF` writer - MCURSTF
pub type MCURSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MPSYSRSTF` reader - MPSYSRSTF
pub type MPSYSRSTF_R = crate::BitReader;
///Field `MPSYSRSTF` writer - MPSYSRSTF
pub type MPSYSRSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCSYSRSTF` reader - MCSYSRSTF
pub type MCSYSRSTF_R = crate::BitReader;
///Field `MCSYSRSTF` writer - MCSYSRSTF
pub type MCSYSRSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWDG1RSTF` reader - IWDG1RSTF
pub type IWDG1RSTF_R = crate::BitReader;
///Field `IWDG1RSTF` writer - IWDG1RSTF
pub type IWDG1RSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWDG2RSTF` reader - IWDG2RSTF
pub type IWDG2RSTF_R = crate::BitReader;
///Field `IWDG2RSTF` writer - IWDG2RSTF
pub type IWDG2RSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WWDG1RSTF` reader - WWDG1RSTF
pub type WWDG1RSTF_R = crate::BitReader;
///Field `WWDG1RSTF` writer - WWDG1RSTF
pub type WWDG1RSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - PORRSTF
    #[inline(always)]
    pub fn porrstf(&self) -> PORRSTF_R {
        PORRSTF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - BORRSTF
    #[inline(always)]
    pub fn borrstf(&self) -> BORRSTF_R {
        BORRSTF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - PADRSTF
    #[inline(always)]
    pub fn padrstf(&self) -> PADRSTF_R {
        PADRSTF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - HCSSRSTF
    #[inline(always)]
    pub fn hcssrstf(&self) -> HCSSRSTF_R {
        HCSSRSTF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - VCORERSTF
    #[inline(always)]
    pub fn vcorerstf(&self) -> VCORERSTF_R {
        VCORERSTF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - MCURSTF
    #[inline(always)]
    pub fn mcurstf(&self) -> MCURSTF_R {
        MCURSTF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - MPSYSRSTF
    #[inline(always)]
    pub fn mpsysrstf(&self) -> MPSYSRSTF_R {
        MPSYSRSTF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - MCSYSRSTF
    #[inline(always)]
    pub fn mcsysrstf(&self) -> MCSYSRSTF_R {
        MCSYSRSTF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - IWDG1RSTF
    #[inline(always)]
    pub fn iwdg1rstf(&self) -> IWDG1RSTF_R {
        IWDG1RSTF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - IWDG2RSTF
    #[inline(always)]
    pub fn iwdg2rstf(&self) -> IWDG2RSTF_R {
        IWDG2RSTF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - WWDG1RSTF
    #[inline(always)]
    pub fn wwdg1rstf(&self) -> WWDG1RSTF_R {
        WWDG1RSTF_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MC_RSTSCLRR")
            .field("porrstf", &self.porrstf())
            .field("borrstf", &self.borrstf())
            .field("padrstf", &self.padrstf())
            .field("hcssrstf", &self.hcssrstf())
            .field("vcorerstf", &self.vcorerstf())
            .field("mcurstf", &self.mcurstf())
            .field("mpsysrstf", &self.mpsysrstf())
            .field("mcsysrstf", &self.mcsysrstf())
            .field("iwdg1rstf", &self.iwdg1rstf())
            .field("iwdg2rstf", &self.iwdg2rstf())
            .field("wwdg1rstf", &self.wwdg1rstf())
            .finish()
    }
}
impl W {
    ///Bit 0 - PORRSTF
    #[inline(always)]
    pub fn porrstf(&mut self) -> PORRSTF_W<'_, MC_RSTSCLRRrs> {
        PORRSTF_W::new(self, 0)
    }
    ///Bit 1 - BORRSTF
    #[inline(always)]
    pub fn borrstf(&mut self) -> BORRSTF_W<'_, MC_RSTSCLRRrs> {
        BORRSTF_W::new(self, 1)
    }
    ///Bit 2 - PADRSTF
    #[inline(always)]
    pub fn padrstf(&mut self) -> PADRSTF_W<'_, MC_RSTSCLRRrs> {
        PADRSTF_W::new(self, 2)
    }
    ///Bit 3 - HCSSRSTF
    #[inline(always)]
    pub fn hcssrstf(&mut self) -> HCSSRSTF_W<'_, MC_RSTSCLRRrs> {
        HCSSRSTF_W::new(self, 3)
    }
    ///Bit 4 - VCORERSTF
    #[inline(always)]
    pub fn vcorerstf(&mut self) -> VCORERSTF_W<'_, MC_RSTSCLRRrs> {
        VCORERSTF_W::new(self, 4)
    }
    ///Bit 5 - MCURSTF
    #[inline(always)]
    pub fn mcurstf(&mut self) -> MCURSTF_W<'_, MC_RSTSCLRRrs> {
        MCURSTF_W::new(self, 5)
    }
    ///Bit 6 - MPSYSRSTF
    #[inline(always)]
    pub fn mpsysrstf(&mut self) -> MPSYSRSTF_W<'_, MC_RSTSCLRRrs> {
        MPSYSRSTF_W::new(self, 6)
    }
    ///Bit 7 - MCSYSRSTF
    #[inline(always)]
    pub fn mcsysrstf(&mut self) -> MCSYSRSTF_W<'_, MC_RSTSCLRRrs> {
        MCSYSRSTF_W::new(self, 7)
    }
    ///Bit 8 - IWDG1RSTF
    #[inline(always)]
    pub fn iwdg1rstf(&mut self) -> IWDG1RSTF_W<'_, MC_RSTSCLRRrs> {
        IWDG1RSTF_W::new(self, 8)
    }
    ///Bit 9 - IWDG2RSTF
    #[inline(always)]
    pub fn iwdg2rstf(&mut self) -> IWDG2RSTF_W<'_, MC_RSTSCLRRrs> {
        IWDG2RSTF_W::new(self, 9)
    }
    ///Bit 10 - WWDG1RSTF
    #[inline(always)]
    pub fn wwdg1rstf(&mut self) -> WWDG1RSTF_W<'_, MC_RSTSCLRRrs> {
        WWDG1RSTF_W::new(self, 10)
    }
}
/**This register is used by the MCU to check the reset source.

You can [`read`](crate::Reg::read) this register and get [`mc_rstsclrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_rstsclrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:MC_RSTSCLRR)*/
pub struct MC_RSTSCLRRrs;
impl crate::RegisterSpec for MC_RSTSCLRRrs {
    type Ux = u32;
}
///`read()` method returns [`mc_rstsclrr::R`](R) reader structure
impl crate::Readable for MC_RSTSCLRRrs {}
///`write(|w| ..)` method takes [`mc_rstsclrr::W`](W) writer structure
impl crate::Writable for MC_RSTSCLRRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MC_RSTSCLRR to value 0x15
impl crate::Resettable for MC_RSTSCLRRrs {
    const RESET_VALUE: u32 = 0x15;
}
