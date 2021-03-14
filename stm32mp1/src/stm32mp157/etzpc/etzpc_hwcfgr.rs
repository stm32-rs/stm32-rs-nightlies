#[doc = "Reader of register ETZPC_HWCFGR"]
pub type R = crate::R<u32, super::ETZPC_HWCFGR>;
#[doc = "Reader of field `NUM_TZMA`"]
pub type NUM_TZMA_R = crate::R<u8, u8>;
#[doc = "Reader of field `NUM_PER_SEC`"]
pub type NUM_PER_SEC_R = crate::R<u8, u8>;
#[doc = "Reader of field `NUM_AHB_SEC`"]
pub type NUM_AHB_SEC_R = crate::R<u8, u8>;
#[doc = "Reader of field `CHUNKS1N4`"]
pub type CHUNKS1N4_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - NUM_TZMA"]
    #[inline(always)]
    pub fn num_tzma(&self) -> NUM_TZMA_R {
        NUM_TZMA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - NUM_PER_SEC"]
    #[inline(always)]
    pub fn num_per_sec(&self) -> NUM_PER_SEC_R {
        NUM_PER_SEC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - NUM_AHB_SEC"]
    #[inline(always)]
    pub fn num_ahb_sec(&self) -> NUM_AHB_SEC_R {
        NUM_AHB_SEC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - CHUNKS1N4"]
    #[inline(always)]
    pub fn chunks1n4(&self) -> CHUNKS1N4_R {
        CHUNKS1N4_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
