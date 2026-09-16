///Register `MTLESTGCLCR` reader
pub type R = crate::R<MTLESTGCLCRrs>;
///Register `MTLESTGCLCR` writer
pub type W = crate::W<MTLESTGCLCRrs>;
///Field `SRWO` reader - Start Read/Write Operation
pub type SRWO_R = crate::BitReader;
///Field `SRWO` writer - Start Read/Write Operation
pub type SRWO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `R1W0` reader - Read 1, Write 0
pub type R1W0_R = crate::BitReader;
///Field `R1W0` writer - Read 1, Write 0
pub type R1W0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GCRR` reader - Gate Control Related Registers
pub type GCRR_R = crate::BitReader;
///Field `GCRR` writer - Gate Control Related Registers
pub type GCRR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBGM` reader - Debug Mode
pub type DBGM_R = crate::BitReader;
///Field `DBGM` writer - Debug Mode
pub type DBGM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBGB` reader - Debug Mode Bank Select
pub type DBGB_R = crate::BitReader;
///Field `DBGB` writer - Debug Mode Bank Select
pub type DBGB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADDR` reader - Gate Control List Address:
pub type ADDR_R = crate::FieldReader;
///Field `ADDR` writer - Gate Control List Address:
pub type ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bit 0 - Start Read/Write Operation
    #[inline(always)]
    pub fn srwo(&self) -> SRWO_R {
        SRWO_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Read 1, Write 0
    #[inline(always)]
    pub fn r1w0(&self) -> R1W0_R {
        R1W0_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Gate Control Related Registers
    #[inline(always)]
    pub fn gcrr(&self) -> GCRR_R {
        GCRR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - Debug Mode
    #[inline(always)]
    pub fn dbgm(&self) -> DBGM_R {
        DBGM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Debug Mode Bank Select
    #[inline(always)]
    pub fn dbgb(&self) -> DBGB_R {
        DBGB_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 8:13 - Gate Control List Address:
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLESTGCLCR")
            .field("srwo", &self.srwo())
            .field("r1w0", &self.r1w0())
            .field("gcrr", &self.gcrr())
            .field("dbgm", &self.dbgm())
            .field("dbgb", &self.dbgb())
            .field("addr", &self.addr())
            .finish()
    }
}
impl W {
    ///Bit 0 - Start Read/Write Operation
    #[inline(always)]
    pub fn srwo(&mut self) -> SRWO_W<'_, MTLESTGCLCRrs> {
        SRWO_W::new(self, 0)
    }
    ///Bit 1 - Read 1, Write 0
    #[inline(always)]
    pub fn r1w0(&mut self) -> R1W0_W<'_, MTLESTGCLCRrs> {
        R1W0_W::new(self, 1)
    }
    ///Bit 2 - Gate Control Related Registers
    #[inline(always)]
    pub fn gcrr(&mut self) -> GCRR_W<'_, MTLESTGCLCRrs> {
        GCRR_W::new(self, 2)
    }
    ///Bit 4 - Debug Mode
    #[inline(always)]
    pub fn dbgm(&mut self) -> DBGM_W<'_, MTLESTGCLCRrs> {
        DBGM_W::new(self, 4)
    }
    ///Bit 5 - Debug Mode Bank Select
    #[inline(always)]
    pub fn dbgb(&mut self) -> DBGB_W<'_, MTLESTGCLCRrs> {
        DBGB_W::new(self, 5)
    }
    ///Bits 8:13 - Gate Control List Address:
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W<'_, MTLESTGCLCRrs> {
        ADDR_W::new(self, 8)
    }
}
/**EST Gate Control List Register

You can [`read`](crate::Reg::read) this register and get [`mtlestgclcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlestgclcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#ETH:MTLESTGCLCR)*/
pub struct MTLESTGCLCRrs;
impl crate::RegisterSpec for MTLESTGCLCRrs {
    type Ux = u32;
}
///`read()` method returns [`mtlestgclcr::R`](R) reader structure
impl crate::Readable for MTLESTGCLCRrs {}
///`write(|w| ..)` method takes [`mtlestgclcr::W`](W) writer structure
impl crate::Writable for MTLESTGCLCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MTLESTGCLCR to value 0
impl crate::Resettable for MTLESTGCLCRrs {}
