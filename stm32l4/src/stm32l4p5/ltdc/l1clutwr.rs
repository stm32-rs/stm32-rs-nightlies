#[doc = "Register `L1CLUTWR` writer"]
pub type W = crate::W<L1CLUTWRrs>;
#[doc = "Field `BLUE` writer - Blue value"]
pub type BLUE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
#[doc = "Field `GREEN` writer - Green value"]
pub type GREEN_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
#[doc = "Field `RED` writer - Red value"]
pub type RED_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
#[doc = "Field `CLUTADD` writer - CLUT Address"]
pub type CLUTADD_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Blue value"]
    #[inline(always)]
    #[must_use]
    pub fn blue(&mut self) -> BLUE_W<L1CLUTWRrs> {
        BLUE_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Green value"]
    #[inline(always)]
    #[must_use]
    pub fn green(&mut self) -> GREEN_W<L1CLUTWRrs> {
        GREEN_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Red value"]
    #[inline(always)]
    #[must_use]
    pub fn red(&mut self) -> RED_W<L1CLUTWRrs> {
        RED_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - CLUT Address"]
    #[inline(always)]
    #[must_use]
    pub fn clutadd(&mut self) -> CLUTADD_W<L1CLUTWRrs> {
        CLUTADD_W::new(self, 24)
    }
}
#[doc = "LTDC Layerx CLUT Write Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1clutwr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1CLUTWRrs;
impl crate::RegisterSpec for L1CLUTWRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`l1clutwr::W`](W) writer structure"]
impl crate::Writable for L1CLUTWRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L1CLUTWR to value 0"]
impl crate::Resettable for L1CLUTWRrs {
    const RESET_VALUE: u32 = 0;
}
