///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `SPDIFRXEN` reader - Peripheral Block Enable
pub type SPDIFRXEN_R = crate::FieldReader;
///Field `SPDIFRXEN` writer - Peripheral Block Enable
pub type SPDIFRXEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RXDMAEN` reader - Receiver DMA ENable for data flow
pub type RXDMAEN_R = crate::BitReader;
///Field `RXDMAEN` writer - Receiver DMA ENable for data flow
pub type RXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXSTEO` reader - STerEO Mode
pub type RXSTEO_R = crate::BitReader;
///Field `RXSTEO` writer - STerEO Mode
pub type RXSTEO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DRFMT` reader - RX Data format
pub type DRFMT_R = crate::FieldReader;
///Field `DRFMT` writer - RX Data format
pub type DRFMT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PMSK` reader - Mask Parity error bit
pub type PMSK_R = crate::BitReader;
///Field `PMSK` writer - Mask Parity error bit
pub type PMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VMSK` reader - Mask of Validity bit
pub type VMSK_R = crate::BitReader;
///Field `VMSK` writer - Mask of Validity bit
pub type VMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CUMSK` reader - Mask of channel status and user bits
pub type CUMSK_R = crate::BitReader;
///Field `CUMSK` writer - Mask of channel status and user bits
pub type CUMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PTMSK` reader - Mask of Preamble Type bits
pub type PTMSK_R = crate::BitReader;
///Field `PTMSK` writer - Mask of Preamble Type bits
pub type PTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CBDMAEN` reader - Control Buffer DMA ENable for control flow
pub type CBDMAEN_R = crate::BitReader;
///Field `CBDMAEN` writer - Control Buffer DMA ENable for control flow
pub type CBDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHSEL` reader - Channel Selection
pub type CHSEL_R = crate::BitReader;
///Field `CHSEL` writer - Channel Selection
pub type CHSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NBTR` reader - Maximum allowed re-tries during synchronization phase
pub type NBTR_R = crate::FieldReader;
///Field `NBTR` writer - Maximum allowed re-tries during synchronization phase
pub type NBTR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `WFA` reader - Wait For Activity
pub type WFA_R = crate::BitReader;
///Field `WFA` writer - Wait For Activity
pub type WFA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INSEL` reader - input selection
pub type INSEL_R = crate::FieldReader;
///Field `INSEL` writer - input selection
pub type INSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `CKSEN` reader - Symbol Clock Enable
pub type CKSEN_R = crate::BitReader;
///Field `CKSEN` writer - Symbol Clock Enable
pub type CKSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CKSBKPEN` reader - Backup Symbol Clock Enable
pub type CKSBKPEN_R = crate::BitReader;
///Field `CKSBKPEN` writer - Backup Symbol Clock Enable
pub type CKSBKPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - Peripheral Block Enable
    #[inline(always)]
    pub fn spdifrxen(&self) -> SPDIFRXEN_R {
        SPDIFRXEN_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - Receiver DMA ENable for data flow
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - STerEO Mode
    #[inline(always)]
    pub fn rxsteo(&self) -> RXSTEO_R {
        RXSTEO_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - RX Data format
    #[inline(always)]
    pub fn drfmt(&self) -> DRFMT_R {
        DRFMT_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 6 - Mask Parity error bit
    #[inline(always)]
    pub fn pmsk(&self) -> PMSK_R {
        PMSK_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Mask of Validity bit
    #[inline(always)]
    pub fn vmsk(&self) -> VMSK_R {
        VMSK_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Mask of channel status and user bits
    #[inline(always)]
    pub fn cumsk(&self) -> CUMSK_R {
        CUMSK_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Mask of Preamble Type bits
    #[inline(always)]
    pub fn ptmsk(&self) -> PTMSK_R {
        PTMSK_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Control Buffer DMA ENable for control flow
    #[inline(always)]
    pub fn cbdmaen(&self) -> CBDMAEN_R {
        CBDMAEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Channel Selection
    #[inline(always)]
    pub fn chsel(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:13 - Maximum allowed re-tries during synchronization phase
    #[inline(always)]
    pub fn nbtr(&self) -> NBTR_R {
        NBTR_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 14 - Wait For Activity
    #[inline(always)]
    pub fn wfa(&self) -> WFA_R {
        WFA_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 16:18 - input selection
    #[inline(always)]
    pub fn insel(&self) -> INSEL_R {
        INSEL_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bit 20 - Symbol Clock Enable
    #[inline(always)]
    pub fn cksen(&self) -> CKSEN_R {
        CKSEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Backup Symbol Clock Enable
    #[inline(always)]
    pub fn cksbkpen(&self) -> CKSBKPEN_R {
        CKSBKPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("spdifrxen", &self.spdifrxen())
            .field("rxdmaen", &self.rxdmaen())
            .field("rxsteo", &self.rxsteo())
            .field("drfmt", &self.drfmt())
            .field("pmsk", &self.pmsk())
            .field("vmsk", &self.vmsk())
            .field("cumsk", &self.cumsk())
            .field("ptmsk", &self.ptmsk())
            .field("cbdmaen", &self.cbdmaen())
            .field("chsel", &self.chsel())
            .field("nbtr", &self.nbtr())
            .field("wfa", &self.wfa())
            .field("insel", &self.insel())
            .field("cksen", &self.cksen())
            .field("cksbkpen", &self.cksbkpen())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Peripheral Block Enable
    #[inline(always)]
    pub fn spdifrxen(&mut self) -> SPDIFRXEN_W<'_, CRrs> {
        SPDIFRXEN_W::new(self, 0)
    }
    ///Bit 2 - Receiver DMA ENable for data flow
    #[inline(always)]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W<'_, CRrs> {
        RXDMAEN_W::new(self, 2)
    }
    ///Bit 3 - STerEO Mode
    #[inline(always)]
    pub fn rxsteo(&mut self) -> RXSTEO_W<'_, CRrs> {
        RXSTEO_W::new(self, 3)
    }
    ///Bits 4:5 - RX Data format
    #[inline(always)]
    pub fn drfmt(&mut self) -> DRFMT_W<'_, CRrs> {
        DRFMT_W::new(self, 4)
    }
    ///Bit 6 - Mask Parity error bit
    #[inline(always)]
    pub fn pmsk(&mut self) -> PMSK_W<'_, CRrs> {
        PMSK_W::new(self, 6)
    }
    ///Bit 7 - Mask of Validity bit
    #[inline(always)]
    pub fn vmsk(&mut self) -> VMSK_W<'_, CRrs> {
        VMSK_W::new(self, 7)
    }
    ///Bit 8 - Mask of channel status and user bits
    #[inline(always)]
    pub fn cumsk(&mut self) -> CUMSK_W<'_, CRrs> {
        CUMSK_W::new(self, 8)
    }
    ///Bit 9 - Mask of Preamble Type bits
    #[inline(always)]
    pub fn ptmsk(&mut self) -> PTMSK_W<'_, CRrs> {
        PTMSK_W::new(self, 9)
    }
    ///Bit 10 - Control Buffer DMA ENable for control flow
    #[inline(always)]
    pub fn cbdmaen(&mut self) -> CBDMAEN_W<'_, CRrs> {
        CBDMAEN_W::new(self, 10)
    }
    ///Bit 11 - Channel Selection
    #[inline(always)]
    pub fn chsel(&mut self) -> CHSEL_W<'_, CRrs> {
        CHSEL_W::new(self, 11)
    }
    ///Bits 12:13 - Maximum allowed re-tries during synchronization phase
    #[inline(always)]
    pub fn nbtr(&mut self) -> NBTR_W<'_, CRrs> {
        NBTR_W::new(self, 12)
    }
    ///Bit 14 - Wait For Activity
    #[inline(always)]
    pub fn wfa(&mut self) -> WFA_W<'_, CRrs> {
        WFA_W::new(self, 14)
    }
    ///Bits 16:18 - input selection
    #[inline(always)]
    pub fn insel(&mut self) -> INSEL_W<'_, CRrs> {
        INSEL_W::new(self, 16)
    }
    ///Bit 20 - Symbol Clock Enable
    #[inline(always)]
    pub fn cksen(&mut self) -> CKSEN_W<'_, CRrs> {
        CKSEN_W::new(self, 20)
    }
    ///Bit 21 - Backup Symbol Clock Enable
    #[inline(always)]
    pub fn cksbkpen(&mut self) -> CKSBKPEN_W<'_, CRrs> {
        CKSBKPEN_W::new(self, 21)
    }
}
/**Control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753.html#SPDIFRX:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
