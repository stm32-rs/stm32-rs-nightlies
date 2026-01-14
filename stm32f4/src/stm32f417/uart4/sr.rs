///Register `SR` reader
pub type R = crate::R<SRrs>;
///Register `SR` writer
pub type W = crate::W<SRrs>;
///Field `PE` reader - Parity error
pub type PE_R = crate::BitReader;
///Field `FE` reader - Framing error
pub type FE_R = crate::BitReader;
///Field `NF` reader - Noise detected flag
pub type NF_R = crate::BitReader;
///Field `ORE` reader - Overrun error
pub type ORE_R = crate::BitReader;
///Field `IDLE` reader - IDLE line detected
pub type IDLE_R = crate::BitReader;
///Field `RXNE` reader - Read data register not empty
pub type RXNE_R = crate::BitReader;
///Field `RXNE` writer - Read data register not empty
pub type RXNE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TC` reader - Transmission complete
pub type TC_R = crate::BitReader;
///Field `TC` writer - Transmission complete
pub type TC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXE` reader - Transmit data register empty
pub type TXE_R = crate::BitReader;
///Field `LBD` reader - LIN break detection flag
pub type LBD_R = crate::BitReader;
///Field `LBD` writer - LIN break detection flag
pub type LBD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Parity error
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Framing error
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Noise detected flag
    #[inline(always)]
    pub fn nf(&self) -> NF_R {
        NF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Overrun error
    #[inline(always)]
    pub fn ore(&self) -> ORE_R {
        ORE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IDLE line detected
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Read data register not empty
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Transmission complete
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Transmit data register empty
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - LIN break detection flag
    #[inline(always)]
    pub fn lbd(&self) -> LBD_R {
        LBD_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("lbd", &self.lbd())
            .field("txe", &self.txe())
            .field("tc", &self.tc())
            .field("rxne", &self.rxne())
            .field("idle", &self.idle())
            .field("ore", &self.ore())
            .field("nf", &self.nf())
            .field("fe", &self.fe())
            .field("pe", &self.pe())
            .finish()
    }
}
impl W {
    ///Bit 5 - Read data register not empty
    #[inline(always)]
    pub fn rxne(&mut self) -> RXNE_W<'_, SRrs> {
        RXNE_W::new(self, 5)
    }
    ///Bit 6 - Transmission complete
    #[inline(always)]
    pub fn tc(&mut self) -> TC_W<'_, SRrs> {
        TC_W::new(self, 6)
    }
    ///Bit 8 - LIN break detection flag
    #[inline(always)]
    pub fn lbd(&mut self) -> LBD_W<'_, SRrs> {
        LBD_W::new(self, 8)
    }
}
/**Status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#UART4:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`write(|w| ..)` method takes [`sr::W`](W) writer structure
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SR to value 0x00c0_0000
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0x00c0_0000;
}
