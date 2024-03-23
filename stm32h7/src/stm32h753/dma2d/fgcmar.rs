#[doc = "Register `FGCMAR` reader"]
pub type R = crate::R<FGCMARrs>;
#[doc = "Register `FGCMAR` writer"]
pub type W = crate::W<FGCMARrs>;
#[doc = "Field `MA` reader - Memory Address Address of the data used for the CLUT address dedicated to the foreground image. This register can only be written when no transfer is ongoing. Once the CLUT transfer has started, this register is read-only. If the foreground CLUT format is 32-bit, the address must be 32-bit aligned."]
pub type MA_R = crate::FieldReader<u32>;
#[doc = "Field `MA` writer - Memory Address Address of the data used for the CLUT address dedicated to the foreground image. This register can only be written when no transfer is ongoing. Once the CLUT transfer has started, this register is read-only. If the foreground CLUT format is 32-bit, the address must be 32-bit aligned."]
pub type MA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Memory Address Address of the data used for the CLUT address dedicated to the foreground image. This register can only be written when no transfer is ongoing. Once the CLUT transfer has started, this register is read-only. If the foreground CLUT format is 32-bit, the address must be 32-bit aligned."]
    #[inline(always)]
    pub fn ma(&self) -> MA_R {
        MA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory Address Address of the data used for the CLUT address dedicated to the foreground image. This register can only be written when no transfer is ongoing. Once the CLUT transfer has started, this register is read-only. If the foreground CLUT format is 32-bit, the address must be 32-bit aligned."]
    #[inline(always)]
    #[must_use]
    pub fn ma(&mut self) -> MA_W<FGCMARrs> {
        MA_W::new(self, 0)
    }
}
#[doc = "DMA2D foreground CLUT memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fgcmar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fgcmar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FGCMARrs;
impl crate::RegisterSpec for FGCMARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fgcmar::R`](R) reader structure"]
impl crate::Readable for FGCMARrs {}
#[doc = "`write(|w| ..)` method takes [`fgcmar::W`](W) writer structure"]
impl crate::Writable for FGCMARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FGCMAR to value 0"]
impl crate::Resettable for FGCMARrs {
    const RESET_VALUE: u32 = 0;
}
