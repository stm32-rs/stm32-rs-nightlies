#[doc = "Register `LUTL` reader"]
pub type R = crate::R<LUTLrs>;
#[doc = "Register `LUTL` writer"]
pub type W = crate::W<LUTLrs>;
#[doc = "Field `EN` reader - Enable"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Enable"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FVB` reader - First Valid Block"]
pub type FVB_R = crate::FieldReader;
#[doc = "Field `FVB` writer - First Valid Block"]
pub type FVB_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LVB` reader - Last Valid Block"]
pub type LVB_R = crate::FieldReader;
#[doc = "Field `LVB` writer - Last Valid Block"]
pub type LVB_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15 - First Valid Block"]
    #[inline(always)]
    pub fn fvb(&self) -> FVB_R {
        FVB_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Last Valid Block"]
    #[inline(always)]
    pub fn lvb(&self) -> LVB_R {
        LVB_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<LUTLrs> {
        EN_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - First Valid Block"]
    #[inline(always)]
    #[must_use]
    pub fn fvb(&mut self) -> FVB_W<LUTLrs> {
        FVB_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Last Valid Block"]
    #[inline(always)]
    #[must_use]
    pub fn lvb(&mut self) -> LVB_W<LUTLrs> {
        LVB_W::new(self, 16)
    }
}
#[doc = "Graphic MMU LUT entry x low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lutl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lutl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LUTLrs;
impl crate::RegisterSpec for LUTLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lutl::R`](R) reader structure"]
impl crate::Readable for LUTLrs {}
#[doc = "`write(|w| ..)` method takes [`lutl::W`](W) writer structure"]
impl crate::Writable for LUTLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LUTL to value 0"]
impl crate::Resettable for LUTLrs {
    const RESET_VALUE: u32 = 0;
}
