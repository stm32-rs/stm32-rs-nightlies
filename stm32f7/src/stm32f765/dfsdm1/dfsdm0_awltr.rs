#[doc = "Register `DFSDM0_AWLTR` reader"]
pub type R = crate::R<DFSDM0_AWLTRrs>;
#[doc = "Register `DFSDM0_AWLTR` writer"]
pub type W = crate::W<DFSDM0_AWLTRrs>;
#[doc = "Field `BKAWL` reader - Break signal assignment to analog watchdog low threshold event"]
pub type BKAWL_R = crate::FieldReader;
#[doc = "Field `BKAWL` writer - Break signal assignment to analog watchdog low threshold event"]
pub type BKAWL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AWLT` reader - Analog watchdog low threshold"]
pub type AWLT_R = crate::FieldReader<u32>;
#[doc = "Field `AWLT` writer - Analog watchdog low threshold"]
pub type AWLT_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:3 - Break signal assignment to analog watchdog low threshold event"]
    #[inline(always)]
    pub fn bkawl(&self) -> BKAWL_R {
        BKAWL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:31 - Analog watchdog low threshold"]
    #[inline(always)]
    pub fn awlt(&self) -> AWLT_R {
        AWLT_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - Break signal assignment to analog watchdog low threshold event"]
    #[inline(always)]
    #[must_use]
    pub fn bkawl(&mut self) -> BKAWL_W<DFSDM0_AWLTRrs> {
        BKAWL_W::new(self, 0)
    }
    #[doc = "Bits 8:31 - Analog watchdog low threshold"]
    #[inline(always)]
    #[must_use]
    pub fn awlt(&mut self) -> AWLT_W<DFSDM0_AWLTRrs> {
        AWLT_W::new(self, 8)
    }
}
#[doc = "DFSDM analog watchdog low threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm0_awltr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm0_awltr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DFSDM0_AWLTRrs;
impl crate::RegisterSpec for DFSDM0_AWLTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsdm0_awltr::R`](R) reader structure"]
impl crate::Readable for DFSDM0_AWLTRrs {}
#[doc = "`write(|w| ..)` method takes [`dfsdm0_awltr::W`](W) writer structure"]
impl crate::Writable for DFSDM0_AWLTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DFSDM0_AWLTR to value 0"]
impl crate::Resettable for DFSDM0_AWLTRrs {
    const RESET_VALUE: u32 = 0;
}
