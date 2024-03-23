#[doc = "Register `OSPEEDR` reader"]
pub type R = crate::R<OSPEEDRrs>;
#[doc = "Register `OSPEEDR` writer"]
pub type W = crate::W<OSPEEDRrs>;
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSPEED0 {
    #[doc = "0: Low speed"]
    LowSpeed = 0,
    #[doc = "1: Medium speed"]
    MediumSpeed = 1,
    #[doc = "2: High speed"]
    HighSpeed = 2,
    #[doc = "3: Very high speed"]
    VeryHighSpeed = 3,
}
impl From<OSPEED0> for u8 {
    #[inline(always)]
    fn from(variant: OSPEED0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OSPEED0 {
    type Ux = u8;
}
#[doc = "Field `OSPEED0` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OSPEED0_R = crate::FieldReader<OSPEED0>;
impl OSPEED0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OSPEED0 {
        match self.bits {
            0 => OSPEED0::LowSpeed,
            1 => OSPEED0::MediumSpeed,
            2 => OSPEED0::HighSpeed,
            3 => OSPEED0::VeryHighSpeed,
            _ => unreachable!(),
        }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn is_low_speed(&self) -> bool {
        *self == OSPEED0::LowSpeed
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn is_medium_speed(&self) -> bool {
        *self == OSPEED0::MediumSpeed
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        *self == OSPEED0::HighSpeed
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn is_very_high_speed(&self) -> bool {
        *self == OSPEED0::VeryHighSpeed
    }
}
#[doc = "Field `OSPEED0` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OSPEED0_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, OSPEED0>;
impl<'a, REG> OSPEED0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED0::LowSpeed)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED0::MediumSpeed)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED0::HighSpeed)
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn very_high_speed(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED0::VeryHighSpeed)
    }
}
#[doc = "Field `OSPEED1` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OSPEED0_R as OSPEED1_R;
#[doc = "Field `OSPEED2` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OSPEED0_R as OSPEED2_R;
#[doc = "Field `OSPEED3` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OSPEED0_R as OSPEED3_R;
#[doc = "Field `OSPEED4` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OSPEED0_R as OSPEED4_R;
#[doc = "Field `OSPEED5` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OSPEED0_R as OSPEED5_R;
#[doc = "Field `OSPEED6` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OSPEED0_R as OSPEED6_R;
#[doc = "Field `OSPEED7` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OSPEED0_R as OSPEED7_R;
#[doc = "Field `OSPEED8` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OSPEED0_R as OSPEED8_R;
#[doc = "Field `OSPEED9` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OSPEED0_R as OSPEED9_R;
#[doc = "Field `OSPEED10` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OSPEED0_R as OSPEED10_R;
#[doc = "Field `OSPEED11` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OSPEED0_R as OSPEED11_R;
#[doc = "Field `OSPEED12` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OSPEED0_R as OSPEED12_R;
#[doc = "Field `OSPEED13` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OSPEED0_R as OSPEED13_R;
#[doc = "Field `OSPEED14` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OSPEED0_R as OSPEED14_R;
#[doc = "Field `OSPEED15` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OSPEED0_R as OSPEED15_R;
#[doc = "Field `OSPEED1` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OSPEED0_W as OSPEED1_W;
#[doc = "Field `OSPEED2` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OSPEED0_W as OSPEED2_W;
#[doc = "Field `OSPEED3` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OSPEED0_W as OSPEED3_W;
#[doc = "Field `OSPEED4` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OSPEED0_W as OSPEED4_W;
#[doc = "Field `OSPEED5` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OSPEED0_W as OSPEED5_W;
#[doc = "Field `OSPEED6` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OSPEED0_W as OSPEED6_W;
#[doc = "Field `OSPEED7` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OSPEED0_W as OSPEED7_W;
#[doc = "Field `OSPEED8` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OSPEED0_W as OSPEED8_W;
#[doc = "Field `OSPEED9` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OSPEED0_W as OSPEED9_W;
#[doc = "Field `OSPEED10` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OSPEED0_W as OSPEED10_W;
#[doc = "Field `OSPEED11` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OSPEED0_W as OSPEED11_W;
#[doc = "Field `OSPEED12` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OSPEED0_W as OSPEED12_W;
#[doc = "Field `OSPEED13` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OSPEED0_W as OSPEED13_W;
#[doc = "Field `OSPEED14` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OSPEED0_W as OSPEED14_W;
#[doc = "Field `OSPEED15` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OSPEED0_W as OSPEED15_W;
impl R {
    #[doc = "Bits 0:1 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn ospeed0(&self) -> OSPEED0_R {
        OSPEED0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn ospeed1(&self) -> OSPEED1_R {
        OSPEED1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn ospeed2(&self) -> OSPEED2_R {
        OSPEED2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn ospeed3(&self) -> OSPEED3_R {
        OSPEED3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn ospeed4(&self) -> OSPEED4_R {
        OSPEED4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn ospeed5(&self) -> OSPEED5_R {
        OSPEED5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn ospeed6(&self) -> OSPEED6_R {
        OSPEED6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn ospeed7(&self) -> OSPEED7_R {
        OSPEED7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn ospeed8(&self) -> OSPEED8_R {
        OSPEED8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn ospeed9(&self) -> OSPEED9_R {
        OSPEED9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn ospeed10(&self) -> OSPEED10_R {
        OSPEED10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn ospeed11(&self) -> OSPEED11_R {
        OSPEED11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn ospeed12(&self) -> OSPEED12_R {
        OSPEED12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn ospeed13(&self) -> OSPEED13_R {
        OSPEED13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn ospeed14(&self) -> OSPEED14_R {
        OSPEED14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn ospeed15(&self) -> OSPEED15_R {
        OSPEED15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn ospeed0(&mut self) -> OSPEED0_W<OSPEEDRrs> {
        OSPEED0_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn ospeed1(&mut self) -> OSPEED1_W<OSPEEDRrs> {
        OSPEED1_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn ospeed2(&mut self) -> OSPEED2_W<OSPEEDRrs> {
        OSPEED2_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn ospeed3(&mut self) -> OSPEED3_W<OSPEEDRrs> {
        OSPEED3_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn ospeed4(&mut self) -> OSPEED4_W<OSPEEDRrs> {
        OSPEED4_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn ospeed5(&mut self) -> OSPEED5_W<OSPEEDRrs> {
        OSPEED5_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn ospeed6(&mut self) -> OSPEED6_W<OSPEEDRrs> {
        OSPEED6_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn ospeed7(&mut self) -> OSPEED7_W<OSPEEDRrs> {
        OSPEED7_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn ospeed8(&mut self) -> OSPEED8_W<OSPEEDRrs> {
        OSPEED8_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn ospeed9(&mut self) -> OSPEED9_W<OSPEEDRrs> {
        OSPEED9_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn ospeed10(&mut self) -> OSPEED10_W<OSPEEDRrs> {
        OSPEED10_W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn ospeed11(&mut self) -> OSPEED11_W<OSPEEDRrs> {
        OSPEED11_W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn ospeed12(&mut self) -> OSPEED12_W<OSPEEDRrs> {
        OSPEED12_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn ospeed13(&mut self) -> OSPEED13_W<OSPEEDRrs> {
        OSPEED13_W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn ospeed14(&mut self) -> OSPEED14_W<OSPEEDRrs> {
        OSPEED14_W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn ospeed15(&mut self) -> OSPEED15_W<OSPEEDRrs> {
        OSPEED15_W::new(self, 30)
    }
}
#[doc = "GPIO port output speed register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospeedr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospeedr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSPEEDRrs;
impl crate::RegisterSpec for OSPEEDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ospeedr::R`](R) reader structure"]
impl crate::Readable for OSPEEDRrs {}
#[doc = "`write(|w| ..)` method takes [`ospeedr::W`](W) writer structure"]
impl crate::Writable for OSPEEDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSPEEDR to value 0x0c00_0000"]
impl crate::Resettable for OSPEEDRrs {
    const RESET_VALUE: u32 = 0x0c00_0000;
}
