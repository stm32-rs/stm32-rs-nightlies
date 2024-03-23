#[doc = "Register `NEXTHDPLCR` reader"]
pub type R = crate::R<NEXTHDPLCRrs>;
#[doc = "Register `NEXTHDPLCR` writer"]
pub type W = crate::W<NEXTHDPLCRrs>;
#[doc = "Field `NEXTHDPL` reader - index to point to a higher HDPL than the current one Index to add to the current HDPL to point (through OBK-HDPL) to the next secure storage areas (OBK-HDPL = HDPL + NEXTHDPL). See for more details."]
pub type NEXTHDPL_R = crate::FieldReader;
#[doc = "Field `NEXTHDPL` writer - index to point to a higher HDPL than the current one Index to add to the current HDPL to point (through OBK-HDPL) to the next secure storage areas (OBK-HDPL = HDPL + NEXTHDPL). See for more details."]
pub type NEXTHDPL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - index to point to a higher HDPL than the current one Index to add to the current HDPL to point (through OBK-HDPL) to the next secure storage areas (OBK-HDPL = HDPL + NEXTHDPL). See for more details."]
    #[inline(always)]
    pub fn nexthdpl(&self) -> NEXTHDPL_R {
        NEXTHDPL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - index to point to a higher HDPL than the current one Index to add to the current HDPL to point (through OBK-HDPL) to the next secure storage areas (OBK-HDPL = HDPL + NEXTHDPL). See for more details."]
    #[inline(always)]
    #[must_use]
    pub fn nexthdpl(&mut self) -> NEXTHDPL_W<NEXTHDPLCRrs> {
        NEXTHDPL_W::new(self, 0)
    }
}
#[doc = "SBS next HDPL control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nexthdplcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nexthdplcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NEXTHDPLCRrs;
impl crate::RegisterSpec for NEXTHDPLCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nexthdplcr::R`](R) reader structure"]
impl crate::Readable for NEXTHDPLCRrs {}
#[doc = "`write(|w| ..)` method takes [`nexthdplcr::W`](W) writer structure"]
impl crate::Writable for NEXTHDPLCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NEXTHDPLCR to value 0"]
impl crate::Resettable for NEXTHDPLCRrs {
    const RESET_VALUE: u32 = 0;
}
