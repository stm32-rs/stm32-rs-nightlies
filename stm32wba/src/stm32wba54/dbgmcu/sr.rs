///Register `SR` reader
pub type R = crate::R<SRrs>;
///Field `AP_PRESENT` reader - Bit n identifies whether access port APn is present in device Bit n = 0: APn absent Bit n = 1: APn present
pub type AP_PRESENT_R = crate::FieldReader<u16>;
///Field `AP_ENABLED` reader - Bit n identifies whether access port APn is open (can be accessed via the debug port) or locked (debug access to the APn is blocked, except for DBGMCU access) Bit n = 0: APn locked (except for access to DBGMCU) Bit n = 1: APn enabled
pub type AP_ENABLED_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Bit n identifies whether access port APn is present in device Bit n = 0: APn absent Bit n = 1: APn present
    #[inline(always)]
    pub fn ap_present(&self) -> AP_PRESENT_R {
        AP_PRESENT_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Bit n identifies whether access port APn is open (can be accessed via the debug port) or locked (debug access to the APn is blocked, except for DBGMCU access) Bit n = 0: APn locked (except for access to DBGMCU) Bit n = 1: APn enabled
    #[inline(always)]
    pub fn ap_enabled(&self) -> AP_ENABLED_R {
        AP_ENABLED_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("ap_present", &self.ap_present())
            .field("ap_enabled", &self.ap_enabled())
            .finish()
    }
}
/**DBGMCU status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#DBGMCU:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0x03
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0x03;
}
