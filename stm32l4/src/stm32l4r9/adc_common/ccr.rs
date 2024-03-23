#[doc = "Register `CCR` reader"]
pub type R = crate::R<CCRrs>;
#[doc = "Register `CCR` writer"]
pub type W = crate::W<CCRrs>;
#[doc = "Field `DUAL` reader - Dual ADC mode selection"]
pub type DUAL_R = crate::FieldReader;
#[doc = "Field `DUAL` writer - Dual ADC mode selection"]
pub type DUAL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DELAY` reader - Delay between 2 sampling phases"]
pub type DELAY_R = crate::FieldReader;
#[doc = "Field `DELAY` writer - Delay between 2 sampling phases"]
pub type DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DMACFG` reader - DMA configuration (for multi-ADC mode)"]
pub type DMACFG_R = crate::BitReader;
#[doc = "Field `DMACFG` writer - DMA configuration (for multi-ADC mode)"]
pub type DMACFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDMA` reader - Direct memory access mode for multi ADC mode"]
pub type MDMA_R = crate::FieldReader;
#[doc = "Field `MDMA` writer - Direct memory access mode for multi ADC mode"]
pub type MDMA_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CKMODE` reader - ADC clock mode"]
pub type CKMODE_R = crate::FieldReader;
#[doc = "Field `CKMODE` writer - ADC clock mode"]
pub type CKMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRESC` reader - ADC prescaler"]
pub type PRESC_R = crate::FieldReader;
#[doc = "Field `PRESC` writer - ADC prescaler"]
pub type PRESC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `VREFEN` reader - VREFINT enable"]
pub type VREFEN_R = crate::BitReader;
#[doc = "Field `VREFEN` writer - VREFINT enable"]
pub type VREFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH17SEL` reader - CH17 selection"]
pub type CH17SEL_R = crate::BitReader;
#[doc = "Field `CH17SEL` writer - CH17 selection"]
pub type CH17SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH18SEL` reader - CH18 selection"]
pub type CH18SEL_R = crate::BitReader;
#[doc = "Field `CH18SEL` writer - CH18 selection"]
pub type CH18SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Dual ADC mode selection"]
    #[inline(always)]
    pub fn dual(&self) -> DUAL_R {
        DUAL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - Delay between 2 sampling phases"]
    #[inline(always)]
    pub fn delay(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - DMA configuration (for multi-ADC mode)"]
    #[inline(always)]
    pub fn dmacfg(&self) -> DMACFG_R {
        DMACFG_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Direct memory access mode for multi ADC mode"]
    #[inline(always)]
    pub fn mdma(&self) -> MDMA_R {
        MDMA_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - ADC clock mode"]
    #[inline(always)]
    pub fn ckmode(&self) -> CKMODE_R {
        CKMODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:21 - ADC prescaler"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - VREFINT enable"]
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CH17 selection"]
    #[inline(always)]
    pub fn ch17sel(&self) -> CH17SEL_R {
        CH17SEL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - CH18 selection"]
    #[inline(always)]
    pub fn ch18sel(&self) -> CH18SEL_R {
        CH18SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Dual ADC mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn dual(&mut self) -> DUAL_W<CCRrs> {
        DUAL_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - Delay between 2 sampling phases"]
    #[inline(always)]
    #[must_use]
    pub fn delay(&mut self) -> DELAY_W<CCRrs> {
        DELAY_W::new(self, 8)
    }
    #[doc = "Bit 13 - DMA configuration (for multi-ADC mode)"]
    #[inline(always)]
    #[must_use]
    pub fn dmacfg(&mut self) -> DMACFG_W<CCRrs> {
        DMACFG_W::new(self, 13)
    }
    #[doc = "Bits 14:15 - Direct memory access mode for multi ADC mode"]
    #[inline(always)]
    #[must_use]
    pub fn mdma(&mut self) -> MDMA_W<CCRrs> {
        MDMA_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - ADC clock mode"]
    #[inline(always)]
    #[must_use]
    pub fn ckmode(&mut self) -> CKMODE_W<CCRrs> {
        CKMODE_W::new(self, 16)
    }
    #[doc = "Bits 18:21 - ADC prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<CCRrs> {
        PRESC_W::new(self, 18)
    }
    #[doc = "Bit 22 - VREFINT enable"]
    #[inline(always)]
    #[must_use]
    pub fn vrefen(&mut self) -> VREFEN_W<CCRrs> {
        VREFEN_W::new(self, 22)
    }
    #[doc = "Bit 23 - CH17 selection"]
    #[inline(always)]
    #[must_use]
    pub fn ch17sel(&mut self) -> CH17SEL_W<CCRrs> {
        CH17SEL_W::new(self, 23)
    }
    #[doc = "Bit 24 - CH18 selection"]
    #[inline(always)]
    #[must_use]
    pub fn ch18sel(&mut self) -> CH18SEL_W<CCRrs> {
        CH18SEL_W::new(self, 24)
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
