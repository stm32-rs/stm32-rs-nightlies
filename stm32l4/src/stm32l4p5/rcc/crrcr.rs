#[doc = "Register `CRRCR` reader"]
pub type R = crate::R<CRRCRrs>;
#[doc = "Register `CRRCR` writer"]
pub type W = crate::W<CRRCRrs>;
#[doc = "HSI48 clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI48ON {
    #[doc = "0: HSI48 oscillator OFF"]
    Disabled = 0,
    #[doc = "1: HSI48 oscillator ON"]
    Enabled = 1,
}
impl From<HSI48ON> for bool {
    #[inline(always)]
    fn from(variant: HSI48ON) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSI48ON` reader - HSI48 clock enable"]
pub type HSI48ON_R = crate::BitReader<HSI48ON>;
impl HSI48ON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSI48ON {
        match self.bits {
            false => HSI48ON::Disabled,
            true => HSI48ON::Enabled,
        }
    }
    #[doc = "HSI48 oscillator OFF"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HSI48ON::Disabled
    }
    #[doc = "HSI48 oscillator ON"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HSI48ON::Enabled
    }
}
#[doc = "Field `HSI48ON` writer - HSI48 clock enable"]
pub type HSI48ON_W<'a, REG> = crate::BitWriter<'a, REG, HSI48ON>;
impl<'a, REG> HSI48ON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSI48 oscillator OFF"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSI48ON::Disabled)
    }
    #[doc = "HSI48 oscillator ON"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSI48ON::Enabled)
    }
}
#[doc = "HSI48 clock ready flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI48RDY {
    #[doc = "0: HSI48 oscillator not ready"]
    NotReady = 0,
    #[doc = "1: HSI48 oscillator ready"]
    Ready = 1,
}
impl From<HSI48RDY> for bool {
    #[inline(always)]
    fn from(variant: HSI48RDY) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSI48RDY` reader - HSI48 clock ready flag"]
pub type HSI48RDY_R = crate::BitReader<HSI48RDY>;
impl HSI48RDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSI48RDY {
        match self.bits {
            false => HSI48RDY::NotReady,
            true => HSI48RDY::Ready,
        }
    }
    #[doc = "HSI48 oscillator not ready"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == HSI48RDY::NotReady
    }
    #[doc = "HSI48 oscillator ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == HSI48RDY::Ready
    }
}
#[doc = "Field `HSI48CAL` reader - HSI48 clock calibration"]
pub type HSI48CAL_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - HSI48 clock enable"]
    #[inline(always)]
    pub fn hsi48on(&self) -> HSI48ON_R {
        HSI48ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HSI48 clock ready flag"]
    #[inline(always)]
    pub fn hsi48rdy(&self) -> HSI48RDY_R {
        HSI48RDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 7:15 - HSI48 clock calibration"]
    #[inline(always)]
    pub fn hsi48cal(&self) -> HSI48CAL_R {
        HSI48CAL_R::new(((self.bits >> 7) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - HSI48 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsi48on(&mut self) -> HSI48ON_W<CRRCRrs> {
        HSI48ON_W::new(self, 0)
    }
}
#[doc = "Clock recovery RC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crrcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crrcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRRCRrs;
impl crate::RegisterSpec for CRRCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crrcr::R`](R) reader structure"]
impl crate::Readable for CRRCRrs {}
#[doc = "`write(|w| ..)` method takes [`crrcr::W`](W) writer structure"]
impl crate::Writable for CRRCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRRCR to value 0"]
impl crate::Resettable for CRRCRrs {
    const RESET_VALUE: u32 = 0;
}
