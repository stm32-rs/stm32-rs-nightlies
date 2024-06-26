///Register `HWCFGR0` reader
pub type R = crate::R<HWCFGR0rs>;
///Field `NUM_CHAN_24` reader - NUM_CHAN_24
pub type NUM_CHAN_24_R = crate::FieldReader;
///Field `EXTRA_AWDS` reader - Extra analog watchdog
pub type EXTRA_AWDS_R = crate::FieldReader;
///Field `OVS` reader - Oversampling
pub type OVS_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - NUM_CHAN_24
    #[inline(always)]
    pub fn num_chan_24(&self) -> NUM_CHAN_24_R {
        NUM_CHAN_24_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Extra analog watchdog
    #[inline(always)]
    pub fn extra_awds(&self) -> EXTRA_AWDS_R {
        EXTRA_AWDS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Oversampling
    #[inline(always)]
    pub fn ovs(&self) -> OVS_R {
        OVS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWCFGR0")
            .field("num_chan_24", &self.num_chan_24())
            .field("extra_awds", &self.extra_awds())
            .field("ovs", &self.ovs())
            .finish()
    }
}
/**Hardware Configuration Register

You can [`read`](crate::Reg::read) this register and get [`hwcfgr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G081.html#ADC:HWCFGR0)*/
pub struct HWCFGR0rs;
impl crate::RegisterSpec for HWCFGR0rs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr0::R`](R) reader structure
impl crate::Readable for HWCFGR0rs {}
///`reset()` method sets HWCFGR0 to value 0x0110
impl crate::Resettable for HWCFGR0rs {
    const RESET_VALUE: u32 = 0x0110;
}
