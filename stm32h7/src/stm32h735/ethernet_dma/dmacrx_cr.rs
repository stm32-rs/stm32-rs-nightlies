#[doc = "Register `DMACRxCR` reader"]
pub type R = crate::R<DMACRX_CRrs>;
#[doc = "Register `DMACRxCR` writer"]
pub type W = crate::W<DMACRX_CRrs>;
#[doc = "Field `SR` reader - Start or Stop Receive"]
pub type SR_R = crate::BitReader;
#[doc = "Field `SR` writer - Start or Stop Receive"]
pub type SR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RBSZ` reader - Receive Buffer size"]
pub type RBSZ_R = crate::FieldReader<u16>;
#[doc = "Field `RBSZ` writer - Receive Buffer size"]
pub type RBSZ_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `RXPBL` reader - Receive Programmable Burst Length"]
pub type RXPBL_R = crate::FieldReader;
#[doc = "Field `RXPBL` writer - Receive Programmable Burst Length"]
pub type RXPBL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RPF` reader - DMA Rx Channel Packet Flush"]
pub type RPF_R = crate::BitReader;
#[doc = "Field `RPF` writer - DMA Rx Channel Packet Flush"]
pub type RPF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Start or Stop Receive"]
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:14 - Receive Buffer size"]
    #[inline(always)]
    pub fn rbsz(&self) -> RBSZ_R {
        RBSZ_R::new(((self.bits >> 1) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:21 - Receive Programmable Burst Length"]
    #[inline(always)]
    pub fn rxpbl(&self) -> RXPBL_R {
        RXPBL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - DMA Rx Channel Packet Flush"]
    #[inline(always)]
    pub fn rpf(&self) -> RPF_R {
        RPF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start or Stop Receive"]
    #[inline(always)]
    #[must_use]
    pub fn sr(&mut self) -> SR_W<DMACRX_CRrs> {
        SR_W::new(self, 0)
    }
    #[doc = "Bits 1:14 - Receive Buffer size"]
    #[inline(always)]
    #[must_use]
    pub fn rbsz(&mut self) -> RBSZ_W<DMACRX_CRrs> {
        RBSZ_W::new(self, 1)
    }
    #[doc = "Bits 16:21 - Receive Programmable Burst Length"]
    #[inline(always)]
    #[must_use]
    pub fn rxpbl(&mut self) -> RXPBL_W<DMACRX_CRrs> {
        RXPBL_W::new(self, 16)
    }
    #[doc = "Bit 31 - DMA Rx Channel Packet Flush"]
    #[inline(always)]
    #[must_use]
    pub fn rpf(&mut self) -> RPF_W<DMACRX_CRrs> {
        RPF_W::new(self, 31)
    }
}
#[doc = "Channel receive control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacrx_cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacrx_cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACRX_CRrs;
impl crate::RegisterSpec for DMACRX_CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacrx_cr::R`](R) reader structure"]
impl crate::Readable for DMACRX_CRrs {}
#[doc = "`write(|w| ..)` method takes [`dmacrx_cr::W`](W) writer structure"]
impl crate::Writable for DMACRX_CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACRxCR to value 0"]
impl crate::Resettable for DMACRX_CRrs {
    const RESET_VALUE: u32 = 0;
}
