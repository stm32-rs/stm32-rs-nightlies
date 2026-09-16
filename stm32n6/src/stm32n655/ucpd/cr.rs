///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `TXMODE` reader - Type of Tx packet
pub type TXMODE_R = crate::FieldReader;
///Field `TXMODE` writer - Type of Tx packet
pub type TXMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TXSEND` reader - Command to send a Tx packet
pub type TXSEND_R = crate::BitReader;
///Field `TXSEND` writer - Command to send a Tx packet
pub type TXSEND_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXHRST` reader - Command to send a Tx Hard Reset
pub type TXHRST_R = crate::BitReader;
///Field `TXHRST` writer - Command to send a Tx Hard Reset
pub type TXHRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXMODE` reader - Receiver mode
pub type RXMODE_R = crate::BitReader;
///Field `RXMODE` writer - Receiver mode
pub type RXMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PHYRXEN` reader - USB Power Delivery receiver enable
pub type PHYRXEN_R = crate::BitReader;
///Field `PHYRXEN` writer - USB Power Delivery receiver enable
pub type PHYRXEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PHYCCSEL` reader - CC1/CC2 line selector for USB Power Delivery signaling
pub type PHYCCSEL_R = crate::BitReader;
///Field `PHYCCSEL` writer - CC1/CC2 line selector for USB Power Delivery signaling
pub type PHYCCSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ANASUBMODE` reader - Analog PHY sub-mode
pub type ANASUBMODE_R = crate::FieldReader;
///Field `ANASUBMODE` writer - Analog PHY sub-mode
pub type ANASUBMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ANAMODE` reader - Analog PHY operating mode
pub type ANAMODE_R = crate::BitReader;
///Field `ANAMODE` writer - Analog PHY operating mode
pub type ANAMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCENABLE` reader - CC line enable
pub type CCENABLE_R = crate::FieldReader;
///Field `CCENABLE` writer - CC line enable
pub type CCENABLE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `FRSRXEN` reader - FRS event detection enable
pub type FRSRXEN_R = crate::BitReader;
///Field `FRSRXEN` writer - FRS event detection enable
pub type FRSRXEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FRSTX` reader - FRS Tx signaling enable.
pub type FRSTX_R = crate::BitReader;
///Field `FRSTX` writer - FRS Tx signaling enable.
pub type FRSTX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDCH` reader - Rdch condition drive
pub type RDCH_R = crate::BitReader;
///Field `RDCH` writer - Rdch condition drive
pub type RDCH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1TCDIS` reader - CC1 Type-C detector disable
pub type CC1TCDIS_R = crate::BitReader;
///Field `CC1TCDIS` writer - CC1 Type-C detector disable
pub type CC1TCDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2TCDIS` reader - CC2 Type-C detector disable
pub type CC2TCDIS_R = crate::BitReader;
///Field `CC2TCDIS` writer - CC2 Type-C detector disable
pub type CC2TCDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - Type of Tx packet
    #[inline(always)]
    pub fn txmode(&self) -> TXMODE_R {
        TXMODE_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - Command to send a Tx packet
    #[inline(always)]
    pub fn txsend(&self) -> TXSEND_R {
        TXSEND_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Command to send a Tx Hard Reset
    #[inline(always)]
    pub fn txhrst(&self) -> TXHRST_R {
        TXHRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Receiver mode
    #[inline(always)]
    pub fn rxmode(&self) -> RXMODE_R {
        RXMODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - USB Power Delivery receiver enable
    #[inline(always)]
    pub fn phyrxen(&self) -> PHYRXEN_R {
        PHYRXEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CC1/CC2 line selector for USB Power Delivery signaling
    #[inline(always)]
    pub fn phyccsel(&self) -> PHYCCSEL_R {
        PHYCCSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 7:8 - Analog PHY sub-mode
    #[inline(always)]
    pub fn anasubmode(&self) -> ANASUBMODE_R {
        ANASUBMODE_R::new(((self.bits >> 7) & 3) as u8)
    }
    ///Bit 9 - Analog PHY operating mode
    #[inline(always)]
    pub fn anamode(&self) -> ANAMODE_R {
        ANAMODE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 10:11 - CC line enable
    #[inline(always)]
    pub fn ccenable(&self) -> CCENABLE_R {
        CCENABLE_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bit 16 - FRS event detection enable
    #[inline(always)]
    pub fn frsrxen(&self) -> FRSRXEN_R {
        FRSRXEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - FRS Tx signaling enable.
    #[inline(always)]
    pub fn frstx(&self) -> FRSTX_R {
        FRSTX_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Rdch condition drive
    #[inline(always)]
    pub fn rdch(&self) -> RDCH_R {
        RDCH_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - CC1 Type-C detector disable
    #[inline(always)]
    pub fn cc1tcdis(&self) -> CC1TCDIS_R {
        CC1TCDIS_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - CC2 Type-C detector disable
    #[inline(always)]
    pub fn cc2tcdis(&self) -> CC2TCDIS_R {
        CC2TCDIS_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("txmode", &self.txmode())
            .field("txsend", &self.txsend())
            .field("txhrst", &self.txhrst())
            .field("rxmode", &self.rxmode())
            .field("phyrxen", &self.phyrxen())
            .field("phyccsel", &self.phyccsel())
            .field("anasubmode", &self.anasubmode())
            .field("anamode", &self.anamode())
            .field("ccenable", &self.ccenable())
            .field("frsrxen", &self.frsrxen())
            .field("frstx", &self.frstx())
            .field("rdch", &self.rdch())
            .field("cc1tcdis", &self.cc1tcdis())
            .field("cc2tcdis", &self.cc2tcdis())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Type of Tx packet
    #[inline(always)]
    pub fn txmode(&mut self) -> TXMODE_W<'_, CRrs> {
        TXMODE_W::new(self, 0)
    }
    ///Bit 2 - Command to send a Tx packet
    #[inline(always)]
    pub fn txsend(&mut self) -> TXSEND_W<'_, CRrs> {
        TXSEND_W::new(self, 2)
    }
    ///Bit 3 - Command to send a Tx Hard Reset
    #[inline(always)]
    pub fn txhrst(&mut self) -> TXHRST_W<'_, CRrs> {
        TXHRST_W::new(self, 3)
    }
    ///Bit 4 - Receiver mode
    #[inline(always)]
    pub fn rxmode(&mut self) -> RXMODE_W<'_, CRrs> {
        RXMODE_W::new(self, 4)
    }
    ///Bit 5 - USB Power Delivery receiver enable
    #[inline(always)]
    pub fn phyrxen(&mut self) -> PHYRXEN_W<'_, CRrs> {
        PHYRXEN_W::new(self, 5)
    }
    ///Bit 6 - CC1/CC2 line selector for USB Power Delivery signaling
    #[inline(always)]
    pub fn phyccsel(&mut self) -> PHYCCSEL_W<'_, CRrs> {
        PHYCCSEL_W::new(self, 6)
    }
    ///Bits 7:8 - Analog PHY sub-mode
    #[inline(always)]
    pub fn anasubmode(&mut self) -> ANASUBMODE_W<'_, CRrs> {
        ANASUBMODE_W::new(self, 7)
    }
    ///Bit 9 - Analog PHY operating mode
    #[inline(always)]
    pub fn anamode(&mut self) -> ANAMODE_W<'_, CRrs> {
        ANAMODE_W::new(self, 9)
    }
    ///Bits 10:11 - CC line enable
    #[inline(always)]
    pub fn ccenable(&mut self) -> CCENABLE_W<'_, CRrs> {
        CCENABLE_W::new(self, 10)
    }
    ///Bit 16 - FRS event detection enable
    #[inline(always)]
    pub fn frsrxen(&mut self) -> FRSRXEN_W<'_, CRrs> {
        FRSRXEN_W::new(self, 16)
    }
    ///Bit 17 - FRS Tx signaling enable.
    #[inline(always)]
    pub fn frstx(&mut self) -> FRSTX_W<'_, CRrs> {
        FRSTX_W::new(self, 17)
    }
    ///Bit 18 - Rdch condition drive
    #[inline(always)]
    pub fn rdch(&mut self) -> RDCH_W<'_, CRrs> {
        RDCH_W::new(self, 18)
    }
    ///Bit 20 - CC1 Type-C detector disable
    #[inline(always)]
    pub fn cc1tcdis(&mut self) -> CC1TCDIS_W<'_, CRrs> {
        CC1TCDIS_W::new(self, 20)
    }
    ///Bit 21 - CC2 Type-C detector disable
    #[inline(always)]
    pub fn cc2tcdis(&mut self) -> CC2TCDIS_W<'_, CRrs> {
        CC2TCDIS_W::new(self, 21)
    }
}
/**UCPD control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#UCPD:CR)*/
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
