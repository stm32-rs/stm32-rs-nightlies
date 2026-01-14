///Register `SYSCFG_ITLINE12` reader
pub type R = crate::R<SYSCFG_ITLINE12rs>;
///Field `ADC` reader - ADC interrupt request pending
pub type ADC_R = crate::BitReader;
impl R {
    ///Bit 0 - ADC interrupt request pending
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSCFG_ITLINE12")
            .field("adc", &self.adc())
            .finish()
    }
}
/**SYSCFG interrupt line 12 status register

You can [`read`](crate::Reg::read) this register and get [`syscfg_itline12::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#SYSCFG:SYSCFG_ITLINE12)*/
pub struct SYSCFG_ITLINE12rs;
impl crate::RegisterSpec for SYSCFG_ITLINE12rs {
    type Ux = u32;
}
///`read()` method returns [`syscfg_itline12::R`](R) reader structure
impl crate::Readable for SYSCFG_ITLINE12rs {}
///`reset()` method sets SYSCFG_ITLINE12 to value 0
impl crate::Resettable for SYSCFG_ITLINE12rs {}
