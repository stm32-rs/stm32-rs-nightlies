///Register `EISR0` reader
pub type R = crate::R<EISR0rs>;
///Field `EISR0` reader - EISR0
pub type EISR0_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - EISR0
    #[inline(always)]
    pub fn eisr0(&self) -> EISR0_R {
        EISR0_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EISR0")
            .field("eisr0", &self.eisr0())
            .finish()
    }
}
/**GICH end of interrupt status register

You can [`read`](crate::Reg::read) this register and get [`eisr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICH:EISR0)*/
pub struct EISR0rs;
impl crate::RegisterSpec for EISR0rs {
    type Ux = u32;
}
///`read()` method returns [`eisr0::R`](R) reader structure
impl crate::Readable for EISR0rs {}
///`reset()` method sets EISR0 to value 0
impl crate::Resettable for EISR0rs {}
