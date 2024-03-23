#[doc = "Register `MCR` reader"]
pub type R = crate::R<MCRrs>;
#[doc = "Register `MCR` writer"]
pub type W = crate::W<MCRrs>;
#[doc = "Field `CMDM` reader - CMDM"]
pub type CMDM_R = crate::BitReader;
#[doc = "Field `CMDM` writer - CMDM"]
pub type CMDM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CMDM"]
    #[inline(always)]
    pub fn cmdm(&self) -> CMDM_R {
        CMDM_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CMDM"]
    #[inline(always)]
    #[must_use]
    pub fn cmdm(&mut self) -> CMDM_W<MCRrs> {
        CMDM_W::new(self, 0)
    }
}
#[doc = "DSI Host mode configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCRrs;
impl crate::RegisterSpec for MCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcr::R`](R) reader structure"]
impl crate::Readable for MCRrs {}
#[doc = "`write(|w| ..)` method takes [`mcr::W`](W) writer structure"]
impl crate::Writable for MCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCR to value 0x01"]
impl crate::Resettable for MCRrs {
    const RESET_VALUE: u32 = 0x01;
}
