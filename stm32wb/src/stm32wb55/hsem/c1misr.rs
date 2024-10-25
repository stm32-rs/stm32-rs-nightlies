///Register `C1MISR` reader
pub type R = crate::R<C1MISRrs>;
///Field `MISFm` reader - masked CPU(n) semaphore m status bit after enable (mask).
pub type MISFM_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - masked CPU(n) semaphore m status bit after enable (mask).
    #[inline(always)]
    pub fn misfm(&self) -> MISFM_R {
        MISFM_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C1MISR")
            .field("misfm", &self.misfm())
            .finish()
    }
}
/**HSEM Masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`c1misr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#HSEM:C1MISR)*/
pub struct C1MISRrs;
impl crate::RegisterSpec for C1MISRrs {
    type Ux = u32;
}
///`read()` method returns [`c1misr::R`](R) reader structure
impl crate::Readable for C1MISRrs {}
///`reset()` method sets C1MISR to value 0
impl crate::Resettable for C1MISRrs {
    const RESET_VALUE: u32 = 0;
}
