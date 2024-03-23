#[doc = "Register `ETH_MACECR` reader"]
pub type R = crate::R<ETH_MACECRrs>;
#[doc = "Register `ETH_MACECR` writer"]
pub type W = crate::W<ETH_MACECRrs>;
#[doc = "Field `GPSL` reader - GPSL"]
pub type GPSL_R = crate::FieldReader<u16>;
#[doc = "Field `GPSL` writer - GPSL"]
pub type GPSL_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `DCRCC` reader - DCRCC"]
pub type DCRCC_R = crate::BitReader;
#[doc = "Field `DCRCC` writer - DCRCC"]
pub type DCRCC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPEN` reader - SPEN"]
pub type SPEN_R = crate::BitReader;
#[doc = "Field `SPEN` writer - SPEN"]
pub type SPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USP` reader - USP"]
pub type USP_R = crate::BitReader;
#[doc = "Field `USP` writer - USP"]
pub type USP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIPGEN` reader - EIPGEN"]
pub type EIPGEN_R = crate::BitReader;
#[doc = "Field `EIPGEN` writer - EIPGEN"]
pub type EIPGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIPG` reader - EIPG"]
pub type EIPG_R = crate::FieldReader;
#[doc = "Field `EIPG` writer - EIPG"]
pub type EIPG_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:13 - GPSL"]
    #[inline(always)]
    pub fn gpsl(&self) -> GPSL_R {
        GPSL_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 16 - DCRCC"]
    #[inline(always)]
    pub fn dcrcc(&self) -> DCRCC_R {
        DCRCC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SPEN"]
    #[inline(always)]
    pub fn spen(&self) -> SPEN_R {
        SPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USP"]
    #[inline(always)]
    pub fn usp(&self) -> USP_R {
        USP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - EIPGEN"]
    #[inline(always)]
    pub fn eipgen(&self) -> EIPGEN_R {
        EIPGEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:29 - EIPG"]
    #[inline(always)]
    pub fn eipg(&self) -> EIPG_R {
        EIPG_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:13 - GPSL"]
    #[inline(always)]
    #[must_use]
    pub fn gpsl(&mut self) -> GPSL_W<ETH_MACECRrs> {
        GPSL_W::new(self, 0)
    }
    #[doc = "Bit 16 - DCRCC"]
    #[inline(always)]
    #[must_use]
    pub fn dcrcc(&mut self) -> DCRCC_W<ETH_MACECRrs> {
        DCRCC_W::new(self, 16)
    }
    #[doc = "Bit 17 - SPEN"]
    #[inline(always)]
    #[must_use]
    pub fn spen(&mut self) -> SPEN_W<ETH_MACECRrs> {
        SPEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - USP"]
    #[inline(always)]
    #[must_use]
    pub fn usp(&mut self) -> USP_W<ETH_MACECRrs> {
        USP_W::new(self, 18)
    }
    #[doc = "Bit 24 - EIPGEN"]
    #[inline(always)]
    #[must_use]
    pub fn eipgen(&mut self) -> EIPGEN_W<ETH_MACECRrs> {
        EIPGEN_W::new(self, 24)
    }
    #[doc = "Bits 25:29 - EIPG"]
    #[inline(always)]
    #[must_use]
    pub fn eipg(&mut self) -> EIPG_W<ETH_MACECRrs> {
        EIPG_W::new(self, 25)
    }
}
#[doc = "The MAC Extended Configuration Register establishes the operating mode of the MAC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macecr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macecr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MACECRrs;
impl crate::RegisterSpec for ETH_MACECRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_macecr::R`](R) reader structure"]
impl crate::Readable for ETH_MACECRrs {}
#[doc = "`write(|w| ..)` method takes [`eth_macecr::W`](W) writer structure"]
impl crate::Writable for ETH_MACECRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_MACECR to value 0"]
impl crate::Resettable for ETH_MACECRrs {
    const RESET_VALUE: u32 = 0;
}
