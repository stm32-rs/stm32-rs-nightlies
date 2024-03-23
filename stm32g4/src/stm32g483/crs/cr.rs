#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `SYNCOKIE` reader - SYNC event OK interrupt enable"]
pub type SYNCOKIE_R = crate::BitReader;
#[doc = "Field `SYNCOKIE` writer - SYNC event OK interrupt enable"]
pub type SYNCOKIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCWARNIE` reader - SYNC warning interrupt enable"]
pub type SYNCWARNIE_R = crate::BitReader;
#[doc = "Field `SYNCWARNIE` writer - SYNC warning interrupt enable"]
pub type SYNCWARNIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIE` reader - Synchronization or trimming error interrupt enable"]
pub type ERRIE_R = crate::BitReader;
#[doc = "Field `ERRIE` writer - Synchronization or trimming error interrupt enable"]
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESYNCIE` reader - Expected SYNC interrupt enable"]
pub type ESYNCIE_R = crate::BitReader;
#[doc = "Field `ESYNCIE` writer - Expected SYNC interrupt enable"]
pub type ESYNCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEN` reader - Frequency error counter enable This bit enables the oscillator clock for the frequency error counter. When this bit is set, the CRS_CFGR register is write-protected and cannot be modified."]
pub type CEN_R = crate::BitReader;
#[doc = "Field `CEN` writer - Frequency error counter enable This bit enables the oscillator clock for the frequency error counter. When this bit is set, the CRS_CFGR register is write-protected and cannot be modified."]
pub type CEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOTRIMEN` reader - Automatic trimming enable This bit enables the automatic hardware adjustment of TRIM bits according to the measured frequency error between two SYNC events. If this bit is set, the TRIM bits are read-only. The TRIM value can be adjusted by hardware by one or two steps at a time, depending on the measured frequency error value. Refer to Section7.3.4: Frequency error evaluation and automatic trimming for more details."]
pub type AUTOTRIMEN_R = crate::BitReader;
#[doc = "Field `AUTOTRIMEN` writer - Automatic trimming enable This bit enables the automatic hardware adjustment of TRIM bits according to the measured frequency error between two SYNC events. If this bit is set, the TRIM bits are read-only. The TRIM value can be adjusted by hardware by one or two steps at a time, depending on the measured frequency error value. Refer to Section7.3.4: Frequency error evaluation and automatic trimming for more details."]
pub type AUTOTRIMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWSYNC` reader - Generate software SYNC event This bit is set by software in order to generate a software SYNC event. It is automatically cleared by hardware."]
pub type SWSYNC_R = crate::BitReader;
#[doc = "Field `SWSYNC` writer - Generate software SYNC event This bit is set by software in order to generate a software SYNC event. It is automatically cleared by hardware."]
pub type SWSYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIM` reader - HSI48 oscillator smooth trimming These bits provide a user-programmable trimming value to the HSI48 oscillator. They can be programmed to adjust to variations in voltage and temperature that influence the frequency of the HSI48. The default value is 32, which corresponds to the middle of the trimming interval. The trimming step is around 67 kHz between two consecutive TRIM steps. A higher TRIM value corresponds to a higher output frequency. When the AUTOTRIMEN bit is set, this field is controlled by hardware and is read-only."]
pub type TRIM_R = crate::FieldReader;
#[doc = "Field `TRIM` writer - HSI48 oscillator smooth trimming These bits provide a user-programmable trimming value to the HSI48 oscillator. They can be programmed to adjust to variations in voltage and temperature that influence the frequency of the HSI48. The default value is 32, which corresponds to the middle of the trimming interval. The trimming step is around 67 kHz between two consecutive TRIM steps. A higher TRIM value corresponds to a higher output frequency. When the AUTOTRIMEN bit is set, this field is controlled by hardware and is read-only."]
pub type TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
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
    #[doc = "Bit 6 - Automatic trimming enable This bit enables the automatic hardware adjustment of TRIM bits according to the measured frequency error between two SYNC events. If this bit is set, the TRIM bits are read-only. The TRIM value can be adjusted by hardware by one or two steps at a time, depending on the measured frequency error value. Refer to Section7.3.4: Frequency error evaluation and automatic trimming for more details."]
    #[inline(always)]
    pub fn autotrimen(&self) -> AUTOTRIMEN_R {
        AUTOTRIMEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Generate software SYNC event This bit is set by software in order to generate a software SYNC event. It is automatically cleared by hardware."]
    #[inline(always)]
    pub fn swsync(&self) -> SWSYNC_R {
        SWSYNC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - HSI48 oscillator smooth trimming These bits provide a user-programmable trimming value to the HSI48 oscillator. They can be programmed to adjust to variations in voltage and temperature that influence the frequency of the HSI48. The default value is 32, which corresponds to the middle of the trimming interval. The trimming step is around 67 kHz between two consecutive TRIM steps. A higher TRIM value corresponds to a higher output frequency. When the AUTOTRIMEN bit is set, this field is controlled by hardware and is read-only."]
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new(((self.bits >> 8) & 0x7f) as u8)
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
    #[doc = "Bit 6 - Automatic trimming enable This bit enables the automatic hardware adjustment of TRIM bits according to the measured frequency error between two SYNC events. If this bit is set, the TRIM bits are read-only. The TRIM value can be adjusted by hardware by one or two steps at a time, depending on the measured frequency error value. Refer to Section7.3.4: Frequency error evaluation and automatic trimming for more details."]
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
    #[doc = "Bits 8:14 - HSI48 oscillator smooth trimming These bits provide a user-programmable trimming value to the HSI48 oscillator. They can be programmed to adjust to variations in voltage and temperature that influence the frequency of the HSI48. The default value is 32, which corresponds to the middle of the trimming interval. The trimming step is around 67 kHz between two consecutive TRIM steps. A higher TRIM value corresponds to a higher output frequency. When the AUTOTRIMEN bit is set, this field is controlled by hardware and is read-only."]
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
#[doc = "`reset()` method sets CR to value 0x4000"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x4000;
}
