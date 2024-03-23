#[doc = "Register `ARG` reader"]
pub type R = crate::R<ARGrs>;
#[doc = "Register `ARG` writer"]
pub type W = crate::W<ARGrs>;
#[doc = "Field `CMDARG` reader - Command argument"]
pub type CMDARG_R = crate::FieldReader<u32>;
#[doc = "Field `CMDARG` writer - Command argument"]
pub type CMDARG_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Command argument"]
    #[inline(always)]
    pub fn cmdarg(&self) -> CMDARG_R {
        CMDARG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Command argument"]
    #[inline(always)]
    #[must_use]
    pub fn cmdarg(&mut self) -> CMDARG_W<ARGrs> {
        CMDARG_W::new(self, 0)
    }
}
#[doc = "argument register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ARGrs;
impl crate::RegisterSpec for ARGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arg::R`](R) reader structure"]
impl crate::Readable for ARGrs {}
#[doc = "`write(|w| ..)` method takes [`arg::W`](W) writer structure"]
impl crate::Writable for ARGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ARG to value 0"]
impl crate::Resettable for ARGrs {
    const RESET_VALUE: u32 = 0;
}
