///Register `TTCPT` reader
pub type R = crate::R<TTCPTrs>;
///Field `CCV` reader - Cycle count value
pub type CCV_R = crate::FieldReader;
///Field `SWV` reader - Stop watch value
pub type SWV_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:5 - Cycle count value
    #[inline(always)]
    pub fn ccv(&self) -> CCV_R {
        CCV_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 16:31 - Stop watch value
    #[inline(always)]
    pub fn swv(&self) -> SWV_R {
        SWV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TTCPT")
            .field("ccv", &self.ccv())
            .field("swv", &self.swv())
            .finish()
    }
}
/**FDCAN TT capture time register

You can [`read`](crate::Reg::read) this register and get [`ttcpt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FDCAN1:TTCPT)*/
pub struct TTCPTrs;
impl crate::RegisterSpec for TTCPTrs {
    type Ux = u32;
}
///`read()` method returns [`ttcpt::R`](R) reader structure
impl crate::Readable for TTCPTrs {}
///`reset()` method sets TTCPT to value 0
impl crate::Resettable for TTCPTrs {}
