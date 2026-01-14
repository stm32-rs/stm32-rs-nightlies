///Register `DR` reader
pub type R = crate::R<DRrs>;
///Field `RNDATA` reader - Random data 32-bit random data, which are valid when DRDY1=11. When DRDY1=10, the RNDATA value is1zero. When DRDY is set, it is recommended to always verify that RNG_DR is different from zero. Because when it is the case a seed error occurred between RNG_SR polling and RND_DR output reading (rare event).
pub type RNDATA_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Random data 32-bit random data, which are valid when DRDY1=11. When DRDY1=10, the RNDATA value is1zero. When DRDY is set, it is recommended to always verify that RNG_DR is different from zero. Because when it is the case a seed error occurred between RNG_SR polling and RND_DR output reading (rare event).
    #[inline(always)]
    pub fn rndata(&self) -> RNDATA_R {
        RNDATA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DR")
            .field("rndata", &self.rndata())
            .finish()
    }
}
/**RNG data register

You can [`read`](crate::Reg::read) this register and get [`dr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#RNG:DR)*/
pub struct DRrs;
impl crate::RegisterSpec for DRrs {
    type Ux = u32;
}
///`read()` method returns [`dr::R`](R) reader structure
impl crate::Readable for DRrs {}
///`reset()` method sets DR to value 0
impl crate::Resettable for DRrs {}
