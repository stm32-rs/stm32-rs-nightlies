///Register `RSR` reader
pub type R = crate::R<RSRrs>;
///Register `RSR` writer
pub type W = crate::W<RSRrs>;
///Field `RMVF` writer - Remove reset flag
pub type RMVF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LCKRSTF` reader - CPU lockup reset flag.
pub type LCKRSTF_R = crate::BitReader;
///Field `BORRSTF` reader - BOR flag
pub type BORRSTF_R = crate::BitReader;
///Field `PINRSTF` reader - Pin reset flag (NRST)
pub type PINRSTF_R = crate::BitReader;
///Field `PORRSTF` reader - POR/PDR flag.
pub type PORRSTF_R = crate::BitReader;
///Field `SFTRSTF` reader - Software System reset flag (1)
pub type SFTRSTF_R = crate::BitReader;
///Field `IWDGRSTF` reader - Independent Watchdog reset flag.
pub type IWDGRSTF_R = crate::BitReader;
///Field `WWDGRSTF` reader - Window Watchdog reset flag
pub type WWDGRSTF_R = crate::BitReader;
///Field `LPWRRSTF` reader - Illegal Stop or Standby flag.
pub type LPWRRSTF_R = crate::BitReader;
impl R {
    ///Bit 17 - CPU lockup reset flag.
    #[inline(always)]
    pub fn lckrstf(&self) -> LCKRSTF_R {
        LCKRSTF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 21 - BOR flag
    #[inline(always)]
    pub fn borrstf(&self) -> BORRSTF_R {
        BORRSTF_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Pin reset flag (NRST)
    #[inline(always)]
    pub fn pinrstf(&self) -> PINRSTF_R {
        PINRSTF_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - POR/PDR flag.
    #[inline(always)]
    pub fn porrstf(&self) -> PORRSTF_R {
        PORRSTF_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Software System reset flag (1)
    #[inline(always)]
    pub fn sftrstf(&self) -> SFTRSTF_R {
        SFTRSTF_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 26 - Independent Watchdog reset flag.
    #[inline(always)]
    pub fn iwdgrstf(&self) -> IWDGRSTF_R {
        IWDGRSTF_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 28 - Window Watchdog reset flag
    #[inline(always)]
    pub fn wwdgrstf(&self) -> WWDGRSTF_R {
        WWDGRSTF_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 30 - Illegal Stop or Standby flag.
    #[inline(always)]
    pub fn lpwrrstf(&self) -> LPWRRSTF_R {
        LPWRRSTF_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSR")
            .field("lckrstf", &self.lckrstf())
            .field("borrstf", &self.borrstf())
            .field("pinrstf", &self.pinrstf())
            .field("porrstf", &self.porrstf())
            .field("sftrstf", &self.sftrstf())
            .field("iwdgrstf", &self.iwdgrstf())
            .field("wwdgrstf", &self.wwdgrstf())
            .field("lpwrrstf", &self.lpwrrstf())
            .finish()
    }
}
impl W {
    ///Bit 16 - Remove reset flag
    #[inline(always)]
    pub fn rmvf(&mut self) -> RMVF_W<RSRrs> {
        RMVF_W::new(self, 16)
    }
}
/**RCC reset register

You can [`read`](crate::Reg::read) this register and get [`rsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:RSR)*/
pub struct RSRrs;
impl crate::RegisterSpec for RSRrs {
    type Ux = u32;
}
///`read()` method returns [`rsr::R`](R) reader structure
impl crate::Readable for RSRrs {}
///`write(|w| ..)` method takes [`rsr::W`](W) writer structure
impl crate::Writable for RSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RSR to value 0x00e0_0000
impl crate::Resettable for RSRrs {
    const RESET_VALUE: u32 = 0x00e0_0000;
}
