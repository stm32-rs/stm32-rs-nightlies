///Register `FDCAN_TTCPT` reader
pub type R = crate::R<FDCAN_TTCPTrs>;
///Field `CT` reader - Cycle Count Value
pub type CT_R = crate::FieldReader;
///Field `SWV` reader - Stop Watch Value
pub type SWV_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:5 - Cycle Count Value
    #[inline(always)]
    pub fn ct(&self) -> CT_R {
        CT_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 16:31 - Stop Watch Value
    #[inline(always)]
    pub fn swv(&self) -> SWV_R {
        SWV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TTCPT")
            .field("ct", &self.ct())
            .field("swv", &self.swv())
            .finish()
    }
}
/**FDCAN TT Capture Time Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ttcpt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7A3.html#CAN_CCU:FDCAN_TTCPT)*/
pub struct FDCAN_TTCPTrs;
impl crate::RegisterSpec for FDCAN_TTCPTrs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_ttcpt::R`](R) reader structure
impl crate::Readable for FDCAN_TTCPTrs {}
///`reset()` method sets FDCAN_TTCPT to value 0
impl crate::Resettable for FDCAN_TTCPTrs {
    const RESET_VALUE: u32 = 0;
}
