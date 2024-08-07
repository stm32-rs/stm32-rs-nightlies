///Register `BSEC_SIDR` reader
pub type R = crate::R<BSEC_SIDRrs>;
///Field `SID` reader - SID
pub type SID_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - SID
    #[inline(always)]
    pub fn sid(&self) -> SID_R {
        SID_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BSEC_SIDR")
            .field("sid", &self.sid())
            .finish()
    }
}
/**BSEC size identification register

You can [`read`](crate::Reg::read) this register and get [`bsec_sidr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#BSEC:BSEC_SIDR)*/
pub struct BSEC_SIDRrs;
impl crate::RegisterSpec for BSEC_SIDRrs {
    type Ux = u32;
}
///`read()` method returns [`bsec_sidr::R`](R) reader structure
impl crate::Readable for BSEC_SIDRrs {}
///`reset()` method sets BSEC_SIDR to value 0xa3c5_dd04
impl crate::Resettable for BSEC_SIDRrs {
    const RESET_VALUE: u32 = 0xa3c5_dd04;
}
