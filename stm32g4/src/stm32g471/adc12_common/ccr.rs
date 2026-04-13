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
///Field `DMACFG` reader - DMA configuration (for multi-ADC mode)
pub type DMACFG_R = crate::BitReader;
///Field `DMACFG` writer - DMA configuration (for multi-ADC mode)
pub type DMACFG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MDMA` reader - Direct memory access mode for multi ADC mode
pub type MDMA_R = crate::FieldReader;
///Field `MDMA` writer - Direct memory access mode for multi ADC mode
pub type MDMA_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
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
///Field `PRESC` reader - ADC prescaler
pub type PRESC_R = crate::FieldReader;
///Field `PRESC` writer - ADC prescaler
pub type PRESC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
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
/**VTS selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VSENSESEL {
    ///0: Temperature sensor channel disabled
    Disabled = 0,
    ///1: Temperature sensor channel enabled
    Enabled = 1,
}
impl From<VSENSESEL> for bool {
    #[inline(always)]
    fn from(variant: VSENSESEL) -> Self {
        variant as u8 != 0
    }
}
///Field `VSENSESEL` reader - VTS selection
pub type VSENSESEL_R = crate::BitReader<VSENSESEL>;
impl VSENSESEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VSENSESEL {
        match self.bits {
            false => VSENSESEL::Disabled,
            true => VSENSESEL::Enabled,
        }
    }
    ///Temperature sensor channel disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VSENSESEL::Disabled
    }
    ///Temperature sensor channel enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VSENSESEL::Enabled
    }
}
///Field `VSENSESEL` writer - VTS selection
pub type VSENSESEL_W<'a, REG> = crate::BitWriter<'a, REG, VSENSESEL>;
impl<'a, REG> VSENSESEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Temperature sensor channel disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(VSENSESEL::Disabled)
    }
    ///Temperature sensor channel enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(VSENSESEL::Enabled)
    }
}
/**VBAT selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBATSEL {
    ///0: V_BAT channel disabled
    Disabled = 0,
    ///1: V_BAT channel enabled
    Enabled = 1,
}
impl From<VBATSEL> for bool {
    #[inline(always)]
    fn from(variant: VBATSEL) -> Self {
        variant as u8 != 0
    }
}
///Field `VBATSEL` reader - VBAT selection
pub type VBATSEL_R = crate::BitReader<VBATSEL>;
impl VBATSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VBATSEL {
        match self.bits {
            false => VBATSEL::Disabled,
            true => VBATSEL::Enabled,
        }
    }
    ///V_BAT channel disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VBATSEL::Disabled
    }
    ///V_BAT channel enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VBATSEL::Enabled
    }
}
///Field `VBATSEL` writer - VBAT selection
pub type VBATSEL_W<'a, REG> = crate::BitWriter<'a, REG, VBATSEL>;
impl<'a, REG> VBATSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///V_BAT channel disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(VBATSEL::Disabled)
    }
    ///V_BAT channel enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(VBATSEL::Enabled)
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
    ///Bits 18:21 - ADC prescaler
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    ///Bit 22 - VREFINT enable
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - VTS selection
    #[inline(always)]
    pub fn vsensesel(&self) -> VSENSESEL_R {
        VSENSESEL_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - VBAT selection
    #[inline(always)]
    pub fn vbatsel(&self) -> VBATSEL_R {
        VBATSEL_R::new(((self.bits >> 24) & 1) != 0)
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
            .field("vsensesel", &self.vsensesel())
            .field("vbatsel", &self.vbatsel())
            .field("presc", &self.presc())
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
    ///Bits 18:21 - ADC prescaler
    #[inline(always)]
    pub fn presc(&mut self) -> PRESC_W<'_, CCRrs> {
        PRESC_W::new(self, 18)
    }
    ///Bit 22 - VREFINT enable
    #[inline(always)]
    pub fn vrefen(&mut self) -> VREFEN_W<'_, CCRrs> {
        VREFEN_W::new(self, 22)
    }
    ///Bit 23 - VTS selection
    #[inline(always)]
    pub fn vsensesel(&mut self) -> VSENSESEL_W<'_, CCRrs> {
        VSENSESEL_W::new(self, 23)
    }
    ///Bit 24 - VBAT selection
    #[inline(always)]
    pub fn vbatsel(&mut self) -> VBATSEL_W<'_, CCRrs> {
        VBATSEL_W::new(self, 24)
    }
}
/**ADC common control register

You can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G471.html#ADC12_Common:CCR)*/
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
