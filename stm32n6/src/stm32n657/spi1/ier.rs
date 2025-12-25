///Register `IER` reader
pub type R = crate::R<IERrs>;
///Register `IER` writer
pub type W = crate::W<IERrs>;
///Field `RXPIE` reader - RXP interrupt enable
pub type RXPIE_R = crate::BitReader;
///Field `RXPIE` writer - RXP interrupt enable
pub type RXPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXPIE` reader - TXP interrupt enable
pub type TXPIE_R = crate::BitReader;
///Field `TXPIE` writer - TXP interrupt enable
pub type TXPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DXPIE` reader - DXP interrupt enabled
pub type DXPIE_R = crate::BitReader;
///Field `DXPIE` writer - DXP interrupt enabled
pub type DXPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOTIE` reader - EOT, SUSP and TXC interrupt enable
pub type EOTIE_R = crate::BitReader;
///Field `EOTIE` writer - EOT, SUSP and TXC interrupt enable
pub type EOTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXTFIE` reader - TXTFIE interrupt enable
pub type TXTFIE_R = crate::BitReader;
///Field `TXTFIE` writer - TXTFIE interrupt enable
pub type TXTFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UDRIE` reader - UDR interrupt enable
pub type UDRIE_R = crate::BitReader;
///Field `UDRIE` writer - UDR interrupt enable
pub type UDRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OVRIE` reader - OVR interrupt enable
pub type OVRIE_R = crate::BitReader;
///Field `OVRIE` writer - OVR interrupt enable
pub type OVRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCEIE` reader - CRC error interrupt enable
pub type CRCEIE_R = crate::BitReader;
///Field `CRCEIE` writer - CRC error interrupt enable
pub type CRCEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIFREIE` reader - TIFRE interrupt enable
pub type TIFREIE_R = crate::BitReader;
///Field `TIFREIE` writer - TIFRE interrupt enable
pub type TIFREIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MODFIE` reader - mode Fault interrupt enable
pub type MODFIE_R = crate::BitReader;
///Field `MODFIE` writer - mode Fault interrupt enable
pub type MODFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - RXP interrupt enable
    #[inline(always)]
    pub fn rxpie(&self) -> RXPIE_R {
        RXPIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TXP interrupt enable
    #[inline(always)]
    pub fn txpie(&self) -> TXPIE_R {
        TXPIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DXP interrupt enabled
    #[inline(always)]
    pub fn dxpie(&self) -> DXPIE_R {
        DXPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - EOT, SUSP and TXC interrupt enable
    #[inline(always)]
    pub fn eotie(&self) -> EOTIE_R {
        EOTIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TXTFIE interrupt enable
    #[inline(always)]
    pub fn txtfie(&self) -> TXTFIE_R {
        TXTFIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - UDR interrupt enable
    #[inline(always)]
    pub fn udrie(&self) -> UDRIE_R {
        UDRIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - OVR interrupt enable
    #[inline(always)]
    pub fn ovrie(&self) -> OVRIE_R {
        OVRIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CRC error interrupt enable
    #[inline(always)]
    pub fn crceie(&self) -> CRCEIE_R {
        CRCEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - TIFRE interrupt enable
    #[inline(always)]
    pub fn tifreie(&self) -> TIFREIE_R {
        TIFREIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - mode Fault interrupt enable
    #[inline(always)]
    pub fn modfie(&self) -> MODFIE_R {
        MODFIE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER")
            .field("rxpie", &self.rxpie())
            .field("txpie", &self.txpie())
            .field("dxpie", &self.dxpie())
            .field("eotie", &self.eotie())
            .field("txtfie", &self.txtfie())
            .field("udrie", &self.udrie())
            .field("ovrie", &self.ovrie())
            .field("crceie", &self.crceie())
            .field("tifreie", &self.tifreie())
            .field("modfie", &self.modfie())
            .finish()
    }
}
impl W {
    ///Bit 0 - RXP interrupt enable
    #[inline(always)]
    pub fn rxpie(&mut self) -> RXPIE_W<'_, IERrs> {
        RXPIE_W::new(self, 0)
    }
    ///Bit 1 - TXP interrupt enable
    #[inline(always)]
    pub fn txpie(&mut self) -> TXPIE_W<'_, IERrs> {
        TXPIE_W::new(self, 1)
    }
    ///Bit 2 - DXP interrupt enabled
    #[inline(always)]
    pub fn dxpie(&mut self) -> DXPIE_W<'_, IERrs> {
        DXPIE_W::new(self, 2)
    }
    ///Bit 3 - EOT, SUSP and TXC interrupt enable
    #[inline(always)]
    pub fn eotie(&mut self) -> EOTIE_W<'_, IERrs> {
        EOTIE_W::new(self, 3)
    }
    ///Bit 4 - TXTFIE interrupt enable
    #[inline(always)]
    pub fn txtfie(&mut self) -> TXTFIE_W<'_, IERrs> {
        TXTFIE_W::new(self, 4)
    }
    ///Bit 5 - UDR interrupt enable
    #[inline(always)]
    pub fn udrie(&mut self) -> UDRIE_W<'_, IERrs> {
        UDRIE_W::new(self, 5)
    }
    ///Bit 6 - OVR interrupt enable
    #[inline(always)]
    pub fn ovrie(&mut self) -> OVRIE_W<'_, IERrs> {
        OVRIE_W::new(self, 6)
    }
    ///Bit 7 - CRC error interrupt enable
    #[inline(always)]
    pub fn crceie(&mut self) -> CRCEIE_W<'_, IERrs> {
        CRCEIE_W::new(self, 7)
    }
    ///Bit 8 - TIFRE interrupt enable
    #[inline(always)]
    pub fn tifreie(&mut self) -> TIFREIE_W<'_, IERrs> {
        TIFREIE_W::new(self, 8)
    }
    ///Bit 9 - mode Fault interrupt enable
    #[inline(always)]
    pub fn modfie(&mut self) -> MODFIE_W<'_, IERrs> {
        MODFIE_W::new(self, 9)
    }
}
/**SPI/I2S interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SPI1:IER)*/
pub struct IERrs;
impl crate::RegisterSpec for IERrs {
    type Ux = u32;
}
///`read()` method returns [`ier::R`](R) reader structure
impl crate::Readable for IERrs {}
///`write(|w| ..)` method takes [`ier::W`](W) writer structure
impl crate::Writable for IERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IER to value 0
impl crate::Resettable for IERrs {}
