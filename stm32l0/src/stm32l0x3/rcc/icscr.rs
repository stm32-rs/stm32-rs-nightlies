#[doc = "Reader of register ICSCR"]
pub type R = crate::R<u32, super::ICSCR>;
#[doc = "Writer for register ICSCR"]
pub type W = crate::W<u32, super::ICSCR>;
#[doc = "Register ICSCR `reset()`'s with value 0xb000"]
impl crate::ResetValue for super::ICSCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xb000
    }
}
#[doc = "Reader of field `MSITRIM`"]
pub type MSITRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MSITRIM`"]
pub struct MSITRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> MSITRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `MSICAL`"]
pub type MSICAL_R = crate::R<u8, u8>;
#[doc = "MSI clock ranges\n\nValue on reset: 5"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MSIRANGE_A {
    #[doc = "0: range 0 around 65.536 kHz"]
    RANGE0 = 0,
    #[doc = "1: range 1 around 131.072 kHz"]
    RANGE1 = 1,
    #[doc = "2: range 2 around 262.144 kHz"]
    RANGE2 = 2,
    #[doc = "3: range 3 around 524.288 kHz"]
    RANGE3 = 3,
    #[doc = "4: range 4 around 1.048 MHz"]
    RANGE4 = 4,
    #[doc = "5: range 5 around 2.097 MHz (reset value)"]
    RANGE5 = 5,
    #[doc = "6: range 6 around 4.194 MHz"]
    RANGE6 = 6,
    #[doc = "7: not allowed"]
    RANGE7 = 7,
}
impl From<MSIRANGE_A> for u8 {
    #[inline(always)]
    fn from(variant: MSIRANGE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MSIRANGE`"]
pub type MSIRANGE_R = crate::R<u8, MSIRANGE_A>;
impl MSIRANGE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSIRANGE_A {
        match self.bits {
            0 => MSIRANGE_A::RANGE0,
            1 => MSIRANGE_A::RANGE1,
            2 => MSIRANGE_A::RANGE2,
            3 => MSIRANGE_A::RANGE3,
            4 => MSIRANGE_A::RANGE4,
            5 => MSIRANGE_A::RANGE5,
            6 => MSIRANGE_A::RANGE6,
            7 => MSIRANGE_A::RANGE7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RANGE0`"]
    #[inline(always)]
    pub fn is_range0(&self) -> bool {
        *self == MSIRANGE_A::RANGE0
    }
    #[doc = "Checks if the value of the field is `RANGE1`"]
    #[inline(always)]
    pub fn is_range1(&self) -> bool {
        *self == MSIRANGE_A::RANGE1
    }
    #[doc = "Checks if the value of the field is `RANGE2`"]
    #[inline(always)]
    pub fn is_range2(&self) -> bool {
        *self == MSIRANGE_A::RANGE2
    }
    #[doc = "Checks if the value of the field is `RANGE3`"]
    #[inline(always)]
    pub fn is_range3(&self) -> bool {
        *self == MSIRANGE_A::RANGE3
    }
    #[doc = "Checks if the value of the field is `RANGE4`"]
    #[inline(always)]
    pub fn is_range4(&self) -> bool {
        *self == MSIRANGE_A::RANGE4
    }
    #[doc = "Checks if the value of the field is `RANGE5`"]
    #[inline(always)]
    pub fn is_range5(&self) -> bool {
        *self == MSIRANGE_A::RANGE5
    }
    #[doc = "Checks if the value of the field is `RANGE6`"]
    #[inline(always)]
    pub fn is_range6(&self) -> bool {
        *self == MSIRANGE_A::RANGE6
    }
    #[doc = "Checks if the value of the field is `RANGE7`"]
    #[inline(always)]
    pub fn is_range7(&self) -> bool {
        *self == MSIRANGE_A::RANGE7
    }
}
#[doc = "Write proxy for field `MSIRANGE`"]
pub struct MSIRANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> MSIRANGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSIRANGE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "range 0 around 65.536 kHz"]
    #[inline(always)]
    pub fn range0(self) -> &'a mut W {
        self.variant(MSIRANGE_A::RANGE0)
    }
    #[doc = "range 1 around 131.072 kHz"]
    #[inline(always)]
    pub fn range1(self) -> &'a mut W {
        self.variant(MSIRANGE_A::RANGE1)
    }
    #[doc = "range 2 around 262.144 kHz"]
    #[inline(always)]
    pub fn range2(self) -> &'a mut W {
        self.variant(MSIRANGE_A::RANGE2)
    }
    #[doc = "range 3 around 524.288 kHz"]
    #[inline(always)]
    pub fn range3(self) -> &'a mut W {
        self.variant(MSIRANGE_A::RANGE3)
    }
    #[doc = "range 4 around 1.048 MHz"]
    #[inline(always)]
    pub fn range4(self) -> &'a mut W {
        self.variant(MSIRANGE_A::RANGE4)
    }
    #[doc = "range 5 around 2.097 MHz (reset value)"]
    #[inline(always)]
    pub fn range5(self) -> &'a mut W {
        self.variant(MSIRANGE_A::RANGE5)
    }
    #[doc = "range 6 around 4.194 MHz"]
    #[inline(always)]
    pub fn range6(self) -> &'a mut W {
        self.variant(MSIRANGE_A::RANGE6)
    }
    #[doc = "not allowed"]
    #[inline(always)]
    pub fn range7(self) -> &'a mut W {
        self.variant(MSIRANGE_A::RANGE7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | (((value as u32) & 0x07) << 13);
        self.w
    }
}
#[doc = "Reader of field `HSI16TRIM`"]
pub type HSI16TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HSI16TRIM`"]
pub struct HSI16TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> HSI16TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `HSI16CAL`"]
pub type HSI16CAL_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 24:31 - MSI clock trimming"]
    #[inline(always)]
    pub fn msitrim(&self) -> MSITRIM_R {
        MSITRIM_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - MSI clock calibration"]
    #[inline(always)]
    pub fn msical(&self) -> MSICAL_R {
        MSICAL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 13:15 - MSI clock ranges"]
    #[inline(always)]
    pub fn msirange(&self) -> MSIRANGE_R {
        MSIRANGE_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bits 8:12 - High speed internal clock trimming"]
    #[inline(always)]
    pub fn hsi16trim(&self) -> HSI16TRIM_R {
        HSI16TRIM_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 0:7 - nternal high speed clock calibration"]
    #[inline(always)]
    pub fn hsi16cal(&self) -> HSI16CAL_R {
        HSI16CAL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - MSI clock trimming"]
    #[inline(always)]
    pub fn msitrim(&mut self) -> MSITRIM_W {
        MSITRIM_W { w: self }
    }
    #[doc = "Bits 13:15 - MSI clock ranges"]
    #[inline(always)]
    pub fn msirange(&mut self) -> MSIRANGE_W {
        MSIRANGE_W { w: self }
    }
    #[doc = "Bits 8:12 - High speed internal clock trimming"]
    #[inline(always)]
    pub fn hsi16trim(&mut self) -> HSI16TRIM_W {
        HSI16TRIM_W { w: self }
    }
}
