///Register `FDCAN_TURNA` reader
pub type R = crate::R<FDCAN_TURNArs>;
///Field `NAV` reader - NAV
pub type NAV_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:17 - NAV
    #[inline(always)]
    pub fn nav(&self) -> NAV_R {
        NAV_R::new(self.bits & 0x0003_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TURNA")
            .field("nav", &self.nav())
            .finish()
    }
}
/**There is no drift compensation in TTCAN level 1.

You can [`read`](crate::Reg::read) this register and get [`fdcan_turna::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:FDCAN_TURNA)*/
pub struct FDCAN_TURNArs;
impl crate::RegisterSpec for FDCAN_TURNArs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_turna::R`](R) reader structure
impl crate::Readable for FDCAN_TURNArs {}
///`reset()` method sets FDCAN_TURNA to value 0
impl crate::Resettable for FDCAN_TURNArs {
    const RESET_VALUE: u32 = 0;
}
