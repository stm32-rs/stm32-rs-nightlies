#[doc = "Register `CNT` reader"]
pub type R = crate::R<CNTrs>;
#[doc = "Register `CNT` writer"]
pub type W = crate::W<CNTrs>;
#[doc = "Field `UIFREMAP_CNT` reader - Counter value when CR1.UIFREMAP=1"]
pub type UIFREMAP_CNT_R = crate::FieldReader<u32>;
#[doc = "Field `UIFREMAP_CNT` writer - Counter value when CR1.UIFREMAP=1"]
pub type UIFREMAP_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `CNT` reader - Counter value"]
pub type CNT_R = crate::FieldReader<u32>;
#[doc = "Field `CNT` writer - Counter value"]
pub type CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[doc = "Field `UIFCPY` reader - Copy of ISR.UIF when CR1.UIFREMAP=1"]
pub type UIFCPY_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:30 - Counter value when CR1.UIFREMAP=1"]
    #[inline(always)]
    pub fn uifremap_cnt(&self) -> UIFREMAP_CNT_R {
        UIFREMAP_CNT_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bits 0:31 - Counter value"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(self.bits)
    }
    #[doc = "Bit 31 - Copy of ISR.UIF when CR1.UIFREMAP=1"]
    #[inline(always)]
    pub fn uifcpy(&self) -> UIFCPY_R {
        UIFCPY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - Counter value when CR1.UIFREMAP=1"]
    #[inline(always)]
    #[must_use]
    pub fn uifremap_cnt(&mut self) -> UIFREMAP_CNT_W<CNTrs> {
        UIFREMAP_CNT_W::new(self, 0)
    }
    #[doc = "Bits 0:31 - Counter value"]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CNT_W<CNTrs> {
        CNT_W::new(self, 0)
    }
}
#[doc = "counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNTrs;
impl crate::RegisterSpec for CNTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnt::R`](R) reader structure"]
impl crate::Readable for CNTrs {}
#[doc = "`write(|w| ..)` method takes [`cnt::W`](W) writer structure"]
impl crate::Writable for CNTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CNT to value 0"]
impl crate::Resettable for CNTrs {
    const RESET_VALUE: u32 = 0;
}
