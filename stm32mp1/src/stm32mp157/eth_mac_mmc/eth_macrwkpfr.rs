#[doc = "Register `ETH_MACRWKPFR` reader"]
pub type R = crate::R<ETH_MACRWKPFRrs>;
#[doc = "Register `ETH_MACRWKPFR` writer"]
pub type W = crate::W<ETH_MACRWKPFRrs>;
#[doc = "Field `TLPIEN` reader - TLPIEN"]
pub type TLPIEN_R = crate::BitReader;
#[doc = "Field `TLPIEX` reader - TLPIEX"]
pub type TLPIEX_R = crate::BitReader;
#[doc = "Field `RLPIEN` reader - RLPIEN"]
pub type RLPIEN_R = crate::BitReader;
#[doc = "Field `RLPIEX` reader - RLPIEX"]
pub type RLPIEX_R = crate::BitReader;
#[doc = "Field `TLPIST` reader - TLPIST"]
pub type TLPIST_R = crate::BitReader;
#[doc = "Field `RLPIST` reader - RLPIST"]
pub type RLPIST_R = crate::BitReader;
#[doc = "Field `LPIEN` reader - LPIEN"]
pub type LPIEN_R = crate::BitReader;
#[doc = "Field `LPIEN` writer - LPIEN"]
pub type LPIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLS` reader - PLS"]
pub type PLS_R = crate::BitReader;
#[doc = "Field `PLS` writer - PLS"]
pub type PLS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLSEN` reader - PLSEN"]
pub type PLSEN_R = crate::BitReader;
#[doc = "Field `PLSEN` writer - PLSEN"]
pub type PLSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPITXA` reader - LPITXA"]
pub type LPITXA_R = crate::BitReader;
#[doc = "Field `LPITXA` writer - LPITXA"]
pub type LPITXA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPITE` reader - LPITE"]
pub type LPITE_R = crate::BitReader;
#[doc = "Field `LPITE` writer - LPITE"]
pub type LPITE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TLPIEN"]
    #[inline(always)]
    pub fn tlpien(&self) -> TLPIEN_R {
        TLPIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TLPIEX"]
    #[inline(always)]
    pub fn tlpiex(&self) -> TLPIEX_R {
        TLPIEX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RLPIEN"]
    #[inline(always)]
    pub fn rlpien(&self) -> RLPIEN_R {
        RLPIEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RLPIEX"]
    #[inline(always)]
    pub fn rlpiex(&self) -> RLPIEX_R {
        RLPIEX_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - TLPIST"]
    #[inline(always)]
    pub fn tlpist(&self) -> TLPIST_R {
        TLPIST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RLPIST"]
    #[inline(always)]
    pub fn rlpist(&self) -> RLPIST_R {
        RLPIST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - LPIEN"]
    #[inline(always)]
    pub fn lpien(&self) -> LPIEN_R {
        LPIEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PLS"]
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PLSEN"]
    #[inline(always)]
    pub fn plsen(&self) -> PLSEN_R {
        PLSEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - LPITXA"]
    #[inline(always)]
    pub fn lpitxa(&self) -> LPITXA_R {
        LPITXA_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - LPITE"]
    #[inline(always)]
    pub fn lpite(&self) -> LPITE_R {
        LPITE_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - LPIEN"]
    #[inline(always)]
    #[must_use]
    pub fn lpien(&mut self) -> LPIEN_W<ETH_MACRWKPFRrs> {
        LPIEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - PLS"]
    #[inline(always)]
    #[must_use]
    pub fn pls(&mut self) -> PLS_W<ETH_MACRWKPFRrs> {
        PLS_W::new(self, 17)
    }
    #[doc = "Bit 18 - PLSEN"]
    #[inline(always)]
    #[must_use]
    pub fn plsen(&mut self) -> PLSEN_W<ETH_MACRWKPFRrs> {
        PLSEN_W::new(self, 18)
    }
    #[doc = "Bit 19 - LPITXA"]
    #[inline(always)]
    #[must_use]
    pub fn lpitxa(&mut self) -> LPITXA_W<ETH_MACRWKPFRrs> {
        LPITXA_W::new(self, 19)
    }
    #[doc = "Bit 20 - LPITE"]
    #[inline(always)]
    #[must_use]
    pub fn lpite(&mut self) -> LPITE_W<ETH_MACRWKPFRrs> {
        LPITE_W::new(self, 20)
    }
}
#[doc = "The LPI Control and Status Register controls the LPI functions and provides the LPI interrupt status. The status bits are cleared when this register is read.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macrwkpfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macrwkpfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MACRWKPFRrs;
impl crate::RegisterSpec for ETH_MACRWKPFRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_macrwkpfr::R`](R) reader structure"]
impl crate::Readable for ETH_MACRWKPFRrs {}
#[doc = "`write(|w| ..)` method takes [`eth_macrwkpfr::W`](W) writer structure"]
impl crate::Writable for ETH_MACRWKPFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_MACRWKPFR to value 0"]
impl crate::Resettable for ETH_MACRWKPFRrs {
    const RESET_VALUE: u32 = 0;
}
