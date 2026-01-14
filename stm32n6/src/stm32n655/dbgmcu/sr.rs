///Register `SR` reader
pub type R = crate::R<SRrs>;
///Field `AP0_PRESENT` reader - Access point 0 presence
pub type AP0_PRESENT_R = crate::BitReader;
///Field `AP1_PRESENT` reader - Access point 1 presence
pub type AP1_PRESENT_R = crate::BitReader;
///Field `AP0_ENABLE` reader - Access point 0 enable
pub type AP0_ENABLE_R = crate::BitReader;
///Field `AP1_ENABLE` reader - Access point 1 enable
pub type AP1_ENABLE_R = crate::BitReader;
impl R {
    ///Bit 0 - Access point 0 presence
    #[inline(always)]
    pub fn ap0_present(&self) -> AP0_PRESENT_R {
        AP0_PRESENT_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Access point 1 presence
    #[inline(always)]
    pub fn ap1_present(&self) -> AP1_PRESENT_R {
        AP1_PRESENT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 16 - Access point 0 enable
    #[inline(always)]
    pub fn ap0_enable(&self) -> AP0_ENABLE_R {
        AP0_ENABLE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Access point 1 enable
    #[inline(always)]
    pub fn ap1_enable(&self) -> AP1_ENABLE_R {
        AP1_ENABLE_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("ap0_present", &self.ap0_present())
            .field("ap1_present", &self.ap1_present())
            .field("ap0_enable", &self.ap0_enable())
            .field("ap1_enable", &self.ap1_enable())
            .finish()
    }
}
/**DBGMCU status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#DBGMCU:SR)*/
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
