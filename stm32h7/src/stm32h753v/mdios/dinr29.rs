///Register `DINR29` reader
pub type R = crate::R<DINR29rs>;
///Field `DIN29` reader - Input data received from MDIO Master during write frames
pub type DIN29_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Input data received from MDIO Master during write frames
    #[inline(always)]
    pub fn din29(&self) -> DIN29_R {
        DIN29_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DINR29")
            .field("din29", &self.din29())
            .finish()
    }
}
/**MDIOS input data register 29

You can [`read`](crate::Reg::read) this register and get [`dinr29::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753V.html#MDIOS:DINR29)*/
pub struct DINR29rs;
impl crate::RegisterSpec for DINR29rs {
    type Ux = u32;
}
///`read()` method returns [`dinr29::R`](R) reader structure
impl crate::Readable for DINR29rs {}
///`reset()` method sets DINR29 to value 0
impl crate::Resettable for DINR29rs {}
