///Register `DINR2` reader
pub type R = crate::R<DINR2rs>;
///Field `DIN2` reader - Input data received from MDIO Master during write frames
pub type DIN2_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Input data received from MDIO Master during write frames
    #[inline(always)]
    pub fn din2(&self) -> DIN2_R {
        DIN2_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DINR2").field("din2", &self.din2()).finish()
    }
}
/**MDIOS input data register 2

You can [`read`](crate::Reg::read) this register and get [`dinr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#MDIOS:DINR2)*/
pub struct DINR2rs;
impl crate::RegisterSpec for DINR2rs {
    type Ux = u32;
}
///`read()` method returns [`dinr2::R`](R) reader structure
impl crate::Readable for DINR2rs {}
///`reset()` method sets DINR2 to value 0
impl crate::Resettable for DINR2rs {}
