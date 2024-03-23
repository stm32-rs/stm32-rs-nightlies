#[doc = "Register `ADC_CCR` reader"]
pub type R = crate::R<ADC_CCRrs>;
#[doc = "Register `ADC_CCR` writer"]
pub type W = crate::W<ADC_CCRrs>;
#[doc = "Field `PRESC` reader - PRESC"]
pub type PRESC_R = crate::FieldReader;
#[doc = "Field `PRESC` writer - PRESC"]
pub type PRESC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `VREFEN` reader - VREFEN"]
pub type VREFEN_R = crate::BitReader;
#[doc = "Field `VREFEN` writer - VREFEN"]
pub type VREFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSENSESEL` reader - VSENSESEL"]
pub type VSENSESEL_R = crate::BitReader;
#[doc = "Field `VSENSESEL` writer - VSENSESEL"]
pub type VSENSESEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBATEN` reader - VBATEN"]
pub type VBATEN_R = crate::BitReader;
#[doc = "Field `VBATEN` writer - VBATEN"]
pub type VBATEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 18:21 - PRESC"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - VREFEN"]
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - VSENSESEL"]
    #[inline(always)]
    pub fn vsensesel(&self) -> VSENSESEL_R {
        VSENSESEL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - VBATEN"]
    #[inline(always)]
    pub fn vbaten(&self) -> VBATEN_R {
        VBATEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 18:21 - PRESC"]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<ADC_CCRrs> {
        PRESC_W::new(self, 18)
    }
    #[doc = "Bit 22 - VREFEN"]
    #[inline(always)]
    #[must_use]
    pub fn vrefen(&mut self) -> VREFEN_W<ADC_CCRrs> {
        VREFEN_W::new(self, 22)
    }
    #[doc = "Bit 23 - VSENSESEL"]
    #[inline(always)]
    #[must_use]
    pub fn vsensesel(&mut self) -> VSENSESEL_W<ADC_CCRrs> {
        VSENSESEL_W::new(self, 23)
    }
    #[doc = "Bit 24 - VBATEN"]
    #[inline(always)]
    #[must_use]
    pub fn vbaten(&mut self) -> VBATEN_W<ADC_CCRrs> {
        VBATEN_W::new(self, 24)
    }
}
#[doc = "ADC common configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_ccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_ccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC_CCRrs;
impl crate::RegisterSpec for ADC_CCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_ccr::R`](R) reader structure"]
impl crate::Readable for ADC_CCRrs {}
#[doc = "`write(|w| ..)` method takes [`adc_ccr::W`](W) writer structure"]
impl crate::Writable for ADC_CCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC_CCR to value 0"]
impl crate::Resettable for ADC_CCRrs {
    const RESET_VALUE: u32 = 0;
}
