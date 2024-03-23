#[doc = "Register `HUFFMIN%s` reader"]
pub type R = crate::R<HUFFMINrs>;
#[doc = "Register `HUFFMIN%s` writer"]
pub type W = crate::W<HUFFMINrs>;
#[doc = "Field `HuffMin_RAM` reader - HuffMin RAM"]
pub type HUFF_MIN_RAM_R = crate::FieldReader<u32>;
#[doc = "Field `HuffMin_RAM` writer - HuffMin RAM"]
pub type HUFF_MIN_RAM_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - HuffMin RAM"]
    #[inline(always)]
    pub fn huff_min_ram(&self) -> HUFF_MIN_RAM_R {
        HUFF_MIN_RAM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - HuffMin RAM"]
    #[inline(always)]
    #[must_use]
    pub fn huff_min_ram(&mut self) -> HUFF_MIN_RAM_W<HUFFMINrs> {
        HUFF_MIN_RAM_W::new(self, 0)
    }
}
#[doc = "JPEG HuffMin tables\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`huffmin::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`huffmin::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HUFFMINrs;
impl crate::RegisterSpec for HUFFMINrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`huffmin::R`](R) reader structure"]
impl crate::Readable for HUFFMINrs {}
#[doc = "`write(|w| ..)` method takes [`huffmin::W`](W) writer structure"]
impl crate::Writable for HUFFMINrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HUFFMIN%s to value 0"]
impl crate::Resettable for HUFFMINrs {
    const RESET_VALUE: u32 = 0;
}
