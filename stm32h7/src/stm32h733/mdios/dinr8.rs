///Register `DINR8` reader
pub type R = crate::R<DINR8rs>;
///Field `DIN8` reader - Input data received from MDIO Master during write frames
pub type DIN8_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Input data received from MDIO Master during write frames
    #[inline(always)]
    pub fn din8(&self) -> DIN8_R {
        DIN8_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DINR8").field("din8", &self.din8()).finish()
    }
}
/**MDIOS input data register 8

You can [`read`](crate::Reg::read) this register and get [`dinr8::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#MDIOS:DINR8)*/
pub struct DINR8rs;
impl crate::RegisterSpec for DINR8rs {
    type Ux = u32;
}
///`read()` method returns [`dinr8::R`](R) reader structure
impl crate::Readable for DINR8rs {}
///`reset()` method sets DINR8 to value 0
impl crate::Resettable for DINR8rs {}
