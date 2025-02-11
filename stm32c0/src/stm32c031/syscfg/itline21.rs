///Register `ITLINE21` reader
pub type R = crate::R<ITLINE21rs>;
///Field `TIM16` reader - Timer 16 interrupt request pending
pub type TIM16_R = crate::BitReader;
impl R {
    ///Bit 0 - Timer 16 interrupt request pending
    #[inline(always)]
    pub fn tim16(&self) -> TIM16_R {
        TIM16_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE21")
            .field("tim16", &self.tim16())
            .finish()
    }
}
/**SYSCFG interrupt line 21 status register

You can [`read`](crate::Reg::read) this register and get [`itline21::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#SYSCFG:ITLINE21)*/
pub struct ITLINE21rs;
impl crate::RegisterSpec for ITLINE21rs {
    type Ux = u32;
}
///`read()` method returns [`itline21::R`](R) reader structure
impl crate::Readable for ITLINE21rs {}
///`reset()` method sets ITLINE21 to value 0
impl crate::Resettable for ITLINE21rs {
    const RESET_VALUE: u32 = 0;
}
