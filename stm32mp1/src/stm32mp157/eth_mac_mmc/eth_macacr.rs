#[doc = "Register `ETH_MACACR` reader"]
pub type R = crate::R<ETH_MACACRrs>;
#[doc = "Register `ETH_MACACR` writer"]
pub type W = crate::W<ETH_MACACRrs>;
#[doc = "Field `ATSFC` reader - ATSFC"]
pub type ATSFC_R = crate::BitReader;
#[doc = "Field `ATSFC` writer - ATSFC"]
pub type ATSFC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATSEN0` reader - ATSEN0"]
pub type ATSEN0_R = crate::BitReader;
#[doc = "Field `ATSEN0` writer - ATSEN0"]
pub type ATSEN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATSEN1` reader - ATSEN1"]
pub type ATSEN1_R = crate::BitReader;
#[doc = "Field `ATSEN1` writer - ATSEN1"]
pub type ATSEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATSEN2` reader - ATSEN2"]
pub type ATSEN2_R = crate::BitReader;
#[doc = "Field `ATSEN2` writer - ATSEN2"]
pub type ATSEN2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATSEN3` reader - ATSEN3"]
pub type ATSEN3_R = crate::BitReader;
#[doc = "Field `ATSEN3` writer - ATSEN3"]
pub type ATSEN3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ATSFC"]
    #[inline(always)]
    pub fn atsfc(&self) -> ATSFC_R {
        ATSFC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - ATSEN0"]
    #[inline(always)]
    pub fn atsen0(&self) -> ATSEN0_R {
        ATSEN0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ATSEN1"]
    #[inline(always)]
    pub fn atsen1(&self) -> ATSEN1_R {
        ATSEN1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ATSEN2"]
    #[inline(always)]
    pub fn atsen2(&self) -> ATSEN2_R {
        ATSEN2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ATSEN3"]
    #[inline(always)]
    pub fn atsen3(&self) -> ATSEN3_R {
        ATSEN3_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ATSFC"]
    #[inline(always)]
    #[must_use]
    pub fn atsfc(&mut self) -> ATSFC_W<ETH_MACACRrs> {
        ATSFC_W::new(self, 0)
    }
    #[doc = "Bit 4 - ATSEN0"]
    #[inline(always)]
    #[must_use]
    pub fn atsen0(&mut self) -> ATSEN0_W<ETH_MACACRrs> {
        ATSEN0_W::new(self, 4)
    }
    #[doc = "Bit 5 - ATSEN1"]
    #[inline(always)]
    #[must_use]
    pub fn atsen1(&mut self) -> ATSEN1_W<ETH_MACACRrs> {
        ATSEN1_W::new(self, 5)
    }
    #[doc = "Bit 6 - ATSEN2"]
    #[inline(always)]
    #[must_use]
    pub fn atsen2(&mut self) -> ATSEN2_W<ETH_MACACRrs> {
        ATSEN2_W::new(self, 6)
    }
    #[doc = "Bit 7 - ATSEN3"]
    #[inline(always)]
    #[must_use]
    pub fn atsen3(&mut self) -> ATSEN3_W<ETH_MACACRrs> {
        ATSEN3_W::new(self, 7)
    }
}
#[doc = "The Auxiliary Timestamp Control register controls the Auxiliary Timestamp snapshot.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macacr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macacr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MACACRrs;
impl crate::RegisterSpec for ETH_MACACRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_macacr::R`](R) reader structure"]
impl crate::Readable for ETH_MACACRrs {}
#[doc = "`write(|w| ..)` method takes [`eth_macacr::W`](W) writer structure"]
impl crate::Writable for ETH_MACACRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_MACACR to value 0"]
impl crate::Resettable for ETH_MACACRrs {
    const RESET_VALUE: u32 = 0;
}
