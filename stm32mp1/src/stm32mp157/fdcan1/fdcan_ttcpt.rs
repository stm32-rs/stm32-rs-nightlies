///Register `FDCAN_TTCPT` reader
pub type R = crate::R<FDCAN_TTCPTrs>;
///Field `CCV` reader - CCV
pub type CCV_R = crate::FieldReader;
///Field `SWV` reader - SWV
pub type SWV_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:5 - CCV
    #[inline(always)]
    pub fn ccv(&self) -> CCV_R {
        CCV_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 16:31 - SWV
    #[inline(always)]
    pub fn swv(&self) -> SWV_R {
        SWV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TTCPT")
            .field("ccv", &self.ccv())
            .field("swv", &self.swv())
            .finish()
    }
}
/**FDCAN TT capture time register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ttcpt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:FDCAN_TTCPT)*/
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
