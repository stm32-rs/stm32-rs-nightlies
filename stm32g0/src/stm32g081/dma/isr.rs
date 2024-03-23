#[doc = "Register `ISR` reader"]
pub type R = crate::R<ISRrs>;
#[doc = "Channel %s Global interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GIF1 {
    #[doc = "0: No transfer error, half event, complete event"]
    NoEvent = 0,
    #[doc = "1: A transfer error, half event or complete event has occured"]
    Event = 1,
}
impl From<GIF1> for bool {
    #[inline(always)]
    fn from(variant: GIF1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GIF(1-7)` reader - Channel %s Global interrupt flag"]
pub type GIF_R = crate::BitReader<GIF1>;
impl GIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GIF1 {
        match self.bits {
            false => GIF1::NoEvent,
            true => GIF1::Event,
        }
    }
    #[doc = "No transfer error, half event, complete event"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == GIF1::NoEvent
    }
    #[doc = "A transfer error, half event or complete event has occured"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == GIF1::Event
    }
}
#[doc = "Channel %s Transfer Complete flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIF1 {
    #[doc = "0: No transfer complete event"]
    NotComplete = 0,
    #[doc = "1: A transfer complete event has occured"]
    Complete = 1,
}
impl From<TCIF1> for bool {
    #[inline(always)]
    fn from(variant: TCIF1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCIF(1-7)` reader - Channel %s Transfer Complete flag"]
pub type TCIF_R = crate::BitReader<TCIF1>;
impl TCIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCIF1 {
        match self.bits {
            false => TCIF1::NotComplete,
            true => TCIF1::Complete,
        }
    }
    #[doc = "No transfer complete event"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == TCIF1::NotComplete
    }
    #[doc = "A transfer complete event has occured"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == TCIF1::Complete
    }
}
#[doc = "Channel %s Half Transfer Complete flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTIF1 {
    #[doc = "0: No half transfer event"]
    NotHalf = 0,
    #[doc = "1: A half transfer event has occured"]
    Half = 1,
}
impl From<HTIF1> for bool {
    #[inline(always)]
    fn from(variant: HTIF1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HTIF(1-7)` reader - Channel %s Half Transfer Complete flag"]
pub type HTIF_R = crate::BitReader<HTIF1>;
impl HTIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HTIF1 {
        match self.bits {
            false => HTIF1::NotHalf,
            true => HTIF1::Half,
        }
    }
    #[doc = "No half transfer event"]
    #[inline(always)]
    pub fn is_not_half(&self) -> bool {
        *self == HTIF1::NotHalf
    }
    #[doc = "A half transfer event has occured"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == HTIF1::Half
    }
}
#[doc = "Channel %s Transfer Error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEIF1 {
    #[doc = "0: No transfer error"]
    NoError = 0,
    #[doc = "1: A transfer error has occured"]
    Error = 1,
}
impl From<TEIF1> for bool {
    #[inline(always)]
    fn from(variant: TEIF1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEIF(1-7)` reader - Channel %s Transfer Error flag"]
pub type TEIF_R = crate::BitReader<TEIF1>;
impl TEIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TEIF1 {
        match self.bits {
            false => TEIF1::NoError,
            true => TEIF1::Error,
        }
    }
    #[doc = "No transfer error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == TEIF1::NoError
    }
    #[doc = "A transfer error has occured"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == TEIF1::Error
    }
}
impl R {
    #[doc = "Channel (1-7) Global interrupt flag"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `GIF1` field"]
    #[inline(always)]
    pub fn gif(&self, n: u8) -> GIF_R {
        #[allow(clippy::no_effect)]
        [(); 7][n as usize];
        GIF_R::new(((self.bits >> (n * 4)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel (1-7) Global interrupt flag"]
    #[inline(always)]
    pub fn gif_iter(&self) -> impl Iterator<Item = GIF_R> + '_ {
        (0..7).map(move |n| GIF_R::new(((self.bits >> (n * 4)) & 1) != 0))
    }
    #[doc = "Bit 0 - Channel 1 Global interrupt flag"]
    #[inline(always)]
    pub fn gif1(&self) -> GIF_R {
        GIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 2 Global interrupt flag"]
    #[inline(always)]
    pub fn gif2(&self) -> GIF_R {
        GIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 3 Global interrupt flag"]
    #[inline(always)]
    pub fn gif3(&self) -> GIF_R {
        GIF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 4 Global interrupt flag"]
    #[inline(always)]
    pub fn gif4(&self) -> GIF_R {
        GIF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Channel 5 Global interrupt flag"]
    #[inline(always)]
    pub fn gif5(&self) -> GIF_R {
        GIF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Channel 6 Global interrupt flag"]
    #[inline(always)]
    pub fn gif6(&self) -> GIF_R {
        GIF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Channel 7 Global interrupt flag"]
    #[inline(always)]
    pub fn gif7(&self) -> GIF_R {
        GIF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Channel (1-7) Transfer Complete flag"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `TCIF1` field"]
    #[inline(always)]
    pub fn tcif(&self, n: u8) -> TCIF_R {
        #[allow(clippy::no_effect)]
        [(); 7][n as usize];
        TCIF_R::new(((self.bits >> (n * 4 + 1)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel (1-7) Transfer Complete flag"]
    #[inline(always)]
    pub fn tcif_iter(&self) -> impl Iterator<Item = TCIF_R> + '_ {
        (0..7).map(move |n| TCIF_R::new(((self.bits >> (n * 4 + 1)) & 1) != 0))
    }
    #[doc = "Bit 1 - Channel 1 Transfer Complete flag"]
    #[inline(always)]
    pub fn tcif1(&self) -> TCIF_R {
        TCIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 2 Transfer Complete flag"]
    #[inline(always)]
    pub fn tcif2(&self) -> TCIF_R {
        TCIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 3 Transfer Complete flag"]
    #[inline(always)]
    pub fn tcif3(&self) -> TCIF_R {
        TCIF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel 4 Transfer Complete flag"]
    #[inline(always)]
    pub fn tcif4(&self) -> TCIF_R {
        TCIF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 17 - Channel 5 Transfer Complete flag"]
    #[inline(always)]
    pub fn tcif5(&self) -> TCIF_R {
        TCIF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 21 - Channel 6 Transfer Complete flag"]
    #[inline(always)]
    pub fn tcif6(&self) -> TCIF_R {
        TCIF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 25 - Channel 7 Transfer Complete flag"]
    #[inline(always)]
    pub fn tcif7(&self) -> TCIF_R {
        TCIF_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Channel (1-7) Half Transfer Complete flag"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `HTIF1` field"]
    #[inline(always)]
    pub fn htif(&self, n: u8) -> HTIF_R {
        #[allow(clippy::no_effect)]
        [(); 7][n as usize];
        HTIF_R::new(((self.bits >> (n * 4 + 2)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel (1-7) Half Transfer Complete flag"]
    #[inline(always)]
    pub fn htif_iter(&self) -> impl Iterator<Item = HTIF_R> + '_ {
        (0..7).map(move |n| HTIF_R::new(((self.bits >> (n * 4 + 2)) & 1) != 0))
    }
    #[doc = "Bit 2 - Channel 1 Half Transfer Complete flag"]
    #[inline(always)]
    pub fn htif1(&self) -> HTIF_R {
        HTIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 2 Half Transfer Complete flag"]
    #[inline(always)]
    pub fn htif2(&self) -> HTIF_R {
        HTIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 3 Half Transfer Complete flag"]
    #[inline(always)]
    pub fn htif3(&self) -> HTIF_R {
        HTIF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel 4 Half Transfer Complete flag"]
    #[inline(always)]
    pub fn htif4(&self) -> HTIF_R {
        HTIF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 18 - Channel 5 Half Transfer Complete flag"]
    #[inline(always)]
    pub fn htif5(&self) -> HTIF_R {
        HTIF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 22 - Channel 6 Half Transfer Complete flag"]
    #[inline(always)]
    pub fn htif6(&self) -> HTIF_R {
        HTIF_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 26 - Channel 7 Half Transfer Complete flag"]
    #[inline(always)]
    pub fn htif7(&self) -> HTIF_R {
        HTIF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Channel (1-7) Transfer Error flag"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `TEIF1` field"]
    #[inline(always)]
    pub fn teif(&self, n: u8) -> TEIF_R {
        #[allow(clippy::no_effect)]
        [(); 7][n as usize];
        TEIF_R::new(((self.bits >> (n * 4 + 3)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel (1-7) Transfer Error flag"]
    #[inline(always)]
    pub fn teif_iter(&self) -> impl Iterator<Item = TEIF_R> + '_ {
        (0..7).map(move |n| TEIF_R::new(((self.bits >> (n * 4 + 3)) & 1) != 0))
    }
    #[doc = "Bit 3 - Channel 1 Transfer Error flag"]
    #[inline(always)]
    pub fn teif1(&self) -> TEIF_R {
        TEIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 2 Transfer Error flag"]
    #[inline(always)]
    pub fn teif2(&self) -> TEIF_R {
        TEIF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 3 Transfer Error flag"]
    #[inline(always)]
    pub fn teif3(&self) -> TEIF_R {
        TEIF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - Channel 4 Transfer Error flag"]
    #[inline(always)]
    pub fn teif4(&self) -> TEIF_R {
        TEIF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 19 - Channel 5 Transfer Error flag"]
    #[inline(always)]
    pub fn teif5(&self) -> TEIF_R {
        TEIF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 23 - Channel 6 Transfer Error flag"]
    #[inline(always)]
    pub fn teif6(&self) -> TEIF_R {
        TEIF_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 27 - Channel 7 Transfer Error flag"]
    #[inline(always)]
    pub fn teif7(&self) -> TEIF_R {
        TEIF_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "low interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
