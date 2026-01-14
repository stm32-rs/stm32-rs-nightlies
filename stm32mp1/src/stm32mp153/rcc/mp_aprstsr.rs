///Register `MP_APRSTSR` reader
pub type R = crate::R<MP_APRSTSRrs>;
///Field `RSTTOV` reader - RSTTOV
pub type RSTTOV_R = crate::FieldReader;
impl R {
    ///Bits 8:14 - RSTTOV
    #[inline(always)]
    pub fn rsttov(&self) -> RSTTOV_R {
        RSTTOV_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MP_APRSTSR")
            .field("rsttov", &self.rsttov())
            .finish()
    }
}
/**This register provides a status of the RDCTL. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`mp_aprstsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_APRSTSR)*/
pub struct MP_APRSTSRrs;
impl crate::RegisterSpec for MP_APRSTSRrs {
    type Ux = u32;
}
///`read()` method returns [`mp_aprstsr::R`](R) reader structure
impl crate::Readable for MP_APRSTSRrs {}
///`reset()` method sets MP_APRSTSR to value 0
impl crate::Resettable for MP_APRSTSRrs {}
