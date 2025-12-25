///Register `HWCFG` reader
pub type R = crate::R<HWCFGrs>;
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
        f.debug_struct("HWCFG").field("ncnt", &self.ncnt()).finish()
    }
}
/**DDRPERFM hardware configuration register

You can [`read`](crate::Reg::read) this register and get [`hwcfg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRPERFM:HWCFG)*/
pub struct HWCFGrs;
impl crate::RegisterSpec for HWCFGrs {
    type Ux = u32;
}
///`read()` method returns [`hwcfg::R`](R) reader structure
impl crate::Readable for HWCFGrs {}
///`reset()` method sets HWCFG to value 0x04
impl crate::Resettable for HWCFGrs {
    const RESET_VALUE: u32 = 0x04;
}
