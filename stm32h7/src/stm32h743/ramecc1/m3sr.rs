///Register `M3SR` reader
pub type R = crate::R<M3SRrs>;
///Field `SEDCF` reader - ECC single error detected flag
pub type SEDCF_R = crate::BitReader;
///Field `DEDF` reader - ECC double error detected flag
pub type DEDF_R = crate::BitReader;
///Field `DEBWDF` reader - ECC double error on byte write flag
pub type DEBWDF_R = crate::BitReader;
impl R {
    ///Bit 0 - ECC single error detected flag
    #[inline(always)]
    pub fn sedcf(&self) -> SEDCF_R {
        SEDCF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ECC double error detected flag
    #[inline(always)]
    pub fn dedf(&self) -> DEDF_R {
        DEDF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ECC double error on byte write flag
    #[inline(always)]
    pub fn debwdf(&self) -> DEBWDF_R {
        DEBWDF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M3SR")
            .field("debwdf", &self.debwdf())
            .field("dedf", &self.dedf())
            .field("sedcf", &self.sedcf())
            .finish()
    }
}
/**RAMECC monitor x status register

You can [`read`](crate::Reg::read) this register and get [`m3sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#RAMECC1:M3SR)*/
pub struct M3SRrs;
impl crate::RegisterSpec for M3SRrs {
    type Ux = u32;
}
///`read()` method returns [`m3sr::R`](R) reader structure
impl crate::Readable for M3SRrs {}
///`reset()` method sets M3SR to value 0
impl crate::Resettable for M3SRrs {
    const RESET_VALUE: u32 = 0;
}
