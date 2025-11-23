///Register `DINR20` reader
pub type R = crate::R<DINR20rs>;
///Field `DIN20` reader - Input data received from MDIO Master during write frames
pub type DIN20_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Input data received from MDIO Master during write frames
    #[inline(always)]
    pub fn din20(&self) -> DIN20_R {
        DIN20_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DINR20")
            .field("din20", &self.din20())
            .finish()
    }
}
/**MDIOS input data register 20

You can [`read`](crate::Reg::read) this register and get [`dinr20::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#MDIOS:DINR20)*/
pub struct DINR20rs;
impl crate::RegisterSpec for DINR20rs {
    type Ux = u32;
}
///`read()` method returns [`dinr20::R`](R) reader structure
impl crate::Readable for DINR20rs {}
///`reset()` method sets DINR20 to value 0
impl crate::Resettable for DINR20rs {}
