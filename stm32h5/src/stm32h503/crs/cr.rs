#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "SYNC event OK interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNCOKIE {
    #[doc = "0: Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt enabled"]
    Enabled = 1,
}
impl From<SYNCOKIE> for bool {
    #[inline(always)]
    fn from(variant: SYNCOKIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNCOKIE` reader - SYNC event OK interrupt enable"]
pub type SYNCOKIE_R = crate::BitReader<SYNCOKIE>;
impl SYNCOKIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SYNCOKIE {
        match self.bits {
            false => SYNCOKIE::Disabled,
            true => SYNCOKIE::Enabled,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYNCOKIE::Disabled
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SYNCOKIE::Enabled
    }
}
#[doc = "Field `SYNCOKIE` writer - SYNC event OK interrupt enable"]
pub type SYNCOKIE_W<'a, REG> = crate::BitWriter<'a, REG, SYNCOKIE>;
impl<'a, REG> SYNCOKIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCOKIE::Disabled)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCOKIE::Enabled)
    }
}
#[doc = "Field `SYNCWARNIE` reader - SYNC warning interrupt enable"]
pub use SYNCOKIE_R as SYNCWARNIE_R;
#[doc = "Field `ERRIE` reader - Synchronization or trimming error interrupt enable"]
pub use SYNCOKIE_R as ERRIE_R;
#[doc = "Field `ESYNCIE` reader - Expected SYNC interrupt enable"]
pub use SYNCOKIE_R as ESYNCIE_R;
#[doc = "Field `SYNCWARNIE` writer - SYNC warning interrupt enable"]
pub use SYNCOKIE_W as SYNCWARNIE_W;
#[doc = "Field `ERRIE` writer - Synchronization or trimming error interrupt enable"]
pub use SYNCOKIE_W as ERRIE_W;
#[doc = "Field `ESYNCIE` writer - Expected SYNC interrupt enable"]
pub use SYNCOKIE_W as ESYNCIE_W;
#[doc = "Frequency error counter enable This bit enables the oscillator clock for the frequency error counter. When this bit is set, the CRS_CFGR register is write-protected and cannot be modified.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEN {
    #[doc = "0: Frequency error counter disabled"]
    Disabled = 0,
    #[doc = "1: Frequency error counter enabled"]
    Enabled = 1,
}
impl From<CEN> for bool {
    #[inline(always)]
    fn from(variant: CEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEN` reader - Frequency error counter enable This bit enables the oscillator clock for the frequency error counter. When this bit is set, the CRS_CFGR register is write-protected and cannot be modified."]
pub type CEN_R = crate::BitReader<CEN>;
impl CEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CEN {
        match self.bits {
            false => CEN::Disabled,
            true => CEN::Enabled,
        }
    }
    #[doc = "Frequency error counter disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CEN::Disabled
    }
    #[doc = "Frequency error counter enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CEN::Enabled
    }
}
#[doc = "Field `CEN` writer - Frequency error counter enable This bit enables the oscillator clock for the frequency error counter. When this bit is set, the CRS_CFGR register is write-protected and cannot be modified."]
pub type CEN_W<'a, REG> = crate::BitWriter<'a, REG, CEN>;
impl<'a, REG> CEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Frequency error counter disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CEN::Disabled)
    }
    #[doc = "Frequency error counter enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CEN::Enabled)
    }
}
#[doc = "Automatic trimming enable This bit enables the automatic hardware adjustment of TRIM bits according to the measured frequency error between two SYNC events. If this bit is set, the TRIM bits are read-only. The TRIM value can be adjusted by hardware by one or two steps at a time, depending on the measured frequency error value. Refer to Section 10.5.3 for more details.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTOTRIMEN {
    #[doc = "0: Automatic trimming disabled"]
    Disabled = 0,
    #[doc = "1: Automatic trimming enabled"]
    Enabled = 1,
}
impl From<AUTOTRIMEN> for bool {
    #[inline(always)]
    fn from(variant: AUTOTRIMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTOTRIMEN` reader - Automatic trimming enable This bit enables the automatic hardware adjustment of TRIM bits according to the measured frequency error between two SYNC events. If this bit is set, the TRIM bits are read-only. The TRIM value can be adjusted by hardware by one or two steps at a time, depending on the measured frequency error value. Refer to Section 10.5.3 for more details."]
pub type AUTOTRIMEN_R = crate::BitReader<AUTOTRIMEN>;
impl AUTOTRIMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AUTOTRIMEN {
        match self.bits {
            false => AUTOTRIMEN::Disabled,
            true => AUTOTRIMEN::Enabled,
        }
    }
    #[doc = "Automatic trimming disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AUTOTRIMEN::Disabled
    }
    #[doc = "Automatic trimming enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AUTOTRIMEN::Enabled
    }
}
#[doc = "Field `AUTOTRIMEN` writer - Automatic trimming enable This bit enables the automatic hardware adjustment of TRIM bits according to the measured frequency error between two SYNC events. If this bit is set, the TRIM bits are read-only. The TRIM value can be adjusted by hardware by one or two steps at a time, depending on the measured frequency error value. Refer to Section 10.5.3 for more details."]
pub type AUTOTRIMEN_W<'a, REG> = crate::BitWriter<'a, REG, AUTOTRIMEN>;
impl<'a, REG> AUTOTRIMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic trimming disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AUTOTRIMEN::Disabled)
    }
    #[doc = "Automatic trimming enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AUTOTRIMEN::Enabled)
    }
}
#[doc = "Generate software SYNC event This bit is set by software in order to generate a software SYNC event. It is automatically cleared by hardware.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWSYNC {
    #[doc = "1: A software sync is generated"]
    Sync = 1,
}
impl From<SWSYNC> for bool {
    #[inline(always)]
    fn from(variant: SWSYNC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWSYNC` reader - Generate software SYNC event This bit is set by software in order to generate a software SYNC event. It is automatically cleared by hardware."]
pub type SWSYNC_R = crate::BitReader<SWSYNC>;
impl SWSYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SWSYNC> {
        match self.bits {
            true => Some(SWSYNC::Sync),
            _ => None,
        }
    }
    #[doc = "A software sync is generated"]
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        *self == SWSYNC::Sync
    }
}
#[doc = "Field `SWSYNC` writer - Generate software SYNC event This bit is set by software in order to generate a software SYNC event. It is automatically cleared by hardware."]
pub type SWSYNC_W<'a, REG> = crate::BitWriter<'a, REG, SWSYNC>;
impl<'a, REG> SWSYNC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A software sync is generated"]
    #[inline(always)]
    pub fn sync(self) -> &'a mut crate::W<REG> {
        self.variant(SWSYNC::Sync)
    }
}
#[doc = "Field `TRIM` reader - HSI48 oscillator smooth trimming These bits provide a user-programmable trimming value to the HSI48 oscillator. They can be programmed to adjust to variations in voltage and temperature that influence the frequency of the HSI48 oscillator. The default value is 32, which corresponds to the middle of the trimming interval. The trimming step is specified in the product datasheet. A higher TRIM value corresponds to a higher output frequency. When the AUTOTRIMEN bit is set, this field is controlled by hardware and is read-only."]
pub type TRIM_R = crate::FieldReader;
#[doc = "Field `TRIM` writer - HSI48 oscillator smooth trimming These bits provide a user-programmable trimming value to the HSI48 oscillator. They can be programmed to adjust to variations in voltage and temperature that influence the frequency of the HSI48 oscillator. The default value is 32, which corresponds to the middle of the trimming interval. The trimming step is specified in the product datasheet. A higher TRIM value corresponds to a higher output frequency. When the AUTOTRIMEN bit is set, this field is controlled by hardware and is read-only."]
pub type TRIM_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - SYNC event OK interrupt enable"]
    #[inline(always)]
    pub fn syncokie(&self) -> SYNCOKIE_R {
        SYNCOKIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SYNC warning interrupt enable"]
    #[inline(always)]
    pub fn syncwarnie(&self) -> SYNCWARNIE_R {
        SYNCWARNIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Synchronization or trimming error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Expected SYNC interrupt enable"]
    #[inline(always)]
    pub fn esyncie(&self) -> ESYNCIE_R {
        ESYNCIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Frequency error counter enable This bit enables the oscillator clock for the frequency error counter. When this bit is set, the CRS_CFGR register is write-protected and cannot be modified."]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Automatic trimming enable This bit enables the automatic hardware adjustment of TRIM bits according to the measured frequency error between two SYNC events. If this bit is set, the TRIM bits are read-only. The TRIM value can be adjusted by hardware by one or two steps at a time, depending on the measured frequency error value. Refer to Section 10.5.3 for more details."]
    #[inline(always)]
    pub fn autotrimen(&self) -> AUTOTRIMEN_R {
        AUTOTRIMEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Generate software SYNC event This bit is set by software in order to generate a software SYNC event. It is automatically cleared by hardware."]
    #[inline(always)]
    pub fn swsync(&self) -> SWSYNC_R {
        SWSYNC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13 - HSI48 oscillator smooth trimming These bits provide a user-programmable trimming value to the HSI48 oscillator. They can be programmed to adjust to variations in voltage and temperature that influence the frequency of the HSI48 oscillator. The default value is 32, which corresponds to the middle of the trimming interval. The trimming step is specified in the product datasheet. A higher TRIM value corresponds to a higher output frequency. When the AUTOTRIMEN bit is set, this field is controlled by hardware and is read-only."]
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - SYNC event OK interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn syncokie(&mut self) -> SYNCOKIE_W<CRrs> {
        SYNCOKIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - SYNC warning interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn syncwarnie(&mut self) -> SYNCWARNIE_W<CRrs> {
        SYNCWARNIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Synchronization or trimming error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<CRrs> {
        ERRIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Expected SYNC interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn esyncie(&mut self) -> ESYNCIE_W<CRrs> {
        ESYNCIE_W::new(self, 3)
    }
    #[doc = "Bit 5 - Frequency error counter enable This bit enables the oscillator clock for the frequency error counter. When this bit is set, the CRS_CFGR register is write-protected and cannot be modified."]
    #[inline(always)]
    #[must_use]
    pub fn cen(&mut self) -> CEN_W<CRrs> {
        CEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Automatic trimming enable This bit enables the automatic hardware adjustment of TRIM bits according to the measured frequency error between two SYNC events. If this bit is set, the TRIM bits are read-only. The TRIM value can be adjusted by hardware by one or two steps at a time, depending on the measured frequency error value. Refer to Section 10.5.3 for more details."]
    #[inline(always)]
    #[must_use]
    pub fn autotrimen(&mut self) -> AUTOTRIMEN_W<CRrs> {
        AUTOTRIMEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Generate software SYNC event This bit is set by software in order to generate a software SYNC event. It is automatically cleared by hardware."]
    #[inline(always)]
    #[must_use]
    pub fn swsync(&mut self) -> SWSYNC_W<CRrs> {
        SWSYNC_W::new(self, 7)
    }
    #[doc = "Bits 8:13 - HSI48 oscillator smooth trimming These bits provide a user-programmable trimming value to the HSI48 oscillator. They can be programmed to adjust to variations in voltage and temperature that influence the frequency of the HSI48 oscillator. The default value is 32, which corresponds to the middle of the trimming interval. The trimming step is specified in the product datasheet. A higher TRIM value corresponds to a higher output frequency. When the AUTOTRIMEN bit is set, this field is controlled by hardware and is read-only."]
    #[inline(always)]
    #[must_use]
    pub fn trim(&mut self) -> TRIM_W<CRrs> {
        TRIM_W::new(self, 8)
    }
}
#[doc = "CRS control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CRrs {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0x2000"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x2000;
}
