///Register `DINR16` reader
pub type R = crate::R<DINR16rs>;
///Field `DIN16` reader - Input data received from MDIO Master during write frames
pub type DIN16_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Input data received from MDIO Master during write frames
    #[inline(always)]
    pub fn din16(&self) -> DIN16_R {
        DIN16_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DINR16")
            .field("din16", &self.din16())
            .finish()
    }
}
/**MDIOS input data register 16

You can [`read`](crate::Reg::read) this register and get [`dinr16::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#MDIOS:DINR16)*/
pub struct DINR16rs;
impl crate::RegisterSpec for DINR16rs {
    type Ux = u32;
}
///`read()` method returns [`dinr16::R`](R) reader structure
impl crate::Readable for DINR16rs {}
///`reset()` method sets DINR16 to value 0
impl crate::Resettable for DINR16rs {}
