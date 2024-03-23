#[doc = "Register `ADC_OFR1` reader"]
pub type R = crate::R<ADC_OFR1rs>;
#[doc = "Register `ADC_OFR1` writer"]
pub type W = crate::W<ADC_OFR1rs>;
#[doc = "Field `OFFSET` reader - OFFSET"]
pub type OFFSET_R = crate::FieldReader<u32>;
#[doc = "Field `OFFSET` writer - OFFSET"]
pub type OFFSET_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `POSOFF` reader - POSOFF"]
pub type POSOFF_R = crate::BitReader;
#[doc = "Field `POSOFF` writer - POSOFF"]
pub type POSOFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USAT` reader - USAT"]
pub type USAT_R = crate::BitReader;
#[doc = "Field `USAT` writer - USAT"]
pub type USAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSAT` reader - SSAT"]
pub type SSAT_R = crate::BitReader;
#[doc = "Field `SSAT` writer - SSAT"]
pub type SSAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OFFSET_CH` reader - OFFSET_CH"]
pub type OFFSET_CH_R = crate::FieldReader;
#[doc = "Field `OFFSET_CH` writer - OFFSET_CH"]
pub type OFFSET_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:23 - OFFSET"]
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - POSOFF"]
    #[inline(always)]
    pub fn posoff(&self) -> POSOFF_R {
        POSOFF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - USAT"]
    #[inline(always)]
    pub fn usat(&self) -> USAT_R {
        USAT_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - SSAT"]
    #[inline(always)]
    pub fn ssat(&self) -> SSAT_R {
        SSAT_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:31 - OFFSET_CH"]
    #[inline(always)]
    pub fn offset_ch(&self) -> OFFSET_CH_R {
        OFFSET_CH_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - OFFSET"]
    #[inline(always)]
    #[must_use]
    pub fn offset(&mut self) -> OFFSET_W<ADC_OFR1rs> {
        OFFSET_W::new(self, 0)
    }
    #[doc = "Bit 24 - POSOFF"]
    #[inline(always)]
    #[must_use]
    pub fn posoff(&mut self) -> POSOFF_W<ADC_OFR1rs> {
        POSOFF_W::new(self, 24)
    }
    #[doc = "Bit 25 - USAT"]
    #[inline(always)]
    #[must_use]
    pub fn usat(&mut self) -> USAT_W<ADC_OFR1rs> {
        USAT_W::new(self, 25)
    }
    #[doc = "Bit 26 - SSAT"]
    #[inline(always)]
    #[must_use]
    pub fn ssat(&mut self) -> SSAT_W<ADC_OFR1rs> {
        SSAT_W::new(self, 26)
    }
    #[doc = "Bits 27:31 - OFFSET_CH"]
    #[inline(always)]
    #[must_use]
    pub fn offset_ch(&mut self) -> OFFSET_CH_W<ADC_OFR1rs> {
        OFFSET_CH_W::new(self, 27)
    }
}
#[doc = "ADC offset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_ofr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_ofr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC_OFR1rs;
impl crate::RegisterSpec for ADC_OFR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_ofr1::R`](R) reader structure"]
impl crate::Readable for ADC_OFR1rs {}
#[doc = "`write(|w| ..)` method takes [`adc_ofr1::W`](W) writer structure"]
impl crate::Writable for ADC_OFR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC_OFR1 to value 0"]
impl crate::Resettable for ADC_OFR1rs {
    const RESET_VALUE: u32 = 0;
}
