#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "HSYNC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSYNC {
    #[doc = "0: Active line"]
    ActiveLine = 0,
    #[doc = "1: Synchronization between lines"]
    BetweenLines = 1,
}
impl From<HSYNC> for bool {
    #[inline(always)]
    fn from(variant: HSYNC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSYNC` reader - HSYNC"]
pub type HSYNC_R = crate::BitReader<HSYNC>;
impl HSYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSYNC {
        match self.bits {
            false => HSYNC::ActiveLine,
            true => HSYNC::BetweenLines,
        }
    }
    #[doc = "Active line"]
    #[inline(always)]
    pub fn is_active_line(&self) -> bool {
        *self == HSYNC::ActiveLine
    }
    #[doc = "Synchronization between lines"]
    #[inline(always)]
    pub fn is_between_lines(&self) -> bool {
        *self == HSYNC::BetweenLines
    }
}
#[doc = "VSYNC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VSYNC {
    #[doc = "0: Active frame"]
    ActiveFrame = 0,
    #[doc = "1: Synchronization between frames"]
    BetweenFrames = 1,
}
impl From<VSYNC> for bool {
    #[inline(always)]
    fn from(variant: VSYNC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VSYNC` reader - VSYNC"]
pub type VSYNC_R = crate::BitReader<VSYNC>;
impl VSYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VSYNC {
        match self.bits {
            false => VSYNC::ActiveFrame,
            true => VSYNC::BetweenFrames,
        }
    }
    #[doc = "Active frame"]
    #[inline(always)]
    pub fn is_active_frame(&self) -> bool {
        *self == VSYNC::ActiveFrame
    }
    #[doc = "Synchronization between frames"]
    #[inline(always)]
    pub fn is_between_frames(&self) -> bool {
        *self == VSYNC::BetweenFrames
    }
}
#[doc = "FIFO not empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FNE {
    #[doc = "0: FIFO contains valid data"]
    NotEmpty = 0,
    #[doc = "1: FIFO empty"]
    Empty = 1,
}
impl From<FNE> for bool {
    #[inline(always)]
    fn from(variant: FNE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FNE` reader - FIFO not empty"]
pub type FNE_R = crate::BitReader<FNE>;
impl FNE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FNE {
        match self.bits {
            false => FNE::NotEmpty,
            true => FNE::Empty,
        }
    }
    #[doc = "FIFO contains valid data"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == FNE::NotEmpty
    }
    #[doc = "FIFO empty"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == FNE::Empty
    }
}
impl R {
    #[doc = "Bit 0 - HSYNC"]
    #[inline(always)]
    pub fn hsync(&self) -> HSYNC_R {
        HSYNC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VSYNC"]
    #[inline(always)]
    pub fn vsync(&self) -> VSYNC_R {
        VSYNC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FIFO not empty"]
    #[inline(always)]
    pub fn fne(&self) -> FNE_R {
        FNE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SRrs {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0;
}
