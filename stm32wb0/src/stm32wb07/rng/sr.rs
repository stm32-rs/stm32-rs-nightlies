///Register `SR` reader
pub type R = crate::R<SRrs>;
///Field `RNGRDY` reader - New random value ready
pub type RNGRDY_R = crate::BitReader;
///Field `REVCLK` reader - RNGCLK clock reveal bit.
pub type REVCLK_R = crate::BitReader;
///Field `FAULT` reader - Fault reveal bit.
pub type FAULT_R = crate::BitReader;
impl R {
    ///Bit 0 - New random value ready
    #[inline(always)]
    pub fn rngrdy(&self) -> RNGRDY_R {
        RNGRDY_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - RNGCLK clock reveal bit.
    #[inline(always)]
    pub fn revclk(&self) -> REVCLK_R {
        REVCLK_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Fault reveal bit.
    #[inline(always)]
    pub fn fault(&self) -> FAULT_R {
        FAULT_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("rngrdy", &self.rngrdy())
            .field("revclk", &self.revclk())
            .field("fault", &self.fault())
            .finish()
    }
}
/**RNG_SR register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#RNG:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
