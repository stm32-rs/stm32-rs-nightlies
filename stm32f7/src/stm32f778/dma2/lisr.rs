///Register `LISR` reader
pub type R = crate::R<LISRrs>;
/**Stream x FIFO error interrupt flag (x=3..0)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FEIF0 {
    ///0: No FIFO error event on stream x
    NoError = 0,
    ///1: A FIFO error event occurred on stream x
    Error = 1,
}
impl From<FEIF0> for bool {
    #[inline(always)]
    fn from(variant: FEIF0) -> Self {
        variant as u8 != 0
    }
}
///Field `FEIF0` reader - Stream x FIFO error interrupt flag (x=3..0)
pub type FEIF0_R = crate::BitReader<FEIF0>;
impl FEIF0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FEIF0 {
        match self.bits {
            false => FEIF0::NoError,
            true => FEIF0::Error,
        }
    }
    ///No FIFO error event on stream x
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == FEIF0::NoError
    }
    ///A FIFO error event occurred on stream x
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == FEIF0::Error
    }
}
/**Stream x direct mode error interrupt flag (x=3..0)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMEIF0 {
    ///0: No Direct Mode error on stream x
    NoError = 0,
    ///1: A Direct Mode error occurred on stream x
    Error = 1,
}
impl From<DMEIF0> for bool {
    #[inline(always)]
    fn from(variant: DMEIF0) -> Self {
        variant as u8 != 0
    }
}
///Field `DMEIF0` reader - Stream x direct mode error interrupt flag (x=3..0)
pub type DMEIF0_R = crate::BitReader<DMEIF0>;
impl DMEIF0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMEIF0 {
        match self.bits {
            false => DMEIF0::NoError,
            true => DMEIF0::Error,
        }
    }
    ///No Direct Mode error on stream x
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == DMEIF0::NoError
    }
    ///A Direct Mode error occurred on stream x
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == DMEIF0::Error
    }
}
/**Stream x transfer error interrupt flag (x=3..0)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEIF0 {
    ///0: No transfer error on stream x
    NoError = 0,
    ///1: A transfer error occurred on stream x
    Error = 1,
}
impl From<TEIF0> for bool {
    #[inline(always)]
    fn from(variant: TEIF0) -> Self {
        variant as u8 != 0
    }
}
///Field `TEIF0` reader - Stream x transfer error interrupt flag (x=3..0)
pub type TEIF0_R = crate::BitReader<TEIF0>;
impl TEIF0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TEIF0 {
        match self.bits {
            false => TEIF0::NoError,
            true => TEIF0::Error,
        }
    }
    ///No transfer error on stream x
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == TEIF0::NoError
    }
    ///A transfer error occurred on stream x
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == TEIF0::Error
    }
}
/**Stream x half transfer interrupt flag (x=3..0)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTIF0 {
    ///0: No half transfer event on stream x
    NotHalf = 0,
    ///1: A half transfer event occurred on stream x
    Half = 1,
}
impl From<HTIF0> for bool {
    #[inline(always)]
    fn from(variant: HTIF0) -> Self {
        variant as u8 != 0
    }
}
///Field `HTIF0` reader - Stream x half transfer interrupt flag (x=3..0)
pub type HTIF0_R = crate::BitReader<HTIF0>;
impl HTIF0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HTIF0 {
        match self.bits {
            false => HTIF0::NotHalf,
            true => HTIF0::Half,
        }
    }
    ///No half transfer event on stream x
    #[inline(always)]
    pub fn is_not_half(&self) -> bool {
        *self == HTIF0::NotHalf
    }
    ///A half transfer event occurred on stream x
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == HTIF0::Half
    }
}
/**Stream x transfer complete interrupt flag (x = 3..0)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIF0 {
    ///0: No transfer complete event on stream x
    NotComplete = 0,
    ///1: A transfer complete event occurred on stream x
    Complete = 1,
}
impl From<TCIF0> for bool {
    #[inline(always)]
    fn from(variant: TCIF0) -> Self {
        variant as u8 != 0
    }
}
///Field `TCIF0` reader - Stream x transfer complete interrupt flag (x = 3..0)
pub type TCIF0_R = crate::BitReader<TCIF0>;
impl TCIF0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCIF0 {
        match self.bits {
            false => TCIF0::NotComplete,
            true => TCIF0::Complete,
        }
    }
    ///No transfer complete event on stream x
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == TCIF0::NotComplete
    }
    ///A transfer complete event occurred on stream x
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == TCIF0::Complete
    }
}
///Field `DMEIF1` reader - Stream x direct mode error interrupt flag (x=3..0)
pub use DMEIF0_R as DMEIF1_R;
///Field `DMEIF2` reader - Stream x direct mode error interrupt flag (x=3..0)
pub use DMEIF0_R as DMEIF2_R;
///Field `DMEIF3` reader - Stream x direct mode error interrupt flag (x=3..0)
pub use DMEIF0_R as DMEIF3_R;
///Field `FEIF1` reader - Stream x FIFO error interrupt flag (x=3..0)
pub use FEIF0_R as FEIF1_R;
///Field `FEIF2` reader - Stream x FIFO error interrupt flag (x=3..0)
pub use FEIF0_R as FEIF2_R;
///Field `FEIF3` reader - Stream x FIFO error interrupt flag (x=3..0)
pub use FEIF0_R as FEIF3_R;
///Field `HTIF1` reader - Stream x half transfer interrupt flag (x=3..0)
pub use HTIF0_R as HTIF1_R;
///Field `HTIF2` reader - Stream x half transfer interrupt flag (x=3..0)
pub use HTIF0_R as HTIF2_R;
///Field `HTIF3` reader - Stream x half transfer interrupt flag (x=3..0)
pub use HTIF0_R as HTIF3_R;
///Field `TCIF1` reader - Stream x transfer complete interrupt flag (x = 3..0)
pub use TCIF0_R as TCIF1_R;
///Field `TCIF2` reader - Stream x transfer complete interrupt flag (x = 3..0)
pub use TCIF0_R as TCIF2_R;
///Field `TCIF3` reader - Stream x transfer complete interrupt flag (x = 3..0)
pub use TCIF0_R as TCIF3_R;
///Field `TEIF1` reader - Stream x transfer error interrupt flag (x=3..0)
pub use TEIF0_R as TEIF1_R;
///Field `TEIF2` reader - Stream x transfer error interrupt flag (x=3..0)
pub use TEIF0_R as TEIF2_R;
///Field `TEIF3` reader - Stream x transfer error interrupt flag (x=3..0)
pub use TEIF0_R as TEIF3_R;
impl R {
    ///Bit 0 - Stream x FIFO error interrupt flag (x=3..0)
    #[inline(always)]
    pub fn feif0(&self) -> FEIF0_R {
        FEIF0_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - Stream x direct mode error interrupt flag (x=3..0)
    #[inline(always)]
    pub fn dmeif0(&self) -> DMEIF0_R {
        DMEIF0_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Stream x transfer error interrupt flag (x=3..0)
    #[inline(always)]
    pub fn teif0(&self) -> TEIF0_R {
        TEIF0_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Stream x half transfer interrupt flag (x=3..0)
    #[inline(always)]
    pub fn htif0(&self) -> HTIF0_R {
        HTIF0_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Stream x transfer complete interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn tcif0(&self) -> TCIF0_R {
        TCIF0_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Stream x FIFO error interrupt flag (x=3..0)
    #[inline(always)]
    pub fn feif1(&self) -> FEIF1_R {
        FEIF1_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - Stream x direct mode error interrupt flag (x=3..0)
    #[inline(always)]
    pub fn dmeif1(&self) -> DMEIF1_R {
        DMEIF1_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Stream x transfer error interrupt flag (x=3..0)
    #[inline(always)]
    pub fn teif1(&self) -> TEIF1_R {
        TEIF1_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Stream x half transfer interrupt flag (x=3..0)
    #[inline(always)]
    pub fn htif1(&self) -> HTIF1_R {
        HTIF1_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Stream x transfer complete interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn tcif1(&self) -> TCIF1_R {
        TCIF1_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 16 - Stream x FIFO error interrupt flag (x=3..0)
    #[inline(always)]
    pub fn feif2(&self) -> FEIF2_R {
        FEIF2_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - Stream x direct mode error interrupt flag (x=3..0)
    #[inline(always)]
    pub fn dmeif2(&self) -> DMEIF2_R {
        DMEIF2_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Stream x transfer error interrupt flag (x=3..0)
    #[inline(always)]
    pub fn teif2(&self) -> TEIF2_R {
        TEIF2_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Stream x half transfer interrupt flag (x=3..0)
    #[inline(always)]
    pub fn htif2(&self) -> HTIF2_R {
        HTIF2_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Stream x transfer complete interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn tcif2(&self) -> TCIF2_R {
        TCIF2_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Stream x FIFO error interrupt flag (x=3..0)
    #[inline(always)]
    pub fn feif3(&self) -> FEIF3_R {
        FEIF3_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - Stream x direct mode error interrupt flag (x=3..0)
    #[inline(always)]
    pub fn dmeif3(&self) -> DMEIF3_R {
        DMEIF3_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Stream x transfer error interrupt flag (x=3..0)
    #[inline(always)]
    pub fn teif3(&self) -> TEIF3_R {
        TEIF3_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Stream x half transfer interrupt flag (x=3..0)
    #[inline(always)]
    pub fn htif3(&self) -> HTIF3_R {
        HTIF3_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Stream x transfer complete interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn tcif3(&self) -> TCIF3_R {
        TCIF3_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LISR")
            .field("tcif0", &self.tcif0())
            .field("tcif3", &self.tcif3())
            .field("htif0", &self.htif0())
            .field("htif3", &self.htif3())
            .field("teif0", &self.teif0())
            .field("teif3", &self.teif3())
            .field("dmeif0", &self.dmeif0())
            .field("dmeif3", &self.dmeif3())
            .field("feif0", &self.feif0())
            .field("feif3", &self.feif3())
            .field("tcif2", &self.tcif2())
            .field("htif2", &self.htif2())
            .field("teif2", &self.teif2())
            .field("dmeif2", &self.dmeif2())
            .field("feif2", &self.feif2())
            .field("tcif1", &self.tcif1())
            .field("htif1", &self.htif1())
            .field("teif1", &self.teif1())
            .field("dmeif1", &self.dmeif1())
            .field("feif1", &self.feif1())
            .finish()
    }
}
/**low interrupt status register

You can [`read`](crate::Reg::read) this register and get [`lisr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F778.html#DMA2:LISR)*/
pub struct LISRrs;
impl crate::RegisterSpec for LISRrs {
    type Ux = u32;
}
///`read()` method returns [`lisr::R`](R) reader structure
impl crate::Readable for LISRrs {}
///`reset()` method sets LISR to value 0
impl crate::Resettable for LISRrs {}
