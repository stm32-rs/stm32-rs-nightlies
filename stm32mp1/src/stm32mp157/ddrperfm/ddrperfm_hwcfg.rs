///Register `DDRPERFM_HWCFG` reader
pub type R = crate::R<DDRPERFM_HWCFGrs>;
///Field `NCNT` reader - NCNT
pub type NCNT_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - NCNT
    #[inline(always)]
    pub fn ncnt(&self) -> NCNT_R {
        NCNT_R::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DDRPERFM_HWCFG")
            .field("ncnt", &self.ncnt())
            .finish()
    }
}
/**DDRPERFM hardware configuration register

You can [`read`](crate::Reg::read) this register and get [`ddrperfm_hwcfg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPERFM:DDRPERFM_HWCFG)*/
pub struct DDRPERFM_HWCFGrs;
impl crate::RegisterSpec for DDRPERFM_HWCFGrs {
    type Ux = u32;
}
///`read()` method returns [`ddrperfm_hwcfg::R`](R) reader structure
impl crate::Readable for DDRPERFM_HWCFGrs {}
///`reset()` method sets DDRPERFM_HWCFG to value 0x04
impl crate::Resettable for DDRPERFM_HWCFGrs {
    const RESET_VALUE: u32 = 0x04;
}
