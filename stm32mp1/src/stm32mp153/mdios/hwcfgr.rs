///Register `HWCFGR` reader
pub type R = crate::R<HWCFGRrs>;
///Field `NBREG` reader - NBREG
pub type NBREG_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - NBREG
    #[inline(always)]
    pub fn nbreg(&self) -> NBREG_R {
        NBREG_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWCFGR")
            .field("nbreg", &self.nbreg())
            .finish()
    }
}
/**MDIOS HW configuration register

You can [`read`](crate::Reg::read) this register and get [`hwcfgr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDIOS:HWCFGR)*/
pub struct HWCFGRrs;
impl crate::RegisterSpec for HWCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr::R`](R) reader structure
impl crate::Readable for HWCFGRrs {}
///`reset()` method sets HWCFGR to value 0x20
impl crate::Resettable for HWCFGRrs {
    const RESET_VALUE: u32 = 0x20;
}
