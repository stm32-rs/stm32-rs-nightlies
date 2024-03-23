#[doc = "Register `CFGR2` reader"]
pub type R = crate::R<CFGR2rs>;
#[doc = "Register `CFGR2` writer"]
pub type W = crate::W<CFGR2rs>;
#[doc = "Field `OVSE` reader - Oversampler Enable This bit is set and cleared by software. Note: Software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type OVSE_R = crate::BitReader;
#[doc = "Field `OVSE` writer - Oversampler Enable This bit is set and cleared by software. Note: Software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type OVSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVSR` reader - Oversampling ratio This bit filed defines the number of oversampling ratio. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type OVSR_R = crate::FieldReader;
#[doc = "Field `OVSR` writer - Oversampling ratio This bit filed defines the number of oversampling ratio. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type OVSR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OVSS` reader - Oversampling shift This bit is set and cleared by software. Others: Reserved Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type OVSS_R = crate::FieldReader;
#[doc = "Field `OVSS` writer - Oversampling shift This bit is set and cleared by software. Others: Reserved Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type OVSS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TOVS` reader - Triggered Oversampling This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type TOVS_R = crate::BitReader;
#[doc = "Field `TOVS` writer - Triggered Oversampling This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type TOVS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFTRIG` reader - Low frequency trigger mode enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type LFTRIG_R = crate::BitReader;
#[doc = "Field `LFTRIG` writer - Low frequency trigger mode enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type LFTRIG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKMODE` reader - ADC clock mode These bits are set and cleared by software to define how the analog ADC is clocked: In all synchronous clock modes, there is no jitter in the delay from a timer trigger to the start of a conversion. Note: The software is allowed to write these bits only when the ADC is disabled (ADCAL = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
pub type CKMODE_R = crate::FieldReader;
#[doc = "Field `CKMODE` writer - ADC clock mode These bits are set and cleared by software to define how the analog ADC is clocked: In all synchronous clock modes, there is no jitter in the delay from a timer trigger to the start of a conversion. Note: The software is allowed to write these bits only when the ADC is disabled (ADCAL = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
pub type CKMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Oversampler Enable This bit is set and cleared by software. Note: Software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn ovse(&self) -> OVSE_R {
        OVSE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:4 - Oversampling ratio This bit filed defines the number of oversampling ratio. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn ovsr(&self) -> OVSR_R {
        OVSR_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:8 - Oversampling shift This bit is set and cleared by software. Others: Reserved Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn ovss(&self) -> OVSS_R {
        OVSS_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - Triggered Oversampling This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn tovs(&self) -> TOVS_R {
        TOVS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 29 - Low frequency trigger mode enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn lftrig(&self) -> LFTRIG_R {
        LFTRIG_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - ADC clock mode These bits are set and cleared by software to define how the analog ADC is clocked: In all synchronous clock modes, there is no jitter in the delay from a timer trigger to the start of a conversion. Note: The software is allowed to write these bits only when the ADC is disabled (ADCAL = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
    #[inline(always)]
    pub fn ckmode(&self) -> CKMODE_R {
        CKMODE_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Oversampler Enable This bit is set and cleared by software. Note: Software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn ovse(&mut self) -> OVSE_W<CFGR2rs> {
        OVSE_W::new(self, 0)
    }
    #[doc = "Bits 2:4 - Oversampling ratio This bit filed defines the number of oversampling ratio. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn ovsr(&mut self) -> OVSR_W<CFGR2rs> {
        OVSR_W::new(self, 2)
    }
    #[doc = "Bits 5:8 - Oversampling shift This bit is set and cleared by software. Others: Reserved Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn ovss(&mut self) -> OVSS_W<CFGR2rs> {
        OVSS_W::new(self, 5)
    }
    #[doc = "Bit 9 - Triggered Oversampling This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn tovs(&mut self) -> TOVS_W<CFGR2rs> {
        TOVS_W::new(self, 9)
    }
    #[doc = "Bit 29 - Low frequency trigger mode enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn lftrig(&mut self) -> LFTRIG_W<CFGR2rs> {
        LFTRIG_W::new(self, 29)
    }
    #[doc = "Bits 30:31 - ADC clock mode These bits are set and cleared by software to define how the analog ADC is clocked: In all synchronous clock modes, there is no jitter in the delay from a timer trigger to the start of a conversion. Note: The software is allowed to write these bits only when the ADC is disabled (ADCAL = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn ckmode(&mut self) -> CKMODE_W<CFGR2rs> {
        CKMODE_W::new(self, 30)
    }
}
#[doc = "ADC configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR2rs;
impl crate::RegisterSpec for CFGR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr2::R`](R) reader structure"]
impl crate::Readable for CFGR2rs {}
#[doc = "`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure"]
impl crate::Writable for CFGR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR2 to value 0"]
impl crate::Resettable for CFGR2rs {
    const RESET_VALUE: u32 = 0;
}
