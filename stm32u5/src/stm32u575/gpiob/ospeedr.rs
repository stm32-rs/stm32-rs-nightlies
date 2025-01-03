///Register `OSPEEDR` reader
pub type R = crate::R<OSPEEDRrs>;
///Register `OSPEEDR` writer
pub type W = crate::W<OSPEEDRrs>;
/**Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSPEED0 {
    ///0: Low speed
    LowSpeed = 0,
    ///1: Medium speed
    MediumSpeed = 1,
    ///2: High speed
    HighSpeed = 2,
    ///3: Very high speed
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
impl crate::IsEnum for OSPEED0 {}
///Field `OSPEED0` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub type OSPEED0_R = crate::FieldReader<OSPEED0>;
impl OSPEED0_R {
    ///Get enumerated values variant
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
    ///Low speed
    #[inline(always)]
    pub fn is_low_speed(&self) -> bool {
        *self == OSPEED0::LowSpeed
    }
    ///Medium speed
    #[inline(always)]
    pub fn is_medium_speed(&self) -> bool {
        *self == OSPEED0::MediumSpeed
    }
    ///High speed
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        *self == OSPEED0::HighSpeed
    }
    ///Very high speed
    #[inline(always)]
    pub fn is_very_high_speed(&self) -> bool {
        *self == OSPEED0::VeryHighSpeed
    }
}
///Field `OSPEED0` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub type OSPEED0_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OSPEED0, crate::Safe>;
impl<'a, REG> OSPEED0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Low speed
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED0::LowSpeed)
    }
    ///Medium speed
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED0::MediumSpeed)
    }
    ///High speed
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED0::HighSpeed)
    }
    ///Very high speed
    #[inline(always)]
    pub fn very_high_speed(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED0::VeryHighSpeed)
    }
}
///Field `OSPEED1` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use OSPEED0_R as OSPEED1_R;
///Field `OSPEED2` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use OSPEED0_R as OSPEED2_R;
///Field `OSPEED3` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use OSPEED0_R as OSPEED3_R;
///Field `OSPEED4` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use OSPEED0_R as OSPEED4_R;
///Field `OSPEED5` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use OSPEED0_R as OSPEED5_R;
///Field `OSPEED6` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use OSPEED0_R as OSPEED6_R;
///Field `OSPEED7` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use OSPEED0_R as OSPEED7_R;
///Field `OSPEED8` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use OSPEED0_R as OSPEED8_R;
///Field `OSPEED9` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use OSPEED0_R as OSPEED9_R;
///Field `OSPEED10` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use OSPEED0_R as OSPEED10_R;
///Field `OSPEED11` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use OSPEED0_R as OSPEED11_R;
///Field `OSPEED12` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use OSPEED0_R as OSPEED12_R;
///Field `OSPEED13` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use OSPEED0_R as OSPEED13_R;
///Field `OSPEED14` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use OSPEED0_R as OSPEED14_R;
///Field `OSPEED15` reader - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use OSPEED0_R as OSPEED15_R;
///Field `OSPEED1` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use OSPEED0_W as OSPEED1_W;
///Field `OSPEED2` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use OSPEED0_W as OSPEED2_W;
///Field `OSPEED3` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use OSPEED0_W as OSPEED3_W;
///Field `OSPEED4` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use OSPEED0_W as OSPEED4_W;
///Field `OSPEED5` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use OSPEED0_W as OSPEED5_W;
///Field `OSPEED6` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use OSPEED0_W as OSPEED6_W;
///Field `OSPEED7` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use OSPEED0_W as OSPEED7_W;
///Field `OSPEED8` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use OSPEED0_W as OSPEED8_W;
///Field `OSPEED9` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use OSPEED0_W as OSPEED9_W;
///Field `OSPEED10` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use OSPEED0_W as OSPEED10_W;
///Field `OSPEED11` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use OSPEED0_W as OSPEED11_W;
///Field `OSPEED12` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use OSPEED0_W as OSPEED12_W;
///Field `OSPEED13` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use OSPEED0_W as OSPEED13_W;
///Field `OSPEED14` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use OSPEED0_W as OSPEED14_W;
///Field `OSPEED15` writer - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
pub use OSPEED0_W as OSPEED15_W;
impl R {
    ///Bits 0:1 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn ospeed0(&self) -> OSPEED0_R {
        OSPEED0_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn ospeed1(&self) -> OSPEED1_R {
        OSPEED1_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn ospeed2(&self) -> OSPEED2_R {
        OSPEED2_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn ospeed3(&self) -> OSPEED3_R {
        OSPEED3_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn ospeed4(&self) -> OSPEED4_R {
        OSPEED4_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn ospeed5(&self) -> OSPEED5_R {
        OSPEED5_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn ospeed6(&self) -> OSPEED6_R {
        OSPEED6_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn ospeed7(&self) -> OSPEED7_R {
        OSPEED7_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:17 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn ospeed8(&self) -> OSPEED8_R {
        OSPEED8_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn ospeed9(&self) -> OSPEED9_R {
        OSPEED9_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn ospeed10(&self) -> OSPEED10_R {
        OSPEED10_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn ospeed11(&self) -> OSPEED11_R {
        OSPEED11_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:25 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn ospeed12(&self) -> OSPEED12_R {
        OSPEED12_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn ospeed13(&self) -> OSPEED13_R {
        OSPEED13_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:29 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn ospeed14(&self) -> OSPEED14_R {
        OSPEED14_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bits 30:31 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn ospeed15(&self) -> OSPEED15_R {
        OSPEED15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OSPEEDR")
            .field("ospeed0", &self.ospeed0())
            .field("ospeed1", &self.ospeed1())
            .field("ospeed2", &self.ospeed2())
            .field("ospeed3", &self.ospeed3())
            .field("ospeed4", &self.ospeed4())
            .field("ospeed5", &self.ospeed5())
            .field("ospeed6", &self.ospeed6())
            .field("ospeed7", &self.ospeed7())
            .field("ospeed8", &self.ospeed8())
            .field("ospeed9", &self.ospeed9())
            .field("ospeed10", &self.ospeed10())
            .field("ospeed11", &self.ospeed11())
            .field("ospeed12", &self.ospeed12())
            .field("ospeed13", &self.ospeed13())
            .field("ospeed14", &self.ospeed14())
            .field("ospeed15", &self.ospeed15())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn ospeed0(&mut self) -> OSPEED0_W<OSPEEDRrs> {
        OSPEED0_W::new(self, 0)
    }
    ///Bits 2:3 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn ospeed1(&mut self) -> OSPEED1_W<OSPEEDRrs> {
        OSPEED1_W::new(self, 2)
    }
    ///Bits 4:5 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn ospeed2(&mut self) -> OSPEED2_W<OSPEEDRrs> {
        OSPEED2_W::new(self, 4)
    }
    ///Bits 6:7 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn ospeed3(&mut self) -> OSPEED3_W<OSPEEDRrs> {
        OSPEED3_W::new(self, 6)
    }
    ///Bits 8:9 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn ospeed4(&mut self) -> OSPEED4_W<OSPEEDRrs> {
        OSPEED4_W::new(self, 8)
    }
    ///Bits 10:11 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn ospeed5(&mut self) -> OSPEED5_W<OSPEEDRrs> {
        OSPEED5_W::new(self, 10)
    }
    ///Bits 12:13 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn ospeed6(&mut self) -> OSPEED6_W<OSPEEDRrs> {
        OSPEED6_W::new(self, 12)
    }
    ///Bits 14:15 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn ospeed7(&mut self) -> OSPEED7_W<OSPEEDRrs> {
        OSPEED7_W::new(self, 14)
    }
    ///Bits 16:17 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn ospeed8(&mut self) -> OSPEED8_W<OSPEEDRrs> {
        OSPEED8_W::new(self, 16)
    }
    ///Bits 18:19 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn ospeed9(&mut self) -> OSPEED9_W<OSPEEDRrs> {
        OSPEED9_W::new(self, 18)
    }
    ///Bits 20:21 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn ospeed10(&mut self) -> OSPEED10_W<OSPEEDRrs> {
        OSPEED10_W::new(self, 20)
    }
    ///Bits 22:23 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn ospeed11(&mut self) -> OSPEED11_W<OSPEEDRrs> {
        OSPEED11_W::new(self, 22)
    }
    ///Bits 24:25 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn ospeed12(&mut self) -> OSPEED12_W<OSPEEDRrs> {
        OSPEED12_W::new(self, 24)
    }
    ///Bits 26:27 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn ospeed13(&mut self) -> OSPEED13_W<OSPEEDRrs> {
        OSPEED13_W::new(self, 26)
    }
    ///Bits 28:29 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn ospeed14(&mut self) -> OSPEED14_W<OSPEEDRrs> {
        OSPEED14_W::new(self, 28)
    }
    ///Bits 30:31 - Port x configuration I/O pin y These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications, and the power supply and load conditions for each speed. Note: This field is reserved and must be kept at reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    pub fn ospeed15(&mut self) -> OSPEED15_W<OSPEEDRrs> {
        OSPEED15_W::new(self, 30)
    }
}
/**GPIO port output speed register

You can [`read`](crate::Reg::read) this register and get [`ospeedr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ospeedr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GPIOB:OSPEEDR)*/
pub struct OSPEEDRrs;
impl crate::RegisterSpec for OSPEEDRrs {
    type Ux = u32;
}
///`read()` method returns [`ospeedr::R`](R) reader structure
impl crate::Readable for OSPEEDRrs {}
///`write(|w| ..)` method takes [`ospeedr::W`](W) writer structure
impl crate::Writable for OSPEEDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OSPEEDR to value 0xc0
impl crate::Resettable for OSPEEDRrs {
    const RESET_VALUE: u32 = 0xc0;
}
