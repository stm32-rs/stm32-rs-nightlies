///Register `ITLINE1` reader
pub type R = crate::R<ITLINE1rs>;
///Field `PVDOUT` reader - PVD supply monitoring interrupt request pending (EXTI line 16).
pub type PVDOUT_R = crate::BitReader;
impl R {
    ///Bit 0 - PVD supply monitoring interrupt request pending (EXTI line 16).
    #[inline(always)]
    pub fn pvdout(&self) -> PVDOUT_R {
        PVDOUT_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE1")
            .field("pvdout", &self.pvdout())
            .finish()
    }
}
/**interrupt line 1 status register

You can [`read`](crate::Reg::read) this register and get [`itline1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G041.html#SYSCFG_ITLINE:ITLINE1)*/
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
