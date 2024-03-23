#[doc = "Register `MMCRIMR` reader"]
pub type R = crate::R<MMCRIMRrs>;
#[doc = "Register `MMCRIMR` writer"]
pub type W = crate::W<MMCRIMRrs>;
#[doc = "Received frame CRC error mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFCEM {
    #[doc = "0: Received-crc-error counter half-full interrupt enabled"]
    Unmasked = 0,
    #[doc = "1: Received-crc-error counter half-full interrupt disabled"]
    Masked = 1,
}
impl From<RFCEM> for bool {
    #[inline(always)]
    fn from(variant: RFCEM) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFCEM` reader - Received frame CRC error mask"]
pub type RFCEM_R = crate::BitReader<RFCEM>;
impl RFCEM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RFCEM {
        match self.bits {
            false => RFCEM::Unmasked,
            true => RFCEM::Masked,
        }
    }
    #[doc = "Received-crc-error counter half-full interrupt enabled"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == RFCEM::Unmasked
    }
    #[doc = "Received-crc-error counter half-full interrupt disabled"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == RFCEM::Masked
    }
}
#[doc = "Field `RFCEM` writer - Received frame CRC error mask"]
pub type RFCEM_W<'a, REG> = crate::BitWriter<'a, REG, RFCEM>;
impl<'a, REG> RFCEM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Received-crc-error counter half-full interrupt enabled"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(RFCEM::Unmasked)
    }
    #[doc = "Received-crc-error counter half-full interrupt disabled"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(RFCEM::Masked)
    }
}
#[doc = "Received frames alignment error mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFAEM {
    #[doc = "0: Received-alignment-error counter half-full interrupt enabled"]
    Unmasked = 0,
    #[doc = "1: Received-alignment-error counter half-full interrupt disabled"]
    Masked = 1,
}
impl From<RFAEM> for bool {
    #[inline(always)]
    fn from(variant: RFAEM) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFAEM` reader - Received frames alignment error mask"]
pub type RFAEM_R = crate::BitReader<RFAEM>;
impl RFAEM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RFAEM {
        match self.bits {
            false => RFAEM::Unmasked,
            true => RFAEM::Masked,
        }
    }
    #[doc = "Received-alignment-error counter half-full interrupt enabled"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == RFAEM::Unmasked
    }
    #[doc = "Received-alignment-error counter half-full interrupt disabled"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == RFAEM::Masked
    }
}
#[doc = "Field `RFAEM` writer - Received frames alignment error mask"]
pub type RFAEM_W<'a, REG> = crate::BitWriter<'a, REG, RFAEM>;
impl<'a, REG> RFAEM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Received-alignment-error counter half-full interrupt enabled"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(RFAEM::Unmasked)
    }
    #[doc = "Received-alignment-error counter half-full interrupt disabled"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(RFAEM::Masked)
    }
}
#[doc = "Received good Unicast frames mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RGUFM {
    #[doc = "0: Received-good-unicast counter half-full interrupt enabled"]
    Unmasked = 0,
    #[doc = "1: Received-good-unicast counter half-full interrupt disabled"]
    Masked = 1,
}
impl From<RGUFM> for bool {
    #[inline(always)]
    fn from(variant: RGUFM) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RGUFM` reader - Received good Unicast frames mask"]
pub type RGUFM_R = crate::BitReader<RGUFM>;
impl RGUFM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RGUFM {
        match self.bits {
            false => RGUFM::Unmasked,
            true => RGUFM::Masked,
        }
    }
    #[doc = "Received-good-unicast counter half-full interrupt enabled"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == RGUFM::Unmasked
    }
    #[doc = "Received-good-unicast counter half-full interrupt disabled"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == RGUFM::Masked
    }
}
#[doc = "Field `RGUFM` writer - Received good Unicast frames mask"]
pub type RGUFM_W<'a, REG> = crate::BitWriter<'a, REG, RGUFM>;
impl<'a, REG> RGUFM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Received-good-unicast counter half-full interrupt enabled"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(RGUFM::Unmasked)
    }
    #[doc = "Received-good-unicast counter half-full interrupt disabled"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(RGUFM::Masked)
    }
}
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
    #[doc = "Bit 17 - Received good Unicast frames mask"]
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
    #[doc = "Bit 17 - Received good Unicast frames mask"]
    #[inline(always)]
    #[must_use]
    pub fn rgufm(&mut self) -> RGUFM_W<MMCRIMRrs> {
        RGUFM_W::new(self, 17)
    }
}
#[doc = "Ethernet MMC receive interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmcrimr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmcrimr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
