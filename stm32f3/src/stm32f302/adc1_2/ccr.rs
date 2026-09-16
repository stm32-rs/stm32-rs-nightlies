///Register `CCR` reader
pub type R = crate::R<CCRrs>;
///Register `CCR` writer
pub type W = crate::W<CCRrs>;
/**Dual ADC mode selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DUAL {
    ///0: Independent mode
    Independent = 0,
    ///1: Dual, combined regular simultaneous + injected simultaneous mode
    DualRj = 1,
    ///2: Dual, combined regular simultaneous + alternate trigger mode
    DualRa = 2,
    ///3: Dual, combined interleaved mode + injected simultaneous mode
    DualIj = 3,
    ///5: Dual, injected simultaneous mode only
    DualJ = 5,
    ///6: Dual, regular simultaneous mode only
    DualR = 6,
    ///7: Dual, interleaved mode only
    DualI = 7,
    ///9: Dual, alternate trigger mode only
    DualA = 9,
}
impl From<DUAL> for u8 {
    #[inline(always)]
    fn from(variant: DUAL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DUAL {
    type Ux = u8;
}
impl crate::IsEnum for DUAL {}
///Field `DUAL` reader - Dual ADC mode selection
pub type DUAL_R = crate::FieldReader<DUAL>;
impl DUAL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<DUAL> {
        match self.bits {
            0 => Some(DUAL::Independent),
            1 => Some(DUAL::DualRj),
            2 => Some(DUAL::DualRa),
            3 => Some(DUAL::DualIj),
            5 => Some(DUAL::DualJ),
            6 => Some(DUAL::DualR),
            7 => Some(DUAL::DualI),
            9 => Some(DUAL::DualA),
            _ => None,
        }
    }
    ///Independent mode
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == DUAL::Independent
    }
    ///Dual, combined regular simultaneous + injected simultaneous mode
    #[inline(always)]
    pub fn is_dual_rj(&self) -> bool {
        *self == DUAL::DualRj
    }
    ///Dual, combined regular simultaneous + alternate trigger mode
    #[inline(always)]
    pub fn is_dual_ra(&self) -> bool {
        *self == DUAL::DualRa
    }
    ///Dual, combined interleaved mode + injected simultaneous mode
    #[inline(always)]
    pub fn is_dual_ij(&self) -> bool {
        *self == DUAL::DualIj
    }
    ///Dual, injected simultaneous mode only
    #[inline(always)]
    pub fn is_dual_j(&self) -> bool {
        *self == DUAL::DualJ
    }
    ///Dual, regular simultaneous mode only
    #[inline(always)]
    pub fn is_dual_r(&self) -> bool {
        *self == DUAL::DualR
    }
    ///Dual, interleaved mode only
    #[inline(always)]
    pub fn is_dual_i(&self) -> bool {
        *self == DUAL::DualI
    }
    ///Dual, alternate trigger mode only
    #[inline(always)]
    pub fn is_dual_a(&self) -> bool {
        *self == DUAL::DualA
    }
}
///Field `DUAL` writer - Dual ADC mode selection
pub type DUAL_W<'a, REG> = crate::FieldWriter<'a, REG, 5, DUAL>;
impl<'a, REG> DUAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Independent mode
    #[inline(always)]
    pub fn independent(self) -> &'a mut crate::W<REG> {
        self.variant(DUAL::Independent)
    }
    ///Dual, combined regular simultaneous + injected simultaneous mode
    #[inline(always)]
    pub fn dual_rj(self) -> &'a mut crate::W<REG> {
        self.variant(DUAL::DualRj)
    }
    ///Dual, combined regular simultaneous + alternate trigger mode
    #[inline(always)]
    pub fn dual_ra(self) -> &'a mut crate::W<REG> {
        self.variant(DUAL::DualRa)
    }
    ///Dual, combined interleaved mode + injected simultaneous mode
    #[inline(always)]
    pub fn dual_ij(self) -> &'a mut crate::W<REG> {
        self.variant(DUAL::DualIj)
    }
    ///Dual, injected simultaneous mode only
    #[inline(always)]
    pub fn dual_j(self) -> &'a mut crate::W<REG> {
        self.variant(DUAL::DualJ)
    }
    ///Dual, regular simultaneous mode only
    #[inline(always)]
    pub fn dual_r(self) -> &'a mut crate::W<REG> {
        self.variant(DUAL::DualR)
    }
    ///Dual, interleaved mode only
    #[inline(always)]
    pub fn dual_i(self) -> &'a mut crate::W<REG> {
        self.variant(DUAL::DualI)
    }
    ///Dual, alternate trigger mode only
    #[inline(always)]
    pub fn dual_a(self) -> &'a mut crate::W<REG> {
        self.variant(DUAL::DualA)
    }
}
///Field `DELAY` reader - Delay between 2 sampling phases
pub type DELAY_R = crate::FieldReader;
///Field `DELAY` writer - Delay between 2 sampling phases
pub type DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
/**DMA configuration (for multi-ADC mode)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMACFG {
    ///0: DMA one shot mode selected
    OneShot = 0,
    ///1: DMA circular mode selected
    Circulator = 1,
}
impl From<DMACFG> for bool {
    #[inline(always)]
    fn from(variant: DMACFG) -> Self {
        variant as u8 != 0
    }
}
///Field `DMACFG` reader - DMA configuration (for multi-ADC mode)
pub type DMACFG_R = crate::BitReader<DMACFG>;
impl DMACFG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMACFG {
        match self.bits {
            false => DMACFG::OneShot,
            true => DMACFG::Circulator,
        }
    }
    ///DMA one shot mode selected
    #[inline(always)]
    pub fn is_one_shot(&self) -> bool {
        *self == DMACFG::OneShot
    }
    ///DMA circular mode selected
    #[inline(always)]
    pub fn is_circulator(&self) -> bool {
        *self == DMACFG::Circulator
    }
}
///Field `DMACFG` writer - DMA configuration (for multi-ADC mode)
pub type DMACFG_W<'a, REG> = crate::BitWriter<'a, REG, DMACFG>;
impl<'a, REG> DMACFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA one shot mode selected
    #[inline(always)]
    pub fn one_shot(self) -> &'a mut crate::W<REG> {
        self.variant(DMACFG::OneShot)
    }
    ///DMA circular mode selected
    #[inline(always)]
    pub fn circulator(self) -> &'a mut crate::W<REG> {
        self.variant(DMACFG::Circulator)
    }
}
/**Direct memory access mode for multi ADC mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MDMA {
    ///0: MDMA mode disabled
    Disabled = 0,
    ///2: MDMA mode enabled for 12 and 10-bit resolution
    Bits12_10 = 2,
    ///3: MDMA mode enabled for 8 and 6-bit resolution
    Bits8_6 = 3,
}
impl From<MDMA> for u8 {
    #[inline(always)]
    fn from(variant: MDMA) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MDMA {
    type Ux = u8;
}
impl crate::IsEnum for MDMA {}
///Field `MDMA` reader - Direct memory access mode for multi ADC mode
pub type MDMA_R = crate::FieldReader<MDMA>;
impl MDMA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<MDMA> {
        match self.bits {
            0 => Some(MDMA::Disabled),
            2 => Some(MDMA::Bits12_10),
            3 => Some(MDMA::Bits8_6),
            _ => None,
        }
    }
    ///MDMA mode disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MDMA::Disabled
    }
    ///MDMA mode enabled for 12 and 10-bit resolution
    #[inline(always)]
    pub fn is_bits12_10(&self) -> bool {
        *self == MDMA::Bits12_10
    }
    ///MDMA mode enabled for 8 and 6-bit resolution
    #[inline(always)]
    pub fn is_bits8_6(&self) -> bool {
        *self == MDMA::Bits8_6
    }
}
///Field `MDMA` writer - Direct memory access mode for multi ADC mode
pub type MDMA_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MDMA>;
impl<'a, REG> MDMA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///MDMA mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MDMA::Disabled)
    }
    ///MDMA mode enabled for 12 and 10-bit resolution
    #[inline(always)]
    pub fn bits12_10(self) -> &'a mut crate::W<REG> {
        self.variant(MDMA::Bits12_10)
    }
    ///MDMA mode enabled for 8 and 6-bit resolution
    #[inline(always)]
    pub fn bits8_6(self) -> &'a mut crate::W<REG> {
        self.variant(MDMA::Bits8_6)
    }
}
/**ADC clock mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKMODE {
    ///0: Use Kernel Clock adc_ker_ck_input divided by PRESC. Asynchronous to AHB clock
    Asynchronous = 0,
    ///1: Use AHB clock rcc_hclk3. In this case rcc_hclk must equal sys_d1cpre_ck
    SyncDiv1 = 1,
    ///2: Use AHB clock rcc_hclk3 divided by 2
    SyncDiv2 = 2,
    ///3: Use AHB clock rcc_hclk3 divided by 4
    SyncDiv4 = 3,
}
impl From<CKMODE> for u8 {
    #[inline(always)]
    fn from(variant: CKMODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CKMODE {
    type Ux = u8;
}
impl crate::IsEnum for CKMODE {}
///Field `CKMODE` reader - ADC clock mode
pub type CKMODE_R = crate::FieldReader<CKMODE>;
impl CKMODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CKMODE {
        match self.bits {
            0 => CKMODE::Asynchronous,
            1 => CKMODE::SyncDiv1,
            2 => CKMODE::SyncDiv2,
            3 => CKMODE::SyncDiv4,
            _ => unreachable!(),
        }
    }
    ///Use Kernel Clock adc_ker_ck_input divided by PRESC. Asynchronous to AHB clock
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == CKMODE::Asynchronous
    }
    ///Use AHB clock rcc_hclk3. In this case rcc_hclk must equal sys_d1cpre_ck
    #[inline(always)]
    pub fn is_sync_div1(&self) -> bool {
        *self == CKMODE::SyncDiv1
    }
    ///Use AHB clock rcc_hclk3 divided by 2
    #[inline(always)]
    pub fn is_sync_div2(&self) -> bool {
        *self == CKMODE::SyncDiv2
    }
    ///Use AHB clock rcc_hclk3 divided by 4
    #[inline(always)]
    pub fn is_sync_div4(&self) -> bool {
        *self == CKMODE::SyncDiv4
    }
}
///Field `CKMODE` writer - ADC clock mode
pub type CKMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CKMODE, crate::Safe>;
impl<'a, REG> CKMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Use Kernel Clock adc_ker_ck_input divided by PRESC. Asynchronous to AHB clock
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut crate::W<REG> {
        self.variant(CKMODE::Asynchronous)
    }
    ///Use AHB clock rcc_hclk3. In this case rcc_hclk must equal sys_d1cpre_ck
    #[inline(always)]
    pub fn sync_div1(self) -> &'a mut crate::W<REG> {
        self.variant(CKMODE::SyncDiv1)
    }
    ///Use AHB clock rcc_hclk3 divided by 2
    #[inline(always)]
    pub fn sync_div2(self) -> &'a mut crate::W<REG> {
        self.variant(CKMODE::SyncDiv2)
    }
    ///Use AHB clock rcc_hclk3 divided by 4
    #[inline(always)]
    pub fn sync_div4(self) -> &'a mut crate::W<REG> {
        self.variant(CKMODE::SyncDiv4)
    }
}
/**VREFINT enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VREFEN {
    ///0: V_REFINT channel disabled
    Disabled = 0,
    ///1: V_REFINT channel enabled
    Enabled = 1,
}
impl From<VREFEN> for bool {
    #[inline(always)]
    fn from(variant: VREFEN) -> Self {
        variant as u8 != 0
    }
}
///Field `VREFEN` reader - VREFINT enable
pub type VREFEN_R = crate::BitReader<VREFEN>;
impl VREFEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VREFEN {
        match self.bits {
            false => VREFEN::Disabled,
            true => VREFEN::Enabled,
        }
    }
    ///V_REFINT channel disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VREFEN::Disabled
    }
    ///V_REFINT channel enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VREFEN::Enabled
    }
}
///Field `VREFEN` writer - VREFINT enable
pub type VREFEN_W<'a, REG> = crate::BitWriter<'a, REG, VREFEN>;
impl<'a, REG> VREFEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///V_REFINT channel disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(VREFEN::Disabled)
    }
    ///V_REFINT channel enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(VREFEN::Enabled)
    }
}
/**Temperature sensor enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSEN {
    ///0: Temperature sensor channel disabled
    Disabled = 0,
    ///1: Temperature sensor channel enabled
    Enabled = 1,
}
impl From<TSEN> for bool {
    #[inline(always)]
    fn from(variant: TSEN) -> Self {
        variant as u8 != 0
    }
}
///Field `TSEN` reader - Temperature sensor enable
pub type TSEN_R = crate::BitReader<TSEN>;
impl TSEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TSEN {
        match self.bits {
            false => TSEN::Disabled,
            true => TSEN::Enabled,
        }
    }
    ///Temperature sensor channel disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TSEN::Disabled
    }
    ///Temperature sensor channel enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TSEN::Enabled
    }
}
///Field `TSEN` writer - Temperature sensor enable
pub type TSEN_W<'a, REG> = crate::BitWriter<'a, REG, TSEN>;
impl<'a, REG> TSEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Temperature sensor channel disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TSEN::Disabled)
    }
    ///Temperature sensor channel enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TSEN::Enabled)
    }
}
/**VBAT enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBATEN {
    ///0: V_BAT channel disabled
    Disabled = 0,
    ///1: V_BAT channel enabled
    Enabled = 1,
}
impl From<VBATEN> for bool {
    #[inline(always)]
    fn from(variant: VBATEN) -> Self {
        variant as u8 != 0
    }
}
///Field `VBATEN` reader - VBAT enable
pub type VBATEN_R = crate::BitReader<VBATEN>;
impl VBATEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VBATEN {
        match self.bits {
            false => VBATEN::Disabled,
            true => VBATEN::Enabled,
        }
    }
    ///V_BAT channel disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VBATEN::Disabled
    }
    ///V_BAT channel enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VBATEN::Enabled
    }
}
///Field `VBATEN` writer - VBAT enable
pub type VBATEN_W<'a, REG> = crate::BitWriter<'a, REG, VBATEN>;
impl<'a, REG> VBATEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///V_BAT channel disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(VBATEN::Disabled)
    }
    ///V_BAT channel enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(VBATEN::Enabled)
    }
}
impl R {
    ///Bits 0:4 - Dual ADC mode selection
    #[inline(always)]
    pub fn dual(&self) -> DUAL_R {
        DUAL_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 8:11 - Delay between 2 sampling phases
    #[inline(always)]
    pub fn delay(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 13 - DMA configuration (for multi-ADC mode)
    #[inline(always)]
    pub fn dmacfg(&self) -> DMACFG_R {
        DMACFG_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 14:15 - Direct memory access mode for multi ADC mode
    #[inline(always)]
    pub fn mdma(&self) -> MDMA_R {
        MDMA_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:17 - ADC clock mode
    #[inline(always)]
    pub fn ckmode(&self) -> CKMODE_R {
        CKMODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 22 - VREFINT enable
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Temperature sensor enable
    #[inline(always)]
    pub fn tsen(&self) -> TSEN_R {
        TSEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - VBAT enable
    #[inline(always)]
    pub fn vbaten(&self) -> VBATEN_R {
        VBATEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR")
            .field("dual", &self.dual())
            .field("delay", &self.delay())
            .field("dmacfg", &self.dmacfg())
            .field("mdma", &self.mdma())
            .field("ckmode", &self.ckmode())
            .field("vrefen", &self.vrefen())
            .field("tsen", &self.tsen())
            .field("vbaten", &self.vbaten())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - Dual ADC mode selection
    #[inline(always)]
    pub fn dual(&mut self) -> DUAL_W<'_, CCRrs> {
        DUAL_W::new(self, 0)
    }
    ///Bits 8:11 - Delay between 2 sampling phases
    #[inline(always)]
    pub fn delay(&mut self) -> DELAY_W<'_, CCRrs> {
        DELAY_W::new(self, 8)
    }
    ///Bit 13 - DMA configuration (for multi-ADC mode)
    #[inline(always)]
    pub fn dmacfg(&mut self) -> DMACFG_W<'_, CCRrs> {
        DMACFG_W::new(self, 13)
    }
    ///Bits 14:15 - Direct memory access mode for multi ADC mode
    #[inline(always)]
    pub fn mdma(&mut self) -> MDMA_W<'_, CCRrs> {
        MDMA_W::new(self, 14)
    }
    ///Bits 16:17 - ADC clock mode
    #[inline(always)]
    pub fn ckmode(&mut self) -> CKMODE_W<'_, CCRrs> {
        CKMODE_W::new(self, 16)
    }
    ///Bit 22 - VREFINT enable
    #[inline(always)]
    pub fn vrefen(&mut self) -> VREFEN_W<'_, CCRrs> {
        VREFEN_W::new(self, 22)
    }
    ///Bit 23 - Temperature sensor enable
    #[inline(always)]
    pub fn tsen(&mut self) -> TSEN_W<'_, CCRrs> {
        TSEN_W::new(self, 23)
    }
    ///Bit 24 - VBAT enable
    #[inline(always)]
    pub fn vbaten(&mut self) -> VBATEN_W<'_, CCRrs> {
        VBATEN_W::new(self, 24)
    }
}
/**ADC common control register

You can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F302.html#ADC1_2:CCR)*/
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
