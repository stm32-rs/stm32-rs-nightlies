///Register `DAC_DHR12L2` reader
pub type R = crate::R<DAC_DHR12L2rs>;
///Register `DAC_DHR12L2` writer
pub type W = crate::W<DAC_DHR12L2rs>;
///Field `DACC2DHR` reader - DACC2DHR
pub type DACC2DHR_R = crate::FieldReader<u16>;
///Field `DACC2DHR` writer - DACC2DHR
pub type DACC2DHR_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 4:15 - DACC2DHR
    #[inline(always)]
    pub fn dacc2dhr(&self) -> DACC2DHR_R {
        DACC2DHR_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAC_DHR12L2")
            .field("dacc2dhr", &self.dacc2dhr())
            .finish()
    }
}
impl W {
    ///Bits 4:15 - DACC2DHR
    #[inline(always)]
    #[must_use]
    pub fn dacc2dhr(&mut self) -> DACC2DHR_W<DAC_DHR12L2rs> {
        DACC2DHR_W::new(self, 4)
    }
}
/**This register is available only on dual-channel DACs. Refer to Section29.3: DAC implementation.

You can [`read`](crate::Reg::read) this register and get [`dac_dhr12l2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_dhr12l2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DAC1:DAC_DHR12L2)*/
pub struct DAC_DHR12L2rs;
impl crate::RegisterSpec for DAC_DHR12L2rs {
    type Ux = u32;
}
///`read()` method returns [`dac_dhr12l2::R`](R) reader structure
impl crate::Readable for DAC_DHR12L2rs {}
///`write(|w| ..)` method takes [`dac_dhr12l2::W`](W) writer structure
impl crate::Writable for DAC_DHR12L2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DAC_DHR12L2 to value 0
impl crate::Resettable for DAC_DHR12L2rs {
    const RESET_VALUE: u32 = 0;
}
