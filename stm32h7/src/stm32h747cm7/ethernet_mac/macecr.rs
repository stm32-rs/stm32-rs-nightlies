#[doc = "Register `MACECR` reader"]
pub type R = crate::R<MACECRrs>;
#[doc = "Register `MACECR` writer"]
pub type W = crate::W<MACECRrs>;
#[doc = "Field `GPSL` reader - Giant Packet Size Limit"]
pub type GPSL_R = crate::FieldReader<u16>;
#[doc = "Field `GPSL` writer - Giant Packet Size Limit"]
pub type GPSL_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `DCRCC` reader - Disable CRC Checking for Received Packets"]
pub type DCRCC_R = crate::BitReader;
#[doc = "Field `DCRCC` writer - Disable CRC Checking for Received Packets"]
pub type DCRCC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPEN` reader - Slow Protocol Detection Enable"]
pub type SPEN_R = crate::BitReader;
#[doc = "Field `SPEN` writer - Slow Protocol Detection Enable"]
pub type SPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USP` reader - Unicast Slow Protocol Packet Detect"]
pub type USP_R = crate::BitReader;
#[doc = "Field `USP` writer - Unicast Slow Protocol Packet Detect"]
pub type USP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIPGEN` reader - Extended Inter-Packet Gap Enable"]
pub type EIPGEN_R = crate::BitReader;
#[doc = "Field `EIPGEN` writer - Extended Inter-Packet Gap Enable"]
pub type EIPGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIPG` reader - Extended Inter-Packet Gap"]
pub type EIPG_R = crate::FieldReader;
#[doc = "Field `EIPG` writer - Extended Inter-Packet Gap"]
pub type EIPG_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:13 - Giant Packet Size Limit"]
    #[inline(always)]
    pub fn gpsl(&self) -> GPSL_R {
        GPSL_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 16 - Disable CRC Checking for Received Packets"]
    #[inline(always)]
    pub fn dcrcc(&self) -> DCRCC_R {
        DCRCC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Slow Protocol Detection Enable"]
    #[inline(always)]
    pub fn spen(&self) -> SPEN_R {
        SPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Unicast Slow Protocol Packet Detect"]
    #[inline(always)]
    pub fn usp(&self) -> USP_R {
        USP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - Extended Inter-Packet Gap Enable"]
    #[inline(always)]
    pub fn eipgen(&self) -> EIPGEN_R {
        EIPGEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:29 - Extended Inter-Packet Gap"]
    #[inline(always)]
    pub fn eipg(&self) -> EIPG_R {
        EIPG_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:13 - Giant Packet Size Limit"]
    #[inline(always)]
    #[must_use]
    pub fn gpsl(&mut self) -> GPSL_W<MACECRrs> {
        GPSL_W::new(self, 0)
    }
    #[doc = "Bit 16 - Disable CRC Checking for Received Packets"]
    #[inline(always)]
    #[must_use]
    pub fn dcrcc(&mut self) -> DCRCC_W<MACECRrs> {
        DCRCC_W::new(self, 16)
    }
    #[doc = "Bit 17 - Slow Protocol Detection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn spen(&mut self) -> SPEN_W<MACECRrs> {
        SPEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - Unicast Slow Protocol Packet Detect"]
    #[inline(always)]
    #[must_use]
    pub fn usp(&mut self) -> USP_W<MACECRrs> {
        USP_W::new(self, 18)
    }
    #[doc = "Bit 24 - Extended Inter-Packet Gap Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eipgen(&mut self) -> EIPGEN_W<MACECRrs> {
        EIPGEN_W::new(self, 24)
    }
    #[doc = "Bits 25:29 - Extended Inter-Packet Gap"]
    #[inline(always)]
    #[must_use]
    pub fn eipg(&mut self) -> EIPG_W<MACECRrs> {
        EIPG_W::new(self, 25)
    }
}
#[doc = "Extended operating mode configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macecr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macecr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACECRrs;
impl crate::RegisterSpec for MACECRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macecr::R`](R) reader structure"]
impl crate::Readable for MACECRrs {}
#[doc = "`write(|w| ..)` method takes [`macecr::W`](W) writer structure"]
impl crate::Writable for MACECRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACECR to value 0"]
impl crate::Resettable for MACECRrs {
    const RESET_VALUE: u32 = 0;
}
