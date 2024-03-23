#[doc = "Register `USBPHYC_MISC` reader"]
pub type R = crate::R<USBPHYC_MISCrs>;
#[doc = "Register `USBPHYC_MISC` writer"]
pub type W = crate::W<USBPHYC_MISCrs>;
#[doc = "Field `SWITHOST` reader - SWITHOST"]
pub type SWITHOST_R = crate::BitReader;
#[doc = "Field `SWITHOST` writer - SWITHOST"]
pub type SWITHOST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PPCKDIS` reader - PPCKDIS"]
pub type PPCKDIS_R = crate::FieldReader;
#[doc = "Field `PPCKDIS` writer - PPCKDIS"]
pub type PPCKDIS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - SWITHOST"]
    #[inline(always)]
    pub fn swithost(&self) -> SWITHOST_R {
        SWITHOST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - PPCKDIS"]
    #[inline(always)]
    pub fn ppckdis(&self) -> PPCKDIS_R {
        PPCKDIS_R::new(((self.bits >> 1) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - SWITHOST"]
    #[inline(always)]
    #[must_use]
    pub fn swithost(&mut self) -> SWITHOST_W<USBPHYC_MISCrs> {
        SWITHOST_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - PPCKDIS"]
    #[inline(always)]
    #[must_use]
    pub fn ppckdis(&mut self) -> PPCKDIS_W<USBPHYC_MISCrs> {
        PPCKDIS_W::new(self, 1)
    }
}
#[doc = "This register is used to control the switch between controllers for the HS PHY.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphyc_misc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphyc_misc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBPHYC_MISCrs;
impl crate::RegisterSpec for USBPHYC_MISCrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbphyc_misc::R`](R) reader structure"]
impl crate::Readable for USBPHYC_MISCrs {}
#[doc = "`write(|w| ..)` method takes [`usbphyc_misc::W`](W) writer structure"]
impl crate::Writable for USBPHYC_MISCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USBPHYC_MISC to value 0"]
impl crate::Resettable for USBPHYC_MISCrs {
    const RESET_VALUE: u32 = 0;
}
