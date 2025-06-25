///Register `DINR11` reader
pub type R = crate::R<DINR11rs>;
///Field `DIN11` reader - Input data received from MDIO Master during write frames
pub type DIN11_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Input data received from MDIO Master during write frames
    #[inline(always)]
    pub fn din11(&self) -> DIN11_R {
        DIN11_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DINR11")
            .field("din11", &self.din11())
            .finish()
    }
}
/**MDIOS input data register 11

You can [`read`](crate::Reg::read) this register and get [`dinr11::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#MDIOS:DINR11)*/
pub struct DINR11rs;
impl crate::RegisterSpec for DINR11rs {
    type Ux = u32;
}
///`read()` method returns [`dinr11::R`](R) reader structure
impl crate::Readable for DINR11rs {}
///`reset()` method sets DINR11 to value 0
impl crate::Resettable for DINR11rs {}
