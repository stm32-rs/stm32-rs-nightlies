///Register `DINR6` reader
pub type R = crate::R<DINR6rs>;
///Field `DIN6` reader - Input data received from MDIO Master during write frames
pub type DIN6_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Input data received from MDIO Master during write frames
    #[inline(always)]
    pub fn din6(&self) -> DIN6_R {
        DIN6_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DINR6").field("din6", &self.din6()).finish()
    }
}
/**MDIOS input data register 6

You can [`read`](crate::Reg::read) this register and get [`dinr6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#MDIOS:DINR6)*/
pub struct DINR6rs;
impl crate::RegisterSpec for DINR6rs {
    type Ux = u32;
}
///`read()` method returns [`dinr6::R`](R) reader structure
impl crate::Readable for DINR6rs {}
///`reset()` method sets DINR6 to value 0
impl crate::Resettable for DINR6rs {}
