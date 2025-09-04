///Register `CSR` reader
pub type R = crate::R<CSRrs>;
///Register `CSR` writer
pub type W = crate::W<CSRrs>;
///Field `LSION` reader - Internal low-speed oscillator enable
pub type LSION_R = crate::BitReader;
///Field `LSION` writer - Internal low-speed oscillator enable
pub type LSION_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSIRDY` reader - Internal low-speed oscillator ready
pub type LSIRDY_R = crate::BitReader;
///Field `LSEON` reader - External low-speed oscillator enable
pub type LSEON_R = crate::BitReader;
///Field `LSEON` writer - External low-speed oscillator enable
pub type LSEON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSERDY` reader - External low-speed oscillator ready
pub type LSERDY_R = crate::BitReader;
///Field `LSEBYP` reader - External low-speed oscillator bypass
pub type LSEBYP_R = crate::BitReader;
///Field `LSEBYP` writer - External low-speed oscillator bypass
pub type LSEBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTCSEL` reader - RTC and LCD clock source selection
pub type RTCSEL_R = crate::FieldReader;
///Field `RTCSEL` writer - RTC and LCD clock source selection
pub type RTCSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RTCEN` reader - RTC clock enable
pub type RTCEN_R = crate::BitReader;
///Field `RTCEN` writer - RTC clock enable
pub type RTCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTCRST` reader - RTC software reset
pub type RTCRST_R = crate::BitReader;
///Field `RTCRST` writer - RTC software reset
pub type RTCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RMVF` reader - Remove reset flag
pub type RMVF_R = crate::BitReader;
///Field `RMVF` writer - Remove reset flag
pub type RMVF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PINRSTF` reader - PIN reset flag
pub type PINRSTF_R = crate::BitReader;
///Field `PINRSTF` writer - PIN reset flag
pub type PINRSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PORRSTF` reader - POR/PDR reset flag
pub type PORRSTF_R = crate::BitReader;
///Field `PORRSTF` writer - POR/PDR reset flag
pub type PORRSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SFTRSTF` reader - Software reset flag
pub type SFTRSTF_R = crate::BitReader;
///Field `SFTRSTF` writer - Software reset flag
pub type SFTRSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWDGRSTF` reader - Independent watchdog reset flag
pub type IWDGRSTF_R = crate::BitReader;
///Field `IWDGRSTF` writer - Independent watchdog reset flag
pub type IWDGRSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WWDGRSTF` reader - Window watchdog reset flag
pub type WWDGRSTF_R = crate::BitReader;
///Field `WWDGRSTF` writer - Window watchdog reset flag
pub type WWDGRSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPWRSTF` reader - Low-power reset flag
pub type LPWRSTF_R = crate::BitReader;
///Field `LPWRSTF` writer - Low-power reset flag
pub type LPWRSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Internal low-speed oscillator enable
    #[inline(always)]
    pub fn lsion(&self) -> LSION_R {
        LSION_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Internal low-speed oscillator ready
    #[inline(always)]
    pub fn lsirdy(&self) -> LSIRDY_R {
        LSIRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - External low-speed oscillator enable
    #[inline(always)]
    pub fn lseon(&self) -> LSEON_R {
        LSEON_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - External low-speed oscillator ready
    #[inline(always)]
    pub fn lserdy(&self) -> LSERDY_R {
        LSERDY_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - External low-speed oscillator bypass
    #[inline(always)]
    pub fn lsebyp(&self) -> LSEBYP_R {
        LSEBYP_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bits 16:17 - RTC and LCD clock source selection
    #[inline(always)]
    pub fn rtcsel(&self) -> RTCSEL_R {
        RTCSEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 22 - RTC clock enable
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - RTC software reset
    #[inline(always)]
    pub fn rtcrst(&self) -> RTCRST_R {
        RTCRST_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Remove reset flag
    #[inline(always)]
    pub fn rmvf(&self) -> RMVF_R {
        RMVF_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 26 - PIN reset flag
    #[inline(always)]
    pub fn pinrstf(&self) -> PINRSTF_R {
        PINRSTF_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - POR/PDR reset flag
    #[inline(always)]
    pub fn porrstf(&self) -> PORRSTF_R {
        PORRSTF_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Software reset flag
    #[inline(always)]
    pub fn sftrstf(&self) -> SFTRSTF_R {
        SFTRSTF_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Independent watchdog reset flag
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
    pub fn lpwrstf(&self) -> LPWRSTF_R {
        LPWRSTF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR")
            .field("lpwrstf", &self.lpwrstf())
            .field("wwdgrstf", &self.wwdgrstf())
            .field("iwdgrstf", &self.iwdgrstf())
            .field("sftrstf", &self.sftrstf())
            .field("porrstf", &self.porrstf())
            .field("pinrstf", &self.pinrstf())
            .field("rmvf", &self.rmvf())
            .field("rtcrst", &self.rtcrst())
            .field("rtcen", &self.rtcen())
            .field("rtcsel", &self.rtcsel())
            .field("lsebyp", &self.lsebyp())
            .field("lserdy", &self.lserdy())
            .field("lseon", &self.lseon())
            .field("lsirdy", &self.lsirdy())
            .field("lsion", &self.lsion())
            .finish()
    }
}
impl W {
    ///Bit 0 - Internal low-speed oscillator enable
    #[inline(always)]
    pub fn lsion(&mut self) -> LSION_W<CSRrs> {
        LSION_W::new(self, 0)
    }
    ///Bit 8 - External low-speed oscillator enable
    #[inline(always)]
    pub fn lseon(&mut self) -> LSEON_W<CSRrs> {
        LSEON_W::new(self, 8)
    }
    ///Bit 10 - External low-speed oscillator bypass
    #[inline(always)]
    pub fn lsebyp(&mut self) -> LSEBYP_W<CSRrs> {
        LSEBYP_W::new(self, 10)
    }
    ///Bits 16:17 - RTC and LCD clock source selection
    #[inline(always)]
    pub fn rtcsel(&mut self) -> RTCSEL_W<CSRrs> {
        RTCSEL_W::new(self, 16)
    }
    ///Bit 22 - RTC clock enable
    #[inline(always)]
    pub fn rtcen(&mut self) -> RTCEN_W<CSRrs> {
        RTCEN_W::new(self, 22)
    }
    ///Bit 23 - RTC software reset
    #[inline(always)]
    pub fn rtcrst(&mut self) -> RTCRST_W<CSRrs> {
        RTCRST_W::new(self, 23)
    }
    ///Bit 24 - Remove reset flag
    #[inline(always)]
    pub fn rmvf(&mut self) -> RMVF_W<CSRrs> {
        RMVF_W::new(self, 24)
    }
    ///Bit 26 - PIN reset flag
    #[inline(always)]
    pub fn pinrstf(&mut self) -> PINRSTF_W<CSRrs> {
        PINRSTF_W::new(self, 26)
    }
    ///Bit 27 - POR/PDR reset flag
    #[inline(always)]
    pub fn porrstf(&mut self) -> PORRSTF_W<CSRrs> {
        PORRSTF_W::new(self, 27)
    }
    ///Bit 28 - Software reset flag
    #[inline(always)]
    pub fn sftrstf(&mut self) -> SFTRSTF_W<CSRrs> {
        SFTRSTF_W::new(self, 28)
    }
    ///Bit 29 - Independent watchdog reset flag
    #[inline(always)]
    pub fn iwdgrstf(&mut self) -> IWDGRSTF_W<CSRrs> {
        IWDGRSTF_W::new(self, 29)
    }
    ///Bit 30 - Window watchdog reset flag
    #[inline(always)]
    pub fn wwdgrstf(&mut self) -> WWDGRSTF_W<CSRrs> {
        WWDGRSTF_W::new(self, 30)
    }
    ///Bit 31 - Low-power reset flag
    #[inline(always)]
    pub fn lpwrstf(&mut self) -> LPWRSTF_W<CSRrs> {
        LPWRSTF_W::new(self, 31)
    }
}
/**Control/status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L162.html#RCC:CSR)*/
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
///`read()` method returns [`csr::R`](R) reader structure
impl crate::Readable for CSRrs {}
///`write(|w| ..)` method takes [`csr::W`](W) writer structure
impl crate::Writable for CSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR to value 0
impl crate::Resettable for CSRrs {}
