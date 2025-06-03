///Register `DINR12` reader
pub type R = crate::R<DINR12rs>;
///Field `DIN12` reader - Input data received from MDIO Master during write frames
pub type DIN12_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Input data received from MDIO Master during write frames
    #[inline(always)]
    pub fn din12(&self) -> DIN12_R {
        DIN12_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DINR12")
            .field("din12", &self.din12())
            .finish()
    }
}
/**MDIOS input data register 12

You can [`read`](crate::Reg::read) this register and get [`dinr12::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#MDIOS:DINR12)*/
pub struct DINR12rs;
impl crate::RegisterSpec for DINR12rs {
    type Ux = u32;
}
///`read()` method returns [`dinr12::R`](R) reader structure
impl crate::Readable for DINR12rs {}
///`reset()` method sets DINR12 to value 0
impl crate::Resettable for DINR12rs {}
