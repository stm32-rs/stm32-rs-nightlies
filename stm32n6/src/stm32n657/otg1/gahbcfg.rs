///Register `GAHBCFG` reader
pub type R = crate::R<GAHBCFGrs>;
///Register `GAHBCFG` writer
pub type W = crate::W<GAHBCFGrs>;
///Field `GINTMSK` reader - Global interrupt mask
pub type GINTMSK_R = crate::BitReader;
///Field `GINTMSK` writer - Global interrupt mask
pub type GINTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HBSTLEN` reader - Burst length/type
pub type HBSTLEN_R = crate::FieldReader;
///Field `HBSTLEN` writer - Burst length/type
pub type HBSTLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DMAEN` reader - DMA enabled
pub type DMAEN_R = crate::BitReader;
///Field `DMAEN` writer - DMA enabled
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFELVL` reader - Tx FIFO empty level
pub type TXFELVL_R = crate::BitReader;
///Field `TXFELVL` writer - Tx FIFO empty level
pub type TXFELVL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PTXFELVL` reader - Periodic Tx FIFO empty level
pub type PTXFELVL_R = crate::BitReader;
///Field `PTXFELVL` writer - Periodic Tx FIFO empty level
pub type PTXFELVL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Global interrupt mask
    #[inline(always)]
    pub fn gintmsk(&self) -> GINTMSK_R {
        GINTMSK_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:4 - Burst length/type
    #[inline(always)]
    pub fn hbstlen(&self) -> HBSTLEN_R {
        HBSTLEN_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    ///Bit 5 - DMA enabled
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - Tx FIFO empty level
    #[inline(always)]
    pub fn txfelvl(&self) -> TXFELVL_R {
        TXFELVL_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Periodic Tx FIFO empty level
    #[inline(always)]
    pub fn ptxfelvl(&self) -> PTXFELVL_R {
        PTXFELVL_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GAHBCFG")
            .field("gintmsk", &self.gintmsk())
            .field("hbstlen", &self.hbstlen())
            .field("dmaen", &self.dmaen())
            .field("txfelvl", &self.txfelvl())
            .field("ptxfelvl", &self.ptxfelvl())
            .finish()
    }
}
impl W {
    ///Bit 0 - Global interrupt mask
    #[inline(always)]
    pub fn gintmsk(&mut self) -> GINTMSK_W<GAHBCFGrs> {
        GINTMSK_W::new(self, 0)
    }
    ///Bits 1:4 - Burst length/type
    #[inline(always)]
    pub fn hbstlen(&mut self) -> HBSTLEN_W<GAHBCFGrs> {
        HBSTLEN_W::new(self, 1)
    }
    ///Bit 5 - DMA enabled
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W<GAHBCFGrs> {
        DMAEN_W::new(self, 5)
    }
    ///Bit 7 - Tx FIFO empty level
    #[inline(always)]
    pub fn txfelvl(&mut self) -> TXFELVL_W<GAHBCFGrs> {
        TXFELVL_W::new(self, 7)
    }
    ///Bit 8 - Periodic Tx FIFO empty level
    #[inline(always)]
    pub fn ptxfelvl(&mut self) -> PTXFELVL_W<GAHBCFGrs> {
        PTXFELVL_W::new(self, 8)
    }
}
/**OTG AHB configuration register

You can [`read`](crate::Reg::read) this register and get [`gahbcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gahbcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#OTG1:GAHBCFG)*/
pub struct GAHBCFGrs;
impl crate::RegisterSpec for GAHBCFGrs {
    type Ux = u32;
}
///`read()` method returns [`gahbcfg::R`](R) reader structure
impl crate::Readable for GAHBCFGrs {}
///`write(|w| ..)` method takes [`gahbcfg::W`](W) writer structure
impl crate::Writable for GAHBCFGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GAHBCFG to value 0
impl crate::Resettable for GAHBCFGrs {}
