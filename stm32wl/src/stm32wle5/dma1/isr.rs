#[doc = "Register `ISR` reader"]
pub type R = crate::R<ISRrs>;
#[doc = "global interrupt flag for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GIF1 {
    #[doc = "0: No TE, HT or TC event on channel x"]
    NoEvent = 0,
    #[doc = "1: A TE, HT or TC event occurred on channel x"]
    Event = 1,
}
impl From<GIF1> for bool {
    #[inline(always)]
    fn from(variant: GIF1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GIF1` reader - global interrupt flag for channel 1"]
pub type GIF1_R = crate::BitReader<GIF1>;
impl GIF1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GIF1 {
        match self.bits {
            false => GIF1::NoEvent,
            true => GIF1::Event,
        }
    }
    #[doc = "No TE, HT or TC event on channel x"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == GIF1::NoEvent
    }
    #[doc = "A TE, HT or TC event occurred on channel x"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == GIF1::Event
    }
}
#[doc = "transfer complete (TC) flag for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIF1 {
    #[doc = "0: No transfer complete event on channel x"]
    NotComplete = 0,
    #[doc = "1: A transfer complete event occurred on channel x"]
    Complete = 1,
}
impl From<TCIF1> for bool {
    #[inline(always)]
    fn from(variant: TCIF1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCIF1` reader - transfer complete (TC) flag for channel 1"]
pub type TCIF1_R = crate::BitReader<TCIF1>;
impl TCIF1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCIF1 {
        match self.bits {
            false => TCIF1::NotComplete,
            true => TCIF1::Complete,
        }
    }
    #[doc = "No transfer complete event on channel x"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == TCIF1::NotComplete
    }
    #[doc = "A transfer complete event occurred on channel x"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == TCIF1::Complete
    }
}
#[doc = "half transfer (HT) flag for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTIF1 {
    #[doc = "0: No half transfer event on channel x"]
    NotHalf = 0,
    #[doc = "1: A half transfer event occurred on channel x"]
    Half = 1,
}
impl From<HTIF1> for bool {
    #[inline(always)]
    fn from(variant: HTIF1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HTIF1` reader - half transfer (HT) flag for channel 1"]
pub type HTIF1_R = crate::BitReader<HTIF1>;
impl HTIF1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HTIF1 {
        match self.bits {
            false => HTIF1::NotHalf,
            true => HTIF1::Half,
        }
    }
    #[doc = "No half transfer event on channel x"]
    #[inline(always)]
    pub fn is_not_half(&self) -> bool {
        *self == HTIF1::NotHalf
    }
    #[doc = "A half transfer event occurred on channel x"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == HTIF1::Half
    }
}
#[doc = "transfer error (TE) flag for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEIF1 {
    #[doc = "0: No transfer error on channel x"]
    NoError = 0,
    #[doc = "1: A transfer error occurred on channel x"]
    Error = 1,
}
impl From<TEIF1> for bool {
    #[inline(always)]
    fn from(variant: TEIF1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEIF1` reader - transfer error (TE) flag for channel 1"]
pub type TEIF1_R = crate::BitReader<TEIF1>;
impl TEIF1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TEIF1 {
        match self.bits {
            false => TEIF1::NoError,
            true => TEIF1::Error,
        }
    }
    #[doc = "No transfer error on channel x"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == TEIF1::NoError
    }
    #[doc = "A transfer error occurred on channel x"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == TEIF1::Error
    }
}
#[doc = "Field `GIF2` reader - global interrupt flag for channel 2"]
pub use GIF1_R as GIF2_R;
#[doc = "Field `GIF3` reader - global interrupt flag for channel 3"]
pub use GIF1_R as GIF3_R;
#[doc = "Field `GIF4` reader - global interrupt flag for channel 4"]
pub use GIF1_R as GIF4_R;
#[doc = "Field `GIF5` reader - global interrupt flag for channel 5"]
pub use GIF1_R as GIF5_R;
#[doc = "Field `GIF6` reader - global interrupt flag for channel 6"]
pub use GIF1_R as GIF6_R;
#[doc = "Field `GIF7` reader - global interrupt flag for channel 7"]
pub use GIF1_R as GIF7_R;
#[doc = "Field `HTIF2` reader - half transfer (HT) flag for channel 2"]
pub use HTIF1_R as HTIF2_R;
#[doc = "Field `HTIF3` reader - half transfer (HT) flag for channel 3"]
pub use HTIF1_R as HTIF3_R;
#[doc = "Field `HTIF4` reader - half transfer (HT) flag for channel 4"]
pub use HTIF1_R as HTIF4_R;
#[doc = "Field `HTIF5` reader - half transfer (HT) flag for channel 5"]
pub use HTIF1_R as HTIF5_R;
#[doc = "Field `HTIF6` reader - half transfer (HT) flag for channel 6"]
pub use HTIF1_R as HTIF6_R;
#[doc = "Field `HTIF7` reader - half transfer (HT) flag for channel 7"]
pub use HTIF1_R as HTIF7_R;
#[doc = "Field `TCIF2` reader - transfer complete (TC) flag for channel 2"]
pub use TCIF1_R as TCIF2_R;
#[doc = "Field `TCIF3` reader - transfer complete (TC) flag for channel 3"]
pub use TCIF1_R as TCIF3_R;
#[doc = "Field `TCIF4` reader - transfer complete (TC) flag for channel 4"]
pub use TCIF1_R as TCIF4_R;
#[doc = "Field `TCIF5` reader - transfer complete (TC) flag for channel 5"]
pub use TCIF1_R as TCIF5_R;
#[doc = "Field `TCIF6` reader - transfer complete (TC) flag for channel 6"]
pub use TCIF1_R as TCIF6_R;
#[doc = "Field `TCIF7` reader - transfer complete (TC) flag for channel 7"]
pub use TCIF1_R as TCIF7_R;
#[doc = "Field `TEIF2` reader - transfer error (TE) flag for channel 2"]
pub use TEIF1_R as TEIF2_R;
#[doc = "Field `TEIF3` reader - transfer error (TE) flag for channel 3"]
pub use TEIF1_R as TEIF3_R;
#[doc = "Field `TEIF4` reader - transfer error (TE) flag for channel 4"]
pub use TEIF1_R as TEIF4_R;
#[doc = "Field `TEIF5` reader - transfer error (TE) flag for channel 5"]
pub use TEIF1_R as TEIF5_R;
#[doc = "Field `TEIF6` reader - transfer error (TE) flag for channel 6"]
pub use TEIF1_R as TEIF6_R;
#[doc = "Field `TEIF7` reader - transfer error (TE) flag for channel 7"]
pub use TEIF1_R as TEIF7_R;
impl R {
    #[doc = "Bit 0 - global interrupt flag for channel 1"]
    #[inline(always)]
    pub fn gif1(&self) -> GIF1_R {
        GIF1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - transfer complete (TC) flag for channel 1"]
    #[inline(always)]
    pub fn tcif1(&self) -> TCIF1_R {
        TCIF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - half transfer (HT) flag for channel 1"]
    #[inline(always)]
    pub fn htif1(&self) -> HTIF1_R {
        HTIF1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - transfer error (TE) flag for channel 1"]
    #[inline(always)]
    pub fn teif1(&self) -> TEIF1_R {
        TEIF1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - global interrupt flag for channel 2"]
    #[inline(always)]
    pub fn gif2(&self) -> GIF2_R {
        GIF2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - transfer complete (TC) flag for channel 2"]
    #[inline(always)]
    pub fn tcif2(&self) -> TCIF2_R {
        TCIF2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - half transfer (HT) flag for channel 2"]
    #[inline(always)]
    pub fn htif2(&self) -> HTIF2_R {
        HTIF2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - transfer error (TE) flag for channel 2"]
    #[inline(always)]
    pub fn teif2(&self) -> TEIF2_R {
        TEIF2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - global interrupt flag for channel 3"]
    #[inline(always)]
    pub fn gif3(&self) -> GIF3_R {
        GIF3_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - transfer complete (TC) flag for channel 3"]
    #[inline(always)]
    pub fn tcif3(&self) -> TCIF3_R {
        TCIF3_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - half transfer (HT) flag for channel 3"]
    #[inline(always)]
    pub fn htif3(&self) -> HTIF3_R {
        HTIF3_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - transfer error (TE) flag for channel 3"]
    #[inline(always)]
    pub fn teif3(&self) -> TEIF3_R {
        TEIF3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - global interrupt flag for channel 4"]
    #[inline(always)]
    pub fn gif4(&self) -> GIF4_R {
        GIF4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - transfer complete (TC) flag for channel 4"]
    #[inline(always)]
    pub fn tcif4(&self) -> TCIF4_R {
        TCIF4_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - half transfer (HT) flag for channel 4"]
    #[inline(always)]
    pub fn htif4(&self) -> HTIF4_R {
        HTIF4_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - transfer error (TE) flag for channel 4"]
    #[inline(always)]
    pub fn teif4(&self) -> TEIF4_R {
        TEIF4_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - global interrupt flag for channel 5"]
    #[inline(always)]
    pub fn gif5(&self) -> GIF5_R {
        GIF5_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - transfer complete (TC) flag for channel 5"]
    #[inline(always)]
    pub fn tcif5(&self) -> TCIF5_R {
        TCIF5_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - half transfer (HT) flag for channel 5"]
    #[inline(always)]
    pub fn htif5(&self) -> HTIF5_R {
        HTIF5_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - transfer error (TE) flag for channel 5"]
    #[inline(always)]
    pub fn teif5(&self) -> TEIF5_R {
        TEIF5_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - global interrupt flag for channel 6"]
    #[inline(always)]
    pub fn gif6(&self) -> GIF6_R {
        GIF6_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - transfer complete (TC) flag for channel 6"]
    #[inline(always)]
    pub fn tcif6(&self) -> TCIF6_R {
        TCIF6_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - half transfer (HT) flag for channel 6"]
    #[inline(always)]
    pub fn htif6(&self) -> HTIF6_R {
        HTIF6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - transfer error (TE) flag for channel 6"]
    #[inline(always)]
    pub fn teif6(&self) -> TEIF6_R {
        TEIF6_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - global interrupt flag for channel 7"]
    #[inline(always)]
    pub fn gif7(&self) -> GIF7_R {
        GIF7_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - transfer complete (TC) flag for channel 7"]
    #[inline(always)]
    pub fn tcif7(&self) -> TCIF7_R {
        TCIF7_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - half transfer (HT) flag for channel 7"]
    #[inline(always)]
    pub fn htif7(&self) -> HTIF7_R {
        HTIF7_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - transfer error (TE) flag for channel 7"]
    #[inline(always)]
    pub fn teif7(&self) -> TEIF7_R {
        TEIF7_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for ISRrs {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISRrs {
    const RESET_VALUE: u32 = 0;
}
