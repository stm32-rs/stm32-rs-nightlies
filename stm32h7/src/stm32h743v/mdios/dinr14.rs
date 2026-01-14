///Register `DINR14` reader
pub type R = crate::R<DINR14rs>;
///Field `DIN14` reader - Input data received from MDIO Master during write frames
pub type DIN14_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Input data received from MDIO Master during write frames
    #[inline(always)]
    pub fn din14(&self) -> DIN14_R {
        DIN14_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DINR14")
            .field("din14", &self.din14())
            .finish()
    }
}
/**MDIOS input data register 14

You can [`read`](crate::Reg::read) this register and get [`dinr14::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743V.html#MDIOS:DINR14)*/
pub struct DINR14rs;
impl crate::RegisterSpec for DINR14rs {
    type Ux = u32;
}
///`read()` method returns [`dinr14::R`](R) reader structure
impl crate::Readable for DINR14rs {}
///`reset()` method sets DINR14 to value 0
impl crate::Resettable for DINR14rs {}
