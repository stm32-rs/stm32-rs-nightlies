#[doc = "Register `CSR` reader"]
pub type R = crate::R<CSRrs>;
#[doc = "Synchronization overrun event flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOF0 {
    #[doc = "0: No synchronization event occured on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ"]
    NoSyncEvent = 0,
    #[doc = "1: Synchronization event occured on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ"]
    SyncEvent = 1,
}
impl From<SOF0> for bool {
    #[inline(always)]
    fn from(variant: SOF0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOF0` reader - Synchronization overrun event flag"]
pub type SOF0_R = crate::BitReader<SOF0>;
impl SOF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SOF0 {
        match self.bits {
            false => SOF0::NoSyncEvent,
            true => SOF0::SyncEvent,
        }
    }
    #[doc = "No synchronization event occured on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ"]
    #[inline(always)]
    pub fn is_no_sync_event(&self) -> bool {
        *self == SOF0::NoSyncEvent
    }
    #[doc = "Synchronization event occured on a DMA request line multiplexer channel x, while the DMA request counter value is lower than NBREQ"]
    #[inline(always)]
    pub fn is_sync_event(&self) -> bool {
        *self == SOF0::SyncEvent
    }
}
#[doc = "Field `SOF1` reader - Synchronization overrun event flag"]
pub use SOF0_R as SOF1_R;
#[doc = "Field `SOF2` reader - Synchronization overrun event flag"]
pub use SOF0_R as SOF2_R;
#[doc = "Field `SOF3` reader - Synchronization overrun event flag"]
pub use SOF0_R as SOF3_R;
#[doc = "Field `SOF4` reader - Synchronization overrun event flag"]
pub use SOF0_R as SOF4_R;
#[doc = "Field `SOF5` reader - Synchronization overrun event flag"]
pub use SOF0_R as SOF5_R;
#[doc = "Field `SOF6` reader - Synchronization overrun event flag"]
pub use SOF0_R as SOF6_R;
#[doc = "Field `SOF7` reader - Synchronization overrun event flag"]
pub use SOF0_R as SOF7_R;
#[doc = "Field `SOF8` reader - Synchronization overrun event flag"]
pub use SOF0_R as SOF8_R;
#[doc = "Field `SOF9` reader - Synchronization overrun event flag"]
pub use SOF0_R as SOF9_R;
#[doc = "Field `SOF10` reader - Synchronization overrun event flag"]
pub use SOF0_R as SOF10_R;
#[doc = "Field `SOF11` reader - Synchronization overrun event flag"]
pub use SOF0_R as SOF11_R;
#[doc = "Field `SOF12` reader - Synchronization overrun event flag"]
pub use SOF0_R as SOF12_R;
#[doc = "Field `SOF13` reader - Synchronization overrun event flag"]
pub use SOF0_R as SOF13_R;
impl R {
    #[doc = "Bit 0 - Synchronization overrun event flag"]
    #[inline(always)]
    pub fn sof0(&self) -> SOF0_R {
        SOF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Synchronization overrun event flag"]
    #[inline(always)]
    pub fn sof1(&self) -> SOF1_R {
        SOF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Synchronization overrun event flag"]
    #[inline(always)]
    pub fn sof2(&self) -> SOF2_R {
        SOF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Synchronization overrun event flag"]
    #[inline(always)]
    pub fn sof3(&self) -> SOF3_R {
        SOF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Synchronization overrun event flag"]
    #[inline(always)]
    pub fn sof4(&self) -> SOF4_R {
        SOF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Synchronization overrun event flag"]
    #[inline(always)]
    pub fn sof5(&self) -> SOF5_R {
        SOF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Synchronization overrun event flag"]
    #[inline(always)]
    pub fn sof6(&self) -> SOF6_R {
        SOF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Synchronization overrun event flag"]
    #[inline(always)]
    pub fn sof7(&self) -> SOF7_R {
        SOF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Synchronization overrun event flag"]
    #[inline(always)]
    pub fn sof8(&self) -> SOF8_R {
        SOF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Synchronization overrun event flag"]
    #[inline(always)]
    pub fn sof9(&self) -> SOF9_R {
        SOF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Synchronization overrun event flag"]
    #[inline(always)]
    pub fn sof10(&self) -> SOF10_R {
        SOF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Synchronization overrun event flag"]
    #[inline(always)]
    pub fn sof11(&self) -> SOF11_R {
        SOF11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Synchronization overrun event flag"]
    #[inline(always)]
    pub fn sof12(&self) -> SOF12_R {
        SOF12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Synchronization overrun event flag"]
    #[inline(always)]
    pub fn sof13(&self) -> SOF13_R {
        SOF13_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[doc = "channel status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CSRrs {}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CSRrs {
    const RESET_VALUE: u32 = 0;
}
