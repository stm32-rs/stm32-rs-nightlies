#[doc = "Register `CWSIZE` reader"]
pub type R = crate::R<CWSIZErs>;
#[doc = "Register `CWSIZE` writer"]
pub type W = crate::W<CWSIZErs>;
#[doc = "Field `CAPCNT` reader - Capture count"]
pub type CAPCNT_R = crate::FieldReader<u16>;
#[doc = "Field `CAPCNT` writer - Capture count"]
pub type CAPCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `VLINE` reader - Vertical line count"]
pub type VLINE_R = crate::FieldReader<u16>;
#[doc = "Field `VLINE` writer - Vertical line count"]
pub type VLINE_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Capture count"]
    #[inline(always)]
    pub fn capcnt(&self) -> CAPCNT_R {
        CAPCNT_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Vertical line count"]
    #[inline(always)]
    pub fn vline(&self) -> VLINE_R {
        VLINE_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Capture count"]
    #[inline(always)]
    #[must_use]
    pub fn capcnt(&mut self) -> CAPCNT_W<CWSIZErs> {
        CAPCNT_W::new(self, 0)
    }
    #[doc = "Bits 16:29 - Vertical line count"]
    #[inline(always)]
    #[must_use]
    pub fn vline(&mut self) -> VLINE_W<CWSIZErs> {
        VLINE_W::new(self, 16)
    }
}
#[doc = "crop window size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cwsize::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cwsize::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CWSIZErs;
impl crate::RegisterSpec for CWSIZErs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cwsize::R`](R) reader structure"]
impl crate::Readable for CWSIZErs {}
#[doc = "`write(|w| ..)` method takes [`cwsize::W`](W) writer structure"]
impl crate::Writable for CWSIZErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CWSIZE to value 0"]
impl crate::Resettable for CWSIZErs {
    const RESET_VALUE: u32 = 0;
}
