#[doc = "Register `ADC_OFR2` reader"]
pub type R = crate::R<ADC_OFR2rs>;
#[doc = "Register `ADC_OFR2` writer"]
pub type W = crate::W<ADC_OFR2rs>;
#[doc = "Field `OFFSET2` reader - OFFSET2"]
pub type OFFSET2_R = crate::FieldReader<u32>;
#[doc = "Field `OFFSET2` writer - OFFSET2"]
pub type OFFSET2_W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
#[doc = "Field `OFFSET2_CH` reader - OFFSET2_CH"]
pub type OFFSET2_CH_R = crate::FieldReader;
#[doc = "Field `OFFSET2_CH` writer - OFFSET2_CH"]
pub type OFFSET2_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SSATE` reader - SSATE"]
pub type SSATE_R = crate::BitReader;
#[doc = "Field `SSATE` writer - SSATE"]
pub type SSATE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:25 - OFFSET2"]
    #[inline(always)]
    pub fn offset2(&self) -> OFFSET2_R {
        OFFSET2_R::new(self.bits & 0x03ff_ffff)
    }
    #[doc = "Bits 26:30 - OFFSET2_CH"]
    #[inline(always)]
    pub fn offset2_ch(&self) -> OFFSET2_CH_R {
        OFFSET2_CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - SSATE"]
    #[inline(always)]
    pub fn ssate(&self) -> SSATE_R {
        SSATE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:25 - OFFSET2"]
    #[inline(always)]
    #[must_use]
    pub fn offset2(&mut self) -> OFFSET2_W<ADC_OFR2rs> {
        OFFSET2_W::new(self, 0)
    }
    #[doc = "Bits 26:30 - OFFSET2_CH"]
    #[inline(always)]
    #[must_use]
    pub fn offset2_ch(&mut self) -> OFFSET2_CH_W<ADC_OFR2rs> {
        OFFSET2_CH_W::new(self, 26)
    }
    #[doc = "Bit 31 - SSATE"]
    #[inline(always)]
    #[must_use]
    pub fn ssate(&mut self) -> SSATE_W<ADC_OFR2rs> {
        SSATE_W::new(self, 31)
    }
}
#[doc = "ADC offset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_ofr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_ofr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC_OFR2rs;
impl crate::RegisterSpec for ADC_OFR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_ofr2::R`](R) reader structure"]
impl crate::Readable for ADC_OFR2rs {}
#[doc = "`write(|w| ..)` method takes [`adc_ofr2::W`](W) writer structure"]
impl crate::Writable for ADC_OFR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC_OFR2 to value 0"]
impl crate::Resettable for ADC_OFR2rs {
    const RESET_VALUE: u32 = 0;
}
