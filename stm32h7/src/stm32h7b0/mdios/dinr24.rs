///Register `DINR24` reader
pub type R = crate::R<DINR24rs>;
///Field `DIN24` reader - Input data received from MDIO Master during write frames
pub type DIN24_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Input data received from MDIO Master during write frames
    #[inline(always)]
    pub fn din24(&self) -> DIN24_R {
        DIN24_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DINR24")
            .field("din24", &self.din24())
            .finish()
    }
}
/**MDIOS input data register 24

You can [`read`](crate::Reg::read) this register and get [`dinr24::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#MDIOS:DINR24)*/
pub struct DINR24rs;
impl crate::RegisterSpec for DINR24rs {
    type Ux = u32;
}
///`read()` method returns [`dinr24::R`](R) reader structure
impl crate::Readable for DINR24rs {}
///`reset()` method sets DINR24 to value 0
impl crate::Resettable for DINR24rs {}
