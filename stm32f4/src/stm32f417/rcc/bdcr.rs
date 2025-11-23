///Register `BDCR` reader
pub type R = crate::R<BDCRrs>;
///Register `BDCR` writer
pub type W = crate::W<BDCRrs>;
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
///Field `RTCSEL0` reader - RTC clock source selection
pub type RTCSEL0_R = crate::BitReader;
///Field `RTCSEL0` writer - RTC clock source selection
pub type RTCSEL0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTCSEL1` reader - RTC clock source selection
pub type RTCSEL1_R = crate::BitReader;
///Field `RTCSEL1` writer - RTC clock source selection
pub type RTCSEL1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTCEN` reader - RTC clock enable
pub type RTCEN_R = crate::BitReader;
///Field `RTCEN` writer - RTC clock enable
pub type RTCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BDRST` reader - Backup domain software reset
pub type BDRST_R = crate::BitReader;
///Field `BDRST` writer - Backup domain software reset
pub type BDRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - External low-speed oscillator enable
    #[inline(always)]
    pub fn lseon(&self) -> LSEON_R {
        LSEON_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - External low-speed oscillator ready
    #[inline(always)]
    pub fn lserdy(&self) -> LSERDY_R {
        LSERDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - External low-speed oscillator bypass
    #[inline(always)]
    pub fn lsebyp(&self) -> LSEBYP_R {
        LSEBYP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 8 - RTC clock source selection
    #[inline(always)]
    pub fn rtcsel0(&self) -> RTCSEL0_R {
        RTCSEL0_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - RTC clock source selection
    #[inline(always)]
    pub fn rtcsel1(&self) -> RTCSEL1_R {
        RTCSEL1_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 15 - RTC clock enable
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Backup domain software reset
    #[inline(always)]
    pub fn bdrst(&self) -> BDRST_R {
        BDRST_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BDCR")
            .field("bdrst", &self.bdrst())
            .field("rtcen", &self.rtcen())
            .field("rtcsel1", &self.rtcsel1())
            .field("rtcsel0", &self.rtcsel0())
            .field("lsebyp", &self.lsebyp())
            .field("lserdy", &self.lserdy())
            .field("lseon", &self.lseon())
            .finish()
    }
}
impl W {
    ///Bit 0 - External low-speed oscillator enable
    #[inline(always)]
    pub fn lseon(&mut self) -> LSEON_W<'_, BDCRrs> {
        LSEON_W::new(self, 0)
    }
    ///Bit 2 - External low-speed oscillator bypass
    #[inline(always)]
    pub fn lsebyp(&mut self) -> LSEBYP_W<'_, BDCRrs> {
        LSEBYP_W::new(self, 2)
    }
    ///Bit 8 - RTC clock source selection
    #[inline(always)]
    pub fn rtcsel0(&mut self) -> RTCSEL0_W<'_, BDCRrs> {
        RTCSEL0_W::new(self, 8)
    }
    ///Bit 9 - RTC clock source selection
    #[inline(always)]
    pub fn rtcsel1(&mut self) -> RTCSEL1_W<'_, BDCRrs> {
        RTCSEL1_W::new(self, 9)
    }
    ///Bit 15 - RTC clock enable
    #[inline(always)]
    pub fn rtcen(&mut self) -> RTCEN_W<'_, BDCRrs> {
        RTCEN_W::new(self, 15)
    }
    ///Bit 16 - Backup domain software reset
    #[inline(always)]
    pub fn bdrst(&mut self) -> BDRST_W<'_, BDCRrs> {
        BDRST_W::new(self, 16)
    }
}
/**Backup domain control register

You can [`read`](crate::Reg::read) this register and get [`bdcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#RCC:BDCR)*/
pub struct BDCRrs;
impl crate::RegisterSpec for BDCRrs {
    type Ux = u32;
}
///`read()` method returns [`bdcr::R`](R) reader structure
impl crate::Readable for BDCRrs {}
///`write(|w| ..)` method takes [`bdcr::W`](W) writer structure
impl crate::Writable for BDCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BDCR to value 0
impl crate::Resettable for BDCRrs {}
