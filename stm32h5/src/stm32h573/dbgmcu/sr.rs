#[doc = "Register `SR` writer"]
pub type W = crate::W<SRrs>;
#[doc = "Field `AP_PRESENT` writer - Bit n identifies whether access port AP n is present in device Bit n = 0: APn absent Bit n = 1: APn present"]
pub type AP_PRESENT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `AP_ENABLED` writer - Bit n identifies whether access port AP n is open (can be accessed via the debug port) or locked (debug access to the AP is blocked) Bit n = 0: APn locked Bit n = 1: APn enabled"]
pub type AP_ENABLED_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Bit n identifies whether access port AP n is present in device Bit n = 0: APn absent Bit n = 1: APn present"]
    #[inline(always)]
    #[must_use]
    pub fn ap_present(&mut self) -> AP_PRESENT_W<SRrs> {
        AP_PRESENT_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Bit n identifies whether access port AP n is open (can be accessed via the debug port) or locked (debug access to the AP is blocked) Bit n = 0: APn locked Bit n = 1: APn enabled"]
    #[inline(always)]
    #[must_use]
    pub fn ap_enabled(&mut self) -> AP_ENABLED_W<SRrs> {
        AP_ENABLED_W::new(self, 16)
    }
}
#[doc = "DBGMCU status register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SR to value 0x0001_0003"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0x0001_0003;
}
