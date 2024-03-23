#[doc = "Register `ODR` reader"]
pub type R = crate::R<ODRrs>;
#[doc = "Register `ODR` writer"]
pub type W = crate::W<ODRrs>;
#[doc = "Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OD0 {
    #[doc = "0: Set output to logic low"]
    Low = 0,
    #[doc = "1: Set output to logic high"]
    High = 1,
}
impl From<OD0> for bool {
    #[inline(always)]
    fn from(variant: OD0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OD0` reader - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OD0_R = crate::BitReader<OD0>;
impl OD0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OD0 {
        match self.bits {
            false => OD0::Low,
            true => OD0::High,
        }
    }
    #[doc = "Set output to logic low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == OD0::Low
    }
    #[doc = "Set output to logic high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == OD0::High
    }
}
#[doc = "Field `OD0` writer - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OD0_W<'a, REG> = crate::BitWriter<'a, REG, OD0>;
impl<'a, REG> OD0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set output to logic low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(OD0::Low)
    }
    #[doc = "Set output to logic high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(OD0::High)
    }
}
#[doc = "Field `OD1` reader - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OD0_R as OD1_R;
#[doc = "Field `OD2` reader - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OD0_R as OD2_R;
#[doc = "Field `OD3` reader - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OD0_R as OD3_R;
#[doc = "Field `OD4` reader - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OD0_R as OD4_R;
#[doc = "Field `OD5` reader - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OD0_R as OD5_R;
#[doc = "Field `OD6` reader - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OD0_R as OD6_R;
#[doc = "Field `OD7` reader - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OD0_R as OD7_R;
#[doc = "Field `OD8` reader - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OD0_R as OD8_R;
#[doc = "Field `OD9` reader - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OD0_R as OD9_R;
#[doc = "Field `OD10` reader - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OD0_R as OD10_R;
#[doc = "Field `OD11` reader - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OD0_R as OD11_R;
#[doc = "Field `OD12` reader - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OD0_R as OD12_R;
#[doc = "Field `OD13` reader - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OD0_R as OD13_R;
#[doc = "Field `OD14` reader - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OD0_R as OD14_R;
#[doc = "Field `OD15` reader - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OD0_R as OD15_R;
#[doc = "Field `OD1` writer - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OD0_W as OD1_W;
#[doc = "Field `OD2` writer - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OD0_W as OD2_W;
#[doc = "Field `OD3` writer - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OD0_W as OD3_W;
#[doc = "Field `OD4` writer - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OD0_W as OD4_W;
#[doc = "Field `OD5` writer - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OD0_W as OD5_W;
#[doc = "Field `OD6` writer - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OD0_W as OD6_W;
#[doc = "Field `OD7` writer - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OD0_W as OD7_W;
#[doc = "Field `OD8` writer - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OD0_W as OD8_W;
#[doc = "Field `OD9` writer - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OD0_W as OD9_W;
#[doc = "Field `OD10` writer - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OD0_W as OD10_W;
#[doc = "Field `OD11` writer - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OD0_W as OD11_W;
#[doc = "Field `OD12` writer - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OD0_W as OD12_W;
#[doc = "Field `OD13` writer - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OD0_W as OD13_W;
#[doc = "Field `OD14` writer - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OD0_W as OD14_W;
#[doc = "Field `OD15` writer - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub use OD0_W as OD15_W;
impl R {
    #[doc = "Bit 0 - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn od0(&self) -> OD0_R {
        OD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn od1(&self) -> OD1_R {
        OD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn od2(&self) -> OD2_R {
        OD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn od3(&self) -> OD3_R {
        OD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn od4(&self) -> OD4_R {
        OD4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn od5(&self) -> OD5_R {
        OD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn od6(&self) -> OD6_R {
        OD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn od7(&self) -> OD7_R {
        OD7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn od8(&self) -> OD8_R {
        OD8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn od9(&self) -> OD9_R {
        OD9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn od10(&self) -> OD10_R {
        OD10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn od11(&self) -> OD11_R {
        OD11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn od12(&self) -> OD12_R {
        OD12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn od13(&self) -> OD13_R {
        OD13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn od14(&self) -> OD14_R {
        OD14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn od15(&self) -> OD15_R {
        OD15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn od0(&mut self) -> OD0_W<ODRrs> {
        OD0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn od1(&mut self) -> OD1_W<ODRrs> {
        OD1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn od2(&mut self) -> OD2_W<ODRrs> {
        OD2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn od3(&mut self) -> OD3_W<ODRrs> {
        OD3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn od4(&mut self) -> OD4_W<ODRrs> {
        OD4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn od5(&mut self) -> OD5_W<ODRrs> {
        OD5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn od6(&mut self) -> OD6_W<ODRrs> {
        OD6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn od7(&mut self) -> OD7_W<ODRrs> {
        OD7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn od8(&mut self) -> OD8_W<ODRrs> {
        OD8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn od9(&mut self) -> OD9_W<ODRrs> {
        OD9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn od10(&mut self) -> OD10_W<ODRrs> {
        OD10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn od11(&mut self) -> OD11_W<ODRrs> {
        OD11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn od12(&mut self) -> OD12_W<ODRrs> {
        OD12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn od13(&mut self) -> OD13_W<ODRrs> {
        OD13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn od14(&mut self) -> OD14_W<ODRrs> {
        OD14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Port output data I/O pin y (y = 15 to 0) These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A to D and H). The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn od15(&mut self) -> OD15_W<ODRrs> {
        OD15_W::new(self, 15)
    }
}
#[doc = "GPIO port output data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`odr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`odr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ODRrs;
impl crate::RegisterSpec for ODRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`odr::R`](R) reader structure"]
impl crate::Readable for ODRrs {}
#[doc = "`write(|w| ..)` method takes [`odr::W`](W) writer structure"]
impl crate::Writable for ODRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ODR to value 0"]
impl crate::Resettable for ODRrs {
    const RESET_VALUE: u32 = 0;
}
