///Register `SDMMC_DTIMER` reader
pub type R = crate::R<SDMMC_DTIMERrs>;
///Register `SDMMC_DTIMER` writer
pub type W = crate::W<SDMMC_DTIMERrs>;
///Field `DATATIME` reader - Data and R1b busy timeout period This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0). Data and R1b busy timeout period expressed in card bus clock periods.
pub type DATATIME_R = crate::FieldReader<u32>;
///Field `DATATIME` writer - Data and R1b busy timeout period This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0). Data and R1b busy timeout period expressed in card bus clock periods.
pub type DATATIME_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Data and R1b busy timeout period This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0). Data and R1b busy timeout period expressed in card bus clock periods.
    #[inline(always)]
    pub fn datatime(&self) -> DATATIME_R {
        DATATIME_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDMMC_DTIMER")
            .field("datatime", &self.datatime())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Data and R1b busy timeout period This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0). Data and R1b busy timeout period expressed in card bus clock periods.
    #[inline(always)]
    pub fn datatime(&mut self) -> DATATIME_W<SDMMC_DTIMERrs> {
        DATATIME_W::new(self, 0)
    }
}
/**The SDMMC_DTIMER register contains the data timeout period, in card bus clock periods. A counter loads the value from the SDMMC_DTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_R or Busy state. If the timer reaches 0 while the DPSM is in either of these states, the timeout status flag is set.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_dtimer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_dtimer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#SDMMC1:SDMMC_DTIMER)*/
pub struct SDMMC_DTIMERrs;
impl crate::RegisterSpec for SDMMC_DTIMERrs {
    type Ux = u32;
}
///`read()` method returns [`sdmmc_dtimer::R`](R) reader structure
impl crate::Readable for SDMMC_DTIMERrs {}
///`write(|w| ..)` method takes [`sdmmc_dtimer::W`](W) writer structure
impl crate::Writable for SDMMC_DTIMERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SDMMC_DTIMER to value 0
impl crate::Resettable for SDMMC_DTIMERrs {
    const RESET_VALUE: u32 = 0;
}
