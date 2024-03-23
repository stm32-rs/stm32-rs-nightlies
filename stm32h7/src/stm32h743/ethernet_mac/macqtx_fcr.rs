#[doc = "Register `MACQTxFCR` reader"]
pub type R = crate::R<MACQTX_FCRrs>;
#[doc = "Register `MACQTxFCR` writer"]
pub type W = crate::W<MACQTX_FCRrs>;
#[doc = "Field `FCB_BPA` reader - Flow Control Busy or Backpressure Activate"]
pub type FCB_BPA_R = crate::BitReader;
#[doc = "Field `FCB_BPA` writer - Flow Control Busy or Backpressure Activate"]
pub type FCB_BPA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFE` reader - Transmit Flow Control Enable"]
pub type TFE_R = crate::BitReader;
#[doc = "Field `TFE` writer - Transmit Flow Control Enable"]
pub type TFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLT` reader - Pause Low Threshold"]
pub type PLT_R = crate::FieldReader;
#[doc = "Field `PLT` writer - Pause Low Threshold"]
pub type PLT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DZPQ` reader - Disable Zero-Quanta Pause"]
pub type DZPQ_R = crate::BitReader;
#[doc = "Field `DZPQ` writer - Disable Zero-Quanta Pause"]
pub type DZPQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PT` reader - Pause Time"]
pub type PT_R = crate::FieldReader<u16>;
#[doc = "Field `PT` writer - Pause Time"]
pub type PT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Flow Control Busy or Backpressure Activate"]
    #[inline(always)]
    pub fn fcb_bpa(&self) -> FCB_BPA_R {
        FCB_BPA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Flow Control Enable"]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Pause Low Threshold"]
    #[inline(always)]
    pub fn plt(&self) -> PLT_R {
        PLT_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Disable Zero-Quanta Pause"]
    #[inline(always)]
    pub fn dzpq(&self) -> DZPQ_R {
        DZPQ_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Pause Time"]
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Flow Control Busy or Backpressure Activate"]
    #[inline(always)]
    #[must_use]
    pub fn fcb_bpa(&mut self) -> FCB_BPA_W<MACQTX_FCRrs> {
        FCB_BPA_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Flow Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tfe(&mut self) -> TFE_W<MACQTX_FCRrs> {
        TFE_W::new(self, 1)
    }
    #[doc = "Bits 4:6 - Pause Low Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn plt(&mut self) -> PLT_W<MACQTX_FCRrs> {
        PLT_W::new(self, 4)
    }
    #[doc = "Bit 7 - Disable Zero-Quanta Pause"]
    #[inline(always)]
    #[must_use]
    pub fn dzpq(&mut self) -> DZPQ_W<MACQTX_FCRrs> {
        DZPQ_W::new(self, 7)
    }
    #[doc = "Bits 16:31 - Pause Time"]
    #[inline(always)]
    #[must_use]
    pub fn pt(&mut self) -> PT_W<MACQTX_FCRrs> {
        PT_W::new(self, 16)
    }
}
#[doc = "Tx Queue flow control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macqtx_fcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macqtx_fcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACQTX_FCRrs;
impl crate::RegisterSpec for MACQTX_FCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macqtx_fcr::R`](R) reader structure"]
impl crate::Readable for MACQTX_FCRrs {}
#[doc = "`write(|w| ..)` method takes [`macqtx_fcr::W`](W) writer structure"]
impl crate::Writable for MACQTX_FCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACQTxFCR to value 0"]
impl crate::Resettable for MACQTX_FCRrs {
    const RESET_VALUE: u32 = 0;
}
