///Register `DINR0` reader
pub type R = crate::R<DINR0rs>;
///Field `DIN0` reader - Input data received from MDIO Master during write frames
pub type DIN0_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Input data received from MDIO Master during write frames
    #[inline(always)]
    pub fn din0(&self) -> DIN0_R {
        DIN0_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DINR0").field("din0", &self.din0()).finish()
    }
}
/**MDIOS input data register 0

You can [`read`](crate::Reg::read) this register and get [`dinr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7A3.html#MDIOS:DINR0)*/
pub struct DINR0rs;
impl crate::RegisterSpec for DINR0rs {
    type Ux = u32;
}
///`read()` method returns [`dinr0::R`](R) reader structure
impl crate::Readable for DINR0rs {}
///`reset()` method sets DINR0 to value 0
impl crate::Resettable for DINR0rs {}
