///Register `ITLINE1` reader
pub type R = crate::R<ITLINE1rs>;
///Field `PVM_VDDIO2_OUT` reader - V<sub>DDIO2</sub> supply monitoring interrupt request pending (EXTI line 34)
pub type PVM_VDDIO2_OUT_R = crate::BitReader;
impl R {
    ///Bit 1 - V<sub>DDIO2</sub> supply monitoring interrupt request pending (EXTI line 34)
    #[inline(always)]
    pub fn pvm_vddio2_out(&self) -> PVM_VDDIO2_OUT_R {
        PVM_VDDIO2_OUT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE1")
            .field("pvm_vddio2_out", &self.pvm_vddio2_out())
            .finish()
    }
}
/**SYSCFG interrupt line 1 status register

You can [`read`](crate::Reg::read) this register and get [`itline1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#SYSCFG:ITLINE1)*/
pub struct ITLINE1rs;
impl crate::RegisterSpec for ITLINE1rs {
    type Ux = u32;
}
///`read()` method returns [`itline1::R`](R) reader structure
impl crate::Readable for ITLINE1rs {}
///`reset()` method sets ITLINE1 to value 0
impl crate::Resettable for ITLINE1rs {
    const RESET_VALUE: u32 = 0;
}
