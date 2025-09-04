///Register `MPUWKUPENR` reader
pub type R = crate::R<MPUWKUPENRrs>;
///Register `MPUWKUPENR` writer
pub type W = crate::W<MPUWKUPENRrs>;
///Field `WKUPEN1` reader - WKUPEN1
pub type WKUPEN1_R = crate::BitReader;
///Field `WKUPEN1` writer - WKUPEN1
pub type WKUPEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPEN2` reader - WKUPEN2
pub type WKUPEN2_R = crate::BitReader;
///Field `WKUPEN2` writer - WKUPEN2
pub type WKUPEN2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPEN3` reader - WKUPEN3
pub type WKUPEN3_R = crate::BitReader;
///Field `WKUPEN3` writer - WKUPEN3
pub type WKUPEN3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPEN4` reader - WKUPEN4
pub type WKUPEN4_R = crate::BitReader;
///Field `WKUPEN4` writer - WKUPEN4
pub type WKUPEN4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPEN5` reader - WKUPEN5
pub type WKUPEN5_R = crate::BitReader;
///Field `WKUPEN5` writer - WKUPEN5
pub type WKUPEN5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPEN6` reader - WKUPEN6
pub type WKUPEN6_R = crate::BitReader;
///Field `WKUPEN6` writer - WKUPEN6
pub type WKUPEN6_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - WKUPEN1
    #[inline(always)]
    pub fn wkupen1(&self) -> WKUPEN1_R {
        WKUPEN1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - WKUPEN2
    #[inline(always)]
    pub fn wkupen2(&self) -> WKUPEN2_R {
        WKUPEN2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - WKUPEN3
    #[inline(always)]
    pub fn wkupen3(&self) -> WKUPEN3_R {
        WKUPEN3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - WKUPEN4
    #[inline(always)]
    pub fn wkupen4(&self) -> WKUPEN4_R {
        WKUPEN4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - WKUPEN5
    #[inline(always)]
    pub fn wkupen5(&self) -> WKUPEN5_R {
        WKUPEN5_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - WKUPEN6
    #[inline(always)]
    pub fn wkupen6(&self) -> WKUPEN6_R {
        WKUPEN6_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MPUWKUPENR")
            .field("wkupen1", &self.wkupen1())
            .field("wkupen2", &self.wkupen2())
            .field("wkupen3", &self.wkupen3())
            .field("wkupen4", &self.wkupen4())
            .field("wkupen5", &self.wkupen5())
            .field("wkupen6", &self.wkupen6())
            .finish()
    }
}
impl W {
    ///Bit 0 - WKUPEN1
    #[inline(always)]
    pub fn wkupen1(&mut self) -> WKUPEN1_W<MPUWKUPENRrs> {
        WKUPEN1_W::new(self, 0)
    }
    ///Bit 1 - WKUPEN2
    #[inline(always)]
    pub fn wkupen2(&mut self) -> WKUPEN2_W<MPUWKUPENRrs> {
        WKUPEN2_W::new(self, 1)
    }
    ///Bit 2 - WKUPEN3
    #[inline(always)]
    pub fn wkupen3(&mut self) -> WKUPEN3_W<MPUWKUPENRrs> {
        WKUPEN3_W::new(self, 2)
    }
    ///Bit 3 - WKUPEN4
    #[inline(always)]
    pub fn wkupen4(&mut self) -> WKUPEN4_W<MPUWKUPENRrs> {
        WKUPEN4_W::new(self, 3)
    }
    ///Bit 4 - WKUPEN5
    #[inline(always)]
    pub fn wkupen5(&mut self) -> WKUPEN5_W<MPUWKUPENRrs> {
        WKUPEN5_W::new(self, 4)
    }
    ///Bit 5 - WKUPEN6
    #[inline(always)]
    pub fn wkupen6(&mut self) -> WKUPEN6_W<MPUWKUPENRrs> {
        WKUPEN6_W::new(self, 5)
    }
}
/**Not reset by wakeup from Standby mode but by any Application reset (NRST, IWDG, ...). Access 6 wait states when writing this register. This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access is discarded and a bus error is generated. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed.

You can [`read`](crate::Reg::read) this register and get [`mpuwkupenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpuwkupenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#PWR:MPUWKUPENR)*/
pub struct MPUWKUPENRrs;
impl crate::RegisterSpec for MPUWKUPENRrs {
    type Ux = u32;
}
///`read()` method returns [`mpuwkupenr::R`](R) reader structure
impl crate::Readable for MPUWKUPENRrs {}
///`write(|w| ..)` method takes [`mpuwkupenr::W`](W) writer structure
impl crate::Writable for MPUWKUPENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MPUWKUPENR to value 0
impl crate::Resettable for MPUWKUPENRrs {}
