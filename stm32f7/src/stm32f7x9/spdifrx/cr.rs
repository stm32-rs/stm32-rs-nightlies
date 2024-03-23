#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `SPDIFEN` reader - Peripheral Block Enable"]
pub type SPDIFEN_R = crate::FieldReader;
#[doc = "Field `SPDIFEN` writer - Peripheral Block Enable"]
pub type SPDIFEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RXDMAEN` reader - Receiver DMA ENable for data flow"]
pub type RXDMAEN_R = crate::BitReader;
#[doc = "Field `RXDMAEN` writer - Receiver DMA ENable for data flow"]
pub type RXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSTEO` reader - STerEO Mode"]
pub type RXSTEO_R = crate::BitReader;
#[doc = "Field `RXSTEO` writer - STerEO Mode"]
pub type RXSTEO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRFMT` reader - RX Data format"]
pub type DRFMT_R = crate::FieldReader;
#[doc = "Field `DRFMT` writer - RX Data format"]
pub type DRFMT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PMSK` reader - Mask Parity error bit"]
pub type PMSK_R = crate::BitReader;
#[doc = "Field `PMSK` writer - Mask Parity error bit"]
pub type PMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMSK` reader - Mask of Validity bit"]
pub type VMSK_R = crate::BitReader;
#[doc = "Field `VMSK` writer - Mask of Validity bit"]
pub type VMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CUMSK` reader - Mask of channel status and user bits"]
pub type CUMSK_R = crate::BitReader;
#[doc = "Field `CUMSK` writer - Mask of channel status and user bits"]
pub type CUMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTMSK` reader - Mask of Preamble Type bits"]
pub type PTMSK_R = crate::BitReader;
#[doc = "Field `PTMSK` writer - Mask of Preamble Type bits"]
pub type PTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBDMAEN` reader - Control Buffer DMA ENable for control flow"]
pub type CBDMAEN_R = crate::BitReader;
#[doc = "Field `CBDMAEN` writer - Control Buffer DMA ENable for control flow"]
pub type CBDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHSEL` reader - Channel Selection"]
pub type CHSEL_R = crate::BitReader;
#[doc = "Field `CHSEL` writer - Channel Selection"]
pub type CHSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NBTR` reader - Maximum allowed re-tries during synchronization phase"]
pub type NBTR_R = crate::FieldReader;
#[doc = "Field `NBTR` writer - Maximum allowed re-tries during synchronization phase"]
pub type NBTR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WFA` reader - Wait For Activity"]
pub type WFA_R = crate::BitReader;
#[doc = "Field `WFA` writer - Wait For Activity"]
pub type WFA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INSEL` reader - input selection"]
pub type INSEL_R = crate::FieldReader;
#[doc = "Field `INSEL` writer - input selection"]
pub type INSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - Peripheral Block Enable"]
    #[inline(always)]
    pub fn spdifen(&self) -> SPDIFEN_R {
        SPDIFEN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Receiver DMA ENable for data flow"]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - STerEO Mode"]
    #[inline(always)]
    pub fn rxsteo(&self) -> RXSTEO_R {
        RXSTEO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - RX Data format"]
    #[inline(always)]
    pub fn drfmt(&self) -> DRFMT_R {
        DRFMT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Mask Parity error bit"]
    #[inline(always)]
    pub fn pmsk(&self) -> PMSK_R {
        PMSK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Mask of Validity bit"]
    #[inline(always)]
    pub fn vmsk(&self) -> VMSK_R {
        VMSK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Mask of channel status and user bits"]
    #[inline(always)]
    pub fn cumsk(&self) -> CUMSK_R {
        CUMSK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Mask of Preamble Type bits"]
    #[inline(always)]
    pub fn ptmsk(&self) -> PTMSK_R {
        PTMSK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Control Buffer DMA ENable for control flow"]
    #[inline(always)]
    pub fn cbdmaen(&self) -> CBDMAEN_R {
        CBDMAEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel Selection"]
    #[inline(always)]
    pub fn chsel(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Maximum allowed re-tries during synchronization phase"]
    #[inline(always)]
    pub fn nbtr(&self) -> NBTR_R {
        NBTR_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Wait For Activity"]
    #[inline(always)]
    pub fn wfa(&self) -> WFA_R {
        WFA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:18 - input selection"]
    #[inline(always)]
    pub fn insel(&self) -> INSEL_R {
        INSEL_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Peripheral Block Enable"]
    #[inline(always)]
    #[must_use]
    pub fn spdifen(&mut self) -> SPDIFEN_W<CRrs> {
        SPDIFEN_W::new(self, 0)
    }
    #[doc = "Bit 2 - Receiver DMA ENable for data flow"]
    #[inline(always)]
    #[must_use]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W<CRrs> {
        RXDMAEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - STerEO Mode"]
    #[inline(always)]
    #[must_use]
    pub fn rxsteo(&mut self) -> RXSTEO_W<CRrs> {
        RXSTEO_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - RX Data format"]
    #[inline(always)]
    #[must_use]
    pub fn drfmt(&mut self) -> DRFMT_W<CRrs> {
        DRFMT_W::new(self, 4)
    }
    #[doc = "Bit 6 - Mask Parity error bit"]
    #[inline(always)]
    #[must_use]
    pub fn pmsk(&mut self) -> PMSK_W<CRrs> {
        PMSK_W::new(self, 6)
    }
    #[doc = "Bit 7 - Mask of Validity bit"]
    #[inline(always)]
    #[must_use]
    pub fn vmsk(&mut self) -> VMSK_W<CRrs> {
        VMSK_W::new(self, 7)
    }
    #[doc = "Bit 8 - Mask of channel status and user bits"]
    #[inline(always)]
    #[must_use]
    pub fn cumsk(&mut self) -> CUMSK_W<CRrs> {
        CUMSK_W::new(self, 8)
    }
    #[doc = "Bit 9 - Mask of Preamble Type bits"]
    #[inline(always)]
    #[must_use]
    pub fn ptmsk(&mut self) -> PTMSK_W<CRrs> {
        PTMSK_W::new(self, 9)
    }
    #[doc = "Bit 10 - Control Buffer DMA ENable for control flow"]
    #[inline(always)]
    #[must_use]
    pub fn cbdmaen(&mut self) -> CBDMAEN_W<CRrs> {
        CBDMAEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Channel Selection"]
    #[inline(always)]
    #[must_use]
    pub fn chsel(&mut self) -> CHSEL_W<CRrs> {
        CHSEL_W::new(self, 11)
    }
    #[doc = "Bits 12:13 - Maximum allowed re-tries during synchronization phase"]
    #[inline(always)]
    #[must_use]
    pub fn nbtr(&mut self) -> NBTR_W<CRrs> {
        NBTR_W::new(self, 12)
    }
    #[doc = "Bit 14 - Wait For Activity"]
    #[inline(always)]
    #[must_use]
    pub fn wfa(&mut self) -> WFA_W<CRrs> {
        WFA_W::new(self, 14)
    }
    #[doc = "Bits 16:18 - input selection"]
    #[inline(always)]
    #[must_use]
    pub fn insel(&mut self) -> INSEL_W<CRrs> {
        INSEL_W::new(self, 16)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CRrs {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
