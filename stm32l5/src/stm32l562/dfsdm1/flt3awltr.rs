#[doc = "Register `FLT3AWLTR` reader"]
pub type R = crate::R<FLT3AWLTRrs>;
#[doc = "Register `FLT3AWLTR` writer"]
pub type W = crate::W<FLT3AWLTRrs>;
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
    pub fn bkawl(&mut self) -> BKAWL_W<FLT3AWLTRrs> {
        BKAWL_W::new(self, 0)
    }
    #[doc = "Bits 8:31 - Analog watchdog low threshold"]
    #[inline(always)]
    #[must_use]
    pub fn awlt(&mut self) -> AWLT_W<FLT3AWLTRrs> {
        AWLT_W::new(self, 8)
    }
}
#[doc = "analog watchdog low threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flt3awltr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flt3awltr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLT3AWLTRrs;
impl crate::RegisterSpec for FLT3AWLTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flt3awltr::R`](R) reader structure"]
impl crate::Readable for FLT3AWLTRrs {}
#[doc = "`write(|w| ..)` method takes [`flt3awltr::W`](W) writer structure"]
impl crate::Writable for FLT3AWLTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLT3AWLTR to value 0"]
impl crate::Resettable for FLT3AWLTRrs {
    const RESET_VALUE: u32 = 0;
}
