#[doc = "Register `ETH_MTLOMR` reader"]
pub type R = crate::R<ETH_MTLOMRrs>;
#[doc = "Register `ETH_MTLOMR` writer"]
pub type W = crate::W<ETH_MTLOMRrs>;
#[doc = "Field `DTXSTS` reader - DTXSTS"]
pub type DTXSTS_R = crate::BitReader;
#[doc = "Field `DTXSTS` writer - DTXSTS"]
pub type DTXSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAA` reader - RAA"]
pub type RAA_R = crate::BitReader;
#[doc = "Field `RAA` writer - RAA"]
pub type RAA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCHALG` reader - SCHALG"]
pub type SCHALG_R = crate::FieldReader;
#[doc = "Field `SCHALG` writer - SCHALG"]
pub type SCHALG_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CNTPRST` reader - CNTPRST"]
pub type CNTPRST_R = crate::BitReader;
#[doc = "Field `CNTPRST` writer - CNTPRST"]
pub type CNTPRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTCLR` reader - CNTCLR"]
pub type CNTCLR_R = crate::BitReader;
#[doc = "Field `CNTCLR` writer - CNTCLR"]
pub type CNTCLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - DTXSTS"]
    #[inline(always)]
    pub fn dtxsts(&self) -> DTXSTS_R {
        DTXSTS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RAA"]
    #[inline(always)]
    pub fn raa(&self) -> RAA_R {
        RAA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 5:6 - SCHALG"]
    #[inline(always)]
    pub fn schalg(&self) -> SCHALG_R {
        SCHALG_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 8 - CNTPRST"]
    #[inline(always)]
    pub fn cntprst(&self) -> CNTPRST_R {
        CNTPRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CNTCLR"]
    #[inline(always)]
    pub fn cntclr(&self) -> CNTCLR_R {
        CNTCLR_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - DTXSTS"]
    #[inline(always)]
    #[must_use]
    pub fn dtxsts(&mut self) -> DTXSTS_W<ETH_MTLOMRrs> {
        DTXSTS_W::new(self, 1)
    }
    #[doc = "Bit 2 - RAA"]
    #[inline(always)]
    #[must_use]
    pub fn raa(&mut self) -> RAA_W<ETH_MTLOMRrs> {
        RAA_W::new(self, 2)
    }
    #[doc = "Bits 5:6 - SCHALG"]
    #[inline(always)]
    #[must_use]
    pub fn schalg(&mut self) -> SCHALG_W<ETH_MTLOMRrs> {
        SCHALG_W::new(self, 5)
    }
    #[doc = "Bit 8 - CNTPRST"]
    #[inline(always)]
    #[must_use]
    pub fn cntprst(&mut self) -> CNTPRST_W<ETH_MTLOMRrs> {
        CNTPRST_W::new(self, 8)
    }
    #[doc = "Bit 9 - CNTCLR"]
    #[inline(always)]
    #[must_use]
    pub fn cntclr(&mut self) -> CNTCLR_W<ETH_MTLOMRrs> {
        CNTCLR_W::new(self, 9)
    }
}
#[doc = "The Operating Mode register establishes the Transmit and Receive operating modes and commands.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mtlomr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_mtlomr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MTLOMRrs;
impl crate::RegisterSpec for ETH_MTLOMRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_mtlomr::R`](R) reader structure"]
impl crate::Readable for ETH_MTLOMRrs {}
#[doc = "`write(|w| ..)` method takes [`eth_mtlomr::W`](W) writer structure"]
impl crate::Writable for ETH_MTLOMRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_MTLOMR to value 0"]
impl crate::Resettable for ETH_MTLOMRrs {
    const RESET_VALUE: u32 = 0;
}
