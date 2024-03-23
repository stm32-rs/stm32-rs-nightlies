#[doc = "Register `WHPCR` reader"]
pub type R = crate::R<WHPCRrs>;
#[doc = "Register `WHPCR` writer"]
pub type W = crate::W<WHPCRrs>;
#[doc = "Field `WHSTPOS` reader - Window Horizontal Start Position"]
pub type WHSTPOS_R = crate::FieldReader<u16>;
#[doc = "Field `WHSTPOS` writer - Window Horizontal Start Position"]
pub type WHSTPOS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 12, u16>;
#[doc = "Field `WHSPPOS` reader - Window Horizontal Stop Position"]
pub type WHSPPOS_R = crate::FieldReader<u16>;
#[doc = "Field `WHSPPOS` writer - Window Horizontal Stop Position"]
pub type WHSPPOS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Window Horizontal Start Position"]
    #[inline(always)]
    pub fn whstpos(&self) -> WHSTPOS_R {
        WHSTPOS_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Window Horizontal Stop Position"]
    #[inline(always)]
    pub fn whsppos(&self) -> WHSPPOS_R {
        WHSPPOS_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Window Horizontal Start Position"]
    #[inline(always)]
    #[must_use]
    pub fn whstpos(&mut self) -> WHSTPOS_W<WHPCRrs> {
        WHSTPOS_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - Window Horizontal Stop Position"]
    #[inline(always)]
    #[must_use]
    pub fn whsppos(&mut self) -> WHSPPOS_W<WHPCRrs> {
        WHSPPOS_W::new(self, 16)
    }
}
#[doc = "Layerx Window Horizontal Position Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`whpcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`whpcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WHPCRrs;
impl crate::RegisterSpec for WHPCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`whpcr::R`](R) reader structure"]
impl crate::Readable for WHPCRrs {}
#[doc = "`write(|w| ..)` method takes [`whpcr::W`](W) writer structure"]
impl crate::Writable for WHPCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WHPCR to value 0"]
impl crate::Resettable for WHPCRrs {
    const RESET_VALUE: u32 = 0;
}
