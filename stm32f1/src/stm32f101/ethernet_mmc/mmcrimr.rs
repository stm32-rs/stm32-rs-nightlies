#[doc = "Register `MMCRIMR` reader"]
pub type R = crate::R<MMCRIMRrs>;
#[doc = "Register `MMCRIMR` writer"]
pub type W = crate::W<MMCRIMRrs>;
#[doc = "Field `RFCEM` reader - Received frame CRC error mask"]
pub type RFCEM_R = crate::BitReader;
#[doc = "Field `RFCEM` writer - Received frame CRC error mask"]
pub type RFCEM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFAEM` reader - Received frames alignment error mask"]
pub type RFAEM_R = crate::BitReader;
#[doc = "Field `RFAEM` writer - Received frames alignment error mask"]
pub type RFAEM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RGUFM` reader - Received good unicast frames mask"]
pub type RGUFM_R = crate::BitReader;
#[doc = "Field `RGUFM` writer - Received good unicast frames mask"]
pub type RGUFM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - Received frame CRC error mask"]
    #[inline(always)]
    pub fn rfcem(&self) -> RFCEM_R {
        RFCEM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Received frames alignment error mask"]
    #[inline(always)]
    pub fn rfaem(&self) -> RFAEM_R {
        RFAEM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 17 - Received good unicast frames mask"]
    #[inline(always)]
    pub fn rgufm(&self) -> RGUFM_R {
        RGUFM_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Received frame CRC error mask"]
    #[inline(always)]
    #[must_use]
    pub fn rfcem(&mut self) -> RFCEM_W<MMCRIMRrs> {
        RFCEM_W::new(self, 5)
    }
    #[doc = "Bit 6 - Received frames alignment error mask"]
    #[inline(always)]
    #[must_use]
    pub fn rfaem(&mut self) -> RFAEM_W<MMCRIMRrs> {
        RFAEM_W::new(self, 6)
    }
    #[doc = "Bit 17 - Received good unicast frames mask"]
    #[inline(always)]
    #[must_use]
    pub fn rgufm(&mut self) -> RGUFM_W<MMCRIMRrs> {
        RGUFM_W::new(self, 17)
    }
}
#[doc = "Ethernet MMC receive interrupt mask register (ETH_MMCRIMR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmcrimr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmcrimr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMCRIMRrs;
impl crate::RegisterSpec for MMCRIMRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmcrimr::R`](R) reader structure"]
impl crate::Readable for MMCRIMRrs {}
#[doc = "`write(|w| ..)` method takes [`mmcrimr::W`](W) writer structure"]
impl crate::Writable for MMCRIMRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMCRIMR to value 0"]
impl crate::Resettable for MMCRIMRrs {
    const RESET_VALUE: u32 = 0;
}
