#[doc = "Register `OSPEEDR` reader"]
pub type R = crate::R<OSPEEDRrs>;
#[doc = "Register `OSPEEDR` writer"]
pub type W = crate::W<OSPEEDRrs>;
#[doc = "Port x configuration bits (y = 0..15)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSPEEDR3 {
    #[doc = "0: Low speed"]
    LowSpeed = 0,
    #[doc = "1: Medium speed"]
    MediumSpeed = 1,
    #[doc = "2: High speed"]
    HighSpeed = 2,
    #[doc = "3: Very high speed"]
    VeryHighSpeed = 3,
}
impl From<OSPEEDR3> for u8 {
    #[inline(always)]
    fn from(variant: OSPEEDR3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OSPEEDR3 {
    type Ux = u8;
}
#[doc = "Field `OSPEEDR3` reader - Port x configuration bits (y = 0..15)"]
pub type OSPEEDR3_R = crate::FieldReader<OSPEEDR3>;
impl OSPEEDR3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OSPEEDR3 {
        match self.bits {
            0 => OSPEEDR3::LowSpeed,
            1 => OSPEEDR3::MediumSpeed,
            2 => OSPEEDR3::HighSpeed,
            3 => OSPEEDR3::VeryHighSpeed,
            _ => unreachable!(),
        }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn is_low_speed(&self) -> bool {
        *self == OSPEEDR3::LowSpeed
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn is_medium_speed(&self) -> bool {
        *self == OSPEEDR3::MediumSpeed
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        *self == OSPEEDR3::HighSpeed
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn is_very_high_speed(&self) -> bool {
        *self == OSPEEDR3::VeryHighSpeed
    }
}
#[doc = "Field `OSPEEDR3` writer - Port x configuration bits (y = 0..15)"]
pub type OSPEEDR3_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, OSPEEDR3>;
impl<'a, REG> OSPEEDR3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEEDR3::LowSpeed)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEEDR3::MediumSpeed)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEEDR3::HighSpeed)
    }
    #[doc = "Very high speed"]
    #[inline(always)]
    pub fn very_high_speed(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEEDR3::VeryHighSpeed)
    }
}
impl R {
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr3(&self) -> OSPEEDR3_R {
        OSPEEDR3_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ospeedr3(&mut self) -> OSPEEDR3_W<OSPEEDRrs> {
        OSPEEDR3_W::new(self, 6)
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
#[doc = "`reset()` method sets OSPEEDR to value 0"]
impl crate::Resettable for OSPEEDRrs {
    const RESET_VALUE: u32 = 0;
}
