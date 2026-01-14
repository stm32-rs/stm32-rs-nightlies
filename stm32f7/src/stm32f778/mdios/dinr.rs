///Register `DINR%s` reader
pub type R = crate::R<DINRrs>;
///Field `DIN` reader - Input data received from MDIO Master during write frames
pub type DIN_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Input data received from MDIO Master during write frames
    #[inline(always)]
    pub fn din(&self) -> DIN_R {
        DIN_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DINR").field("din", &self.din()).finish()
    }
}
/**MDIOS input data register %s

You can [`read`](crate::Reg::read) this register and get [`dinr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F778.html#MDIOS:DINR[0])*/
pub struct DINRrs;
impl crate::RegisterSpec for DINRrs {
    type Ux = u32;
}
///`read()` method returns [`dinr::R`](R) reader structure
impl crate::Readable for DINRrs {}
///`reset()` method sets DINR%s to value 0
impl crate::Resettable for DINRrs {}
