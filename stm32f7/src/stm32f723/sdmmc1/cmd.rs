///Register `CMD` reader
pub type R = crate::R<CMDrs>;
///Register `CMD` writer
pub type W = crate::W<CMDrs>;
///Field `CMDINDEX` reader - Command index
pub type CMDINDEX_R = crate::FieldReader;
///Field `CMDINDEX` writer - Command index
pub type CMDINDEX_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `WAITRESP` reader - Wait for response bits
pub type WAITRESP_R = crate::FieldReader;
///Field `WAITRESP` writer - Wait for response bits
pub type WAITRESP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `WAITINT` reader - CPSM waits for interrupt request
pub type WAITINT_R = crate::BitReader;
///Field `WAITINT` writer - CPSM waits for interrupt request
pub type WAITINT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WAITPEND` reader - CPSM Waits for ends of data transfer (CmdPend internal signal)
pub type WAITPEND_R = crate::BitReader;
///Field `WAITPEND` writer - CPSM Waits for ends of data transfer (CmdPend internal signal)
pub type WAITPEND_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CPSMEN` reader - Command path state machine (CPSM) Enable bit
pub type CPSMEN_R = crate::BitReader;
///Field `CPSMEN` writer - Command path state machine (CPSM) Enable bit
pub type CPSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDIOSuspend` reader - SD I/O suspend command
pub type SDIOSUSPEND_R = crate::BitReader;
///Field `SDIOSuspend` writer - SD I/O suspend command
pub type SDIOSUSPEND_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:5 - Command index
    #[inline(always)]
    pub fn cmdindex(&self) -> CMDINDEX_R {
        CMDINDEX_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 6:7 - Wait for response bits
    #[inline(always)]
    pub fn waitresp(&self) -> WAITRESP_R {
        WAITRESP_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bit 8 - CPSM waits for interrupt request
    #[inline(always)]
    pub fn waitint(&self) -> WAITINT_R {
        WAITINT_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CPSM Waits for ends of data transfer (CmdPend internal signal)
    #[inline(always)]
    pub fn waitpend(&self) -> WAITPEND_R {
        WAITPEND_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Command path state machine (CPSM) Enable bit
    #[inline(always)]
    pub fn cpsmen(&self) -> CPSMEN_R {
        CPSMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - SD I/O suspend command
    #[inline(always)]
    pub fn sdiosuspend(&self) -> SDIOSUSPEND_R {
        SDIOSUSPEND_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD")
            .field("sdiosuspend", &self.sdiosuspend())
            .field("cpsmen", &self.cpsmen())
            .field("waitpend", &self.waitpend())
            .field("waitint", &self.waitint())
            .field("waitresp", &self.waitresp())
            .field("cmdindex", &self.cmdindex())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - Command index
    #[inline(always)]
    pub fn cmdindex(&mut self) -> CMDINDEX_W<CMDrs> {
        CMDINDEX_W::new(self, 0)
    }
    ///Bits 6:7 - Wait for response bits
    #[inline(always)]
    pub fn waitresp(&mut self) -> WAITRESP_W<CMDrs> {
        WAITRESP_W::new(self, 6)
    }
    ///Bit 8 - CPSM waits for interrupt request
    #[inline(always)]
    pub fn waitint(&mut self) -> WAITINT_W<CMDrs> {
        WAITINT_W::new(self, 8)
    }
    ///Bit 9 - CPSM Waits for ends of data transfer (CmdPend internal signal)
    #[inline(always)]
    pub fn waitpend(&mut self) -> WAITPEND_W<CMDrs> {
        WAITPEND_W::new(self, 9)
    }
    ///Bit 10 - Command path state machine (CPSM) Enable bit
    #[inline(always)]
    pub fn cpsmen(&mut self) -> CPSMEN_W<CMDrs> {
        CPSMEN_W::new(self, 10)
    }
    ///Bit 11 - SD I/O suspend command
    #[inline(always)]
    pub fn sdiosuspend(&mut self) -> SDIOSUSPEND_W<CMDrs> {
        SDIOSUSPEND_W::new(self, 11)
    }
}
/**command register

You can [`read`](crate::Reg::read) this register and get [`cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F723.html#SDMMC1:CMD)*/
pub struct CMDrs;
impl crate::RegisterSpec for CMDrs {
    type Ux = u32;
}
///`read()` method returns [`cmd::R`](R) reader structure
impl crate::Readable for CMDrs {}
///`write(|w| ..)` method takes [`cmd::W`](W) writer structure
impl crate::Writable for CMDrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CMD to value 0
impl crate::Resettable for CMDrs {}
