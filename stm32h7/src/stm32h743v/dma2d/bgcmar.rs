#[doc = "Register `BGCMAR` reader"]
pub type R = crate::R<BGCMARrs>;
#[doc = "Register `BGCMAR` writer"]
pub type W = crate::W<BGCMARrs>;
#[doc = "Field `MA` reader - Memory address Address of the data used for the CLUT address dedicated to the background image. This register can only be written when no transfer is on going. Once the CLUT transfer has started, this register is read-only. If the background CLUT format is 32-bit, the address must be 32-bit aligned."]
pub type MA_R = crate::FieldReader<u32>;
#[doc = "Field `MA` writer - Memory address Address of the data used for the CLUT address dedicated to the background image. This register can only be written when no transfer is on going. Once the CLUT transfer has started, this register is read-only. If the background CLUT format is 32-bit, the address must be 32-bit aligned."]
pub type MA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Memory address Address of the data used for the CLUT address dedicated to the background image. This register can only be written when no transfer is on going. Once the CLUT transfer has started, this register is read-only. If the background CLUT format is 32-bit, the address must be 32-bit aligned."]
    #[inline(always)]
    pub fn ma(&self) -> MA_R {
        MA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory address Address of the data used for the CLUT address dedicated to the background image. This register can only be written when no transfer is on going. Once the CLUT transfer has started, this register is read-only. If the background CLUT format is 32-bit, the address must be 32-bit aligned."]
    #[inline(always)]
    #[must_use]
    pub fn ma(&mut self) -> MA_W<BGCMARrs> {
        MA_W::new(self, 0)
    }
}
#[doc = "DMA2D background CLUT memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bgcmar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bgcmar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BGCMARrs;
impl crate::RegisterSpec for BGCMARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bgcmar::R`](R) reader structure"]
impl crate::Readable for BGCMARrs {}
#[doc = "`write(|w| ..)` method takes [`bgcmar::W`](W) writer structure"]
impl crate::Writable for BGCMARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BGCMAR to value 0"]
impl crate::Resettable for BGCMARrs {
    const RESET_VALUE: u32 = 0;
}
