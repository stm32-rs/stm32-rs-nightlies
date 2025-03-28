///Register `DINR7` reader
pub type R = crate::R<DINR7rs>;
///Field `DIN7` reader - Input data received from MDIO Master during write frames
pub type DIN7_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Input data received from MDIO Master during write frames
    #[inline(always)]
    pub fn din7(&self) -> DIN7_R {
        DIN7_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DINR7").field("din7", &self.din7()).finish()
    }
}
/**MDIOS input data register 7

You can [`read`](crate::Reg::read) this register and get [`dinr7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#MDIOS:DINR7)*/
pub struct DINR7rs;
impl crate::RegisterSpec for DINR7rs {
    type Ux = u32;
}
///`read()` method returns [`dinr7::R`](R) reader structure
impl crate::Readable for DINR7rs {}
///`reset()` method sets DINR7 to value 0
impl crate::Resettable for DINR7rs {}
