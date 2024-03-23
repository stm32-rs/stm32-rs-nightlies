#[doc = "Register `PUPDR` reader"]
pub type R = crate::R<PUPDRrs>;
#[doc = "Register `PUPDR` writer"]
pub type W = crate::W<PUPDRrs>;
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PUPD0 {
    #[doc = "0: No pull-up, pull-down"]
    Floating = 0,
    #[doc = "1: Pull-up"]
    PullUp = 1,
    #[doc = "2: Pull-down"]
    PullDown = 2,
}
impl From<PUPD0> for u8 {
    #[inline(always)]
    fn from(variant: PUPD0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PUPD0 {
    type Ux = u8;
}
#[doc = "Field `PUPD0` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type PUPD0_R = crate::FieldReader<PUPD0>;
impl PUPD0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PUPD0> {
        match self.bits {
            0 => Some(PUPD0::Floating),
            1 => Some(PUPD0::PullUp),
            2 => Some(PUPD0::PullDown),
            _ => None,
        }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn is_floating(&self) -> bool {
        *self == PUPD0::Floating
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PUPD0::PullUp
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PUPD0::PullDown
    }
}
#[doc = "Field `PUPD0` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type PUPD0_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PUPD0>;
impl<'a, REG> PUPD0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn floating(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD0::Floating)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD0::PullUp)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD0::PullDown)
    }
}
#[doc = "Field `PUPD1` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use PUPD0_R as PUPD1_R;
#[doc = "Field `PUPD2` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use PUPD0_R as PUPD2_R;
#[doc = "Field `PUPD3` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use PUPD0_R as PUPD3_R;
#[doc = "Field `PUPD4` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use PUPD0_R as PUPD4_R;
#[doc = "Field `PUPD5` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use PUPD0_R as PUPD5_R;
#[doc = "Field `PUPD6` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use PUPD0_R as PUPD6_R;
#[doc = "Field `PUPD7` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use PUPD0_R as PUPD7_R;
#[doc = "Field `PUPD8` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use PUPD0_R as PUPD8_R;
#[doc = "Field `PUPD9` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use PUPD0_R as PUPD9_R;
#[doc = "Field `PUPD10` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use PUPD0_R as PUPD10_R;
#[doc = "Field `PUPD11` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use PUPD0_R as PUPD11_R;
#[doc = "Field `PUPD12` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use PUPD0_R as PUPD12_R;
#[doc = "Field `PUPD13` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use PUPD0_R as PUPD13_R;
#[doc = "Field `PUPD14` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use PUPD0_R as PUPD14_R;
#[doc = "Field `PUPD15` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use PUPD0_R as PUPD15_R;
#[doc = "Field `PUPD1` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use PUPD0_W as PUPD1_W;
#[doc = "Field `PUPD2` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use PUPD0_W as PUPD2_W;
#[doc = "Field `PUPD3` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use PUPD0_W as PUPD3_W;
#[doc = "Field `PUPD4` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use PUPD0_W as PUPD4_W;
#[doc = "Field `PUPD5` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use PUPD0_W as PUPD5_W;
#[doc = "Field `PUPD6` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use PUPD0_W as PUPD6_W;
#[doc = "Field `PUPD7` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use PUPD0_W as PUPD7_W;
#[doc = "Field `PUPD8` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use PUPD0_W as PUPD8_W;
#[doc = "Field `PUPD9` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use PUPD0_W as PUPD9_W;
#[doc = "Field `PUPD10` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use PUPD0_W as PUPD10_W;
#[doc = "Field `PUPD11` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use PUPD0_W as PUPD11_W;
#[doc = "Field `PUPD12` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use PUPD0_W as PUPD12_W;
#[doc = "Field `PUPD13` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use PUPD0_W as PUPD13_W;
#[doc = "Field `PUPD14` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use PUPD0_W as PUPD14_W;
#[doc = "Field `PUPD15` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use PUPD0_W as PUPD15_W;
impl R {
    #[doc = "Bits 0:1 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn pupd0(&self) -> PUPD0_R {
        PUPD0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn pupd1(&self) -> PUPD1_R {
        PUPD1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn pupd2(&self) -> PUPD2_R {
        PUPD2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn pupd3(&self) -> PUPD3_R {
        PUPD3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn pupd4(&self) -> PUPD4_R {
        PUPD4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn pupd5(&self) -> PUPD5_R {
        PUPD5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn pupd6(&self) -> PUPD6_R {
        PUPD6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn pupd7(&self) -> PUPD7_R {
        PUPD7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn pupd8(&self) -> PUPD8_R {
        PUPD8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn pupd9(&self) -> PUPD9_R {
        PUPD9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn pupd10(&self) -> PUPD10_R {
        PUPD10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn pupd11(&self) -> PUPD11_R {
        PUPD11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn pupd12(&self) -> PUPD12_R {
        PUPD12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn pupd13(&self) -> PUPD13_R {
        PUPD13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn pupd14(&self) -> PUPD14_R {
        PUPD14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn pupd15(&self) -> PUPD15_R {
        PUPD15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn pupd0(&mut self) -> PUPD0_W<PUPDRrs> {
        PUPD0_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn pupd1(&mut self) -> PUPD1_W<PUPDRrs> {
        PUPD1_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn pupd2(&mut self) -> PUPD2_W<PUPDRrs> {
        PUPD2_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn pupd3(&mut self) -> PUPD3_W<PUPDRrs> {
        PUPD3_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn pupd4(&mut self) -> PUPD4_W<PUPDRrs> {
        PUPD4_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn pupd5(&mut self) -> PUPD5_W<PUPDRrs> {
        PUPD5_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn pupd6(&mut self) -> PUPD6_W<PUPDRrs> {
        PUPD6_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn pupd7(&mut self) -> PUPD7_W<PUPDRrs> {
        PUPD7_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn pupd8(&mut self) -> PUPD8_W<PUPDRrs> {
        PUPD8_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn pupd9(&mut self) -> PUPD9_W<PUPDRrs> {
        PUPD9_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn pupd10(&mut self) -> PUPD10_W<PUPDRrs> {
        PUPD10_W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn pupd11(&mut self) -> PUPD11_W<PUPDRrs> {
        PUPD11_W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn pupd12(&mut self) -> PUPD12_W<PUPDRrs> {
        PUPD12_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn pupd13(&mut self) -> PUPD13_W<PUPDRrs> {
        PUPD13_W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn pupd14(&mut self) -> PUPD14_W<PUPDRrs> {
        PUPD14_W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn pupd15(&mut self) -> PUPD15_W<PUPDRrs> {
        PUPD15_W::new(self, 30)
    }
}
#[doc = "GPIO port pull-up/pull-down register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pupdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pupdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PUPDRrs;
impl crate::RegisterSpec for PUPDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pupdr::R`](R) reader structure"]
impl crate::Readable for PUPDRrs {}
#[doc = "`write(|w| ..)` method takes [`pupdr::W`](W) writer structure"]
impl crate::Writable for PUPDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PUPDR to value 0x6400_0000"]
impl crate::Resettable for PUPDRrs {
    const RESET_VALUE: u32 = 0x6400_0000;
}
