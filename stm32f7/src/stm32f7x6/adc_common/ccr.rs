#[doc = "Register `CCR` reader"]
pub type R = crate::R<CCRrs>;
#[doc = "Register `CCR` writer"]
pub type W = crate::W<CCRrs>;
#[doc = "Multi ADC mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MULTI {
    #[doc = "0: All the ADCs independent: independent mode"]
    Independent = 0,
    #[doc = "1: Dual ADC1 and ADC2, combined regular and injected simultaneous mode"]
    DualRj = 1,
    #[doc = "2: Dual ADC1 and ADC2, combined regular and alternate trigger mode"]
    DualRa = 2,
    #[doc = "5: Dual ADC1 and ADC2, injected simultaneous mode only"]
    DualJ = 5,
    #[doc = "6: Dual ADC1 and ADC2, regular simultaneous mode only"]
    DualR = 6,
    #[doc = "7: Dual ADC1 and ADC2, interleaved mode only"]
    DualI = 7,
    #[doc = "9: Dual ADC1 and ADC2, alternate trigger mode only"]
    DualA = 9,
    #[doc = "17: Triple ADC, regular and injected simultaneous mode"]
    TripleRj = 17,
    #[doc = "18: Triple ADC, regular and alternate trigger mode"]
    TripleRa = 18,
    #[doc = "21: Triple ADC, injected simultaneous mode only"]
    TripleJ = 21,
    #[doc = "22: Triple ADC, regular simultaneous mode only"]
    TripleR = 22,
    #[doc = "23: Triple ADC, interleaved mode only"]
    TripleI = 23,
    #[doc = "24: Triple ADC, alternate trigger mode only"]
    TripleA = 24,
}
impl From<MULTI> for u8 {
    #[inline(always)]
    fn from(variant: MULTI) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MULTI {
    type Ux = u8;
}
#[doc = "Field `MULTI` reader - Multi ADC mode selection"]
pub type MULTI_R = crate::FieldReader<MULTI>;
impl MULTI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MULTI> {
        match self.bits {
            0 => Some(MULTI::Independent),
            1 => Some(MULTI::DualRj),
            2 => Some(MULTI::DualRa),
            5 => Some(MULTI::DualJ),
            6 => Some(MULTI::DualR),
            7 => Some(MULTI::DualI),
            9 => Some(MULTI::DualA),
            17 => Some(MULTI::TripleRj),
            18 => Some(MULTI::TripleRa),
            21 => Some(MULTI::TripleJ),
            22 => Some(MULTI::TripleR),
            23 => Some(MULTI::TripleI),
            24 => Some(MULTI::TripleA),
            _ => None,
        }
    }
    #[doc = "All the ADCs independent: independent mode"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == MULTI::Independent
    }
    #[doc = "Dual ADC1 and ADC2, combined regular and injected simultaneous mode"]
    #[inline(always)]
    pub fn is_dual_rj(&self) -> bool {
        *self == MULTI::DualRj
    }
    #[doc = "Dual ADC1 and ADC2, combined regular and alternate trigger mode"]
    #[inline(always)]
    pub fn is_dual_ra(&self) -> bool {
        *self == MULTI::DualRa
    }
    #[doc = "Dual ADC1 and ADC2, injected simultaneous mode only"]
    #[inline(always)]
    pub fn is_dual_j(&self) -> bool {
        *self == MULTI::DualJ
    }
    #[doc = "Dual ADC1 and ADC2, regular simultaneous mode only"]
    #[inline(always)]
    pub fn is_dual_r(&self) -> bool {
        *self == MULTI::DualR
    }
    #[doc = "Dual ADC1 and ADC2, interleaved mode only"]
    #[inline(always)]
    pub fn is_dual_i(&self) -> bool {
        *self == MULTI::DualI
    }
    #[doc = "Dual ADC1 and ADC2, alternate trigger mode only"]
    #[inline(always)]
    pub fn is_dual_a(&self) -> bool {
        *self == MULTI::DualA
    }
    #[doc = "Triple ADC, regular and injected simultaneous mode"]
    #[inline(always)]
    pub fn is_triple_rj(&self) -> bool {
        *self == MULTI::TripleRj
    }
    #[doc = "Triple ADC, regular and alternate trigger mode"]
    #[inline(always)]
    pub fn is_triple_ra(&self) -> bool {
        *self == MULTI::TripleRa
    }
    #[doc = "Triple ADC, injected simultaneous mode only"]
    #[inline(always)]
    pub fn is_triple_j(&self) -> bool {
        *self == MULTI::TripleJ
    }
    #[doc = "Triple ADC, regular simultaneous mode only"]
    #[inline(always)]
    pub fn is_triple_r(&self) -> bool {
        *self == MULTI::TripleR
    }
    #[doc = "Triple ADC, interleaved mode only"]
    #[inline(always)]
    pub fn is_triple_i(&self) -> bool {
        *self == MULTI::TripleI
    }
    #[doc = "Triple ADC, alternate trigger mode only"]
    #[inline(always)]
    pub fn is_triple_a(&self) -> bool {
        *self == MULTI::TripleA
    }
}
#[doc = "Field `MULTI` writer - Multi ADC mode selection"]
pub type MULTI_W<'a, REG> = crate::FieldWriter<'a, REG, 5, MULTI>;
impl<'a, REG> MULTI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "All the ADCs independent: independent mode"]
    #[inline(always)]
    pub fn independent(self) -> &'a mut crate::W<REG> {
        self.variant(MULTI::Independent)
    }
    #[doc = "Dual ADC1 and ADC2, combined regular and injected simultaneous mode"]
    #[inline(always)]
    pub fn dual_rj(self) -> &'a mut crate::W<REG> {
        self.variant(MULTI::DualRj)
    }
    #[doc = "Dual ADC1 and ADC2, combined regular and alternate trigger mode"]
    #[inline(always)]
    pub fn dual_ra(self) -> &'a mut crate::W<REG> {
        self.variant(MULTI::DualRa)
    }
    #[doc = "Dual ADC1 and ADC2, injected simultaneous mode only"]
    #[inline(always)]
    pub fn dual_j(self) -> &'a mut crate::W<REG> {
        self.variant(MULTI::DualJ)
    }
    #[doc = "Dual ADC1 and ADC2, regular simultaneous mode only"]
    #[inline(always)]
    pub fn dual_r(self) -> &'a mut crate::W<REG> {
        self.variant(MULTI::DualR)
    }
    #[doc = "Dual ADC1 and ADC2, interleaved mode only"]
    #[inline(always)]
    pub fn dual_i(self) -> &'a mut crate::W<REG> {
        self.variant(MULTI::DualI)
    }
    #[doc = "Dual ADC1 and ADC2, alternate trigger mode only"]
    #[inline(always)]
    pub fn dual_a(self) -> &'a mut crate::W<REG> {
        self.variant(MULTI::DualA)
    }
    #[doc = "Triple ADC, regular and injected simultaneous mode"]
    #[inline(always)]
    pub fn triple_rj(self) -> &'a mut crate::W<REG> {
        self.variant(MULTI::TripleRj)
    }
    #[doc = "Triple ADC, regular and alternate trigger mode"]
    #[inline(always)]
    pub fn triple_ra(self) -> &'a mut crate::W<REG> {
        self.variant(MULTI::TripleRa)
    }
    #[doc = "Triple ADC, injected simultaneous mode only"]
    #[inline(always)]
    pub fn triple_j(self) -> &'a mut crate::W<REG> {
        self.variant(MULTI::TripleJ)
    }
    #[doc = "Triple ADC, regular simultaneous mode only"]
    #[inline(always)]
    pub fn triple_r(self) -> &'a mut crate::W<REG> {
        self.variant(MULTI::TripleR)
    }
    #[doc = "Triple ADC, interleaved mode only"]
    #[inline(always)]
    pub fn triple_i(self) -> &'a mut crate::W<REG> {
        self.variant(MULTI::TripleI)
    }
    #[doc = "Triple ADC, alternate trigger mode only"]
    #[inline(always)]
    pub fn triple_a(self) -> &'a mut crate::W<REG> {
        self.variant(MULTI::TripleA)
    }
}
#[doc = "Field `DELAY` reader - Delay between 2 sampling phases"]
pub type DELAY_R = crate::FieldReader;
#[doc = "Field `DELAY` writer - Delay between 2 sampling phases"]
pub type DELAY_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
#[doc = "DMA disable selection for multi-ADC mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DDS {
    #[doc = "0: No new DMA request is issued after the last transfer"]
    Single = 0,
    #[doc = "1: DMA requests are issued as long as data are converted and DMA=01, 10 or 11"]
    Continuous = 1,
}
impl From<DDS> for bool {
    #[inline(always)]
    fn from(variant: DDS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DDS` reader - DMA disable selection for multi-ADC mode"]
pub type DDS_R = crate::BitReader<DDS>;
impl DDS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DDS {
        match self.bits {
            false => DDS::Single,
            true => DDS::Continuous,
        }
    }
    #[doc = "No new DMA request is issued after the last transfer"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == DDS::Single
    }
    #[doc = "DMA requests are issued as long as data are converted and DMA=01, 10 or 11"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == DDS::Continuous
    }
}
#[doc = "Field `DDS` writer - DMA disable selection for multi-ADC mode"]
pub type DDS_W<'a, REG> = crate::BitWriter<'a, REG, DDS>;
impl<'a, REG> DDS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No new DMA request is issued after the last transfer"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(DDS::Single)
    }
    #[doc = "DMA requests are issued as long as data are converted and DMA=01, 10 or 11"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(DDS::Continuous)
    }
}
#[doc = "Direct memory access mode for multi ADC mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMA {
    #[doc = "0: DMA mode disabled"]
    Disabled = 0,
    #[doc = "1: DMA mode 1 enabled (2 / 3 half-words one by one - 1 then 2 then 3)"]
    Mode1 = 1,
    #[doc = "2: DMA mode 2 enabled (2 / 3 half-words by pairs - 2&amp;1 then 1&amp;3 then 3&amp;2)"]
    Mode2 = 2,
    #[doc = "3: DMA mode 3 enabled (2 / 3 half-words by pairs - 2&amp;1 then 1&amp;3 then 3&amp;2)"]
    Mode3 = 3,
}
impl From<DMA> for u8 {
    #[inline(always)]
    fn from(variant: DMA) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DMA {
    type Ux = u8;
}
#[doc = "Field `DMA` reader - Direct memory access mode for multi ADC mode"]
pub type DMA_R = crate::FieldReader<DMA>;
impl DMA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA {
        match self.bits {
            0 => DMA::Disabled,
            1 => DMA::Mode1,
            2 => DMA::Mode2,
            3 => DMA::Mode3,
            _ => unreachable!(),
        }
    }
    #[doc = "DMA mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA::Disabled
    }
    #[doc = "DMA mode 1 enabled (2 / 3 half-words one by one - 1 then 2 then 3)"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == DMA::Mode1
    }
    #[doc = "DMA mode 2 enabled (2 / 3 half-words by pairs - 2&amp;1 then 1&amp;3 then 3&amp;2)"]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == DMA::Mode2
    }
    #[doc = "DMA mode 3 enabled (2 / 3 half-words by pairs - 2&amp;1 then 1&amp;3 then 3&amp;2)"]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == DMA::Mode3
    }
}
#[doc = "Field `DMA` writer - Direct memory access mode for multi ADC mode"]
pub type DMA_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, DMA>;
impl<'a, REG> DMA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DMA mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMA::Disabled)
    }
    #[doc = "DMA mode 1 enabled (2 / 3 half-words one by one - 1 then 2 then 3)"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut crate::W<REG> {
        self.variant(DMA::Mode1)
    }
    #[doc = "DMA mode 2 enabled (2 / 3 half-words by pairs - 2&amp;1 then 1&amp;3 then 3&amp;2)"]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut crate::W<REG> {
        self.variant(DMA::Mode2)
    }
    #[doc = "DMA mode 3 enabled (2 / 3 half-words by pairs - 2&amp;1 then 1&amp;3 then 3&amp;2)"]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut crate::W<REG> {
        self.variant(DMA::Mode3)
    }
}
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
    #[doc = "Bits 0:4 - Multi ADC mode selection"]
    #[inline(always)]
    pub fn multi(&self) -> MULTI_R {
        MULTI_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - Delay between 2 sampling phases"]
    #[inline(always)]
    pub fn delay(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - DMA disable selection for multi-ADC mode"]
    #[inline(always)]
    pub fn dds(&self) -> DDS_R {
        DDS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Direct memory access mode for multi ADC mode"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 14) & 3) as u8)
    }
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
    #[doc = "Bits 0:4 - Multi ADC mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn multi(&mut self) -> MULTI_W<CCRrs> {
        MULTI_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - Delay between 2 sampling phases"]
    #[inline(always)]
    #[must_use]
    pub fn delay(&mut self) -> DELAY_W<CCRrs> {
        DELAY_W::new(self, 8)
    }
    #[doc = "Bit 13 - DMA disable selection for multi-ADC mode"]
    #[inline(always)]
    #[must_use]
    pub fn dds(&mut self) -> DDS_W<CCRrs> {
        DDS_W::new(self, 13)
    }
    #[doc = "Bits 14:15 - Direct memory access mode for multi ADC mode"]
    #[inline(always)]
    #[must_use]
    pub fn dma(&mut self) -> DMA_W<CCRrs> {
        DMA_W::new(self, 14)
    }
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
