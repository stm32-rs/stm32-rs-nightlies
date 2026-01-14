///Register `DINR18` reader
pub type R = crate::R<DINR18rs>;
///Field `DIN` reader - input data received from MDIO master during write frames
pub type DIN_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - input data received from MDIO master during write frames
    #[inline(always)]
    pub fn din(&self) -> DIN_R {
        DIN_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DINR18").field("din", &self.din()).finish()
    }
}
/**MDIOS input data register 18

You can [`read`](crate::Reg::read) this register and get [`dinr18::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#MDIOS:DINR18)*/
pub struct DINR18rs;
impl crate::RegisterSpec for DINR18rs {
    type Ux = u32;
}
///`read()` method returns [`dinr18::R`](R) reader structure
impl crate::Readable for DINR18rs {}
///`reset()` method sets DINR18 to value 0
impl crate::Resettable for DINR18rs {}
