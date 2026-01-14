///Register `DINR26` reader
pub type R = crate::R<DINR26rs>;
///Field `DIN26` reader - Input data received from MDIO Master during write frames
pub type DIN26_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Input data received from MDIO Master during write frames
    #[inline(always)]
    pub fn din26(&self) -> DIN26_R {
        DIN26_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DINR26")
            .field("din26", &self.din26())
            .finish()
    }
}
/**MDIOS input data register 26

You can [`read`](crate::Reg::read) this register and get [`dinr26::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#MDIOS:DINR26)*/
pub struct DINR26rs;
impl crate::RegisterSpec for DINR26rs {
    type Ux = u32;
}
///`read()` method returns [`dinr26::R`](R) reader structure
impl crate::Readable for DINR26rs {}
///`reset()` method sets DINR26 to value 0
impl crate::Resettable for DINR26rs {}
