///Register `DINR1` reader
pub type R = crate::R<DINR1rs>;
///Field `DIN1` reader - Input data received from MDIO Master during write frames
pub type DIN1_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Input data received from MDIO Master during write frames
    #[inline(always)]
    pub fn din1(&self) -> DIN1_R {
        DIN1_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DINR1").field("din1", &self.din1()).finish()
    }
}
/**MDIOS input data register 1

You can [`read`](crate::Reg::read) this register and get [`dinr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#MDIOS:DINR1)*/
pub struct DINR1rs;
impl crate::RegisterSpec for DINR1rs {
    type Ux = u32;
}
///`read()` method returns [`dinr1::R`](R) reader structure
impl crate::Readable for DINR1rs {}
///`reset()` method sets DINR1 to value 0
impl crate::Resettable for DINR1rs {}
