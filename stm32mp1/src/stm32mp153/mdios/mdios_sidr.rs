///Register `MDIOS_SIDR` reader
pub type R = crate::R<MDIOS_SIDRrs>;
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
        f.debug_struct("MDIOS_SIDR")
            .field("sid", &self.sid())
            .finish()
    }
}
/**MDIOS size identification register

You can [`read`](crate::Reg::read) this register and get [`mdios_sidr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDIOS:MDIOS_SIDR)*/
pub struct MDIOS_SIDRrs;
impl crate::RegisterSpec for MDIOS_SIDRrs {
    type Ux = u32;
}
///`read()` method returns [`mdios_sidr::R`](R) reader structure
impl crate::Readable for MDIOS_SIDRrs {}
///`reset()` method sets MDIOS_SIDR to value 0xa3c5_dd01
impl crate::Resettable for MDIOS_SIDRrs {
    const RESET_VALUE: u32 = 0xa3c5_dd01;
}
