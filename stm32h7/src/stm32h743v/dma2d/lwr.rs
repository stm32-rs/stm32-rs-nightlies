#[doc = "Register `LWR` reader"]
pub type R = crate::R<LWRrs>;
#[doc = "Register `LWR` writer"]
pub type W = crate::W<LWRrs>;
#[doc = "Field `LW` reader - Line watermark These bits allow to configure the line watermark for interrupt generation. An interrupt is raised when the last pixel of the watermarked line has been transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
pub type LW_R = crate::FieldReader<u16>;
#[doc = "Field `LW` writer - Line watermark These bits allow to configure the line watermark for interrupt generation. An interrupt is raised when the last pixel of the watermarked line has been transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
pub type LW_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Line watermark These bits allow to configure the line watermark for interrupt generation. An interrupt is raised when the last pixel of the watermarked line has been transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
    #[inline(always)]
    pub fn lw(&self) -> LW_R {
        LW_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Line watermark These bits allow to configure the line watermark for interrupt generation. An interrupt is raised when the last pixel of the watermarked line has been transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
    #[inline(always)]
    #[must_use]
    pub fn lw(&mut self) -> LW_W<LWRrs> {
        LW_W::new(self, 0)
    }
}
#[doc = "DMA2D line watermark register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lwr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lwr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LWRrs;
impl crate::RegisterSpec for LWRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lwr::R`](R) reader structure"]
impl crate::Readable for LWRrs {}
#[doc = "`write(|w| ..)` method takes [`lwr::W`](W) writer structure"]
impl crate::Writable for LWRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LWR to value 0"]
impl crate::Resettable for LWRrs {
    const RESET_VALUE: u32 = 0;
}
