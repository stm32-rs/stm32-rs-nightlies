///Register `CCU_IE` reader
pub type R = crate::R<CCU_IErs>;
///Register `CCU_IE` writer
pub type W = crate::W<CCU_IErs>;
///Field `CWEE` reader - Calibration watchdog event enable
pub type CWEE_R = crate::BitReader;
///Field `CWEE` writer - Calibration watchdog event enable
pub type CWEE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSCE` reader - Calibration state changed enable
pub type CSCE_R = crate::BitReader;
///Field `CSCE` writer - Calibration state changed enable
pub type CSCE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Calibration watchdog event enable
    #[inline(always)]
    pub fn cwee(&self) -> CWEE_R {
        CWEE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Calibration state changed enable
    #[inline(always)]
    pub fn csce(&self) -> CSCE_R {
        CSCE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCU_IE")
            .field("cwee", &self.cwee())
            .field("csce", &self.csce())
            .finish()
    }
}
impl W {
    ///Bit 0 - Calibration watchdog event enable
    #[inline(always)]
    pub fn cwee(&mut self) -> CWEE_W<'_, CCU_IErs> {
        CWEE_W::new(self, 0)
    }
    ///Bit 1 - Calibration state changed enable
    #[inline(always)]
    pub fn csce(&mut self) -> CSCE_W<'_, CCU_IErs> {
        CSCE_W::new(self, 1)
    }
}
/**FDCAN RAM watchdog register

You can [`read`](crate::Reg::read) this register and get [`ccu_ie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccu_ie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:CCU_IE)*/
pub struct CCU_IErs;
impl crate::RegisterSpec for CCU_IErs {
    type Ux = u32;
}
///`read()` method returns [`ccu_ie::R`](R) reader structure
impl crate::Readable for CCU_IErs {}
///`write(|w| ..)` method takes [`ccu_ie::W`](W) writer structure
impl crate::Writable for CCU_IErs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCU_IE to value 0
impl crate::Resettable for CCU_IErs {}
