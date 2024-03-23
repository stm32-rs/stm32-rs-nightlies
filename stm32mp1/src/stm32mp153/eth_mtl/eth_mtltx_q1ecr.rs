#[doc = "Register `ETH_MTLTxQ1ECR` reader"]
pub type R = crate::R<ETH_MTLTX_Q1ECRrs>;
#[doc = "Register `ETH_MTLTxQ1ECR` writer"]
pub type W = crate::W<ETH_MTLTX_Q1ECRrs>;
#[doc = "Field `AVALG` reader - AVALG"]
pub type AVALG_R = crate::BitReader;
#[doc = "Field `AVALG` writer - AVALG"]
pub type AVALG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC` reader - CC"]
pub type CC_R = crate::BitReader;
#[doc = "Field `CC` writer - CC"]
pub type CC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC` reader - SLC"]
pub type SLC_R = crate::FieldReader;
#[doc = "Field `SLC` writer - SLC"]
pub type SLC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 2 - AVALG"]
    #[inline(always)]
    pub fn avalg(&self) -> AVALG_R {
        AVALG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CC"]
    #[inline(always)]
    pub fn cc(&self) -> CC_R {
        CC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - SLC"]
    #[inline(always)]
    pub fn slc(&self) -> SLC_R {
        SLC_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - AVALG"]
    #[inline(always)]
    #[must_use]
    pub fn avalg(&mut self) -> AVALG_W<ETH_MTLTX_Q1ECRrs> {
        AVALG_W::new(self, 2)
    }
    #[doc = "Bit 3 - CC"]
    #[inline(always)]
    #[must_use]
    pub fn cc(&mut self) -> CC_W<ETH_MTLTX_Q1ECRrs> {
        CC_W::new(self, 3)
    }
    #[doc = "Bits 4:6 - SLC"]
    #[inline(always)]
    #[must_use]
    pub fn slc(&mut self) -> SLC_W<ETH_MTLTX_Q1ECRrs> {
        SLC_W::new(self, 4)
    }
}
#[doc = "The Queue ETS Control register controls the enhanced transmission selection operation.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mtltx_q1ecr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_mtltx_q1ecr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MTLTX_Q1ECRrs;
impl crate::RegisterSpec for ETH_MTLTX_Q1ECRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_mtltx_q1ecr::R`](R) reader structure"]
impl crate::Readable for ETH_MTLTX_Q1ECRrs {}
#[doc = "`write(|w| ..)` method takes [`eth_mtltx_q1ecr::W`](W) writer structure"]
impl crate::Writable for ETH_MTLTX_Q1ECRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_MTLTxQ1ECR to value 0"]
impl crate::Resettable for ETH_MTLTX_Q1ECRrs {
    const RESET_VALUE: u32 = 0;
}
