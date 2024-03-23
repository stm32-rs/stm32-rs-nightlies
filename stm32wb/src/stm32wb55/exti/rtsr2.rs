#[doc = "Register `RTSR2` reader"]
pub type R = crate::R<RTSR2rs>;
#[doc = "Register `RTSR2` writer"]
pub type W = crate::W<RTSR2rs>;
#[doc = "Field `RT33` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type RT33_R = crate::BitReader;
#[doc = "Field `RT33` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type RT33_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT40_41` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type RT40_41_R = crate::FieldReader;
#[doc = "Field `RT40_41` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type RT40_41_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 1 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn rt33(&self) -> RT33_R {
        RT33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn rt40_41(&self) -> RT40_41_R {
        RT40_41_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn rt33(&mut self) -> RT33_W<RTSR2rs> {
        RT33_W::new(self, 1)
    }
    #[doc = "Bits 8:9 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    #[must_use]
    pub fn rt40_41(&mut self) -> RT40_41_W<RTSR2rs> {
        RT40_41_W::new(self, 8)
    }
}
#[doc = "rising trigger selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtsr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtsr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTSR2rs;
impl crate::RegisterSpec for RTSR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtsr2::R`](R) reader structure"]
impl crate::Readable for RTSR2rs {}
#[doc = "`write(|w| ..)` method takes [`rtsr2::W`](W) writer structure"]
impl crate::Writable for RTSR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTSR2 to value 0"]
impl crate::Resettable for RTSR2rs {
    const RESET_VALUE: u32 = 0;
}
