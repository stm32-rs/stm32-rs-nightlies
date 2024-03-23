#[doc = "Register `CR2` reader"]
pub type R = crate::R<CR2rs>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<CR2rs>;
#[doc = "HSI14 clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI14ON {
    #[doc = "0: HSI14 oscillator off"]
    Off = 0,
    #[doc = "1: HSI14 oscillator on"]
    On = 1,
}
impl From<HSI14ON> for bool {
    #[inline(always)]
    fn from(variant: HSI14ON) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSI14ON` reader - HSI14 clock enable"]
pub type HSI14ON_R = crate::BitReader<HSI14ON>;
impl HSI14ON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSI14ON {
        match self.bits {
            false => HSI14ON::Off,
            true => HSI14ON::On,
        }
    }
    #[doc = "HSI14 oscillator off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == HSI14ON::Off
    }
    #[doc = "HSI14 oscillator on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == HSI14ON::On
    }
}
#[doc = "Field `HSI14ON` writer - HSI14 clock enable"]
pub type HSI14ON_W<'a, REG> = crate::BitWriter<'a, REG, HSI14ON>;
impl<'a, REG> HSI14ON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSI14 oscillator off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(HSI14ON::Off)
    }
    #[doc = "HSI14 oscillator on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(HSI14ON::On)
    }
}
#[doc = "HR14 clock ready flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI14RDYR {
    #[doc = "0: HSI14 oscillator not ready"]
    NotReady = 0,
    #[doc = "1: HSI14 oscillator ready"]
    Ready = 1,
}
impl From<HSI14RDYR> for bool {
    #[inline(always)]
    fn from(variant: HSI14RDYR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSI14RDY` reader - HR14 clock ready flag"]
pub type HSI14RDY_R = crate::BitReader<HSI14RDYR>;
impl HSI14RDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSI14RDYR {
        match self.bits {
            false => HSI14RDYR::NotReady,
            true => HSI14RDYR::Ready,
        }
    }
    #[doc = "HSI14 oscillator not ready"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == HSI14RDYR::NotReady
    }
    #[doc = "HSI14 oscillator ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == HSI14RDYR::Ready
    }
}
#[doc = "HSI14 clock request from ADC disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI14DIS {
    #[doc = "0: ADC can turn on the HSI14 oscillator"]
    Allow = 0,
    #[doc = "1: ADC can not turn on the HSI14 oscillator"]
    Disallow = 1,
}
impl From<HSI14DIS> for bool {
    #[inline(always)]
    fn from(variant: HSI14DIS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSI14DIS` reader - HSI14 clock request from ADC disable"]
pub type HSI14DIS_R = crate::BitReader<HSI14DIS>;
impl HSI14DIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSI14DIS {
        match self.bits {
            false => HSI14DIS::Allow,
            true => HSI14DIS::Disallow,
        }
    }
    #[doc = "ADC can turn on the HSI14 oscillator"]
    #[inline(always)]
    pub fn is_allow(&self) -> bool {
        *self == HSI14DIS::Allow
    }
    #[doc = "ADC can not turn on the HSI14 oscillator"]
    #[inline(always)]
    pub fn is_disallow(&self) -> bool {
        *self == HSI14DIS::Disallow
    }
}
#[doc = "Field `HSI14DIS` writer - HSI14 clock request from ADC disable"]
pub type HSI14DIS_W<'a, REG> = crate::BitWriter<'a, REG, HSI14DIS>;
impl<'a, REG> HSI14DIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC can turn on the HSI14 oscillator"]
    #[inline(always)]
    pub fn allow(self) -> &'a mut crate::W<REG> {
        self.variant(HSI14DIS::Allow)
    }
    #[doc = "ADC can not turn on the HSI14 oscillator"]
    #[inline(always)]
    pub fn disallow(self) -> &'a mut crate::W<REG> {
        self.variant(HSI14DIS::Disallow)
    }
}
#[doc = "Field `HSI14TRIM` reader - HSI14 clock trimming"]
pub type HSI14TRIM_R = crate::FieldReader;
#[doc = "Field `HSI14TRIM` writer - HSI14 clock trimming"]
pub type HSI14TRIM_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
#[doc = "Field `HSI14CAL` reader - HSI14 clock calibration"]
pub type HSI14CAL_R = crate::FieldReader;
#[doc = "HSI48 clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI48ON {
    #[doc = "0: HSI48 oscillator off"]
    Off = 0,
    #[doc = "1: HSI48 oscillator on"]
    On = 1,
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
            false => HSI48ON::Off,
            true => HSI48ON::On,
        }
    }
    #[doc = "HSI48 oscillator off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == HSI48ON::Off
    }
    #[doc = "HSI48 oscillator on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == HSI48ON::On
    }
}
#[doc = "Field `HSI48ON` writer - HSI48 clock enable"]
pub type HSI48ON_W<'a, REG> = crate::BitWriter<'a, REG, HSI48ON>;
impl<'a, REG> HSI48ON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSI48 oscillator off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(HSI48ON::Off)
    }
    #[doc = "HSI48 oscillator on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(HSI48ON::On)
    }
}
#[doc = "HSI48 clock ready flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI48RDYR {
    #[doc = "0: HSI48 oscillator ready"]
    NotReady = 0,
    #[doc = "1: HSI48 oscillator ready"]
    Ready = 1,
}
impl From<HSI48RDYR> for bool {
    #[inline(always)]
    fn from(variant: HSI48RDYR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSI48RDY` reader - HSI48 clock ready flag"]
pub type HSI48RDY_R = crate::BitReader<HSI48RDYR>;
impl HSI48RDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSI48RDYR {
        match self.bits {
            false => HSI48RDYR::NotReady,
            true => HSI48RDYR::Ready,
        }
    }
    #[doc = "HSI48 oscillator ready"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == HSI48RDYR::NotReady
    }
    #[doc = "HSI48 oscillator ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == HSI48RDYR::Ready
    }
}
#[doc = "Field `HSI48CAL` reader - HSI48 factory clock calibration"]
pub type HSI48CAL_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - HSI14 clock enable"]
    #[inline(always)]
    pub fn hsi14on(&self) -> HSI14ON_R {
        HSI14ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HR14 clock ready flag"]
    #[inline(always)]
    pub fn hsi14rdy(&self) -> HSI14RDY_R {
        HSI14RDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSI14 clock request from ADC disable"]
    #[inline(always)]
    pub fn hsi14dis(&self) -> HSI14DIS_R {
        HSI14DIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:7 - HSI14 clock trimming"]
    #[inline(always)]
    pub fn hsi14trim(&self) -> HSI14TRIM_R {
        HSI14TRIM_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:15 - HSI14 clock calibration"]
    #[inline(always)]
    pub fn hsi14cal(&self) -> HSI14CAL_R {
        HSI14CAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - HSI48 clock enable"]
    #[inline(always)]
    pub fn hsi48on(&self) -> HSI48ON_R {
        HSI48ON_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HSI48 clock ready flag"]
    #[inline(always)]
    pub fn hsi48rdy(&self) -> HSI48RDY_R {
        HSI48RDY_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 24:31 - HSI48 factory clock calibration"]
    #[inline(always)]
    pub fn hsi48cal(&self) -> HSI48CAL_R {
        HSI48CAL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - HSI14 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsi14on(&mut self) -> HSI14ON_W<CR2rs> {
        HSI14ON_W::new(self, 0)
    }
    #[doc = "Bit 2 - HSI14 clock request from ADC disable"]
    #[inline(always)]
    #[must_use]
    pub fn hsi14dis(&mut self) -> HSI14DIS_W<CR2rs> {
        HSI14DIS_W::new(self, 2)
    }
    #[doc = "Bits 3:7 - HSI14 clock trimming"]
    #[inline(always)]
    #[must_use]
    pub fn hsi14trim(&mut self) -> HSI14TRIM_W<CR2rs> {
        HSI14TRIM_W::new(self, 3)
    }
    #[doc = "Bit 16 - HSI48 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsi48on(&mut self) -> HSI48ON_W<CR2rs> {
        HSI48ON_W::new(self, 16)
    }
}
#[doc = "Clock control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for CR2rs {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR2 to value 0x80"]
impl crate::Resettable for CR2rs {
    const RESET_VALUE: u32 = 0x80;
}
