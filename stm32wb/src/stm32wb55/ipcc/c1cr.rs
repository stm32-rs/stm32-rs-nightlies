///Register `C1CR` reader
pub type R = crate::R<C1CRrs>;
///Register `C1CR` writer
pub type W = crate::W<C1CRrs>;
///Field `RXOIE` reader - processor 1 Receive channel occupied interrupt enable
pub type RXOIE_R = crate::BitReader;
///Field `RXOIE` writer - processor 1 Receive channel occupied interrupt enable
pub type RXOIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFIE` reader - processor 1 Transmit channel free interrupt enable
pub type TXFIE_R = crate::BitReader;
///Field `TXFIE` writer - processor 1 Transmit channel free interrupt enable
pub type TXFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - processor 1 Receive channel occupied interrupt enable
    #[inline(always)]
    pub fn rxoie(&self) -> RXOIE_R {
        RXOIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 16 - processor 1 Transmit channel free interrupt enable
    #[inline(always)]
    pub fn txfie(&self) -> TXFIE_R {
        TXFIE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C1CR")
            .field("txfie", &self.txfie())
            .field("rxoie", &self.rxoie())
            .finish()
    }
}
impl W {
    ///Bit 0 - processor 1 Receive channel occupied interrupt enable
    #[inline(always)]
    pub fn rxoie(&mut self) -> RXOIE_W<'_, C1CRrs> {
        RXOIE_W::new(self, 0)
    }
    ///Bit 16 - processor 1 Transmit channel free interrupt enable
    #[inline(always)]
    pub fn txfie(&mut self) -> TXFIE_W<'_, C1CRrs> {
        TXFIE_W::new(self, 16)
    }
}
/**Control register CPU1

You can [`read`](crate::Reg::read) this register and get [`c1cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#IPCC:C1CR)*/
pub struct C1CRrs;
impl crate::RegisterSpec for C1CRrs {
    type Ux = u32;
}
///`read()` method returns [`c1cr::R`](R) reader structure
impl crate::Readable for C1CRrs {}
///`write(|w| ..)` method takes [`c1cr::W`](W) writer structure
impl crate::Writable for C1CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C1CR to value 0
impl crate::Resettable for C1CRrs {}
