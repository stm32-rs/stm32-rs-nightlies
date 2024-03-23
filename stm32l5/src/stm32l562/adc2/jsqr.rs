#[doc = "Register `JSQR` reader"]
pub type R = crate::R<JSQRrs>;
#[doc = "Register `JSQR` writer"]
pub type W = crate::W<JSQRrs>;
#[doc = "Field `JL` reader - JL"]
pub type JL_R = crate::FieldReader;
#[doc = "Field `JL` writer - JL"]
pub type JL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `JEXTSEL` reader - JEXTSEL"]
pub type JEXTSEL_R = crate::FieldReader;
#[doc = "Field `JEXTSEL` writer - JEXTSEL"]
pub type JEXTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `JEXTEN` reader - JEXTEN"]
pub type JEXTEN_R = crate::FieldReader;
#[doc = "Field `JEXTEN` writer - JEXTEN"]
pub type JEXTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `JSQ1` reader - JSQ1"]
pub type JSQ1_R = crate::FieldReader;
#[doc = "Field `JSQ1` writer - JSQ1"]
pub type JSQ1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `JSQ2` reader - JSQ2"]
pub type JSQ2_R = crate::FieldReader;
#[doc = "Field `JSQ2` writer - JSQ2"]
pub type JSQ2_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `JSQ3` reader - JSQ3"]
pub type JSQ3_R = crate::FieldReader;
#[doc = "Field `JSQ3` writer - JSQ3"]
pub type JSQ3_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `JSQ4` reader - JSQ4"]
pub type JSQ4_R = crate::FieldReader;
#[doc = "Field `JSQ4` writer - JSQ4"]
pub type JSQ4_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:1 - JL"]
    #[inline(always)]
    pub fn jl(&self) -> JL_R {
        JL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:5 - JEXTSEL"]
    #[inline(always)]
    pub fn jextsel(&self) -> JEXTSEL_R {
        JEXTSEL_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:7 - JEXTEN"]
    #[inline(always)]
    pub fn jexten(&self) -> JEXTEN_R {
        JEXTEN_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:12 - JSQ1"]
    #[inline(always)]
    pub fn jsq1(&self) -> JSQ1_R {
        JSQ1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 14:18 - JSQ2"]
    #[inline(always)]
    pub fn jsq2(&self) -> JSQ2_R {
        JSQ2_R::new(((self.bits >> 14) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - JSQ3"]
    #[inline(always)]
    pub fn jsq3(&self) -> JSQ3_R {
        JSQ3_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 26:30 - JSQ4"]
    #[inline(always)]
    pub fn jsq4(&self) -> JSQ4_R {
        JSQ4_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - JL"]
    #[inline(always)]
    #[must_use]
    pub fn jl(&mut self) -> JL_W<JSQRrs> {
        JL_W::new(self, 0)
    }
    #[doc = "Bits 2:5 - JEXTSEL"]
    #[inline(always)]
    #[must_use]
    pub fn jextsel(&mut self) -> JEXTSEL_W<JSQRrs> {
        JEXTSEL_W::new(self, 2)
    }
    #[doc = "Bits 6:7 - JEXTEN"]
    #[inline(always)]
    #[must_use]
    pub fn jexten(&mut self) -> JEXTEN_W<JSQRrs> {
        JEXTEN_W::new(self, 6)
    }
    #[doc = "Bits 8:12 - JSQ1"]
    #[inline(always)]
    #[must_use]
    pub fn jsq1(&mut self) -> JSQ1_W<JSQRrs> {
        JSQ1_W::new(self, 8)
    }
    #[doc = "Bits 14:18 - JSQ2"]
    #[inline(always)]
    #[must_use]
    pub fn jsq2(&mut self) -> JSQ2_W<JSQRrs> {
        JSQ2_W::new(self, 14)
    }
    #[doc = "Bits 20:24 - JSQ3"]
    #[inline(always)]
    #[must_use]
    pub fn jsq3(&mut self) -> JSQ3_W<JSQRrs> {
        JSQ3_W::new(self, 20)
    }
    #[doc = "Bits 26:30 - JSQ4"]
    #[inline(always)]
    #[must_use]
    pub fn jsq4(&mut self) -> JSQ4_W<JSQRrs> {
        JSQ4_W::new(self, 26)
    }
}
#[doc = "injected sequence register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jsqr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jsqr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct JSQRrs;
impl crate::RegisterSpec for JSQRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jsqr::R`](R) reader structure"]
impl crate::Readable for JSQRrs {}
#[doc = "`write(|w| ..)` method takes [`jsqr::W`](W) writer structure"]
impl crate::Writable for JSQRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets JSQR to value 0"]
impl crate::Resettable for JSQRrs {
    const RESET_VALUE: u32 = 0;
}
