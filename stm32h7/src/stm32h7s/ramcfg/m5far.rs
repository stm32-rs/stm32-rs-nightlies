///Register `M5FAR` reader
pub type R = crate::R<M5FARrs>;
///Field `FADD` reader - ECC error failing address When an ECC error occurs the FADD bitfield contains the address that generated the ECC error.
pub type FADD_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - ECC error failing address When an ECC error occurs the FADD bitfield contains the address that generated the ECC error.
    #[inline(always)]
    pub fn fadd(&self) -> FADD_R {
        FADD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M5FAR").field("fadd", &self.fadd()).finish()
    }
}
/**RAMECC monitor 5 failing address register

You can [`read`](crate::Reg::read) this register and get [`m5far::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#RAMCFG:M5FAR)*/
pub struct M5FARrs;
impl crate::RegisterSpec for M5FARrs {
    type Ux = u32;
}
///`read()` method returns [`m5far::R`](R) reader structure
impl crate::Readable for M5FARrs {}
///`reset()` method sets M5FAR to value 0
impl crate::Resettable for M5FARrs {}
