#[doc = "Register `PSMKR` reader"]
pub type R = crate::R<PSMKRrs>;
#[doc = "Register `PSMKR` writer"]
pub type W = crate::W<PSMKRrs>;
#[doc = "Field `MASK` reader - Status mask Mask to be applied to the status bytes received in Automatic status-polling mode For bit n:"]
pub type MASK_R = crate::FieldReader<u32>;
#[doc = "Field `MASK` writer - Status mask Mask to be applied to the status bytes received in Automatic status-polling mode For bit n:"]
pub type MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Status mask Mask to be applied to the status bytes received in Automatic status-polling mode For bit n:"]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Status mask Mask to be applied to the status bytes received in Automatic status-polling mode For bit n:"]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MASK_W<PSMKRrs> {
        MASK_W::new(self, 0)
    }
}
#[doc = "OCTOSPI polling status mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psmkr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psmkr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSMKRrs;
impl crate::RegisterSpec for PSMKRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psmkr::R`](R) reader structure"]
impl crate::Readable for PSMKRrs {}
#[doc = "`write(|w| ..)` method takes [`psmkr::W`](W) writer structure"]
impl crate::Writable for PSMKRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSMKR to value 0"]
impl crate::Resettable for PSMKRrs {
    const RESET_VALUE: u32 = 0;
}
