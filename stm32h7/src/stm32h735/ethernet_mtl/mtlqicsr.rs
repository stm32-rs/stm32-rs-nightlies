///Register `MTLQICSR` reader
pub type R = crate::R<MTLQICSRrs>;
///Register `MTLQICSR` writer
pub type W = crate::W<MTLQICSRrs>;
///Field `TXUNFIS` reader - Transmit Queue Underflow Interrupt Status
pub type TXUNFIS_R = crate::BitReader;
///Field `TXUNFIS` writer - Transmit Queue Underflow Interrupt Status
pub type TXUNFIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXUIE` reader - Transmit Queue Underflow Interrupt Enable
pub type TXUIE_R = crate::BitReader;
///Field `TXUIE` writer - Transmit Queue Underflow Interrupt Enable
pub type TXUIE_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 8 - Transmit Queue Underflow Interrupt Enable
    #[inline(always)]
    pub fn txuie(&self) -> TXUIE_R {
        TXUIE_R::new(((self.bits >> 8) & 1) != 0)
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
        f.debug_struct("MTLQICSR")
            .field("rxoie", &self.rxoie())
            .field("rxovfis", &self.rxovfis())
            .field("txuie", &self.txuie())
            .field("txunfis", &self.txunfis())
            .finish()
    }
}
impl W {
    ///Bit 0 - Transmit Queue Underflow Interrupt Status
    #[inline(always)]
    pub fn txunfis(&mut self) -> TXUNFIS_W<'_, MTLQICSRrs> {
        TXUNFIS_W::new(self, 0)
    }
    ///Bit 8 - Transmit Queue Underflow Interrupt Enable
    #[inline(always)]
    pub fn txuie(&mut self) -> TXUIE_W<'_, MTLQICSRrs> {
        TXUIE_W::new(self, 8)
    }
    ///Bit 16 - Receive Queue Overflow Interrupt Status
    #[inline(always)]
    pub fn rxovfis(&mut self) -> RXOVFIS_W<'_, MTLQICSRrs> {
        RXOVFIS_W::new(self, 16)
    }
    ///Bit 24 - Receive Queue Overflow Interrupt Enable
    #[inline(always)]
    pub fn rxoie(&mut self) -> RXOIE_W<'_, MTLQICSRrs> {
        RXOIE_W::new(self, 24)
    }
}
/**Queue interrupt control status Register

You can [`read`](crate::Reg::read) this register and get [`mtlqicsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlqicsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#Ethernet_MTL:MTLQICSR)*/
pub struct MTLQICSRrs;
impl crate::RegisterSpec for MTLQICSRrs {
    type Ux = u32;
}
///`read()` method returns [`mtlqicsr::R`](R) reader structure
impl crate::Readable for MTLQICSRrs {}
///`write(|w| ..)` method takes [`mtlqicsr::W`](W) writer structure
impl crate::Writable for MTLQICSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MTLQICSR to value 0
impl crate::Resettable for MTLQICSRrs {}
