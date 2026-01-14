///Register `FS_GRSTCTL` reader
pub type R = crate::R<FS_GRSTCTLrs>;
///Register `FS_GRSTCTL` writer
pub type W = crate::W<FS_GRSTCTLrs>;
///Field `CSRST` reader - Core soft reset
pub type CSRST_R = crate::BitReader;
///Field `CSRST` writer - Core soft reset
pub type CSRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSRST` reader - HCLK soft reset
pub type HSRST_R = crate::BitReader;
///Field `HSRST` writer - HCLK soft reset
pub type HSRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FCRST` reader - Host frame counter reset
pub type FCRST_R = crate::BitReader;
///Field `FCRST` writer - Host frame counter reset
pub type FCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXFFLSH` reader - RxFIFO flush
pub type RXFFLSH_R = crate::BitReader;
///Field `RXFFLSH` writer - RxFIFO flush
pub type RXFFLSH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFFLSH` reader - TxFIFO flush
pub type TXFFLSH_R = crate::BitReader;
///Field `TXFFLSH` writer - TxFIFO flush
pub type TXFFLSH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFNUM` reader - TxFIFO number
pub type TXFNUM_R = crate::FieldReader;
///Field `TXFNUM` writer - TxFIFO number
pub type TXFNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `AHBIDL` reader - AHB master idle
pub type AHBIDL_R = crate::BitReader;
impl R {
    ///Bit 0 - Core soft reset
    #[inline(always)]
    pub fn csrst(&self) -> CSRST_R {
        CSRST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - HCLK soft reset
    #[inline(always)]
    pub fn hsrst(&self) -> HSRST_R {
        HSRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Host frame counter reset
    #[inline(always)]
    pub fn fcrst(&self) -> FCRST_R {
        FCRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - RxFIFO flush
    #[inline(always)]
    pub fn rxfflsh(&self) -> RXFFLSH_R {
        RXFFLSH_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TxFIFO flush
    #[inline(always)]
    pub fn txfflsh(&self) -> TXFFLSH_R {
        TXFFLSH_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:10 - TxFIFO number
    #[inline(always)]
    pub fn txfnum(&self) -> TXFNUM_R {
        TXFNUM_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    ///Bit 31 - AHB master idle
    #[inline(always)]
    pub fn ahbidl(&self) -> AHBIDL_R {
        AHBIDL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FS_GRSTCTL")
            .field("csrst", &self.csrst())
            .field("hsrst", &self.hsrst())
            .field("fcrst", &self.fcrst())
            .field("rxfflsh", &self.rxfflsh())
            .field("txfflsh", &self.txfflsh())
            .field("txfnum", &self.txfnum())
            .field("ahbidl", &self.ahbidl())
            .finish()
    }
}
impl W {
    ///Bit 0 - Core soft reset
    #[inline(always)]
    pub fn csrst(&mut self) -> CSRST_W<'_, FS_GRSTCTLrs> {
        CSRST_W::new(self, 0)
    }
    ///Bit 1 - HCLK soft reset
    #[inline(always)]
    pub fn hsrst(&mut self) -> HSRST_W<'_, FS_GRSTCTLrs> {
        HSRST_W::new(self, 1)
    }
    ///Bit 2 - Host frame counter reset
    #[inline(always)]
    pub fn fcrst(&mut self) -> FCRST_W<'_, FS_GRSTCTLrs> {
        FCRST_W::new(self, 2)
    }
    ///Bit 4 - RxFIFO flush
    #[inline(always)]
    pub fn rxfflsh(&mut self) -> RXFFLSH_W<'_, FS_GRSTCTLrs> {
        RXFFLSH_W::new(self, 4)
    }
    ///Bit 5 - TxFIFO flush
    #[inline(always)]
    pub fn txfflsh(&mut self) -> TXFFLSH_W<'_, FS_GRSTCTLrs> {
        TXFFLSH_W::new(self, 5)
    }
    ///Bits 6:10 - TxFIFO number
    #[inline(always)]
    pub fn txfnum(&mut self) -> TXFNUM_W<'_, FS_GRSTCTLrs> {
        TXFNUM_W::new(self, 6)
    }
}
/**OTG_FS reset register (OTG_FS_GRSTCTL)

You can [`read`](crate::Reg::read) this register and get [`fs_grstctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_grstctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_FS_GLOBAL:FS_GRSTCTL)*/
pub struct FS_GRSTCTLrs;
impl crate::RegisterSpec for FS_GRSTCTLrs {
    type Ux = u32;
}
///`read()` method returns [`fs_grstctl::R`](R) reader structure
impl crate::Readable for FS_GRSTCTLrs {}
///`write(|w| ..)` method takes [`fs_grstctl::W`](W) writer structure
impl crate::Writable for FS_GRSTCTLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FS_GRSTCTL to value 0x2000_0000
impl crate::Resettable for FS_GRSTCTLrs {
    const RESET_VALUE: u32 = 0x2000_0000;
}
