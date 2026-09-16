///Register `CCR` reader
pub type R = crate::R<CCRrs>;
///Register `CCR` writer
pub type W = crate::W<CCRrs>;
/**ADC prescaler

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADCPRE {
    ///0: PCLK2 divided by 2
    Div2 = 0,
    ///1: PCLK2 divided by 4
    Div4 = 1,
    ///2: PCLK2 divided by 6
    Div6 = 2,
    ///3: PCLK2 divided by 8
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
impl crate::IsEnum for ADCPRE {}
///Field `ADCPRE` reader - ADC prescaler
pub type ADCPRE_R = crate::FieldReader<ADCPRE>;
impl ADCPRE_R {
    ///Get enumerated values variant
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
    ///PCLK2 divided by 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == ADCPRE::Div2
    }
    ///PCLK2 divided by 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == ADCPRE::Div4
    }
    ///PCLK2 divided by 6
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == ADCPRE::Div6
    }
    ///PCLK2 divided by 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == ADCPRE::Div8
    }
}
///Field `ADCPRE` writer - ADC prescaler
pub type ADCPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ADCPRE, crate::Safe>;
impl<'a, REG> ADCPRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PCLK2 divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(ADCPRE::Div2)
    }
    ///PCLK2 divided by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(ADCPRE::Div4)
    }
    ///PCLK2 divided by 6
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(ADCPRE::Div6)
    }
    ///PCLK2 divided by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(ADCPRE::Div8)
    }
}
/**VBAT enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBATE {
    ///0: V_BAT channel disabled
    Disabled = 0,
    ///1: V_BAT channel enabled
    Enabled = 1,
}
impl From<VBATE> for bool {
    #[inline(always)]
    fn from(variant: VBATE) -> Self {
        variant as u8 != 0
    }
}
///Field `VBATE` reader - VBAT enable
pub type VBATE_R = crate::BitReader<VBATE>;
impl VBATE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VBATE {
        match self.bits {
            false => VBATE::Disabled,
            true => VBATE::Enabled,
        }
    }
    ///V_BAT channel disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VBATE::Disabled
    }
    ///V_BAT channel enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VBATE::Enabled
    }
}
///Field `VBATE` writer - VBAT enable
pub type VBATE_W<'a, REG> = crate::BitWriter<'a, REG, VBATE>;
impl<'a, REG> VBATE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///V_BAT channel disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(VBATE::Disabled)
    }
    ///V_BAT channel enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(VBATE::Enabled)
    }
}
/**Temperature sensor and VREFINT enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSVREFE {
    ///0: Temperature sensor and V_REFINT channel disabled
    Disabled = 0,
    ///1: Temperature sensor and V_REFINT channel enabled
    Enabled = 1,
}
impl From<TSVREFE> for bool {
    #[inline(always)]
    fn from(variant: TSVREFE) -> Self {
        variant as u8 != 0
    }
}
///Field `TSVREFE` reader - Temperature sensor and VREFINT enable
pub type TSVREFE_R = crate::BitReader<TSVREFE>;
impl TSVREFE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TSVREFE {
        match self.bits {
            false => TSVREFE::Disabled,
            true => TSVREFE::Enabled,
        }
    }
    ///Temperature sensor and V_REFINT channel disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TSVREFE::Disabled
    }
    ///Temperature sensor and V_REFINT channel enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TSVREFE::Enabled
    }
}
///Field `TSVREFE` writer - Temperature sensor and VREFINT enable
pub type TSVREFE_W<'a, REG> = crate::BitWriter<'a, REG, TSVREFE>;
impl<'a, REG> TSVREFE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Temperature sensor and V_REFINT channel disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TSVREFE::Disabled)
    }
    ///Temperature sensor and V_REFINT channel enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TSVREFE::Enabled)
    }
}
impl R {
    ///Bits 16:17 - ADC prescaler
    #[inline(always)]
    pub fn adcpre(&self) -> ADCPRE_R {
        ADCPRE_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 22 - VBAT enable
    #[inline(always)]
    pub fn vbate(&self) -> VBATE_R {
        VBATE_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Temperature sensor and VREFINT enable
    #[inline(always)]
    pub fn tsvrefe(&self) -> TSVREFE_R {
        TSVREFE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR")
            .field("tsvrefe", &self.tsvrefe())
            .field("vbate", &self.vbate())
            .field("adcpre", &self.adcpre())
            .finish()
    }
}
impl W {
    ///Bits 16:17 - ADC prescaler
    #[inline(always)]
    pub fn adcpre(&mut self) -> ADCPRE_W<'_, CCRrs> {
        ADCPRE_W::new(self, 16)
    }
    ///Bit 22 - VBAT enable
    #[inline(always)]
    pub fn vbate(&mut self) -> VBATE_W<'_, CCRrs> {
        VBATE_W::new(self, 22)
    }
    ///Bit 23 - Temperature sensor and VREFINT enable
    #[inline(always)]
    pub fn tsvrefe(&mut self) -> TSVREFE_W<'_, CCRrs> {
        TSVREFE_W::new(self, 23)
    }
}
/**ADC common control register

You can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F412.html#ADC_Common:CCR)*/
pub struct CCRrs;
impl crate::RegisterSpec for CCRrs {
    type Ux = u32;
}
///`read()` method returns [`ccr::R`](R) reader structure
impl crate::Readable for CCRrs {}
///`write(|w| ..)` method takes [`ccr::W`](W) writer structure
impl crate::Writable for CCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCR to value 0
impl crate::Resettable for CCRrs {}
