#[doc = "Register `DLLCR` reader"]
pub type R = crate::R<DLLCRrs>;
#[doc = "Register `DLLCR` writer"]
pub type W = crate::W<DLLCRrs>;
#[doc = "DLL Calibration Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAL {
    #[doc = "1: Calibration start"]
    Start = 1,
}
impl From<CAL> for bool {
    #[inline(always)]
    fn from(variant: CAL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAL` reader - DLL Calibration Start"]
pub type CAL_R = crate::BitReader<CAL>;
impl CAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CAL> {
        match self.bits {
            true => Some(CAL::Start),
            _ => None,
        }
    }
    #[doc = "Calibration start"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == CAL::Start
    }
}
#[doc = "Field `CAL` writer - DLL Calibration Start"]
pub type CAL_W<'a, REG> = crate::BitWriter<'a, REG, CAL>;
impl<'a, REG> CAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Calibration start"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(CAL::Start)
    }
}
#[doc = "DLL Calibration Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALEN {
    #[doc = "0: Periodic calibration disabled"]
    Disabled = 0,
    #[doc = "1: Calibration is performed periodically, as per CALRTE setting"]
    Enabled = 1,
}
impl From<CALEN> for bool {
    #[inline(always)]
    fn from(variant: CALEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALEN` reader - DLL Calibration Enable"]
pub type CALEN_R = crate::BitReader<CALEN>;
impl CALEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CALEN {
        match self.bits {
            false => CALEN::Disabled,
            true => CALEN::Enabled,
        }
    }
    #[doc = "Periodic calibration disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CALEN::Disabled
    }
    #[doc = "Calibration is performed periodically, as per CALRTE setting"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CALEN::Enabled
    }
}
#[doc = "Field `CALEN` writer - DLL Calibration Enable"]
pub type CALEN_W<'a, REG> = crate::BitWriter<'a, REG, CALEN>;
impl<'a, REG> CALEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Periodic calibration disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CALEN::Disabled)
    }
    #[doc = "Calibration is performed periodically, as per CALRTE setting"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CALEN::Enabled)
    }
}
#[doc = "DLL Calibration rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CALRTE {
    #[doc = "0: 1048576*t_HRTIM (7.3ms)"]
    Millis7_3 = 0,
    #[doc = "1: 131072*t_HRTIM (910µs)"]
    Micros910 = 1,
    #[doc = "2: 16384*t_HRTIM (114µs)"]
    Micros114 = 2,
    #[doc = "3: 2048*t_HRTIM (14µs)"]
    Micros14 = 3,
}
impl From<CALRTE> for u8 {
    #[inline(always)]
    fn from(variant: CALRTE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CALRTE {
    type Ux = u8;
}
#[doc = "Field `CALRTE` reader - DLL Calibration rate"]
pub type CALRTE_R = crate::FieldReader<CALRTE>;
impl CALRTE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CALRTE {
        match self.bits {
            0 => CALRTE::Millis7_3,
            1 => CALRTE::Micros910,
            2 => CALRTE::Micros114,
            3 => CALRTE::Micros14,
            _ => unreachable!(),
        }
    }
    #[doc = "1048576*t_HRTIM (7.3ms)"]
    #[inline(always)]
    pub fn is_millis7_3(&self) -> bool {
        *self == CALRTE::Millis7_3
    }
    #[doc = "131072*t_HRTIM (910µs)"]
    #[inline(always)]
    pub fn is_micros910(&self) -> bool {
        *self == CALRTE::Micros910
    }
    #[doc = "16384*t_HRTIM (114µs)"]
    #[inline(always)]
    pub fn is_micros114(&self) -> bool {
        *self == CALRTE::Micros114
    }
    #[doc = "2048*t_HRTIM (14µs)"]
    #[inline(always)]
    pub fn is_micros14(&self) -> bool {
        *self == CALRTE::Micros14
    }
}
#[doc = "Field `CALRTE` writer - DLL Calibration rate"]
pub type CALRTE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CALRTE>;
impl<'a, REG> CALRTE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1048576*t_HRTIM (7.3ms)"]
    #[inline(always)]
    pub fn millis7_3(self) -> &'a mut crate::W<REG> {
        self.variant(CALRTE::Millis7_3)
    }
    #[doc = "131072*t_HRTIM (910µs)"]
    #[inline(always)]
    pub fn micros910(self) -> &'a mut crate::W<REG> {
        self.variant(CALRTE::Micros910)
    }
    #[doc = "16384*t_HRTIM (114µs)"]
    #[inline(always)]
    pub fn micros114(self) -> &'a mut crate::W<REG> {
        self.variant(CALRTE::Micros114)
    }
    #[doc = "2048*t_HRTIM (14µs)"]
    #[inline(always)]
    pub fn micros14(self) -> &'a mut crate::W<REG> {
        self.variant(CALRTE::Micros14)
    }
}
impl R {
    #[doc = "Bit 0 - DLL Calibration Start"]
    #[inline(always)]
    pub fn cal(&self) -> CAL_R {
        CAL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DLL Calibration Enable"]
    #[inline(always)]
    pub fn calen(&self) -> CALEN_R {
        CALEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - DLL Calibration rate"]
    #[inline(always)]
    pub fn calrte(&self) -> CALRTE_R {
        CALRTE_R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DLL Calibration Start"]
    #[inline(always)]
    #[must_use]
    pub fn cal(&mut self) -> CAL_W<DLLCRrs> {
        CAL_W::new(self, 0)
    }
    #[doc = "Bit 1 - DLL Calibration Enable"]
    #[inline(always)]
    #[must_use]
    pub fn calen(&mut self) -> CALEN_W<DLLCRrs> {
        CALEN_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - DLL Calibration rate"]
    #[inline(always)]
    #[must_use]
    pub fn calrte(&mut self) -> CALRTE_W<DLLCRrs> {
        CALRTE_W::new(self, 2)
    }
}
#[doc = "DLL Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dllcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dllcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DLLCRrs;
impl crate::RegisterSpec for DLLCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dllcr::R`](R) reader structure"]
impl crate::Readable for DLLCRrs {}
#[doc = "`write(|w| ..)` method takes [`dllcr::W`](W) writer structure"]
impl crate::Writable for DLLCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DLLCR to value 0"]
impl crate::Resettable for DLLCRrs {
    const RESET_VALUE: u32 = 0;
}
