#[doc = "Register `BGCLUT` reader"]
pub type R = crate::R<BGCLUTrs>;
#[doc = "Register `BGCLUT` writer"]
pub type W = crate::W<BGCLUTrs>;
#[doc = "Field `BLUE` reader - BLUE"]
pub type BLUE_R = crate::FieldReader;
#[doc = "Field `BLUE` writer - BLUE"]
pub type BLUE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GREEN` reader - GREEN"]
pub type GREEN_R = crate::FieldReader;
#[doc = "Field `GREEN` writer - GREEN"]
pub type GREEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RED` reader - RED"]
pub type RED_R = crate::FieldReader;
#[doc = "Field `RED` writer - RED"]
pub type RED_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `APLHA` reader - APLHA"]
pub type APLHA_R = crate::FieldReader;
#[doc = "Field `APLHA` writer - APLHA"]
pub type APLHA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - BLUE"]
    #[inline(always)]
    pub fn blue(&self) -> BLUE_R {
        BLUE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GREEN"]
    #[inline(always)]
    pub fn green(&self) -> GREEN_R {
        GREEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - RED"]
    #[inline(always)]
    pub fn red(&self) -> RED_R {
        RED_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - APLHA"]
    #[inline(always)]
    pub fn aplha(&self) -> APLHA_R {
        APLHA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - BLUE"]
    #[inline(always)]
    #[must_use]
    pub fn blue(&mut self) -> BLUE_W<BGCLUTrs> {
        BLUE_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - GREEN"]
    #[inline(always)]
    #[must_use]
    pub fn green(&mut self) -> GREEN_W<BGCLUTrs> {
        GREEN_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - RED"]
    #[inline(always)]
    #[must_use]
    pub fn red(&mut self) -> RED_W<BGCLUTrs> {
        RED_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - APLHA"]
    #[inline(always)]
    #[must_use]
    pub fn aplha(&mut self) -> APLHA_W<BGCLUTrs> {
        APLHA_W::new(self, 24)
    }
}
#[doc = "BGCLUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bgclut::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bgclut::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BGCLUTrs;
impl crate::RegisterSpec for BGCLUTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bgclut::R`](R) reader structure"]
impl crate::Readable for BGCLUTrs {}
#[doc = "`write(|w| ..)` method takes [`bgclut::W`](W) writer structure"]
impl crate::Writable for BGCLUTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BGCLUT to value 0"]
impl crate::Resettable for BGCLUTrs {
    const RESET_VALUE: u32 = 0;
}
