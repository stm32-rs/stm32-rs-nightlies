///Register `ISR` reader
pub type R = crate::R<ISRrs>;
/**Channel %s Global interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GIF0 {
    ///0: No TE, HT or TC event on channel x
    NoEvent = 0,
    ///1: A TE, HT or TC event occurred on channel x
    Event = 1,
}
impl From<GIF0> for bool {
    #[inline(always)]
    fn from(variant: GIF0) -> Self {
        variant as u8 != 0
    }
}
///Field `GIF(0-7)` reader - Channel %s Global interrupt flag
pub type GIF_R = crate::BitReader<GIF0>;
impl GIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GIF0 {
        match self.bits {
            false => GIF0::NoEvent,
            true => GIF0::Event,
        }
    }
    ///No TE, HT or TC event on channel x
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == GIF0::NoEvent
    }
    ///A TE, HT or TC event occurred on channel x
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == GIF0::Event
    }
}
/**Channel %s Transfer Complete flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIF0 {
    ///0: No transfer complete event on channel x
    NotComplete = 0,
    ///1: A transfer complete event occurred on channel x
    Complete = 1,
}
impl From<TCIF0> for bool {
    #[inline(always)]
    fn from(variant: TCIF0) -> Self {
        variant as u8 != 0
    }
}
///Field `TCIF(0-7)` reader - Channel %s Transfer Complete flag
pub type TCIF_R = crate::BitReader<TCIF0>;
impl TCIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCIF0 {
        match self.bits {
            false => TCIF0::NotComplete,
            true => TCIF0::Complete,
        }
    }
    ///No transfer complete event on channel x
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == TCIF0::NotComplete
    }
    ///A transfer complete event occurred on channel x
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == TCIF0::Complete
    }
}
/**Channel %s Half Transfer Complete flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTIF0 {
    ///0: No half transfer event on channel x
    NotHalf = 0,
    ///1: A half transfer event occurred on channel x
    Half = 1,
}
impl From<HTIF0> for bool {
    #[inline(always)]
    fn from(variant: HTIF0) -> Self {
        variant as u8 != 0
    }
}
///Field `HTIF(0-7)` reader - Channel %s Half Transfer Complete flag
pub type HTIF_R = crate::BitReader<HTIF0>;
impl HTIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HTIF0 {
        match self.bits {
            false => HTIF0::NotHalf,
            true => HTIF0::Half,
        }
    }
    ///No half transfer event on channel x
    #[inline(always)]
    pub fn is_not_half(&self) -> bool {
        *self == HTIF0::NotHalf
    }
    ///A half transfer event occurred on channel x
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == HTIF0::Half
    }
}
/**Channel %s Transfer Error flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEIF0 {
    ///0: No transfer error on channel x
    NoError = 0,
    ///1: A transfer error occurred on channel x
    Error = 1,
}
impl From<TEIF0> for bool {
    #[inline(always)]
    fn from(variant: TEIF0) -> Self {
        variant as u8 != 0
    }
}
///Field `TEIF(0-7)` reader - Channel %s Transfer Error flag
pub type TEIF_R = crate::BitReader<TEIF0>;
impl TEIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TEIF0 {
        match self.bits {
            false => TEIF0::NoError,
            true => TEIF0::Error,
        }
    }
    ///No transfer error on channel x
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == TEIF0::NoError
    }
    ///A transfer error occurred on channel x
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == TEIF0::Error
    }
}
impl R {
    ///Channel (0-7) Global interrupt flag
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `GIF0` field.</div>
    #[inline(always)]
    pub fn gif(&self, n: u8) -> GIF_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        GIF_R::new(((self.bits >> (n * 4)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Channel (0-7) Global interrupt flag
    #[inline(always)]
    pub fn gif_iter(&self) -> impl Iterator<Item = GIF_R> + '_ {
        (0..8).map(move |n| GIF_R::new(((self.bits >> (n * 4)) & 1) != 0))
    }
    ///Bit 0 - Channel 0 Global interrupt flag
    #[inline(always)]
    pub fn gif0(&self) -> GIF_R {
        GIF_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - Channel 1 Global interrupt flag
    #[inline(always)]
    pub fn gif1(&self) -> GIF_R {
        GIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - Channel 2 Global interrupt flag
    #[inline(always)]
    pub fn gif2(&self) -> GIF_R {
        GIF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - Channel 3 Global interrupt flag
    #[inline(always)]
    pub fn gif3(&self) -> GIF_R {
        GIF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - Channel 4 Global interrupt flag
    #[inline(always)]
    pub fn gif4(&self) -> GIF_R {
        GIF_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 20 - Channel 5 Global interrupt flag
    #[inline(always)]
    pub fn gif5(&self) -> GIF_R {
        GIF_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 24 - Channel 6 Global interrupt flag
    #[inline(always)]
    pub fn gif6(&self) -> GIF_R {
        GIF_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 28 - Channel 7 Global interrupt flag
    #[inline(always)]
    pub fn gif7(&self) -> GIF_R {
        GIF_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Channel (0-7) Transfer Complete flag
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `TCIF0` field.</div>
    #[inline(always)]
    pub fn tcif(&self, n: u8) -> TCIF_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        TCIF_R::new(((self.bits >> (n * 4 + 1)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Channel (0-7) Transfer Complete flag
    #[inline(always)]
    pub fn tcif_iter(&self) -> impl Iterator<Item = TCIF_R> + '_ {
        (0..8).map(move |n| TCIF_R::new(((self.bits >> (n * 4 + 1)) & 1) != 0))
    }
    ///Bit 1 - Channel 0 Transfer Complete flag
    #[inline(always)]
    pub fn tcif0(&self) -> TCIF_R {
        TCIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - Channel 1 Transfer Complete flag
    #[inline(always)]
    pub fn tcif1(&self) -> TCIF_R {
        TCIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 9 - Channel 2 Transfer Complete flag
    #[inline(always)]
    pub fn tcif2(&self) -> TCIF_R {
        TCIF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 13 - Channel 3 Transfer Complete flag
    #[inline(always)]
    pub fn tcif3(&self) -> TCIF_R {
        TCIF_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 17 - Channel 4 Transfer Complete flag
    #[inline(always)]
    pub fn tcif4(&self) -> TCIF_R {
        TCIF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 21 - Channel 5 Transfer Complete flag
    #[inline(always)]
    pub fn tcif5(&self) -> TCIF_R {
        TCIF_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 25 - Channel 6 Transfer Complete flag
    #[inline(always)]
    pub fn tcif6(&self) -> TCIF_R {
        TCIF_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 29 - Channel 7 Transfer Complete flag
    #[inline(always)]
    pub fn tcif7(&self) -> TCIF_R {
        TCIF_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Channel (0-7) Half Transfer Complete flag
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `HTIF0` field.</div>
    #[inline(always)]
    pub fn htif(&self, n: u8) -> HTIF_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        HTIF_R::new(((self.bits >> (n * 4 + 2)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Channel (0-7) Half Transfer Complete flag
    #[inline(always)]
    pub fn htif_iter(&self) -> impl Iterator<Item = HTIF_R> + '_ {
        (0..8).map(move |n| HTIF_R::new(((self.bits >> (n * 4 + 2)) & 1) != 0))
    }
    ///Bit 2 - Channel 0 Half Transfer Complete flag
    #[inline(always)]
    pub fn htif0(&self) -> HTIF_R {
        HTIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 6 - Channel 1 Half Transfer Complete flag
    #[inline(always)]
    pub fn htif1(&self) -> HTIF_R {
        HTIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 10 - Channel 2 Half Transfer Complete flag
    #[inline(always)]
    pub fn htif2(&self) -> HTIF_R {
        HTIF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 14 - Channel 3 Half Transfer Complete flag
    #[inline(always)]
    pub fn htif3(&self) -> HTIF_R {
        HTIF_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 18 - Channel 4 Half Transfer Complete flag
    #[inline(always)]
    pub fn htif4(&self) -> HTIF_R {
        HTIF_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 22 - Channel 5 Half Transfer Complete flag
    #[inline(always)]
    pub fn htif5(&self) -> HTIF_R {
        HTIF_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 26 - Channel 6 Half Transfer Complete flag
    #[inline(always)]
    pub fn htif6(&self) -> HTIF_R {
        HTIF_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 30 - Channel 7 Half Transfer Complete flag
    #[inline(always)]
    pub fn htif7(&self) -> HTIF_R {
        HTIF_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Channel (0-7) Transfer Error flag
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `TEIF0` field.</div>
    #[inline(always)]
    pub fn teif(&self, n: u8) -> TEIF_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        TEIF_R::new(((self.bits >> (n * 4 + 3)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Channel (0-7) Transfer Error flag
    #[inline(always)]
    pub fn teif_iter(&self) -> impl Iterator<Item = TEIF_R> + '_ {
        (0..8).map(move |n| TEIF_R::new(((self.bits >> (n * 4 + 3)) & 1) != 0))
    }
    ///Bit 3 - Channel 0 Transfer Error flag
    #[inline(always)]
    pub fn teif0(&self) -> TEIF_R {
        TEIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 7 - Channel 1 Transfer Error flag
    #[inline(always)]
    pub fn teif1(&self) -> TEIF_R {
        TEIF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 11 - Channel 2 Transfer Error flag
    #[inline(always)]
    pub fn teif2(&self) -> TEIF_R {
        TEIF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 15 - Channel 3 Transfer Error flag
    #[inline(always)]
    pub fn teif3(&self) -> TEIF_R {
        TEIF_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 19 - Channel 4 Transfer Error flag
    #[inline(always)]
    pub fn teif4(&self) -> TEIF_R {
        TEIF_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 23 - Channel 5 Transfer Error flag
    #[inline(always)]
    pub fn teif5(&self) -> TEIF_R {
        TEIF_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 27 - Channel 6 Transfer Error flag
    #[inline(always)]
    pub fn teif6(&self) -> TEIF_R {
        TEIF_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 31 - Channel 7 Transfer Error flag
    #[inline(always)]
    pub fn teif7(&self) -> TEIF_R {
        TEIF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("gif0", &self.gif0())
            .field("gif1", &self.gif1())
            .field("gif2", &self.gif2())
            .field("gif3", &self.gif3())
            .field("gif4", &self.gif4())
            .field("gif5", &self.gif5())
            .field("gif6", &self.gif6())
            .field("gif7", &self.gif7())
            .field("tcif0", &self.tcif0())
            .field("tcif1", &self.tcif1())
            .field("tcif2", &self.tcif2())
            .field("tcif3", &self.tcif3())
            .field("tcif4", &self.tcif4())
            .field("tcif5", &self.tcif5())
            .field("tcif6", &self.tcif6())
            .field("tcif7", &self.tcif7())
            .field("htif0", &self.htif0())
            .field("htif1", &self.htif1())
            .field("htif2", &self.htif2())
            .field("htif3", &self.htif3())
            .field("htif4", &self.htif4())
            .field("htif5", &self.htif5())
            .field("htif6", &self.htif6())
            .field("htif7", &self.htif7())
            .field("teif0", &self.teif0())
            .field("teif1", &self.teif1())
            .field("teif2", &self.teif2())
            .field("teif3", &self.teif3())
            .field("teif4", &self.teif4())
            .field("teif5", &self.teif5())
            .field("teif6", &self.teif6())
            .field("teif7", &self.teif7())
            .finish()
    }
}
/**BDMA interrupt status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#BDMA:ISR)*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISRrs {}
