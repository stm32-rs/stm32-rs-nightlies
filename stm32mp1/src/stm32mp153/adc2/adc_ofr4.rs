#[doc = "Register `ADC_OFR4` reader"]
pub type R = crate::R<ADC_OFR4rs>;
#[doc = "Register `ADC_OFR4` writer"]
pub type W = crate::W<ADC_OFR4rs>;
#[doc = "Field `OFFSET4` reader - OFFSET4"]
pub type OFFSET4_R = crate::FieldReader<u32>;
#[doc = "Field `OFFSET4` writer - OFFSET4"]
pub type OFFSET4_W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
#[doc = "Field `OFFSET4_CH` reader - OFFSET4_CH"]
pub type OFFSET4_CH_R = crate::FieldReader;
#[doc = "Field `OFFSET4_CH` writer - OFFSET4_CH"]
pub type OFFSET4_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SSATE` reader - SSATE"]
pub type SSATE_R = crate::BitReader;
#[doc = "Field `SSATE` writer - SSATE"]
pub type SSATE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:25 - OFFSET4"]
    #[inline(always)]
    pub fn offset4(&self) -> OFFSET4_R {
        OFFSET4_R::new(self.bits & 0x03ff_ffff)
    }
    #[doc = "Bits 26:30 - OFFSET4_CH"]
    #[inline(always)]
    pub fn offset4_ch(&self) -> OFFSET4_CH_R {
        OFFSET4_CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - SSATE"]
    #[inline(always)]
    pub fn ssate(&self) -> SSATE_R {
        SSATE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:25 - OFFSET4"]
    #[inline(always)]
    #[must_use]
    pub fn offset4(&mut self) -> OFFSET4_W<ADC_OFR4rs> {
        OFFSET4_W::new(self, 0)
    }
    #[doc = "Bits 26:30 - OFFSET4_CH"]
    #[inline(always)]
    #[must_use]
    pub fn offset4_ch(&mut self) -> OFFSET4_CH_W<ADC_OFR4rs> {
        OFFSET4_CH_W::new(self, 26)
    }
    #[doc = "Bit 31 - SSATE"]
    #[inline(always)]
    #[must_use]
    pub fn ssate(&mut self) -> SSATE_W<ADC_OFR4rs> {
        SSATE_W::new(self, 31)
    }
}
#[doc = "ADC offset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_ofr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_ofr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC_OFR4rs;
impl crate::RegisterSpec for ADC_OFR4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_ofr4::R`](R) reader structure"]
impl crate::Readable for ADC_OFR4rs {}
#[doc = "`write(|w| ..)` method takes [`adc_ofr4::W`](W) writer structure"]
impl crate::Writable for ADC_OFR4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC_OFR4 to value 0"]
impl crate::Resettable for ADC_OFR4rs {
    const RESET_VALUE: u32 = 0;
}
