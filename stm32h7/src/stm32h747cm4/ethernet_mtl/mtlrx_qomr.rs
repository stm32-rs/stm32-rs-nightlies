#[doc = "Register `MTLRxQOMR` reader"]
pub type R = crate::R<MTLRX_QOMRrs>;
#[doc = "Register `MTLRxQOMR` writer"]
pub type W = crate::W<MTLRX_QOMRrs>;
#[doc = "Field `RTC` reader - Receive Queue Threshold Control"]
pub type RTC_R = crate::FieldReader;
#[doc = "Field `RTC` writer - Receive Queue Threshold Control"]
pub type RTC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FUP` reader - Forward Undersized Good Packets"]
pub type FUP_R = crate::BitReader;
#[doc = "Field `FUP` writer - Forward Undersized Good Packets"]
pub type FUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEP` reader - Forward Error Packets"]
pub type FEP_R = crate::BitReader;
#[doc = "Field `FEP` writer - Forward Error Packets"]
pub type FEP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSF` reader - Receive Queue Store and Forward"]
pub type RSF_R = crate::BitReader;
#[doc = "Field `RSF` writer - Receive Queue Store and Forward"]
pub type RSF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIS_TCP_EF` reader - Disable Dropping of TCP"]
pub type DIS_TCP_EF_R = crate::BitReader;
#[doc = "Field `DIS_TCP_EF` writer - Disable Dropping of TCP"]
pub type DIS_TCP_EF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EHFC` reader - Enable Hardware Flow Control"]
pub type EHFC_R = crate::BitReader;
#[doc = "Field `EHFC` writer - Enable Hardware Flow Control"]
pub type EHFC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFA` reader - Threshold for Activating Flow Control"]
pub type RFA_R = crate::FieldReader;
#[doc = "Field `RFA` writer - Threshold for Activating Flow Control"]
pub type RFA_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RFD` reader - Threshold for Deactivating Flow Control"]
pub type RFD_R = crate::FieldReader;
#[doc = "Field `RFD` writer - Threshold for Deactivating Flow Control"]
pub type RFD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RQS` reader - Receive Queue Size"]
pub type RQS_R = crate::FieldReader;
#[doc = "Field `RQS` writer - Receive Queue Size"]
pub type RQS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - Receive Queue Threshold Control"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Forward Undersized Good Packets"]
    #[inline(always)]
    pub fn fup(&self) -> FUP_R {
        FUP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Forward Error Packets"]
    #[inline(always)]
    pub fn fep(&self) -> FEP_R {
        FEP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive Queue Store and Forward"]
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Disable Dropping of TCP"]
    #[inline(always)]
    pub fn dis_tcp_ef(&self) -> DIS_TCP_EF_R {
        DIS_TCP_EF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Hardware Flow Control"]
    #[inline(always)]
    pub fn ehfc(&self) -> EHFC_R {
        EHFC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Threshold for Activating Flow Control"]
    #[inline(always)]
    pub fn rfa(&self) -> RFA_R {
        RFA_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 14:16 - Threshold for Deactivating Flow Control"]
    #[inline(always)]
    pub fn rfd(&self) -> RFD_R {
        RFD_R::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Receive Queue Size"]
    #[inline(always)]
    pub fn rqs(&self) -> RQS_R {
        RQS_R::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Receive Queue Threshold Control"]
    #[inline(always)]
    #[must_use]
    pub fn rtc(&mut self) -> RTC_W<MTLRX_QOMRrs> {
        RTC_W::new(self, 0)
    }
    #[doc = "Bit 3 - Forward Undersized Good Packets"]
    #[inline(always)]
    #[must_use]
    pub fn fup(&mut self) -> FUP_W<MTLRX_QOMRrs> {
        FUP_W::new(self, 3)
    }
    #[doc = "Bit 4 - Forward Error Packets"]
    #[inline(always)]
    #[must_use]
    pub fn fep(&mut self) -> FEP_W<MTLRX_QOMRrs> {
        FEP_W::new(self, 4)
    }
    #[doc = "Bit 5 - Receive Queue Store and Forward"]
    #[inline(always)]
    #[must_use]
    pub fn rsf(&mut self) -> RSF_W<MTLRX_QOMRrs> {
        RSF_W::new(self, 5)
    }
    #[doc = "Bit 6 - Disable Dropping of TCP"]
    #[inline(always)]
    #[must_use]
    pub fn dis_tcp_ef(&mut self) -> DIS_TCP_EF_W<MTLRX_QOMRrs> {
        DIS_TCP_EF_W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Hardware Flow Control"]
    #[inline(always)]
    #[must_use]
    pub fn ehfc(&mut self) -> EHFC_W<MTLRX_QOMRrs> {
        EHFC_W::new(self, 7)
    }
    #[doc = "Bits 8:10 - Threshold for Activating Flow Control"]
    #[inline(always)]
    #[must_use]
    pub fn rfa(&mut self) -> RFA_W<MTLRX_QOMRrs> {
        RFA_W::new(self, 8)
    }
    #[doc = "Bits 14:16 - Threshold for Deactivating Flow Control"]
    #[inline(always)]
    #[must_use]
    pub fn rfd(&mut self) -> RFD_W<MTLRX_QOMRrs> {
        RFD_W::new(self, 14)
    }
    #[doc = "Bits 20:22 - Receive Queue Size"]
    #[inline(always)]
    #[must_use]
    pub fn rqs(&mut self) -> RQS_W<MTLRX_QOMRrs> {
        RQS_W::new(self, 20)
    }
}
#[doc = "Rx queue operating mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtlrx_qomr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtlrx_qomr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTLRX_QOMRrs;
impl crate::RegisterSpec for MTLRX_QOMRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtlrx_qomr::R`](R) reader structure"]
impl crate::Readable for MTLRX_QOMRrs {}
#[doc = "`write(|w| ..)` method takes [`mtlrx_qomr::W`](W) writer structure"]
impl crate::Writable for MTLRX_QOMRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MTLRxQOMR to value 0x0070_0000"]
impl crate::Resettable for MTLRX_QOMRrs {
    const RESET_VALUE: u32 = 0x0070_0000;
}
