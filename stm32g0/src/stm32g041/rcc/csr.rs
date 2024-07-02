///Register `CSR` reader
pub type R = crate::R<CSRrs>;
///Register `CSR` writer
pub type W = crate::W<CSRrs>;
///Field `LSION` reader - LSI oscillator enable
pub type LSION_R = crate::BitReader;
///Field `LSION` writer - LSI oscillator enable
pub type LSION_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSIRDY` reader - LSI oscillator ready
pub type LSIRDY_R = crate::BitReader;
///Field `LSIRDY` writer - LSI oscillator ready
pub type LSIRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RMVF` reader - Remove reset flags
pub type RMVF_R = crate::BitReader;
///Field `RMVF` writer - Remove reset flags
pub type RMVF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OBLRSTF` reader - Option byte loader reset flag
pub type OBLRSTF_R = crate::BitReader;
///Field `OBLRSTF` writer - Option byte loader reset flag
pub type OBLRSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PINRSTF` reader - Pin reset flag
pub type PINRSTF_R = crate::BitReader;
///Field `PINRSTF` writer - Pin reset flag
pub type PINRSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PWRRSTF` reader - BOR or POR/PDR flag
pub type PWRRSTF_R = crate::BitReader;
///Field `PWRRSTF` writer - BOR or POR/PDR flag
pub type PWRRSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SFTRSTF` reader - Software reset flag
pub type SFTRSTF_R = crate::BitReader;
///Field `SFTRSTF` writer - Software reset flag
pub type SFTRSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWDGRSTF` reader - Independent window watchdog reset flag
pub type IWDGRSTF_R = crate::BitReader;
///Field `IWDGRSTF` writer - Independent window watchdog reset flag
pub type IWDGRSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WWDGRSTF` reader - Window watchdog reset flag
pub type WWDGRSTF_R = crate::BitReader;
///Field `WWDGRSTF` writer - Window watchdog reset flag
pub type WWDGRSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPWRRSTF` reader - Low-power reset flag
pub type LPWRRSTF_R = crate::BitReader;
///Field `LPWRRSTF` writer - Low-power reset flag
pub type LPWRRSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - LSI oscillator enable
    #[inline(always)]
    pub fn lsion(&self) -> LSION_R {
        LSION_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSI oscillator ready
    #[inline(always)]
    pub fn lsirdy(&self) -> LSIRDY_R {
        LSIRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 23 - Remove reset flags
    #[inline(always)]
    pub fn rmvf(&self) -> RMVF_R {
        RMVF_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 25 - Option byte loader reset flag
    #[inline(always)]
    pub fn oblrstf(&self) -> OBLRSTF_R {
        OBLRSTF_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Pin reset flag
    #[inline(always)]
    pub fn pinrstf(&self) -> PINRSTF_R {
        PINRSTF_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - BOR or POR/PDR flag
    #[inline(always)]
    pub fn pwrrstf(&self) -> PWRRSTF_R {
        PWRRSTF_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Software reset flag
    #[inline(always)]
    pub fn sftrstf(&self) -> SFTRSTF_R {
        SFTRSTF_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Independent window watchdog reset flag
    #[inline(always)]
    pub fn iwdgrstf(&self) -> IWDGRSTF_R {
        IWDGRSTF_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Window watchdog reset flag
    #[inline(always)]
    pub fn wwdgrstf(&self) -> WWDGRSTF_R {
        WWDGRSTF_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Low-power reset flag
    #[inline(always)]
    pub fn lpwrrstf(&self) -> LPWRRSTF_R {
        LPWRRSTF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR")
            .field("lsion", &self.lsion())
            .field("lsirdy", &self.lsirdy())
            .field("rmvf", &self.rmvf())
            .field("oblrstf", &self.oblrstf())
            .field("pinrstf", &self.pinrstf())
            .field("pwrrstf", &self.pwrrstf())
            .field("sftrstf", &self.sftrstf())
            .field("iwdgrstf", &self.iwdgrstf())
            .field("wwdgrstf", &self.wwdgrstf())
            .field("lpwrrstf", &self.lpwrrstf())
            .finish()
    }
}
impl W {
    ///Bit 0 - LSI oscillator enable
    #[inline(always)]
    #[must_use]
    pub fn lsion(&mut self) -> LSION_W<CSRrs> {
        LSION_W::new(self, 0)
    }
    ///Bit 1 - LSI oscillator ready
    #[inline(always)]
    #[must_use]
    pub fn lsirdy(&mut self) -> LSIRDY_W<CSRrs> {
        LSIRDY_W::new(self, 1)
    }
    ///Bit 23 - Remove reset flags
    #[inline(always)]
    #[must_use]
    pub fn rmvf(&mut self) -> RMVF_W<CSRrs> {
        RMVF_W::new(self, 23)
    }
    ///Bit 25 - Option byte loader reset flag
    #[inline(always)]
    #[must_use]
    pub fn oblrstf(&mut self) -> OBLRSTF_W<CSRrs> {
        OBLRSTF_W::new(self, 25)
    }
    ///Bit 26 - Pin reset flag
    #[inline(always)]
    #[must_use]
    pub fn pinrstf(&mut self) -> PINRSTF_W<CSRrs> {
        PINRSTF_W::new(self, 26)
    }
    ///Bit 27 - BOR or POR/PDR flag
    #[inline(always)]
    #[must_use]
    pub fn pwrrstf(&mut self) -> PWRRSTF_W<CSRrs> {
        PWRRSTF_W::new(self, 27)
    }
    ///Bit 28 - Software reset flag
    #[inline(always)]
    #[must_use]
    pub fn sftrstf(&mut self) -> SFTRSTF_W<CSRrs> {
        SFTRSTF_W::new(self, 28)
    }
    ///Bit 29 - Independent window watchdog reset flag
    #[inline(always)]
    #[must_use]
    pub fn iwdgrstf(&mut self) -> IWDGRSTF_W<CSRrs> {
        IWDGRSTF_W::new(self, 29)
    }
    ///Bit 30 - Window watchdog reset flag
    #[inline(always)]
    #[must_use]
    pub fn wwdgrstf(&mut self) -> WWDGRSTF_W<CSRrs> {
        WWDGRSTF_W::new(self, 30)
    }
    ///Bit 31 - Low-power reset flag
    #[inline(always)]
    #[must_use]
    pub fn lpwrrstf(&mut self) -> LPWRRSTF_W<CSRrs> {
        LPWRRSTF_W::new(self, 31)
    }
}
/**Control/status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G041.html#RCC:CSR)*/
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
///`read()` method returns [`csr::R`](R) reader structure
impl crate::Readable for CSRrs {}
///`write(|w| ..)` method takes [`csr::W`](W) writer structure
impl crate::Writable for CSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CSR to value 0
impl crate::Resettable for CSRrs {
    const RESET_VALUE: u32 = 0;
}
