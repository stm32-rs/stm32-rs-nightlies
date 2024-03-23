#[doc = "Register `WDATA` reader"]
pub type R = crate::R<WDATArs>;
#[doc = "Register `WDATA` writer"]
pub type W = crate::W<WDATArs>;
#[doc = "Field `ARG` reader - Function input arguments"]
pub type ARG_R = crate::FieldReader<u32>;
#[doc = "Field `ARG` writer - Function input arguments"]
pub type ARG_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Function input arguments"]
    #[inline(always)]
    pub fn arg(&self) -> ARG_R {
        ARG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Function input arguments"]
    #[inline(always)]
    #[must_use]
    pub fn arg(&mut self) -> ARG_W<WDATArs> {
        ARG_W::new(self, 0)
    }
}
#[doc = "Argument register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdata::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdata::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDATArs;
impl crate::RegisterSpec for WDATArs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdata::R`](R) reader structure"]
impl crate::Readable for WDATArs {}
#[doc = "`write(|w| ..)` method takes [`wdata::W`](W) writer structure"]
impl crate::Writable for WDATArs {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDATA to value 0xffff_ffff"]
impl crate::Resettable for WDATArs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
