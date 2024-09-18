///Register `HASH_HWCFGR` reader
pub type R = crate::R<HASH_HWCFGRrs>;
///Field `CFG1` reader - CFG1
pub type CFG1_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - CFG1
    #[inline(always)]
    pub fn cfg1(&self) -> CFG1_R {
        CFG1_R::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_HWCFGR")
            .field("cfg1", &self.cfg1())
            .finish()
    }
}
/**HASH Hardware Configuration Register

You can [`read`](crate::Reg::read) this register and get [`hash_hwcfgr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HASH1:HASH_HWCFGR)*/
pub struct HASH_HWCFGRrs;
impl crate::RegisterSpec for HASH_HWCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`hash_hwcfgr::R`](R) reader structure
impl crate::Readable for HASH_HWCFGRrs {}
///`reset()` method sets HASH_HWCFGR to value 0x01
impl crate::Resettable for HASH_HWCFGRrs {
    const RESET_VALUE: u32 = 0x01;
}
