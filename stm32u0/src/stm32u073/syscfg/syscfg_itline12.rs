///Register `SYSCFG_ITLINE12` reader
pub type R = crate::R<SYSCFG_ITLINE12rs>;
///Field `ADC` reader - ADC interrupt request pending
pub type ADC_R = crate::BitReader;
///Field `COMP1` reader - Comparator 1 interrupt request pending (EXTI line 17)
pub type COMP1_R = crate::BitReader;
///Field `COMP2` reader - Comparator 2 interrupt request pending (EXTI line 18)
pub type COMP2_R = crate::BitReader;
impl R {
    ///Bit 0 - ADC interrupt request pending
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Comparator 1 interrupt request pending (EXTI line 17)
    #[inline(always)]
    pub fn comp1(&self) -> COMP1_R {
        COMP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Comparator 2 interrupt request pending (EXTI line 18)
    #[inline(always)]
    pub fn comp2(&self) -> COMP2_R {
        COMP2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSCFG_ITLINE12")
            .field("adc", &self.adc())
            .field("comp1", &self.comp1())
            .field("comp2", &self.comp2())
            .finish()
    }
}
/**SYSCFG interrupt line 12 status register

You can [`read`](crate::Reg::read) this register and get [`syscfg_itline12::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#SYSCFG:SYSCFG_ITLINE12)*/
pub struct SYSCFG_ITLINE12rs;
impl crate::RegisterSpec for SYSCFG_ITLINE12rs {
    type Ux = u32;
}
///`read()` method returns [`syscfg_itline12::R`](R) reader structure
impl crate::Readable for SYSCFG_ITLINE12rs {}
///`reset()` method sets SYSCFG_ITLINE12 to value 0
impl crate::Resettable for SYSCFG_ITLINE12rs {
    const RESET_VALUE: u32 = 0;
}