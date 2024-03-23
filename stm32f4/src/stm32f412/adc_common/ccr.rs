#[doc = "Register `CCR` reader"]
pub type R = crate::R<CCRrs>;
#[doc = "Register `CCR` writer"]
pub type W = crate::W<CCRrs>;
#[doc = "ADC prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADCPRE {
    #[doc = "0: PCLK2 divided by 2"]
    Div2 = 0,
    #[doc = "1: PCLK2 divided by 4"]
    Div4 = 1,
    #[doc = "2: PCLK2 divided by 6"]
    Div6 = 2,
    #[doc = "3: PCLK2 divided by 8"]
    Div8 = 3,
}
impl From<ADCPRE> for u8 {
    #[inline(always)]
    fn from(variant: ADCPRE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADCPRE {
    type Ux = u8;
}
#[doc = "Field `ADCPRE` reader - ADC prescaler"]
pub type ADCPRE_R = crate::FieldReader<ADCPRE>;
impl ADCPRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADCPRE {
        match self.bits {
            0 => ADCPRE::Div2,
            1 => ADCPRE::Div4,
            2 => ADCPRE::Div6,
            3 => ADCPRE::Div8,
            _ => unreachable!(),
        }
    }
    #[doc = "PCLK2 divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == ADCPRE::Div2
    }
    #[doc = "PCLK2 divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == ADCPRE::Div4
    }
    #[doc = "PCLK2 divided by 6"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == ADCPRE::Div6
    }
    #[doc = "PCLK2 divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == ADCPRE::Div8
    }
}
#[doc = "Field `ADCPRE` writer - ADC prescaler"]
pub type ADCPRE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, ADCPRE>;
impl<'a, REG> ADCPRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLK2 divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(ADCPRE::Div2)
    }
    #[doc = "PCLK2 divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(ADCPRE::Div4)
    }
    #[doc = "PCLK2 divided by 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(ADCPRE::Div6)
    }
    #[doc = "PCLK2 divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(ADCPRE::Div8)
    }
}
#[doc = "VBAT enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBATE {
    #[doc = "0: V_BAT channel disabled"]
    Disabled = 0,
    #[doc = "1: V_BAT channel enabled"]
    Enabled = 1,
}
impl From<VBATE> for bool {
    #[inline(always)]
    fn from(variant: VBATE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATE` reader - VBAT enable"]
pub type VBATE_R = crate::BitReader<VBATE>;
impl VBATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VBATE {
        match self.bits {
            false => VBATE::Disabled,
            true => VBATE::Enabled,
        }
    }
    #[doc = "V_BAT channel disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VBATE::Disabled
    }
    #[doc = "V_BAT channel enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VBATE::Enabled
    }
}
#[doc = "Field `VBATE` writer - VBAT enable"]
pub type VBATE_W<'a, REG> = crate::BitWriter<'a, REG, VBATE>;
impl<'a, REG> VBATE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "V_BAT channel disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(VBATE::Disabled)
    }
    #[doc = "V_BAT channel enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(VBATE::Enabled)
    }
}
#[doc = "Temperature sensor and VREFINT enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSVREFE {
    #[doc = "0: Temperature sensor and V_REFINT channel disabled"]
    Disabled = 0,
    #[doc = "1: Temperature sensor and V_REFINT channel enabled"]
    Enabled = 1,
}
impl From<TSVREFE> for bool {
    #[inline(always)]
    fn from(variant: TSVREFE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSVREFE` reader - Temperature sensor and VREFINT enable"]
pub type TSVREFE_R = crate::BitReader<TSVREFE>;
impl TSVREFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TSVREFE {
        match self.bits {
            false => TSVREFE::Disabled,
            true => TSVREFE::Enabled,
        }
    }
    #[doc = "Temperature sensor and V_REFINT channel disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TSVREFE::Disabled
    }
    #[doc = "Temperature sensor and V_REFINT channel enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TSVREFE::Enabled
    }
}
#[doc = "Field `TSVREFE` writer - Temperature sensor and VREFINT enable"]
pub type TSVREFE_W<'a, REG> = crate::BitWriter<'a, REG, TSVREFE>;
impl<'a, REG> TSVREFE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Temperature sensor and V_REFINT channel disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TSVREFE::Disabled)
    }
    #[doc = "Temperature sensor and V_REFINT channel enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TSVREFE::Enabled)
    }
}
impl R {
    #[doc = "Bits 16:17 - ADC prescaler"]
    #[inline(always)]
    pub fn adcpre(&self) -> ADCPRE_R {
        ADCPRE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 22 - VBAT enable"]
    #[inline(always)]
    pub fn vbate(&self) -> VBATE_R {
        VBATE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Temperature sensor and VREFINT enable"]
    #[inline(always)]
    pub fn tsvrefe(&self) -> TSVREFE_R {
        TSVREFE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:17 - ADC prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn adcpre(&mut self) -> ADCPRE_W<CCRrs> {
        ADCPRE_W::new(self, 16)
    }
    #[doc = "Bit 22 - VBAT enable"]
    #[inline(always)]
    #[must_use]
    pub fn vbate(&mut self) -> VBATE_W<CCRrs> {
        VBATE_W::new(self, 22)
    }
    #[doc = "Bit 23 - Temperature sensor and VREFINT enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsvrefe(&mut self) -> TSVREFE_W<CCRrs> {
        TSVREFE_W::new(self, 23)
    }
}
#[doc = "ADC common control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
