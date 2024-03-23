#[doc = "Register `ICSCR` reader"]
pub type R = crate::R<ICSCRrs>;
#[doc = "Register `ICSCR` writer"]
pub type W = crate::W<ICSCRrs>;
#[doc = "Field `HSI16CAL` reader - nternal high speed clock calibration"]
pub type HSI16CAL_R = crate::FieldReader;
#[doc = "Field `HSI16TRIM` reader - High speed internal clock trimming"]
pub type HSI16TRIM_R = crate::FieldReader;
#[doc = "Field `HSI16TRIM` writer - High speed internal clock trimming"]
pub type HSI16TRIM_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
#[doc = "MSI clock ranges\n\nValue on reset: 5"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MSIRANGE {
    #[doc = "0: range 0 around 65.536 kHz"]
    Range0 = 0,
    #[doc = "1: range 1 around 131.072 kHz"]
    Range1 = 1,
    #[doc = "2: range 2 around 262.144 kHz"]
    Range2 = 2,
    #[doc = "3: range 3 around 524.288 kHz"]
    Range3 = 3,
    #[doc = "4: range 4 around 1.048 MHz"]
    Range4 = 4,
    #[doc = "5: range 5 around 2.097 MHz (reset value)"]
    Range5 = 5,
    #[doc = "6: range 6 around 4.194 MHz"]
    Range6 = 6,
    #[doc = "7: not allowed"]
    Range7 = 7,
}
impl From<MSIRANGE> for u8 {
    #[inline(always)]
    fn from(variant: MSIRANGE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MSIRANGE {
    type Ux = u8;
}
#[doc = "Field `MSIRANGE` reader - MSI clock ranges"]
pub type MSIRANGE_R = crate::FieldReader<MSIRANGE>;
impl MSIRANGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSIRANGE {
        match self.bits {
            0 => MSIRANGE::Range0,
            1 => MSIRANGE::Range1,
            2 => MSIRANGE::Range2,
            3 => MSIRANGE::Range3,
            4 => MSIRANGE::Range4,
            5 => MSIRANGE::Range5,
            6 => MSIRANGE::Range6,
            7 => MSIRANGE::Range7,
            _ => unreachable!(),
        }
    }
    #[doc = "range 0 around 65.536 kHz"]
    #[inline(always)]
    pub fn is_range0(&self) -> bool {
        *self == MSIRANGE::Range0
    }
    #[doc = "range 1 around 131.072 kHz"]
    #[inline(always)]
    pub fn is_range1(&self) -> bool {
        *self == MSIRANGE::Range1
    }
    #[doc = "range 2 around 262.144 kHz"]
    #[inline(always)]
    pub fn is_range2(&self) -> bool {
        *self == MSIRANGE::Range2
    }
    #[doc = "range 3 around 524.288 kHz"]
    #[inline(always)]
    pub fn is_range3(&self) -> bool {
        *self == MSIRANGE::Range3
    }
    #[doc = "range 4 around 1.048 MHz"]
    #[inline(always)]
    pub fn is_range4(&self) -> bool {
        *self == MSIRANGE::Range4
    }
    #[doc = "range 5 around 2.097 MHz (reset value)"]
    #[inline(always)]
    pub fn is_range5(&self) -> bool {
        *self == MSIRANGE::Range5
    }
    #[doc = "range 6 around 4.194 MHz"]
    #[inline(always)]
    pub fn is_range6(&self) -> bool {
        *self == MSIRANGE::Range6
    }
    #[doc = "not allowed"]
    #[inline(always)]
    pub fn is_range7(&self) -> bool {
        *self == MSIRANGE::Range7
    }
}
#[doc = "Field `MSIRANGE` writer - MSI clock ranges"]
pub type MSIRANGE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, MSIRANGE>;
impl<'a, REG> MSIRANGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "range 0 around 65.536 kHz"]
    #[inline(always)]
    pub fn range0(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRANGE::Range0)
    }
    #[doc = "range 1 around 131.072 kHz"]
    #[inline(always)]
    pub fn range1(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRANGE::Range1)
    }
    #[doc = "range 2 around 262.144 kHz"]
    #[inline(always)]
    pub fn range2(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRANGE::Range2)
    }
    #[doc = "range 3 around 524.288 kHz"]
    #[inline(always)]
    pub fn range3(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRANGE::Range3)
    }
    #[doc = "range 4 around 1.048 MHz"]
    #[inline(always)]
    pub fn range4(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRANGE::Range4)
    }
    #[doc = "range 5 around 2.097 MHz (reset value)"]
    #[inline(always)]
    pub fn range5(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRANGE::Range5)
    }
    #[doc = "range 6 around 4.194 MHz"]
    #[inline(always)]
    pub fn range6(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRANGE::Range6)
    }
    #[doc = "not allowed"]
    #[inline(always)]
    pub fn range7(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRANGE::Range7)
    }
}
#[doc = "Field `MSICAL` reader - MSI clock calibration"]
pub type MSICAL_R = crate::FieldReader;
#[doc = "Field `MSITRIM` reader - MSI clock trimming"]
pub type MSITRIM_R = crate::FieldReader;
#[doc = "Field `MSITRIM` writer - MSI clock trimming"]
pub type MSITRIM_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - nternal high speed clock calibration"]
    #[inline(always)]
    pub fn hsi16cal(&self) -> HSI16CAL_R {
        HSI16CAL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:12 - High speed internal clock trimming"]
    #[inline(always)]
    pub fn hsi16trim(&self) -> HSI16TRIM_R {
        HSI16TRIM_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15 - MSI clock ranges"]
    #[inline(always)]
    pub fn msirange(&self) -> MSIRANGE_R {
        MSIRANGE_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:23 - MSI clock calibration"]
    #[inline(always)]
    pub fn msical(&self) -> MSICAL_R {
        MSICAL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - MSI clock trimming"]
    #[inline(always)]
    pub fn msitrim(&self) -> MSITRIM_R {
        MSITRIM_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:12 - High speed internal clock trimming"]
    #[inline(always)]
    #[must_use]
    pub fn hsi16trim(&mut self) -> HSI16TRIM_W<ICSCRrs> {
        HSI16TRIM_W::new(self, 8)
    }
    #[doc = "Bits 13:15 - MSI clock ranges"]
    #[inline(always)]
    #[must_use]
    pub fn msirange(&mut self) -> MSIRANGE_W<ICSCRrs> {
        MSIRANGE_W::new(self, 13)
    }
    #[doc = "Bits 24:31 - MSI clock trimming"]
    #[inline(always)]
    #[must_use]
    pub fn msitrim(&mut self) -> MSITRIM_W<ICSCRrs> {
        MSITRIM_W::new(self, 24)
    }
}
#[doc = "Internal clock sources calibration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICSCRrs;
impl crate::RegisterSpec for ICSCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icscr::R`](R) reader structure"]
impl crate::Readable for ICSCRrs {}
#[doc = "`write(|w| ..)` method takes [`icscr::W`](W) writer structure"]
impl crate::Writable for ICSCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICSCR to value 0xb000"]
impl crate::Resettable for ICSCRrs {
    const RESET_VALUE: u32 = 0xb000;
}
