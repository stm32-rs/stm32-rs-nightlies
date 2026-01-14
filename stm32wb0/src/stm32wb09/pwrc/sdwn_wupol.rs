///Register `SDWN_WUPOL` reader
pub type R = crate::R<SDWN_WUPOLrs>;
///Register `SDWN_WUPOL` writer
pub type W = crate::W<SDWN_WUPOLrs>;
///Field `WUPOL` reader - PB0 I/O wakeup from shutdown polarity.
pub type WUPOL_R = crate::BitReader;
///Field `WUPOL` writer - PB0 I/O wakeup from shutdown polarity.
pub type WUPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - PB0 I/O wakeup from shutdown polarity.
    #[inline(always)]
    pub fn wupol(&self) -> WUPOL_R {
        WUPOL_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDWN_WUPOL")
            .field("wupol", &self.wupol())
            .finish()
    }
}
impl W {
    ///Bit 0 - PB0 I/O wakeup from shutdown polarity.
    #[inline(always)]
    pub fn wupol(&mut self) -> WUPOL_W<'_, SDWN_WUPOLrs> {
        WUPOL_W::new(self, 0)
    }
}
/**Contains Shutdown wakeup polarity fields for IO.

You can [`read`](crate::Reg::read) this register and get [`sdwn_wupol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdwn_wupol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#PWRC:SDWN_WUPOL)*/
pub struct SDWN_WUPOLrs;
impl crate::RegisterSpec for SDWN_WUPOLrs {
    type Ux = u32;
}
///`read()` method returns [`sdwn_wupol::R`](R) reader structure
impl crate::Readable for SDWN_WUPOLrs {}
///`write(|w| ..)` method takes [`sdwn_wupol::W`](W) writer structure
impl crate::Writable for SDWN_WUPOLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SDWN_WUPOL to value 0
impl crate::Resettable for SDWN_WUPOLrs {}
