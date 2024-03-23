#[doc = "Register `DCMI_CWSIZE` reader"]
pub type R = crate::R<DCMI_CWSIZErs>;
#[doc = "Register `DCMI_CWSIZE` writer"]
pub type W = crate::W<DCMI_CWSIZErs>;
#[doc = "Field `CAPCNT` reader - CAPCNT"]
pub type CAPCNT_R = crate::FieldReader<u16>;
#[doc = "Field `CAPCNT` writer - CAPCNT"]
pub type CAPCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `VLINE` reader - VLINE"]
pub type VLINE_R = crate::FieldReader<u16>;
#[doc = "Field `VLINE` writer - VLINE"]
pub type VLINE_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - CAPCNT"]
    #[inline(always)]
    pub fn capcnt(&self) -> CAPCNT_R {
        CAPCNT_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - VLINE"]
    #[inline(always)]
    pub fn vline(&self) -> VLINE_R {
        VLINE_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - CAPCNT"]
    #[inline(always)]
    #[must_use]
    pub fn capcnt(&mut self) -> CAPCNT_W<DCMI_CWSIZErs> {
        CAPCNT_W::new(self, 0)
    }
    #[doc = "Bits 16:29 - VLINE"]
    #[inline(always)]
    #[must_use]
    pub fn vline(&mut self) -> VLINE_W<DCMI_CWSIZErs> {
        VLINE_W::new(self, 16)
    }
}
#[doc = "DCMI crop window size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcmi_cwsize::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcmi_cwsize::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCMI_CWSIZErs;
impl crate::RegisterSpec for DCMI_CWSIZErs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcmi_cwsize::R`](R) reader structure"]
impl crate::Readable for DCMI_CWSIZErs {}
#[doc = "`write(|w| ..)` method takes [`dcmi_cwsize::W`](W) writer structure"]
impl crate::Writable for DCMI_CWSIZErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCMI_CWSIZE to value 0"]
impl crate::Resettable for DCMI_CWSIZErs {
    const RESET_VALUE: u32 = 0;
}
