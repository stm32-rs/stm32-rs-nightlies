///Register `IADDR` reader
pub type R = crate::R<IADDRrs>;
///Field `IADD` reader - Illegal address
pub type IADD_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Illegal address
    #[inline(always)]
    pub fn iadd(&self) -> IADD_R {
        IADD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IADDR").field("iadd", &self.iadd()).finish()
    }
}
/**MCE illegal address register

You can [`read`](crate::Reg::read) this register and get [`iaddr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MCE1:IADDR)*/
pub struct IADDRrs;
impl crate::RegisterSpec for IADDRrs {
    type Ux = u32;
}
///`read()` method returns [`iaddr::R`](R) reader structure
impl crate::Readable for IADDRrs {}
///`reset()` method sets IADDR to value 0
impl crate::Resettable for IADDRrs {}
