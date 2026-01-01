///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
/**SYNC event OK interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNCOKIE {
    ///0: Interrupt disabled
    Disabled = 0,
    ///1: Interrupt enabled
    Enabled = 1,
}
impl From<SYNCOKIE> for bool {
    #[inline(always)]
    fn from(variant: SYNCOKIE) -> Self {
        variant as u8 != 0
    }
}
///Field `SYNCOKIE` reader - SYNC event OK interrupt enable
pub type SYNCOKIE_R = crate::BitReader<SYNCOKIE>;
impl SYNCOKIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SYNCOKIE {
        match self.bits {
            false => SYNCOKIE::Disabled,
            true => SYNCOKIE::Enabled,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYNCOKIE::Disabled
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SYNCOKIE::Enabled
    }
}
///Field `SYNCOKIE` writer - SYNC event OK interrupt enable
pub type SYNCOKIE_W<'a, REG> = crate::BitWriter<'a, REG, SYNCOKIE>;
impl<'a, REG> SYNCOKIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCOKIE::Disabled)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCOKIE::Enabled)
    }
}
///Field `SYNCWARNIE` reader - SYNC warning interrupt enable
pub use SYNCOKIE_R as SYNCWARNIE_R;
///Field `ERRIE` reader - Synchronization or trimming error interrupt enable
pub use SYNCOKIE_R as ERRIE_R;
///Field `ESYNCIE` reader - Expected SYNC interrupt enable
pub use SYNCOKIE_R as ESYNCIE_R;
///Field `SYNCWARNIE` writer - SYNC warning interrupt enable
pub use SYNCOKIE_W as SYNCWARNIE_W;
///Field `ERRIE` writer - Synchronization or trimming error interrupt enable
pub use SYNCOKIE_W as ERRIE_W;
///Field `ESYNCIE` writer - Expected SYNC interrupt enable
pub use SYNCOKIE_W as ESYNCIE_W;
/**Frequency error counter enable This bit enables the oscillator clock for the frequency error counter. When this bit is set, the CRS_CFGR register is write-protected and cannot be modified.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEN {
    ///0: Frequency error counter disabled
    Disabled = 0,
    ///1: Frequency error counter enabled
    Enabled = 1,
}
impl From<CEN> for bool {
    #[inline(always)]
    fn from(variant: CEN) -> Self {
        variant as u8 != 0
    }
}
///Field `CEN` reader - Frequency error counter enable This bit enables the oscillator clock for the frequency error counter. When this bit is set, the CRS_CFGR register is write-protected and cannot be modified.
pub type CEN_R = crate::BitReader<CEN>;
impl CEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CEN {
        match self.bits {
            false => CEN::Disabled,
            true => CEN::Enabled,
        }
    }
    ///Frequency error counter disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CEN::Disabled
    }
    ///Frequency error counter enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CEN::Enabled
    }
}
///Field `CEN` writer - Frequency error counter enable This bit enables the oscillator clock for the frequency error counter. When this bit is set, the CRS_CFGR register is write-protected and cannot be modified.
pub type CEN_W<'a, REG> = crate::BitWriter<'a, REG, CEN>;
impl<'a, REG> CEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Frequency error counter disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CEN::Disabled)
    }
    ///Frequency error counter enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CEN::Enabled)
    }
}
/**Automatic trimming enable This bit enables the automatic hardware adjustment of TRIM bits according to the measured frequency error between two SYNC events. If this bit is set, the TRIM bits are read-only. The TRIM value can be adjusted by hardware by one or two steps at a time, depending on the measured frequency error value. Refer to Section7.3.4: Frequency error evaluation and automatic trimming for more details.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTOTRIMEN {
    ///0: Automatic trimming disabled
    Disabled = 0,
    ///1: Automatic trimming enabled
    Enabled = 1,
}
impl From<AUTOTRIMEN> for bool {
    #[inline(always)]
    fn from(variant: AUTOTRIMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `AUTOTRIMEN` reader - Automatic trimming enable This bit enables the automatic hardware adjustment of TRIM bits according to the measured frequency error between two SYNC events. If this bit is set, the TRIM bits are read-only. The TRIM value can be adjusted by hardware by one or two steps at a time, depending on the measured frequency error value. Refer to Section7.3.4: Frequency error evaluation and automatic trimming for more details.
pub type AUTOTRIMEN_R = crate::BitReader<AUTOTRIMEN>;
impl AUTOTRIMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AUTOTRIMEN {
        match self.bits {
            false => AUTOTRIMEN::Disabled,
            true => AUTOTRIMEN::Enabled,
        }
    }
    ///Automatic trimming disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AUTOTRIMEN::Disabled
    }
    ///Automatic trimming enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AUTOTRIMEN::Enabled
    }
}
///Field `AUTOTRIMEN` writer - Automatic trimming enable This bit enables the automatic hardware adjustment of TRIM bits according to the measured frequency error between two SYNC events. If this bit is set, the TRIM bits are read-only. The TRIM value can be adjusted by hardware by one or two steps at a time, depending on the measured frequency error value. Refer to Section7.3.4: Frequency error evaluation and automatic trimming for more details.
pub type AUTOTRIMEN_W<'a, REG> = crate::BitWriter<'a, REG, AUTOTRIMEN>;
impl<'a, REG> AUTOTRIMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Automatic trimming disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AUTOTRIMEN::Disabled)
    }
    ///Automatic trimming enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AUTOTRIMEN::Enabled)
    }
}
/**Generate software SYNC event This bit is set by software in order to generate a software SYNC event. It is automatically cleared by hardware.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWSYNC {
    ///1: A software sync is generated
    Sync = 1,
}
impl From<SWSYNC> for bool {
    #[inline(always)]
    fn from(variant: SWSYNC) -> Self {
        variant as u8 != 0
    }
}
///Field `SWSYNC` reader - Generate software SYNC event This bit is set by software in order to generate a software SYNC event. It is automatically cleared by hardware.
pub type SWSYNC_R = crate::BitReader<SWSYNC>;
impl SWSYNC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SWSYNC> {
        match self.bits {
            true => Some(SWSYNC::Sync),
            _ => None,
        }
    }
    ///A software sync is generated
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        *self == SWSYNC::Sync
    }
}
///Field `TRIM` reader - HSI48 oscillator smooth trimming These bits provide a user-programmable trimming value to the HSI48 oscillator. They can be programmed to adjust to variations in voltage and temperature that influence the frequency of the HSI48. The default value is 32, which corresponds to the middle of the trimming interval. The trimming step is around 67 kHz between two consecutive TRIM steps. A higher TRIM value corresponds to a higher output frequency. When the AUTOTRIMEN bit is set, this field is controlled by hardware and is read-only.
pub type TRIM_R = crate::FieldReader;
///Field `TRIM` writer - HSI48 oscillator smooth trimming These bits provide a user-programmable trimming value to the HSI48 oscillator. They can be programmed to adjust to variations in voltage and temperature that influence the frequency of the HSI48. The default value is 32, which corresponds to the middle of the trimming interval. The trimming step is around 67 kHz between two consecutive TRIM steps. A higher TRIM value corresponds to a higher output frequency. When the AUTOTRIMEN bit is set, this field is controlled by hardware and is read-only.
pub type TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 6, u8, crate::Safe>;
impl R {
    ///Bit 0 - SYNC event OK interrupt enable
    #[inline(always)]
    pub fn syncokie(&self) -> SYNCOKIE_R {
        SYNCOKIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SYNC warning interrupt enable
    #[inline(always)]
    pub fn syncwarnie(&self) -> SYNCWARNIE_R {
        SYNCWARNIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Synchronization or trimming error interrupt enable
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Expected SYNC interrupt enable
    #[inline(always)]
    pub fn esyncie(&self) -> ESYNCIE_R {
        ESYNCIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - Frequency error counter enable This bit enables the oscillator clock for the frequency error counter. When this bit is set, the CRS_CFGR register is write-protected and cannot be modified.
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Automatic trimming enable This bit enables the automatic hardware adjustment of TRIM bits according to the measured frequency error between two SYNC events. If this bit is set, the TRIM bits are read-only. The TRIM value can be adjusted by hardware by one or two steps at a time, depending on the measured frequency error value. Refer to Section7.3.4: Frequency error evaluation and automatic trimming for more details.
    #[inline(always)]
    pub fn autotrimen(&self) -> AUTOTRIMEN_R {
        AUTOTRIMEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Generate software SYNC event This bit is set by software in order to generate a software SYNC event. It is automatically cleared by hardware.
    #[inline(always)]
    pub fn swsync(&self) -> SWSYNC_R {
        SWSYNC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:13 - HSI48 oscillator smooth trimming These bits provide a user-programmable trimming value to the HSI48 oscillator. They can be programmed to adjust to variations in voltage and temperature that influence the frequency of the HSI48. The default value is 32, which corresponds to the middle of the trimming interval. The trimming step is around 67 kHz between two consecutive TRIM steps. A higher TRIM value corresponds to a higher output frequency. When the AUTOTRIMEN bit is set, this field is controlled by hardware and is read-only.
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("syncokie", &self.syncokie())
            .field("syncwarnie", &self.syncwarnie())
            .field("errie", &self.errie())
            .field("esyncie", &self.esyncie())
            .field("cen", &self.cen())
            .field("autotrimen", &self.autotrimen())
            .field("swsync", &self.swsync())
            .field("trim", &self.trim())
            .finish()
    }
}
impl W {
    ///Bit 0 - SYNC event OK interrupt enable
    #[inline(always)]
    pub fn syncokie(&mut self) -> SYNCOKIE_W<'_, CRrs> {
        SYNCOKIE_W::new(self, 0)
    }
    ///Bit 1 - SYNC warning interrupt enable
    #[inline(always)]
    pub fn syncwarnie(&mut self) -> SYNCWARNIE_W<'_, CRrs> {
        SYNCWARNIE_W::new(self, 1)
    }
    ///Bit 2 - Synchronization or trimming error interrupt enable
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W<'_, CRrs> {
        ERRIE_W::new(self, 2)
    }
    ///Bit 3 - Expected SYNC interrupt enable
    #[inline(always)]
    pub fn esyncie(&mut self) -> ESYNCIE_W<'_, CRrs> {
        ESYNCIE_W::new(self, 3)
    }
    ///Bit 5 - Frequency error counter enable This bit enables the oscillator clock for the frequency error counter. When this bit is set, the CRS_CFGR register is write-protected and cannot be modified.
    #[inline(always)]
    pub fn cen(&mut self) -> CEN_W<'_, CRrs> {
        CEN_W::new(self, 5)
    }
    ///Bit 6 - Automatic trimming enable This bit enables the automatic hardware adjustment of TRIM bits according to the measured frequency error between two SYNC events. If this bit is set, the TRIM bits are read-only. The TRIM value can be adjusted by hardware by one or two steps at a time, depending on the measured frequency error value. Refer to Section7.3.4: Frequency error evaluation and automatic trimming for more details.
    #[inline(always)]
    pub fn autotrimen(&mut self) -> AUTOTRIMEN_W<'_, CRrs> {
        AUTOTRIMEN_W::new(self, 6)
    }
    ///Bits 8:13 - HSI48 oscillator smooth trimming These bits provide a user-programmable trimming value to the HSI48 oscillator. They can be programmed to adjust to variations in voltage and temperature that influence the frequency of the HSI48. The default value is 32, which corresponds to the middle of the trimming interval. The trimming step is around 67 kHz between two consecutive TRIM steps. A higher TRIM value corresponds to a higher output frequency. When the AUTOTRIMEN bit is set, this field is controlled by hardware and is read-only.
    #[inline(always)]
    pub fn trim(&mut self) -> TRIM_W<'_, CRrs> {
        TRIM_W::new(self, 8)
    }
}
/**CRS control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H742.html#CRS:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0x2000
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x2000;
}
