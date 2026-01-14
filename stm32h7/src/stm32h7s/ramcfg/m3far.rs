///Register `M3FAR` reader
pub type R = crate::R<M3FARrs>;
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
        f.debug_struct("M3FAR").field("fadd", &self.fadd()).finish()
    }
}
/**RAMECC monitor 3 failing address register

You can [`read`](crate::Reg::read) this register and get [`m3far::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#RAMCFG:M3FAR)*/
pub struct M3FARrs;
impl crate::RegisterSpec for M3FARrs {
    type Ux = u32;
}
///`read()` method returns [`m3far::R`](R) reader structure
impl crate::Readable for M3FARrs {}
///`reset()` method sets M3FAR to value 0
impl crate::Resettable for M3FARrs {}
