///Register `ITLINE12` reader
pub type R = crate::R<ITLINE12rs>;
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
        f.debug_struct("ITLINE12")
            .field("adc", &self.adc())
            .finish()
    }
}
/**SYSCFG interrupt line 12 status register

You can [`read`](crate::Reg::read) this register and get [`itline12::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#SYSCFG:ITLINE12)*/
pub struct ITLINE12rs;
impl crate::RegisterSpec for ITLINE12rs {
    type Ux = u32;
}
///`read()` method returns [`itline12::R`](R) reader structure
impl crate::Readable for ITLINE12rs {}
///`reset()` method sets ITLINE12 to value 0
impl crate::Resettable for ITLINE12rs {
    const RESET_VALUE: u32 = 0;
}
