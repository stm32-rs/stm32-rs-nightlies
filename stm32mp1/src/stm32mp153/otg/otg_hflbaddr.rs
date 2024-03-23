#[doc = "Register `OTG_HFLBADDR` reader"]
pub type R = crate::R<OTG_HFLBADDRrs>;
#[doc = "Register `OTG_HFLBADDR` writer"]
pub type W = crate::W<OTG_HFLBADDRrs>;
#[doc = "Field `HFLBADDR` reader - HFLBADDR"]
pub type HFLBADDR_R = crate::FieldReader<u32>;
#[doc = "Field `HFLBADDR` writer - HFLBADDR"]
pub type HFLBADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - HFLBADDR"]
    #[inline(always)]
    pub fn hflbaddr(&self) -> HFLBADDR_R {
        HFLBADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - HFLBADDR"]
    #[inline(always)]
    #[must_use]
    pub fn hflbaddr(&mut self) -> HFLBADDR_W<OTG_HFLBADDRrs> {
        HFLBADDR_W::new(self, 0)
    }
}
#[doc = "This register holds the starting address of the frame list information (scatter/gather mode).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hflbaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hflbaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_HFLBADDRrs;
impl crate::RegisterSpec for OTG_HFLBADDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hflbaddr::R`](R) reader structure"]
impl crate::Readable for OTG_HFLBADDRrs {}
#[doc = "`write(|w| ..)` method takes [`otg_hflbaddr::W`](W) writer structure"]
impl crate::Writable for OTG_HFLBADDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTG_HFLBADDR to value 0"]
impl crate::Resettable for OTG_HFLBADDRrs {
    const RESET_VALUE: u32 = 0;
}
