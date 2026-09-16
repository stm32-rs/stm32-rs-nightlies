///Register `DINR17` reader
pub type R = crate::R<DINR17rs>;
///Field `DIN17` reader - Input data received from MDIO Master during write frames
pub type DIN17_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Input data received from MDIO Master during write frames
    #[inline(always)]
    pub fn din17(&self) -> DIN17_R {
        DIN17_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DINR17")
            .field("din17", &self.din17())
            .finish()
    }
}
/**MDIOS input data register 17

You can [`read`](crate::Reg::read) this register and get [`dinr17::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753.html#MDIOS:DINR17)*/
pub struct DINR17rs;
impl crate::RegisterSpec for DINR17rs {
    type Ux = u32;
}
///`read()` method returns [`dinr17::R`](R) reader structure
impl crate::Readable for DINR17rs {}
///`reset()` method sets DINR17 to value 0
impl crate::Resettable for DINR17rs {}
