///Register `DINR28` reader
pub type R = crate::R<DINR28rs>;
///Field `DIN28` reader - Input data received from MDIO Master during write frames
pub type DIN28_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Input data received from MDIO Master during write frames
    #[inline(always)]
    pub fn din28(&self) -> DIN28_R {
        DIN28_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DINR28")
            .field("din28", &self.din28())
            .finish()
    }
}
/**MDIOS input data register 28

You can [`read`](crate::Reg::read) this register and get [`dinr28::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#MDIOS:DINR28)*/
pub struct DINR28rs;
impl crate::RegisterSpec for DINR28rs {
    type Ux = u32;
}
///`read()` method returns [`dinr28::R`](R) reader structure
impl crate::Readable for DINR28rs {}
///`reset()` method sets DINR28 to value 0
impl crate::Resettable for DINR28rs {}
