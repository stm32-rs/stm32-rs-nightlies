///Register `DINR25` reader
pub type R = crate::R<DINR25rs>;
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
        f.debug_struct("DINR25").field("din", &self.din()).finish()
    }
}
/**MDIOS input data register 25

You can [`read`](crate::Reg::read) this register and get [`dinr25::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#MDIOS:DINR25)*/
pub struct DINR25rs;
impl crate::RegisterSpec for DINR25rs {
    type Ux = u32;
}
///`read()` method returns [`dinr25::R`](R) reader structure
impl crate::Readable for DINR25rs {}
///`reset()` method sets DINR25 to value 0
impl crate::Resettable for DINR25rs {}
