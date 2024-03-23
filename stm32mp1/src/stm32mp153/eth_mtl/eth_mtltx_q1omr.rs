#[doc = "Register `ETH_MTLTxQ1OMR` reader"]
pub type R = crate::R<ETH_MTLTX_Q1OMRrs>;
#[doc = "Register `ETH_MTLTxQ1OMR` writer"]
pub type W = crate::W<ETH_MTLTX_Q1OMRrs>;
#[doc = "Field `FTQ` reader - FTQ"]
pub type FTQ_R = crate::BitReader;
#[doc = "Field `FTQ` writer - FTQ"]
pub type FTQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSF` reader - TSF"]
pub type TSF_R = crate::BitReader;
#[doc = "Field `TSF` writer - TSF"]
pub type TSF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXQEN` reader - TXQEN"]
pub type TXQEN_R = crate::FieldReader;
#[doc = "Field `TXQEN` writer - TXQEN"]
pub type TXQEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TTC` reader - TTC"]
pub type TTC_R = crate::FieldReader;
#[doc = "Field `TTC` writer - TTC"]
pub type TTC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TQS` reader - TQS"]
pub type TQS_R = crate::FieldReader<u16>;
#[doc = "Field `TQS` writer - TQS"]
pub type TQS_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bit 0 - FTQ"]
    #[inline(always)]
    pub fn ftq(&self) -> FTQ_R {
        FTQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TSF"]
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - TXQEN"]
    #[inline(always)]
    pub fn txqen(&self) -> TXQEN_R {
        TXQEN_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - TTC"]
    #[inline(always)]
    pub fn ttc(&self) -> TTC_R {
        TTC_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 16:24 - TQS"]
    #[inline(always)]
    pub fn tqs(&self) -> TQS_R {
        TQS_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - FTQ"]
    #[inline(always)]
    #[must_use]
    pub fn ftq(&mut self) -> FTQ_W<ETH_MTLTX_Q1OMRrs> {
        FTQ_W::new(self, 0)
    }
    #[doc = "Bit 1 - TSF"]
    #[inline(always)]
    #[must_use]
    pub fn tsf(&mut self) -> TSF_W<ETH_MTLTX_Q1OMRrs> {
        TSF_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - TXQEN"]
    #[inline(always)]
    #[must_use]
    pub fn txqen(&mut self) -> TXQEN_W<ETH_MTLTX_Q1OMRrs> {
        TXQEN_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - TTC"]
    #[inline(always)]
    #[must_use]
    pub fn ttc(&mut self) -> TTC_W<ETH_MTLTX_Q1OMRrs> {
        TTC_W::new(self, 4)
    }
    #[doc = "Bits 16:24 - TQS"]
    #[inline(always)]
    #[must_use]
    pub fn tqs(&mut self) -> TQS_W<ETH_MTLTX_Q1OMRrs> {
        TQS_W::new(self, 16)
    }
}
#[doc = "Tx queue 1 operating mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mtltx_q1omr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_mtltx_q1omr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MTLTX_Q1OMRrs;
impl crate::RegisterSpec for ETH_MTLTX_Q1OMRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_mtltx_q1omr::R`](R) reader structure"]
impl crate::Readable for ETH_MTLTX_Q1OMRrs {}
#[doc = "`write(|w| ..)` method takes [`eth_mtltx_q1omr::W`](W) writer structure"]
impl crate::Writable for ETH_MTLTX_Q1OMRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_MTLTxQ1OMR to value 0"]
impl crate::Resettable for ETH_MTLTX_Q1OMRrs {
    const RESET_VALUE: u32 = 0;
}
