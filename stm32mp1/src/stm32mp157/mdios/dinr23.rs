///Register `DINR23` reader
pub type R = crate::R<DINR23rs>;
///Field `DIN` reader - DIN
pub type DIN_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - DIN
    #[inline(always)]
    pub fn din(&self) -> DIN_R {
        DIN_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DINR23").field("din", &self.din()).finish()
    }
}
/**MDIOS input data register

You can [`read`](crate::Reg::read) this register and get [`dinr23::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:DINR23)*/
pub struct DINR23rs;
impl crate::RegisterSpec for DINR23rs {
    type Ux = u32;
}
///`read()` method returns [`dinr23::R`](R) reader structure
impl crate::Readable for DINR23rs {}
///`reset()` method sets DINR23 to value 0
impl crate::Resettable for DINR23rs {}
