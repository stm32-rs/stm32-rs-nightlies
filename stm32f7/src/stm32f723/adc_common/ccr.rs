///Register `CCR` reader
pub type R = crate::R<CCRrs>;
///Register `CCR` writer
pub type W = crate::W<CCRrs>;
/**Multi ADC mode selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MULTI {
    ///0: All the ADCs independent: independent mode
    Independent = 0,
    ///1: Dual ADC1 and ADC2, combined regular and injected simultaneous mode
    DualRj = 1,
    ///2: Dual ADC1 and ADC2, combined regular and alternate trigger mode
    DualRa = 2,
    ///5: Dual ADC1 and ADC2, injected simultaneous mode only
    DualJ = 5,
    ///6: Dual ADC1 and ADC2, regular simultaneous mode only
    DualR = 6,
    ///7: Dual ADC1 and ADC2, interleaved mode only
    DualI = 7,
    ///9: Dual ADC1 and ADC2, alternate trigger mode only
    DualA = 9,
    ///17: Triple ADC, regular and injected simultaneous mode
    TripleRj = 17,
    ///18: Triple ADC, regular and alternate trigger mode
    TripleRa = 18,
    ///21: Triple ADC, injected simultaneous mode only
    TripleJ = 21,
    ///22: Triple ADC, regular simultaneous mode only
    TripleR = 22,
    ///23: Triple ADC, interleaved mode only
    TripleI = 23,
    ///24: Triple ADC, alternate trigger mode only
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
impl crate::IsEnum for MULTI {}
///Field `MULTI` reader - Multi ADC mode selection
pub type MULTI_R = crate::FieldReader<MULTI>;
impl MULTI_R {
    ///Get enumerated values variant
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
    ///All the ADCs independent: independent mode
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == MULTI::Independent
    }
    ///Dual ADC1 and ADC2, combined regular and injected simultaneous mode
    #[inline(always)]
    pub fn is_dual_rj(&self) -> bool {
        *self == MULTI::DualRj
    }
    ///Dual ADC1 and ADC2, combined regular and alternate trigger mode
    #[inline(always)]
    pub fn is_dual_ra(&self) -> bool {
        *self == MULTI::DualRa
    }
    ///Dual ADC1 and ADC2, injected simultaneous mode only
    #[inline(always)]
    pub fn is_dual_j(&self) -> bool {
        *self == MULTI::DualJ
    }
    ///Dual ADC1 and ADC2, regular simultaneous mode only
    #[inline(always)]
    pub fn is_dual_r(&self) -> bool {
        *self == MULTI::DualR
    }
    ///Dual ADC1 and ADC2, interleaved mode only
    #[inline(always)]
    pub fn is_dual_i(&self) -> bool {
        *self == MULTI::DualI
    }
    ///Dual ADC1 and ADC2, alternate trigger mode only
    #[inline(always)]
    pub fn is_dual_a(&self) -> bool {
        *self == MULTI::DualA
    }
    ///Triple ADC, regular and injected simultaneous mode
    #[inline(always)]
    pub fn is_triple_rj(&self) -> bool {
        *self == MULTI::TripleRj
    }
    ///Triple ADC, regular and alternate trigger mode
    #[inline(always)]
    pub fn is_triple_ra(&self) -> bool {
        *self == MULTI::TripleRa
    }
    ///Triple ADC, injected simultaneous mode only
    #[inline(always)]
    pub fn is_triple_j(&self) -> bool {
        *self == MULTI::TripleJ
    }
    ///Triple ADC, regular simultaneous mode only
    #[inline(always)]
    pub fn is_triple_r(&self) -> bool {
        *self == MULTI::TripleR
    }
    ///Triple ADC, interleaved mode only
    #[inline(always)]
    pub fn is_triple_i(&self) -> bool {
        *self == MULTI::TripleI
    }
    ///Triple ADC, alternate trigger mode only
    #[inline(always)]
    pub fn is_triple_a(&self) -> bool {
        *self == MULTI::TripleA
    }
}
///Field `MULTI` writer - Multi ADC mode selection
pub type MULTI_W<'a, REG> = crate::FieldWriter<'a, REG, 5, MULTI>;
impl<'a, REG> MULTI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///All the ADCs independent: independent mode
    #[inline(always)]
    pub fn independent(self) -> &'a mut crate::W<REG> {
        self.variant(MULTI::Independent)
    }
    ///Dual ADC1 and ADC2, combined regular and injected simultaneous mode
    #[inline(always)]
    pub fn dual_rj(self) -> &'a mut crate::W<REG> {
        self.variant(MULTI::DualRj)
    }
    ///Dual ADC1 and ADC2, combined regular and alternate trigger mode
    #[inline(always)]
    pub fn dual_ra(self) -> &'a mut crate::W<REG> {
        self.variant(MULTI::DualRa)
    }
    ///Dual ADC1 and ADC2, injected simultaneous mode only
    #[inline(always)]
    pub fn dual_j(self) -> &'a mut crate::W<REG> {
        self.variant(MULTI::DualJ)
    }
    ///Dual ADC1 and ADC2, regular simultaneous mode only
    #[inline(always)]
    pub fn dual_r(self) -> &'a mut crate::W<REG> {
        self.variant(MULTI::DualR)
    }
    ///Dual ADC1 and ADC2, interleaved mode only
    #[inline(always)]
    pub fn dual_i(self) -> &'a mut crate::W<REG> {
        self.variant(MULTI::DualI)
    }
    ///Dual ADC1 and ADC2, alternate trigger mode only
    #[inline(always)]
    pub fn dual_a(self) -> &'a mut crate::W<REG> {
        self.variant(MULTI::DualA)
    }
    ///Triple ADC, regular and injected simultaneous mode
    #[inline(always)]
    pub fn triple_rj(self) -> &'a mut crate::W<REG> {
        self.variant(MULTI::TripleRj)
    }
    ///Triple ADC, regular and alternate trigger mode
    #[inline(always)]
    pub fn triple_ra(self) -> &'a mut crate::W<REG> {
        self.variant(MULTI::TripleRa)
    }
    ///Triple ADC, injected simultaneous mode only
    #[inline(always)]
    pub fn triple_j(self) -> &'a mut crate::W<REG> {
        self.variant(MULTI::TripleJ)
    }
    ///Triple ADC, regular simultaneous mode only
    #[inline(always)]
    pub fn triple_r(self) -> &'a mut crate::W<REG> {
        self.variant(MULTI::TripleR)
    }
    ///Triple ADC, interleaved mode only
    #[inline(always)]
    pub fn triple_i(self) -> &'a mut crate::W<REG> {
        self.variant(MULTI::TripleI)
    }
    ///Triple ADC, alternate trigger mode only
    #[inline(always)]
    pub fn triple_a(self) -> &'a mut crate::W<REG> {
        self.variant(MULTI::TripleA)
    }
}
///Field `DELAY` reader - Delay between 2 sampling phases
pub type DELAY_R = crate::FieldReader;
///Field `DELAY` writer - Delay between 2 sampling phases
pub type DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
/**DMA disable selection for multi-ADC mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DDS {
    ///0: No new DMA request is issued after the last transfer
    Single = 0,
    ///1: DMA requests are issued as long as data are converted and DMA=01, 10 or 11
    Continuous = 1,
}
impl From<DDS> for bool {
    #[inline(always)]
    fn from(variant: DDS) -> Self {
        variant as u8 != 0
    }
}
///Field `DDS` reader - DMA disable selection for multi-ADC mode
pub type DDS_R = crate::BitReader<DDS>;
impl DDS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DDS {
        match self.bits {
            false => DDS::Single,
            true => DDS::Continuous,
        }
    }
    ///No new DMA request is issued after the last transfer
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == DDS::Single
    }
    ///DMA requests are issued as long as data are converted and DMA=01, 10 or 11
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == DDS::Continuous
    }
}
///Field `DDS` writer - DMA disable selection for multi-ADC mode
pub type DDS_W<'a, REG> = crate::BitWriter<'a, REG, DDS>;
impl<'a, REG> DDS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No new DMA request is issued after the last transfer
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(DDS::Single)
    }
    ///DMA requests are issued as long as data are converted and DMA=01, 10 or 11
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(DDS::Continuous)
    }
}
/**Direct memory access mode for multi ADC mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMA {
    ///0: DMA mode disabled
    Disabled = 0,
    ///1: DMA mode 1 enabled (2 / 3 half-words one by one - 1 then 2 then 3)
    Mode1 = 1,
    ///2: DMA mode 2 enabled (2 / 3 half-words by pairs - 2&1 then 1&3 then 3&2)
    Mode2 = 2,
    ///3: DMA mode 3 enabled (2 / 3 half-words by pairs - 2&1 then 1&3 then 3&2)
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
impl crate::IsEnum for DMA {}
///Field `DMA` reader - Direct memory access mode for multi ADC mode
pub type DMA_R = crate::FieldReader<DMA>;
impl DMA_R {
    ///Get enumerated values variant
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
    ///DMA mode disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA::Disabled
    }
    ///DMA mode 1 enabled (2 / 3 half-words one by one - 1 then 2 then 3)
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == DMA::Mode1
    }
    ///DMA mode 2 enabled (2 / 3 half-words by pairs - 2&1 then 1&3 then 3&2)
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == DMA::Mode2
    }
    ///DMA mode 3 enabled (2 / 3 half-words by pairs - 2&1 then 1&3 then 3&2)
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == DMA::Mode3
    }
}
///Field `DMA` writer - Direct memory access mode for multi ADC mode
pub type DMA_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DMA, crate::Safe>;
impl<'a, REG> DMA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///DMA mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMA::Disabled)
    }
    ///DMA mode 1 enabled (2 / 3 half-words one by one - 1 then 2 then 3)
    #[inline(always)]
    pub fn mode1(self) -> &'a mut crate::W<REG> {
        self.variant(DMA::Mode1)
    }
    ///DMA mode 2 enabled (2 / 3 half-words by pairs - 2&1 then 1&3 then 3&2)
    #[inline(always)]
    pub fn mode2(self) -> &'a mut crate::W<REG> {
        self.variant(DMA::Mode2)
    }
    ///DMA mode 3 enabled (2 / 3 half-words by pairs - 2&1 then 1&3 then 3&2)
    #[inline(always)]
    pub fn mode3(self) -> &'a mut crate::W<REG> {
        self.variant(DMA::Mode3)
    }
}
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
    ///Bits 0:4 - Multi ADC mode selection
    #[inline(always)]
    pub fn multi(&self) -> MULTI_R {
        MULTI_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 8:11 - Delay between 2 sampling phases
    #[inline(always)]
    pub fn delay(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 13 - DMA disable selection for multi-ADC mode
    #[inline(always)]
    pub fn dds(&self) -> DDS_R {
        DDS_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 14:15 - Direct memory access mode for multi ADC mode
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 14) & 3) as u8)
    }
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
            .field("dma", &self.dma())
            .field("dds", &self.dds())
            .field("delay", &self.delay())
            .field("multi", &self.multi())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - Multi ADC mode selection
    #[inline(always)]
    pub fn multi(&mut self) -> MULTI_W<'_, CCRrs> {
        MULTI_W::new(self, 0)
    }
    ///Bits 8:11 - Delay between 2 sampling phases
    #[inline(always)]
    pub fn delay(&mut self) -> DELAY_W<'_, CCRrs> {
        DELAY_W::new(self, 8)
    }
    ///Bit 13 - DMA disable selection for multi-ADC mode
    #[inline(always)]
    pub fn dds(&mut self) -> DDS_W<'_, CCRrs> {
        DDS_W::new(self, 13)
    }
    ///Bits 14:15 - Direct memory access mode for multi ADC mode
    #[inline(always)]
    pub fn dma(&mut self) -> DMA_W<'_, CCRrs> {
        DMA_W::new(self, 14)
    }
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

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F723.html#ADC_Common:CCR)*/
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
