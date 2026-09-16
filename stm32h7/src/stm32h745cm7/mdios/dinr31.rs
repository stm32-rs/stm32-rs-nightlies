///Register `DINR31` reader
pub type R = crate::R<DINR31rs>;
///Field `DIN31` reader - Input data received from MDIO Master during write frames
pub type DIN31_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Input data received from MDIO Master during write frames
    #[inline(always)]
    pub fn din31(&self) -> DIN31_R {
        DIN31_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DINR31")
            .field("din31", &self.din31())
            .finish()
    }
}
/**MDIOS input data register 31

You can [`read`](crate::Reg::read) this register and get [`dinr31::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM7.html#MDIOS:DINR31)*/
pub struct DINR31rs;
impl crate::RegisterSpec for DINR31rs {
    type Ux = u32;
}
///`read()` method returns [`dinr31::R`](R) reader structure
impl crate::Readable for DINR31rs {}
///`reset()` method sets DINR31 to value 0
impl crate::Resettable for DINR31rs {}
