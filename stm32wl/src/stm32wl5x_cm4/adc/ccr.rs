#[doc = "Register `CCR` reader"]
pub type R = crate::R<CCRrs>;
#[doc = "Register `CCR` writer"]
pub type W = crate::W<CCRrs>;
#[doc = "PRESC0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESC {
    #[doc = "0: Input ADC clock not divided"]
    Div1 = 0,
    #[doc = "1: Input ADC clock divided by 2"]
    Div2 = 1,
    #[doc = "2: Input ADC clock divided by 4"]
    Div4 = 2,
    #[doc = "3: Input ADC clock divided by 6"]
    Div6 = 3,
    #[doc = "4: Input ADC clock divided by 8"]
    Div8 = 4,
    #[doc = "5: Input ADC clock divided by 10"]
    Div10 = 5,
    #[doc = "6: Input ADC clock divided by 12"]
    Div12 = 6,
    #[doc = "7: Input ADC clock divided by 16"]
    Div16 = 7,
    #[doc = "8: Input ADC clock divided by 32"]
    Div32 = 8,
    #[doc = "9: Input ADC clock divided by 64"]
    Div64 = 9,
    #[doc = "10: Input ADC clock divided by 128"]
    Div128 = 10,
    #[doc = "11: Input ADC clock divided by 256"]
    Div256 = 11,
}
impl From<PRESC> for u8 {
    #[inline(always)]
    fn from(variant: PRESC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRESC {
    type Ux = u8;
}
#[doc = "Field `PRESC` reader - PRESC0"]
pub type PRESC_R = crate::FieldReader<PRESC>;
impl PRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PRESC> {
        match self.bits {
            0 => Some(PRESC::Div1),
            1 => Some(PRESC::Div2),
            2 => Some(PRESC::Div4),
            3 => Some(PRESC::Div6),
            4 => Some(PRESC::Div8),
            5 => Some(PRESC::Div10),
            6 => Some(PRESC::Div12),
            7 => Some(PRESC::Div16),
            8 => Some(PRESC::Div32),
            9 => Some(PRESC::Div64),
            10 => Some(PRESC::Div128),
            11 => Some(PRESC::Div256),
            _ => None,
        }
    }
    #[doc = "Input ADC clock not divided"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PRESC::Div1
    }
    #[doc = "Input ADC clock divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PRESC::Div2
    }
    #[doc = "Input ADC clock divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PRESC::Div4
    }
    #[doc = "Input ADC clock divided by 6"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PRESC::Div6
    }
    #[doc = "Input ADC clock divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PRESC::Div8
    }
    #[doc = "Input ADC clock divided by 10"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PRESC::Div10
    }
    #[doc = "Input ADC clock divided by 12"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PRESC::Div12
    }
    #[doc = "Input ADC clock divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PRESC::Div16
    }
    #[doc = "Input ADC clock divided by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PRESC::Div32
    }
    #[doc = "Input ADC clock divided by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PRESC::Div64
    }
    #[doc = "Input ADC clock divided by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PRESC::Div128
    }
    #[doc = "Input ADC clock divided by 256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == PRESC::Div256
    }
}
#[doc = "Field `PRESC` writer - PRESC0"]
pub type PRESC_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PRESC>;
impl<'a, REG> PRESC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input ADC clock not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div1)
    }
    #[doc = "Input ADC clock divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div2)
    }
    #[doc = "Input ADC clock divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div4)
    }
    #[doc = "Input ADC clock divided by 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div6)
    }
    #[doc = "Input ADC clock divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div8)
    }
    #[doc = "Input ADC clock divided by 10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div10)
    }
    #[doc = "Input ADC clock divided by 12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div12)
    }
    #[doc = "Input ADC clock divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div16)
    }
    #[doc = "Input ADC clock divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div32)
    }
    #[doc = "Input ADC clock divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div64)
    }
    #[doc = "Input ADC clock divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div128)
    }
    #[doc = "Input ADC clock divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div256)
    }
}
#[doc = "VREFEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VREFEN {
    #[doc = "0: VREFINT disabled"]
    Disabled = 0,
    #[doc = "1: VREFINT enabled"]
    Enabled = 1,
}
impl From<VREFEN> for bool {
    #[inline(always)]
    fn from(variant: VREFEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VREFEN` reader - VREFEN"]
pub type VREFEN_R = crate::BitReader<VREFEN>;
impl VREFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VREFEN {
        match self.bits {
            false => VREFEN::Disabled,
            true => VREFEN::Enabled,
        }
    }
    #[doc = "VREFINT disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VREFEN::Disabled
    }
    #[doc = "VREFINT enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VREFEN::Enabled
    }
}
#[doc = "Field `VREFEN` writer - VREFEN"]
pub type VREFEN_W<'a, REG> = crate::BitWriter<'a, REG, VREFEN>;
impl<'a, REG> VREFEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VREFINT disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(VREFEN::Disabled)
    }
    #[doc = "VREFINT enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(VREFEN::Enabled)
    }
}
#[doc = "TSEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSEN {
    #[doc = "0: Temperature sensor disabled"]
    Disabled = 0,
    #[doc = "1: Temperature sensor enabled"]
    Enabled = 1,
}
impl From<TSEN> for bool {
    #[inline(always)]
    fn from(variant: TSEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSEN` reader - TSEN"]
pub type TSEN_R = crate::BitReader<TSEN>;
impl TSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TSEN {
        match self.bits {
            false => TSEN::Disabled,
            true => TSEN::Enabled,
        }
    }
    #[doc = "Temperature sensor disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TSEN::Disabled
    }
    #[doc = "Temperature sensor enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TSEN::Enabled
    }
}
#[doc = "Field `TSEN` writer - TSEN"]
pub type TSEN_W<'a, REG> = crate::BitWriter<'a, REG, TSEN>;
impl<'a, REG> TSEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Temperature sensor disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TSEN::Disabled)
    }
    #[doc = "Temperature sensor enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TSEN::Enabled)
    }
}
#[doc = "VBATEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBATEN {
    #[doc = "0: VBAT channel disabled"]
    Disabled = 0,
    #[doc = "1: VBAT channel enabled"]
    Enabled = 1,
}
impl From<VBATEN> for bool {
    #[inline(always)]
    fn from(variant: VBATEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATEN` reader - VBATEN"]
pub type VBATEN_R = crate::BitReader<VBATEN>;
impl VBATEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VBATEN {
        match self.bits {
            false => VBATEN::Disabled,
            true => VBATEN::Enabled,
        }
    }
    #[doc = "VBAT channel disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VBATEN::Disabled
    }
    #[doc = "VBAT channel enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VBATEN::Enabled
    }
}
#[doc = "Field `VBATEN` writer - VBATEN"]
pub type VBATEN_W<'a, REG> = crate::BitWriter<'a, REG, VBATEN>;
impl<'a, REG> VBATEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBAT channel disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(VBATEN::Disabled)
    }
    #[doc = "VBAT channel enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(VBATEN::Enabled)
    }
}
impl R {
    #[doc = "Bits 18:21 - PRESC0"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - VREFEN"]
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - TSEN"]
    #[inline(always)]
    pub fn tsen(&self) -> TSEN_R {
        TSEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - VBATEN"]
    #[inline(always)]
    pub fn vbaten(&self) -> VBATEN_R {
        VBATEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 18:21 - PRESC0"]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<CCRrs> {
        PRESC_W::new(self, 18)
    }
    #[doc = "Bit 22 - VREFEN"]
    #[inline(always)]
    #[must_use]
    pub fn vrefen(&mut self) -> VREFEN_W<CCRrs> {
        VREFEN_W::new(self, 22)
    }
    #[doc = "Bit 23 - TSEN"]
    #[inline(always)]
    #[must_use]
    pub fn tsen(&mut self) -> TSEN_W<CCRrs> {
        TSEN_W::new(self, 23)
    }
    #[doc = "Bit 24 - VBATEN"]
    #[inline(always)]
    #[must_use]
    pub fn vbaten(&mut self) -> VBATEN_W<CCRrs> {
        VBATEN_W::new(self, 24)
    }
}
#[doc = "ADC common configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCRrs;
impl crate::RegisterSpec for CCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CCRrs {}
#[doc = "`write(|w| ..)` method takes [`ccr::W`](W) writer structure"]
impl crate::Writable for CCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CCRrs {
    const RESET_VALUE: u32 = 0;
}
