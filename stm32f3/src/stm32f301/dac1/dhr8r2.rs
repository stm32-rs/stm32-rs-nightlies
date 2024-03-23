#[doc = "Register `DHR8R2` reader"]
pub type R = crate::R<DHR8R2rs>;
#[doc = "Register `DHR8R2` writer"]
pub type W = crate::W<DHR8R2rs>;
#[doc = "Field `DACC2DHR` reader - DAC channel2 8-bit right-aligned data"]
pub type DACC2DHR_R = crate::FieldReader;
#[doc = "Field `DACC2DHR` writer - DAC channel2 8-bit right-aligned data"]
pub type DACC2DHR_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DAC channel2 8-bit right-aligned data"]
    #[inline(always)]
    pub fn dacc2dhr(&self) -> DACC2DHR_R {
        DACC2DHR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DAC channel2 8-bit right-aligned data"]
    #[inline(always)]
    #[must_use]
    pub fn dacc2dhr(&mut self) -> DACC2DHR_W<DHR8R2rs> {
        DACC2DHR_W::new(self, 0)
    }
}
#[doc = "channel2 8-bit right-aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dhr8r2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dhr8r2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DHR8R2rs;
impl crate::RegisterSpec for DHR8R2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dhr8r2::R`](R) reader structure"]
impl crate::Readable for DHR8R2rs {}
#[doc = "`write(|w| ..)` method takes [`dhr8r2::W`](W) writer structure"]
impl crate::Writable for DHR8R2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DHR8R2 to value 0"]
impl crate::Resettable for DHR8R2rs {
    const RESET_VALUE: u32 = 0;
}
