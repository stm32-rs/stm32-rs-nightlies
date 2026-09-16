///Register `HWCFGR` reader
pub type R = crate::R<HWCFGRrs>;
///Field `SIZE` reader - SIZE
pub type SIZE_R = crate::FieldReader;
///Field `ECC_USE` reader - ECC_USE
pub type ECC_USE_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - SIZE
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - ECC_USE
    #[inline(always)]
    pub fn ecc_use(&self) -> ECC_USE_R {
        ECC_USE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWCFGR")
            .field("size", &self.size())
            .field("ecc_use", &self.ecc_use())
            .finish()
    }
}
/**BSEC hardware configuration register

You can [`read`](crate::Reg::read) this register and get [`hwcfgr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#BSEC:HWCFGR)*/
pub struct HWCFGRrs;
impl crate::RegisterSpec for HWCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr::R`](R) reader structure
impl crate::Readable for HWCFGRrs {}
///`reset()` method sets HWCFGR to value 0x14
impl crate::Resettable for HWCFGRrs {
    const RESET_VALUE: u32 = 0x14;
}
