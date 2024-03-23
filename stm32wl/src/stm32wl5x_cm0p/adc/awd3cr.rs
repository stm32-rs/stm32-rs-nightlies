#[doc = "Register `AWD3CR` reader"]
pub type R = crate::R<AWD3CRrs>;
#[doc = "Register `AWD3CR` writer"]
pub type W = crate::W<AWD3CRrs>;
#[doc = "Field `AWD3CH` reader - AWD3CH"]
pub type AWD3CH_R = crate::FieldReader<u32>;
#[doc = "Field `AWD3CH` writer - AWD3CH"]
pub type AWD3CH_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch(&self) -> AWD3CH_R {
        AWD3CH_R::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - AWD3CH"]
    #[inline(always)]
    #[must_use]
    pub fn awd3ch(&mut self) -> AWD3CH_W<AWD3CRrs> {
        AWD3CH_W::new(self, 0)
    }
}
#[doc = "ADC Analog Watchdog 3 Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awd3cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awd3cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AWD3CRrs;
impl crate::RegisterSpec for AWD3CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awd3cr::R`](R) reader structure"]
impl crate::Readable for AWD3CRrs {}
#[doc = "`write(|w| ..)` method takes [`awd3cr::W`](W) writer structure"]
impl crate::Writable for AWD3CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AWD3CR to value 0"]
impl crate::Resettable for AWD3CRrs {
    const RESET_VALUE: u32 = 0;
}
