#[doc = "Register `DMACTxCR` reader"]
pub type R = crate::R<DMACTX_CRrs>;
#[doc = "Register `DMACTxCR` writer"]
pub type W = crate::W<DMACTX_CRrs>;
#[doc = "Field `ST` reader - Start or Stop Transmission Command"]
pub type ST_R = crate::BitReader;
#[doc = "Field `ST` writer - Start or Stop Transmission Command"]
pub type ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSF` reader - Operate on Second Packet"]
pub type OSF_R = crate::BitReader;
#[doc = "Field `OSF` writer - Operate on Second Packet"]
pub type OSF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSE` reader - TCP Segmentation Enabled"]
pub type TSE_R = crate::BitReader;
#[doc = "Field `TSE` writer - TCP Segmentation Enabled"]
pub type TSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXPBL` reader - Transmit Programmable Burst Length"]
pub type TXPBL_R = crate::FieldReader;
#[doc = "Field `TXPBL` writer - Transmit Programmable Burst Length"]
pub type TXPBL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Start or Stop Transmission Command"]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Operate on Second Packet"]
    #[inline(always)]
    pub fn osf(&self) -> OSF_R {
        OSF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 12 - TCP Segmentation Enabled"]
    #[inline(always)]
    pub fn tse(&self) -> TSE_R {
        TSE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:21 - Transmit Programmable Burst Length"]
    #[inline(always)]
    pub fn txpbl(&self) -> TXPBL_R {
        TXPBL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Start or Stop Transmission Command"]
    #[inline(always)]
    #[must_use]
    pub fn st(&mut self) -> ST_W<DMACTX_CRrs> {
        ST_W::new(self, 0)
    }
    #[doc = "Bit 4 - Operate on Second Packet"]
    #[inline(always)]
    #[must_use]
    pub fn osf(&mut self) -> OSF_W<DMACTX_CRrs> {
        OSF_W::new(self, 4)
    }
    #[doc = "Bit 12 - TCP Segmentation Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn tse(&mut self) -> TSE_W<DMACTX_CRrs> {
        TSE_W::new(self, 12)
    }
    #[doc = "Bits 16:21 - Transmit Programmable Burst Length"]
    #[inline(always)]
    #[must_use]
    pub fn txpbl(&mut self) -> TXPBL_W<DMACTX_CRrs> {
        TXPBL_W::new(self, 16)
    }
}
#[doc = "Channel transmit control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmactx_cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmactx_cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACTX_CRrs;
impl crate::RegisterSpec for DMACTX_CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmactx_cr::R`](R) reader structure"]
impl crate::Readable for DMACTX_CRrs {}
#[doc = "`write(|w| ..)` method takes [`dmactx_cr::W`](W) writer structure"]
impl crate::Writable for DMACTX_CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACTxCR to value 0"]
impl crate::Resettable for DMACTX_CRrs {
    const RESET_VALUE: u32 = 0;
}
