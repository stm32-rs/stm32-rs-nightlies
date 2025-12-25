///Register `MACQ0TxFCR` reader
pub type R = crate::R<MACQ0TX_FCRrs>;
///Register `MACQ0TxFCR` writer
pub type W = crate::W<MACQ0TX_FCRrs>;
///Field `FCB_BPA` reader - FCB_BPA
pub type FCB_BPA_R = crate::BitReader;
///Field `FCB_BPA` writer - FCB_BPA
pub type FCB_BPA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TFE` reader - TFE
pub type TFE_R = crate::BitReader;
///Field `TFE` writer - TFE
pub type TFE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLT` reader - PLT
pub type PLT_R = crate::FieldReader;
///Field `PLT` writer - PLT
pub type PLT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DZPQ` reader - DZPQ
pub type DZPQ_R = crate::BitReader;
///Field `DZPQ` writer - DZPQ
pub type DZPQ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PT` reader - PT
pub type PT_R = crate::FieldReader<u16>;
///Field `PT` writer - PT
pub type PT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bit 0 - FCB_BPA
    #[inline(always)]
    pub fn fcb_bpa(&self) -> FCB_BPA_R {
        FCB_BPA_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TFE
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 4:6 - PLT
    #[inline(always)]
    pub fn plt(&self) -> PLT_R {
        PLT_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - DZPQ
    #[inline(always)]
    pub fn dzpq(&self) -> DZPQ_R {
        DZPQ_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 16:31 - PT
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACQ0TxFCR")
            .field("fcb_bpa", &self.fcb_bpa())
            .field("tfe", &self.tfe())
            .field("plt", &self.plt())
            .field("dzpq", &self.dzpq())
            .field("pt", &self.pt())
            .finish()
    }
}
impl W {
    ///Bit 0 - FCB_BPA
    #[inline(always)]
    pub fn fcb_bpa(&mut self) -> FCB_BPA_W<'_, MACQ0TX_FCRrs> {
        FCB_BPA_W::new(self, 0)
    }
    ///Bit 1 - TFE
    #[inline(always)]
    pub fn tfe(&mut self) -> TFE_W<'_, MACQ0TX_FCRrs> {
        TFE_W::new(self, 1)
    }
    ///Bits 4:6 - PLT
    #[inline(always)]
    pub fn plt(&mut self) -> PLT_W<'_, MACQ0TX_FCRrs> {
        PLT_W::new(self, 4)
    }
    ///Bit 7 - DZPQ
    #[inline(always)]
    pub fn dzpq(&mut self) -> DZPQ_W<'_, MACQ0TX_FCRrs> {
        DZPQ_W::new(self, 7)
    }
    ///Bits 16:31 - PT
    #[inline(always)]
    pub fn pt(&mut self) -> PT_W<'_, MACQ0TX_FCRrs> {
        PT_W::new(self, 16)
    }
}
/**The Flow Control register controls the generation and reception of the Control (Pause Command) packets by the Flow control module of the MAC. A Write to a register with the Busy bit set to 1 triggers the Flow Control block to generate a Pause packet. The fields of the control packet are selected as specified in the 802.3x specification, and the Pause Time value from this register is used in the Pause Time field of the control packet. The Busy bit remains set until the control packet is transferred onto the cable. The application must make sure that the Busy bit is cleared before writing to the register.

You can [`read`](crate::Reg::read) this register and get [`macq0tx_fcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macq0tx_fcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MAC_MMC:MACQ0TxFCR)*/
pub struct MACQ0TX_FCRrs;
impl crate::RegisterSpec for MACQ0TX_FCRrs {
    type Ux = u32;
}
///`read()` method returns [`macq0tx_fcr::R`](R) reader structure
impl crate::Readable for MACQ0TX_FCRrs {}
///`write(|w| ..)` method takes [`macq0tx_fcr::W`](W) writer structure
impl crate::Writable for MACQ0TX_FCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACQ0TxFCR to value 0
impl crate::Resettable for MACQ0TX_FCRrs {}
