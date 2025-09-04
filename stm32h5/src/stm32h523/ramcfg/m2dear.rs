///Register `M2DEAR` reader
pub type R = crate::R<M2DEARrs>;
///Field `EDEA` reader - ECC double error address
pub type EDEA_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - ECC double error address
    #[inline(always)]
    pub fn edea(&self) -> EDEA_R {
        EDEA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M2DEAR")
            .field("edea", &self.edea())
            .finish()
    }
}
/**RAMCFG memory 2 ECC double error address register

You can [`read`](crate::Reg::read) this register and get [`m2dear::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#RAMCFG:M2DEAR)*/
pub struct M2DEARrs;
impl crate::RegisterSpec for M2DEARrs {
    type Ux = u32;
}
///`read()` method returns [`m2dear::R`](R) reader structure
impl crate::Readable for M2DEARrs {}
///`reset()` method sets M2DEAR to value 0
impl crate::Resettable for M2DEARrs {}
