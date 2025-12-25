///Register `DINR13` reader
pub type R = crate::R<DINR13rs>;
///Field `DIN13` reader - Input data received from MDIO Master during write frames
pub type DIN13_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Input data received from MDIO Master during write frames
    #[inline(always)]
    pub fn din13(&self) -> DIN13_R {
        DIN13_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DINR13")
            .field("din13", &self.din13())
            .finish()
    }
}
/**MDIOS input data register 13

You can [`read`](crate::Reg::read) this register and get [`dinr13::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#MDIOS:DINR13)*/
pub struct DINR13rs;
impl crate::RegisterSpec for DINR13rs {
    type Ux = u32;
}
///`read()` method returns [`dinr13::R`](R) reader structure
impl crate::Readable for DINR13rs {}
///`reset()` method sets DINR13 to value 0
impl crate::Resettable for DINR13rs {}
