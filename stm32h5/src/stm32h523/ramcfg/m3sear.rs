///Register `M3SEAR` reader
pub type R = crate::R<M3SEARrs>;
///Field `ESEA` reader - ECC single error address
pub type ESEA_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - ECC single error address
    #[inline(always)]
    pub fn esea(&self) -> ESEA_R {
        ESEA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M3SEAR")
            .field("esea", &self.esea())
            .finish()
    }
}
/**RAMCFG memory 3 ECC single error address register

You can [`read`](crate::Reg::read) this register and get [`m3sear::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#RAMCFG:M3SEAR)*/
pub struct M3SEARrs;
impl crate::RegisterSpec for M3SEARrs {
    type Ux = u32;
}
///`read()` method returns [`m3sear::R`](R) reader structure
impl crate::Readable for M3SEARrs {}
///`reset()` method sets M3SEAR to value 0
impl crate::Resettable for M3SEARrs {}
