///Register `SWTRIGR` reader
pub type R = crate::R<SWTRIGRrs>;
///Register `SWTRIGR` writer
pub type W = crate::W<SWTRIGRrs>;
///Field `SWTRIG` writer - SWTRIG: DAC channel software trigger This bit is set by software to enable/disable the software trigger. 0: Software trigger disabled 1: Software trigger enabled Note: This bit is cleared by hardware (one APB0 clock cycle later) once the DAC_DHR register value has been loaded into the DAC_DOR register.
pub type SWTRIG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWTRIGR").finish()
    }
}
impl W {
    ///Bit 0 - SWTRIG: DAC channel software trigger This bit is set by software to enable/disable the software trigger. 0: Software trigger disabled 1: Software trigger enabled Note: This bit is cleared by hardware (one APB0 clock cycle later) once the DAC_DHR register value has been loaded into the DAC_DOR register.
    #[inline(always)]
    pub fn swtrig(&mut self) -> SWTRIG_W<'_, SWTRIGRrs> {
        SWTRIG_W::new(self, 0)
    }
}
/**SWTRIGR register

You can [`read`](crate::Reg::read) this register and get [`swtrigr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swtrigr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DAC:SWTRIGR)*/
pub struct SWTRIGRrs;
impl crate::RegisterSpec for SWTRIGRrs {
    type Ux = u32;
}
///`read()` method returns [`swtrigr::R`](R) reader structure
impl crate::Readable for SWTRIGRrs {}
///`write(|w| ..)` method takes [`swtrigr::W`](W) writer structure
impl crate::Writable for SWTRIGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWTRIGR to value 0
impl crate::Resettable for SWTRIGRrs {}
