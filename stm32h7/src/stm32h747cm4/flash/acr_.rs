#[doc = "Register `ACR_` reader"]
pub type R = crate::R<ACR_rs>;
#[doc = "Register `ACR_` writer"]
pub type W = crate::W<ACR_rs>;
#[doc = "Field `LATENCY` reader - Read latency"]
pub type LATENCY_R = crate::FieldReader;
#[doc = "Field `LATENCY` writer - Read latency"]
pub type LATENCY_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WRHIGHFREQ` reader - Flash signal delay"]
pub type WRHIGHFREQ_R = crate::FieldReader;
#[doc = "Field `WRHIGHFREQ` writer - Flash signal delay"]
pub type WRHIGHFREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - Read latency"]
    #[inline(always)]
    pub fn latency(&self) -> LATENCY_R {
        LATENCY_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Flash signal delay"]
    #[inline(always)]
    pub fn wrhighfreq(&self) -> WRHIGHFREQ_R {
        WRHIGHFREQ_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Read latency"]
    #[inline(always)]
    #[must_use]
    pub fn latency(&mut self) -> LATENCY_W<ACR_rs> {
        LATENCY_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - Flash signal delay"]
    #[inline(always)]
    #[must_use]
    pub fn wrhighfreq(&mut self) -> WRHIGHFREQ_W<ACR_rs> {
        WRHIGHFREQ_W::new(self, 4)
    }
}
#[doc = "FLASH access control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acr_::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acr_::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACR_rs;
impl crate::RegisterSpec for ACR_rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acr_::R`](R) reader structure"]
impl crate::Readable for ACR_rs {}
#[doc = "`write(|w| ..)` method takes [`acr_::W`](W) writer structure"]
impl crate::Writable for ACR_rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ACR_ to value 0x37"]
impl crate::Resettable for ACR_rs {
    const RESET_VALUE: u32 = 0x37;
}
