#[doc = "Register `MMCRIR` reader"]
pub type R = crate::R<MMCRIRrs>;
#[doc = "Register `MMCRIR` writer"]
pub type W = crate::W<MMCRIRrs>;
#[doc = "Field `RFCES` reader - Received frames CRC error status"]
pub type RFCES_R = crate::BitReader;
#[doc = "Field `RFCES` writer - Received frames CRC error status"]
pub type RFCES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFAES` reader - Received frames alignment error status"]
pub type RFAES_R = crate::BitReader;
#[doc = "Field `RFAES` writer - Received frames alignment error status"]
pub type RFAES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RGUFS` reader - Received good Unicast frames status"]
pub type RGUFS_R = crate::BitReader;
#[doc = "Field `RGUFS` writer - Received good Unicast frames status"]
pub type RGUFS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - Received frames CRC error status"]
    #[inline(always)]
    pub fn rfces(&self) -> RFCES_R {
        RFCES_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Received frames alignment error status"]
    #[inline(always)]
    pub fn rfaes(&self) -> RFAES_R {
        RFAES_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 17 - Received good Unicast frames status"]
    #[inline(always)]
    pub fn rgufs(&self) -> RGUFS_R {
        RGUFS_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Received frames CRC error status"]
    #[inline(always)]
    #[must_use]
    pub fn rfces(&mut self) -> RFCES_W<MMCRIRrs> {
        RFCES_W::new(self, 5)
    }
    #[doc = "Bit 6 - Received frames alignment error status"]
    #[inline(always)]
    #[must_use]
    pub fn rfaes(&mut self) -> RFAES_W<MMCRIRrs> {
        RFAES_W::new(self, 6)
    }
    #[doc = "Bit 17 - Received good Unicast frames status"]
    #[inline(always)]
    #[must_use]
    pub fn rgufs(&mut self) -> RGUFS_W<MMCRIRrs> {
        RGUFS_W::new(self, 17)
    }
}
#[doc = "Ethernet MMC receive interrupt register (ETH_MMCRIR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmcrir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmcrir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMCRIRrs;
impl crate::RegisterSpec for MMCRIRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmcrir::R`](R) reader structure"]
impl crate::Readable for MMCRIRrs {}
#[doc = "`write(|w| ..)` method takes [`mmcrir::W`](W) writer structure"]
impl crate::Writable for MMCRIRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMCRIR to value 0"]
impl crate::Resettable for MMCRIRrs {
    const RESET_VALUE: u32 = 0;
}
