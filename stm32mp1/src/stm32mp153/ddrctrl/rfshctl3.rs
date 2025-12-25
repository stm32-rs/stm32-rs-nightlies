///Register `RFSHCTL3` reader
pub type R = crate::R<RFSHCTL3rs>;
///Register `RFSHCTL3` writer
pub type W = crate::W<RFSHCTL3rs>;
///Field `DIS_AUTO_REFRESH` reader - DIS_AUTO_REFRESH
pub type DIS_AUTO_REFRESH_R = crate::BitReader;
///Field `DIS_AUTO_REFRESH` writer - DIS_AUTO_REFRESH
pub type DIS_AUTO_REFRESH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REFRESH_UPDATE_LEVEL` reader - REFRESH_UPDATE_LEVEL
pub type REFRESH_UPDATE_LEVEL_R = crate::BitReader;
///Field `REFRESH_UPDATE_LEVEL` writer - REFRESH_UPDATE_LEVEL
pub type REFRESH_UPDATE_LEVEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DIS_AUTO_REFRESH
    #[inline(always)]
    pub fn dis_auto_refresh(&self) -> DIS_AUTO_REFRESH_R {
        DIS_AUTO_REFRESH_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - REFRESH_UPDATE_LEVEL
    #[inline(always)]
    pub fn refresh_update_level(&self) -> REFRESH_UPDATE_LEVEL_R {
        REFRESH_UPDATE_LEVEL_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFSHCTL3")
            .field("dis_auto_refresh", &self.dis_auto_refresh())
            .field("refresh_update_level", &self.refresh_update_level())
            .finish()
    }
}
impl W {
    ///Bit 0 - DIS_AUTO_REFRESH
    #[inline(always)]
    pub fn dis_auto_refresh(&mut self) -> DIS_AUTO_REFRESH_W<'_, RFSHCTL3rs> {
        DIS_AUTO_REFRESH_W::new(self, 0)
    }
    ///Bit 1 - REFRESH_UPDATE_LEVEL
    #[inline(always)]
    pub fn refresh_update_level(&mut self) -> REFRESH_UPDATE_LEVEL_W<'_, RFSHCTL3rs> {
        REFRESH_UPDATE_LEVEL_W::new(self, 1)
    }
}
/**DDRCTRL refresh control register 3

You can [`read`](crate::Reg::read) this register and get [`rfshctl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfshctl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRCTRL:RFSHCTL3)*/
pub struct RFSHCTL3rs;
impl crate::RegisterSpec for RFSHCTL3rs {
    type Ux = u32;
}
///`read()` method returns [`rfshctl3::R`](R) reader structure
impl crate::Readable for RFSHCTL3rs {}
///`write(|w| ..)` method takes [`rfshctl3::W`](W) writer structure
impl crate::Writable for RFSHCTL3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RFSHCTL3 to value 0
impl crate::Resettable for RFSHCTL3rs {}
