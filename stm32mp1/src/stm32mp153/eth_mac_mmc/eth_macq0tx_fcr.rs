#[doc = "Register `ETH_MACQ0TxFCR` reader"]
pub type R = crate::R<ETH_MACQ0TX_FCRrs>;
#[doc = "Register `ETH_MACQ0TxFCR` writer"]
pub type W = crate::W<ETH_MACQ0TX_FCRrs>;
#[doc = "Field `FCB_BPA` reader - FCB_BPA"]
pub type FCB_BPA_R = crate::BitReader;
#[doc = "Field `FCB_BPA` writer - FCB_BPA"]
pub type FCB_BPA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFE` reader - TFE"]
pub type TFE_R = crate::BitReader;
#[doc = "Field `TFE` writer - TFE"]
pub type TFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLT` reader - PLT"]
pub type PLT_R = crate::FieldReader;
#[doc = "Field `PLT` writer - PLT"]
pub type PLT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DZPQ` reader - DZPQ"]
pub type DZPQ_R = crate::BitReader;
#[doc = "Field `DZPQ` writer - DZPQ"]
pub type DZPQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PT` reader - PT"]
pub type PT_R = crate::FieldReader<u16>;
#[doc = "Field `PT` writer - PT"]
pub type PT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - FCB_BPA"]
    #[inline(always)]
    pub fn fcb_bpa(&self) -> FCB_BPA_R {
        FCB_BPA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TFE"]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:6 - PLT"]
    #[inline(always)]
    pub fn plt(&self) -> PLT_R {
        PLT_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - DZPQ"]
    #[inline(always)]
    pub fn dzpq(&self) -> DZPQ_R {
        DZPQ_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:31 - PT"]
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - FCB_BPA"]
    #[inline(always)]
    #[must_use]
    pub fn fcb_bpa(&mut self) -> FCB_BPA_W<ETH_MACQ0TX_FCRrs> {
        FCB_BPA_W::new(self, 0)
    }
    #[doc = "Bit 1 - TFE"]
    #[inline(always)]
    #[must_use]
    pub fn tfe(&mut self) -> TFE_W<ETH_MACQ0TX_FCRrs> {
        TFE_W::new(self, 1)
    }
    #[doc = "Bits 4:6 - PLT"]
    #[inline(always)]
    #[must_use]
    pub fn plt(&mut self) -> PLT_W<ETH_MACQ0TX_FCRrs> {
        PLT_W::new(self, 4)
    }
    #[doc = "Bit 7 - DZPQ"]
    #[inline(always)]
    #[must_use]
    pub fn dzpq(&mut self) -> DZPQ_W<ETH_MACQ0TX_FCRrs> {
        DZPQ_W::new(self, 7)
    }
    #[doc = "Bits 16:31 - PT"]
    #[inline(always)]
    #[must_use]
    pub fn pt(&mut self) -> PT_W<ETH_MACQ0TX_FCRrs> {
        PT_W::new(self, 16)
    }
}
#[doc = "The Flow Control register controls the generation and reception of the Control (Pause Command) packets by the Flow control module of the MAC. A Write to a register with the Busy bit set to 1 triggers the Flow Control block to generate a Pause packet. The fields of the control packet are selected as specified in the 802.3x specification, and the Pause Time value from this register is used in the Pause Time field of the control packet. The Busy bit remains set until the control packet is transferred onto the cable. The application must make sure that the Busy bit is cleared before writing to the register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macq0tx_fcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macq0tx_fcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MACQ0TX_FCRrs;
impl crate::RegisterSpec for ETH_MACQ0TX_FCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_macq0tx_fcr::R`](R) reader structure"]
impl crate::Readable for ETH_MACQ0TX_FCRrs {}
#[doc = "`write(|w| ..)` method takes [`eth_macq0tx_fcr::W`](W) writer structure"]
impl crate::Writable for ETH_MACQ0TX_FCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_MACQ0TxFCR to value 0"]
impl crate::Resettable for ETH_MACQ0TX_FCRrs {
    const RESET_VALUE: u32 = 0;
}
