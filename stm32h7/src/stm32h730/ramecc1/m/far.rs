///Register `FAR` reader
pub type R = crate::R<FARrs>;
///Field `FADD` reader - ECC error failing address
pub type FADD_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - ECC error failing address
    #[inline(always)]
    pub fn fadd(&self) -> FADD_R {
        FADD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FAR").field("fadd", &self.fadd()).finish()
    }
}
/**RAMECC monitor x failing address register

You can [`read`](crate::Reg::read) this register and get [`far::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FARrs;
impl crate::RegisterSpec for FARrs {
    type Ux = u32;
}
///`read()` method returns [`far::R`](R) reader structure
impl crate::Readable for FARrs {}
///`reset()` method sets FAR to value 0
impl crate::Resettable for FARrs {}
