///Register `GRSTCTL` reader
pub type R = crate::R<GRSTCTLrs>;
///Register `GRSTCTL` writer
pub type W = crate::W<GRSTCTLrs>;
///Field `CSRST` reader - Core soft reset
pub type CSRST_R = crate::BitReader;
///Field `CSRST` writer - Core soft reset
pub type CSRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PSRST` reader - Partial soft reset
pub type PSRST_R = crate::BitReader;
///Field `PSRST` writer - Partial soft reset
pub type PSRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FCRST` reader - Host frame counter reset
pub type FCRST_R = crate::BitReader;
///Field `FCRST` writer - Host frame counter reset
pub type FCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXFFLSH` reader - Rx FIFO flush
pub type RXFFLSH_R = crate::BitReader;
///Field `RXFFLSH` writer - Rx FIFO flush
pub type RXFFLSH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFFLSH` reader - Tx FIFO flush
pub type TXFFLSH_R = crate::BitReader;
///Field `TXFFLSH` writer - Tx FIFO flush
pub type TXFFLSH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFNUM` reader - Tx FIFO number
pub type TXFNUM_R = crate::FieldReader;
///Field `TXFNUM` writer - Tx FIFO number
pub type TXFNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `DMAREQ` reader - DMA request signal enabled
pub type DMAREQ_R = crate::BitReader;
///Field `AHBIDL` reader - AHB master idle
pub type AHBIDL_R = crate::BitReader;
impl R {
    ///Bit 0 - Core soft reset
    #[inline(always)]
    pub fn csrst(&self) -> CSRST_R {
        CSRST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Partial soft reset
    #[inline(always)]
    pub fn psrst(&self) -> PSRST_R {
        PSRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Host frame counter reset
    #[inline(always)]
    pub fn fcrst(&self) -> FCRST_R {
        FCRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - Rx FIFO flush
    #[inline(always)]
    pub fn rxfflsh(&self) -> RXFFLSH_R {
        RXFFLSH_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Tx FIFO flush
    #[inline(always)]
    pub fn txfflsh(&self) -> TXFFLSH_R {
        TXFFLSH_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:10 - Tx FIFO number
    #[inline(always)]
    pub fn txfnum(&self) -> TXFNUM_R {
        TXFNUM_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    ///Bit 30 - DMA request signal enabled
    #[inline(always)]
    pub fn dmareq(&self) -> DMAREQ_R {
        DMAREQ_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - AHB master idle
    #[inline(always)]
    pub fn ahbidl(&self) -> AHBIDL_R {
        AHBIDL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GRSTCTL")
            .field("csrst", &self.csrst())
            .field("psrst", &self.psrst())
            .field("fcrst", &self.fcrst())
            .field("rxfflsh", &self.rxfflsh())
            .field("txfflsh", &self.txfflsh())
            .field("txfnum", &self.txfnum())
            .field("dmareq", &self.dmareq())
            .field("ahbidl", &self.ahbidl())
            .finish()
    }
}
impl W {
    ///Bit 0 - Core soft reset
    #[inline(always)]
    pub fn csrst(&mut self) -> CSRST_W<'_, GRSTCTLrs> {
        CSRST_W::new(self, 0)
    }
    ///Bit 1 - Partial soft reset
    #[inline(always)]
    pub fn psrst(&mut self) -> PSRST_W<'_, GRSTCTLrs> {
        PSRST_W::new(self, 1)
    }
    ///Bit 2 - Host frame counter reset
    #[inline(always)]
    pub fn fcrst(&mut self) -> FCRST_W<'_, GRSTCTLrs> {
        FCRST_W::new(self, 2)
    }
    ///Bit 4 - Rx FIFO flush
    #[inline(always)]
    pub fn rxfflsh(&mut self) -> RXFFLSH_W<'_, GRSTCTLrs> {
        RXFFLSH_W::new(self, 4)
    }
    ///Bit 5 - Tx FIFO flush
    #[inline(always)]
    pub fn txfflsh(&mut self) -> TXFFLSH_W<'_, GRSTCTLrs> {
        TXFFLSH_W::new(self, 5)
    }
    ///Bits 6:10 - Tx FIFO number
    #[inline(always)]
    pub fn txfnum(&mut self) -> TXFNUM_W<'_, GRSTCTLrs> {
        TXFNUM_W::new(self, 6)
    }
}
/**OTG reset register

You can [`read`](crate::Reg::read) this register and get [`grstctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grstctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#OTG1:GRSTCTL)*/
pub struct GRSTCTLrs;
impl crate::RegisterSpec for GRSTCTLrs {
    type Ux = u32;
}
///`read()` method returns [`grstctl::R`](R) reader structure
impl crate::Readable for GRSTCTLrs {}
///`write(|w| ..)` method takes [`grstctl::W`](W) writer structure
impl crate::Writable for GRSTCTLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GRSTCTL to value 0x8000_0000
impl crate::Resettable for GRSTCTLrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
