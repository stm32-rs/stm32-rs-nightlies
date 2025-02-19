///Register `SR` reader
pub type R = crate::R<SRrs>;
///Field `AP1_PRESENT` reader - Identifies whether access port AP1 is present in device
pub type AP1_PRESENT_R = crate::BitReader;
///Field `AP0_PRESENT` reader - Identifies whether access port AP0 is present in device
pub type AP0_PRESENT_R = crate::BitReader;
///Field `AP1_ENABLED` reader - Identifies whether access port AP0 is open (can be accessed via the debug port) or locked (debug access to the AP is blocked)
pub type AP1_ENABLED_R = crate::BitReader;
///Field `AP0_ENABLED` reader - Identifies whether access port AP0 is open (can be accessed via the debug port) or locked (debug access to the AP is blocked)
pub type AP0_ENABLED_R = crate::BitReader;
impl R {
    ///Bit 0 - Identifies whether access port AP1 is present in device
    #[inline(always)]
    pub fn ap1_present(&self) -> AP1_PRESENT_R {
        AP1_PRESENT_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Identifies whether access port AP0 is present in device
    #[inline(always)]
    pub fn ap0_present(&self) -> AP0_PRESENT_R {
        AP0_PRESENT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 16 - Identifies whether access port AP0 is open (can be accessed via the debug port) or locked (debug access to the AP is blocked)
    #[inline(always)]
    pub fn ap1_enabled(&self) -> AP1_ENABLED_R {
        AP1_ENABLED_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Identifies whether access port AP0 is open (can be accessed via the debug port) or locked (debug access to the AP is blocked)
    #[inline(always)]
    pub fn ap0_enabled(&self) -> AP0_ENABLED_R {
        AP0_ENABLED_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("ap1_present", &self.ap1_present())
            .field("ap0_present", &self.ap0_present())
            .field("ap1_enabled", &self.ap1_enabled())
            .field("ap0_enabled", &self.ap0_enabled())
            .finish()
    }
}
/**DBGMCU status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DBGMCU:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0x0001_0003
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0x0001_0003;
}
