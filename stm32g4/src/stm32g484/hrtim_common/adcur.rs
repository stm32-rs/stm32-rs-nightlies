#[doc = "Register `ADCUR` reader"]
pub type R = crate::R<ADCURrs>;
#[doc = "Register `ADCUR` writer"]
pub type W = crate::W<ADCURrs>;
#[doc = "Field `AD5USRC` reader - AD5USRC"]
pub type AD5USRC_R = crate::FieldReader;
#[doc = "Field `AD5USRC` writer - AD5USRC"]
pub type AD5USRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `AD6USRC` reader - AD6USRC"]
pub type AD6USRC_R = crate::FieldReader;
#[doc = "Field `AD6USRC` writer - AD6USRC"]
pub type AD6USRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `AD7USRC` reader - AD7USRC"]
pub type AD7USRC_R = crate::FieldReader;
#[doc = "Field `AD7USRC` writer - AD7USRC"]
pub type AD7USRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `AD8USRC` reader - AD8USRC"]
pub type AD8USRC_R = crate::FieldReader;
#[doc = "Field `AD8USRC` writer - AD8USRC"]
pub type AD8USRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `AD9USRC` reader - AD9USRC"]
pub type AD9USRC_R = crate::FieldReader;
#[doc = "Field `AD9USRC` writer - AD9USRC"]
pub type AD9USRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `AD10USRC` reader - AD10USRC"]
pub type AD10USRC_R = crate::FieldReader;
#[doc = "Field `AD10USRC` writer - AD10USRC"]
pub type AD10USRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - AD5USRC"]
    #[inline(always)]
    pub fn ad5usrc(&self) -> AD5USRC_R {
        AD5USRC_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - AD6USRC"]
    #[inline(always)]
    pub fn ad6usrc(&self) -> AD6USRC_R {
        AD6USRC_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - AD7USRC"]
    #[inline(always)]
    pub fn ad7usrc(&self) -> AD7USRC_R {
        AD7USRC_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - AD8USRC"]
    #[inline(always)]
    pub fn ad8usrc(&self) -> AD8USRC_R {
        AD8USRC_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - AD9USRC"]
    #[inline(always)]
    pub fn ad9usrc(&self) -> AD9USRC_R {
        AD9USRC_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - AD10USRC"]
    #[inline(always)]
    pub fn ad10usrc(&self) -> AD10USRC_R {
        AD10USRC_R::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - AD5USRC"]
    #[inline(always)]
    #[must_use]
    pub fn ad5usrc(&mut self) -> AD5USRC_W<ADCURrs> {
        AD5USRC_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - AD6USRC"]
    #[inline(always)]
    #[must_use]
    pub fn ad6usrc(&mut self) -> AD6USRC_W<ADCURrs> {
        AD6USRC_W::new(self, 4)
    }
    #[doc = "Bits 8:10 - AD7USRC"]
    #[inline(always)]
    #[must_use]
    pub fn ad7usrc(&mut self) -> AD7USRC_W<ADCURrs> {
        AD7USRC_W::new(self, 8)
    }
    #[doc = "Bits 12:14 - AD8USRC"]
    #[inline(always)]
    #[must_use]
    pub fn ad8usrc(&mut self) -> AD8USRC_W<ADCURrs> {
        AD8USRC_W::new(self, 12)
    }
    #[doc = "Bits 16:18 - AD9USRC"]
    #[inline(always)]
    #[must_use]
    pub fn ad9usrc(&mut self) -> AD9USRC_W<ADCURrs> {
        AD9USRC_W::new(self, 16)
    }
    #[doc = "Bits 20:22 - AD10USRC"]
    #[inline(always)]
    #[must_use]
    pub fn ad10usrc(&mut self) -> AD10USRC_W<ADCURrs> {
        AD10USRC_W::new(self, 20)
    }
}
#[doc = "HRTIM ADC Trigger Update Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcur::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcur::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADCURrs;
impl crate::RegisterSpec for ADCURrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcur::R`](R) reader structure"]
impl crate::Readable for ADCURrs {}
#[doc = "`write(|w| ..)` method takes [`adcur::W`](W) writer structure"]
impl crate::Writable for ADCURrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCUR to value 0"]
impl crate::Resettable for ADCURrs {
    const RESET_VALUE: u32 = 0;
}
