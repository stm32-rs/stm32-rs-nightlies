#[doc = "Reader of register HWCFGR0"]
pub type R = crate::R<u32, super::HWCFGR0>;
#[doc = "Reader of field `NUM_CHAN_24`"]
pub type NUM_CHAN_24_R = crate::R<u8, u8>;
#[doc = "Reader of field `EXTRA_AWDS`"]
pub type EXTRA_AWDS_R = crate::R<u8, u8>;
#[doc = "Reader of field `OVS`"]
pub type OVS_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - NUM_CHAN_24"]
    #[inline(always)]
    pub fn num_chan_24(&self) -> NUM_CHAN_24_R {
        NUM_CHAN_24_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Extra analog watchdog"]
    #[inline(always)]
    pub fn extra_awds(&self) -> EXTRA_AWDS_R {
        EXTRA_AWDS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Oversampling"]
    #[inline(always)]
    pub fn ovs(&self) -> OVS_R {
        OVS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
