///Register `DTIMER` reader
pub type R = crate::R<DTIMERrs>;
///Register `DTIMER` writer
pub type W = crate::W<DTIMERrs>;
///Field `DATATIME` reader - DATATIME
pub type DATATIME_R = crate::FieldReader<u32>;
///Field `DATATIME` writer - DATATIME
pub type DATATIME_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - DATATIME
    #[inline(always)]
    pub fn datatime(&self) -> DATATIME_R {
        DATATIME_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTIMER")
            .field("datatime", &self.datatime())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - DATATIME
    #[inline(always)]
    pub fn datatime(&mut self) -> DATATIME_W<DTIMERrs> {
        DATATIME_W::new(self, 0)
    }
}
/**The SDMMC_DTIMER register contains the data timeout period, in card bus clock periods. A counter loads the value from the SDMMC_DTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_R or Busy state. If the timer reaches 0 while the DPSM is in either of these states, the timeout status flag is set.

You can [`read`](crate::Reg::read) this register and get [`dtimer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtimer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:DTIMER)*/
pub struct DTIMERrs;
impl crate::RegisterSpec for DTIMERrs {
    type Ux = u32;
}
///`read()` method returns [`dtimer::R`](R) reader structure
impl crate::Readable for DTIMERrs {}
///`write(|w| ..)` method takes [`dtimer::W`](W) writer structure
impl crate::Writable for DTIMERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DTIMER to value 0
impl crate::Resettable for DTIMERrs {}
