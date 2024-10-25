///Register `AUTOCR` reader
pub type R = crate::R<AUTOCRrs>;
///Register `AUTOCR` writer
pub type W = crate::W<AUTOCRrs>;
///Field `AUTOMODE` reader - DAC Autonomous mode
pub type AUTOMODE_R = crate::BitReader;
///Field `AUTOMODE` writer - DAC Autonomous mode
pub type AUTOMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 22 - DAC Autonomous mode
    #[inline(always)]
    pub fn automode(&self) -> AUTOMODE_R {
        AUTOMODE_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUTOCR")
            .field("automode", &self.automode())
            .finish()
    }
}
impl W {
    ///Bit 22 - DAC Autonomous mode
    #[inline(always)]
    #[must_use]
    pub fn automode(&mut self) -> AUTOMODE_W<AUTOCRrs> {
        AUTOMODE_W::new(self, 22)
    }
}
/**Autonomous mode control register

You can [`read`](crate::Reg::read) this register and get [`autocr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`autocr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#DAC1:AUTOCR)*/
pub struct AUTOCRrs;
impl crate::RegisterSpec for AUTOCRrs {
    type Ux = u32;
}
///`read()` method returns [`autocr::R`](R) reader structure
impl crate::Readable for AUTOCRrs {}
///`write(|w| ..)` method takes [`autocr::W`](W) writer structure
impl crate::Writable for AUTOCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AUTOCR to value 0
impl crate::Resettable for AUTOCRrs {
    const RESET_VALUE: u32 = 0;
}
