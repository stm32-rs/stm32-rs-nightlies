///Register `DINR5` reader
pub type R = crate::R<DINR5rs>;
///Field `DIN` reader - input data received from MDIO master during write frames This field is written by hardware with the 16-bit data received in a write frame which is addressed to MDIOS register x.
pub type DIN_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - input data received from MDIO master during write frames This field is written by hardware with the 16-bit data received in a write frame which is addressed to MDIOS register x.
    #[inline(always)]
    pub fn din(&self) -> DIN_R {
        DIN_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DINR5").field("din", &self.din()).finish()
    }
}
/**MDIOS input data register 5

You can [`read`](crate::Reg::read) this register and get [`dinr5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#MDIOS:DINR5)*/
pub struct DINR5rs;
impl crate::RegisterSpec for DINR5rs {
    type Ux = u32;
}
///`read()` method returns [`dinr5::R`](R) reader structure
impl crate::Readable for DINR5rs {}
///`reset()` method sets DINR5 to value 0
impl crate::Resettable for DINR5rs {}
