///Register `SR` writer
pub type W = crate::W<SRrs>;
///Field `AP_PRESENT` writer - Bit n identifies whether access port AP n is present in device Bit n = 0: APn absent Bit n = 1: APn present
pub type AP_PRESENT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `AP_ENABLED` writer - Bit n identifies whether access port AP n is open (can be accessed via the debug port) or locked (debug access to the AP is blocked) Bit n = 0: APn locked Bit n = 1: APn enabled
pub type AP_ENABLED_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl core::fmt::Debug for crate::generic::Reg<SRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:15 - Bit n identifies whether access port AP n is present in device Bit n = 0: APn absent Bit n = 1: APn present
    #[inline(always)]
    pub fn ap_present(&mut self) -> AP_PRESENT_W<'_, SRrs> {
        AP_PRESENT_W::new(self, 0)
    }
    ///Bits 16:31 - Bit n identifies whether access port AP n is open (can be accessed via the debug port) or locked (debug access to the AP is blocked) Bit n = 0: APn locked Bit n = 1: APn enabled
    #[inline(always)]
    pub fn ap_enabled(&mut self) -> AP_ENABLED_W<'_, SRrs> {
        AP_ENABLED_W::new(self, 16)
    }
}
/**DBGMCU status register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#DBGMCU:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`sr::W`](W) writer structure
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SR to value 0x0001_0003
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0x0001_0003;
}
