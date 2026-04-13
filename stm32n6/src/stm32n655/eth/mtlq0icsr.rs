///Register `MTLQ0ICSR` reader
pub type R = crate::R<MTLQ0ICSRrs>;
///Register `MTLQ0ICSR` writer
pub type W = crate::W<MTLQ0ICSRrs>;
///Field `TXUNFIS` reader - Transmit Queue Underflow Interrupt Status
pub type TXUNFIS_R = crate::BitReader;
///Field `TXUNFIS` writer - Transmit Queue Underflow Interrupt Status
pub type TXUNFIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ABPSIS` reader - Average Bits Per Slot Interrupt Status
pub type ABPSIS_R = crate::BitReader;
///Field `ABPSIS` writer - Average Bits Per Slot Interrupt Status
pub type ABPSIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXUIE` reader - Transmit Queue Underflow Interrupt Enable
pub type TXUIE_R = crate::BitReader;
///Field `TXUIE` writer - Transmit Queue Underflow Interrupt Enable
pub type TXUIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ABPSIE` reader - Average Bits Per Slot Interrupt Enable
pub type ABPSIE_R = crate::BitReader;
///Field `ABPSIE` writer - Average Bits Per Slot Interrupt Enable
pub type ABPSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXOVFIS` reader - Receive Queue Overflow Interrupt Status
pub type RXOVFIS_R = crate::BitReader;
///Field `RXOVFIS` writer - Receive Queue Overflow Interrupt Status
pub type RXOVFIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXOIE` reader - Receive Queue Overflow Interrupt Enable
pub type RXOIE_R = crate::BitReader;
///Field `RXOIE` writer - Receive Queue Overflow Interrupt Enable
pub type RXOIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Transmit Queue Underflow Interrupt Status
    #[inline(always)]
    pub fn txunfis(&self) -> TXUNFIS_R {
        TXUNFIS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Average Bits Per Slot Interrupt Status
    #[inline(always)]
    pub fn abpsis(&self) -> ABPSIS_R {
        ABPSIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - Transmit Queue Underflow Interrupt Enable
    #[inline(always)]
    pub fn txuie(&self) -> TXUIE_R {
        TXUIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Average Bits Per Slot Interrupt Enable
    #[inline(always)]
    pub fn abpsie(&self) -> ABPSIE_R {
        ABPSIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 16 - Receive Queue Overflow Interrupt Status
    #[inline(always)]
    pub fn rxovfis(&self) -> RXOVFIS_R {
        RXOVFIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - Receive Queue Overflow Interrupt Enable
    #[inline(always)]
    pub fn rxoie(&self) -> RXOIE_R {
        RXOIE_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLQ0ICSR")
            .field("txunfis", &self.txunfis())
            .field("abpsis", &self.abpsis())
            .field("txuie", &self.txuie())
            .field("abpsie", &self.abpsie())
            .field("rxovfis", &self.rxovfis())
            .field("rxoie", &self.rxoie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Transmit Queue Underflow Interrupt Status
    #[inline(always)]
    pub fn txunfis(&mut self) -> TXUNFIS_W<'_, MTLQ0ICSRrs> {
        TXUNFIS_W::new(self, 0)
    }
    ///Bit 1 - Average Bits Per Slot Interrupt Status
    #[inline(always)]
    pub fn abpsis(&mut self) -> ABPSIS_W<'_, MTLQ0ICSRrs> {
        ABPSIS_W::new(self, 1)
    }
    ///Bit 8 - Transmit Queue Underflow Interrupt Enable
    #[inline(always)]
    pub fn txuie(&mut self) -> TXUIE_W<'_, MTLQ0ICSRrs> {
        TXUIE_W::new(self, 8)
    }
    ///Bit 9 - Average Bits Per Slot Interrupt Enable
    #[inline(always)]
    pub fn abpsie(&mut self) -> ABPSIE_W<'_, MTLQ0ICSRrs> {
        ABPSIE_W::new(self, 9)
    }
    ///Bit 16 - Receive Queue Overflow Interrupt Status
    #[inline(always)]
    pub fn rxovfis(&mut self) -> RXOVFIS_W<'_, MTLQ0ICSRrs> {
        RXOVFIS_W::new(self, 16)
    }
    ///Bit 24 - Receive Queue Overflow Interrupt Enable
    #[inline(always)]
    pub fn rxoie(&mut self) -> RXOIE_W<'_, MTLQ0ICSRrs> {
        RXOIE_W::new(self, 24)
    }
}
/**Queue 0 interrupt control status Register

You can [`read`](crate::Reg::read) this register and get [`mtlq0icsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlq0icsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MTLQ0ICSR)*/
pub struct MTLQ0ICSRrs;
impl crate::RegisterSpec for MTLQ0ICSRrs {
    type Ux = u32;
}
///`read()` method returns [`mtlq0icsr::R`](R) reader structure
impl crate::Readable for MTLQ0ICSRrs {}
///`write(|w| ..)` method takes [`mtlq0icsr::W`](W) writer structure
impl crate::Writable for MTLQ0ICSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MTLQ0ICSR to value 0
impl crate::Resettable for MTLQ0ICSRrs {}
