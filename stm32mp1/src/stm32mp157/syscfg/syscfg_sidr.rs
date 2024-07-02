///Register `SYSCFG_SIDR` reader
pub type R = crate::R<SYSCFG_SIDRrs>;
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
        f.debug_struct("SYSCFG_SIDR")
            .field("sid", &self.sid())
            .finish()
    }
}
/**SYSCFG size identification register

You can [`read`](crate::Reg::read) this register and get [`syscfg_sidr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#SYSCFG:SYSCFG_SIDR)*/
pub struct SYSCFG_SIDRrs;
impl crate::RegisterSpec for SYSCFG_SIDRrs {
    type Ux = u32;
}
///`read()` method returns [`syscfg_sidr::R`](R) reader structure
impl crate::Readable for SYSCFG_SIDRrs {}
///`reset()` method sets SYSCFG_SIDR to value 0xa3c5_dd01
impl crate::Resettable for SYSCFG_SIDRrs {
    const RESET_VALUE: u32 = 0xa3c5_dd01;
}
