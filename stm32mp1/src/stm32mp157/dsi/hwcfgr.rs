///Register `HWCFGR` reader
pub type R = crate::R<HWCFGRrs>;
///Field `TECHNO` reader - TECHNO
pub type TECHNO_R = crate::FieldReader;
///Field `FIFOSIZE` reader - FIFOSIZE
pub type FIFOSIZE_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:3 - TECHNO
    #[inline(always)]
    pub fn techno(&self) -> TECHNO_R {
        TECHNO_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:15 - FIFOSIZE
    #[inline(always)]
    pub fn fifosize(&self) -> FIFOSIZE_R {
        FIFOSIZE_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWCFGR")
            .field("techno", &self.techno())
            .field("fifosize", &self.fifosize())
            .finish()
    }
}
/**DSI Host hardware configuration register

You can [`read`](crate::Reg::read) this register and get [`hwcfgr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DSI:HWCFGR)*/
pub struct HWCFGRrs;
impl crate::RegisterSpec for HWCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr::R`](R) reader structure
impl crate::Readable for HWCFGRrs {}
///`reset()` method sets HWCFGR to value 0x5a01
impl crate::Resettable for HWCFGRrs {
    const RESET_VALUE: u32 = 0x5a01;
}
