///Register `PIDR6` reader
pub type R = crate::R<PIDR6rs>;
///Field `PIDR6` reader - PIDR6
pub type PIDR6_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - PIDR6
    #[inline(always)]
    pub fn pidr6(&self) -> PIDR6_R {
        PIDR6_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIDR6")
            .field("pidr6", &self.pidr6())
            .finish()
    }
}
/**STGENC peripheral ID6 register

You can [`read`](crate::Reg::read) this register and get [`pidr6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#STGENC:PIDR6)*/
pub struct PIDR6rs;
impl crate::RegisterSpec for PIDR6rs {
    type Ux = u32;
}
///`read()` method returns [`pidr6::R`](R) reader structure
impl crate::Readable for PIDR6rs {}
///`reset()` method sets PIDR6 to value 0
impl crate::Resettable for PIDR6rs {}
