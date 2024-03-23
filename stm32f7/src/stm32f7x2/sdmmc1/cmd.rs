#[doc = "Register `CMD` reader"]
pub type R = crate::R<CMDrs>;
#[doc = "Register `CMD` writer"]
pub type W = crate::W<CMDrs>;
#[doc = "Field `CMDINDEX` reader - Command index"]
pub type CMDINDEX_R = crate::FieldReader;
#[doc = "Field `CMDINDEX` writer - Command index"]
pub type CMDINDEX_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `WAITRESP` reader - Wait for response bits"]
pub type WAITRESP_R = crate::FieldReader;
#[doc = "Field `WAITRESP` writer - Wait for response bits"]
pub type WAITRESP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WAITINT` reader - CPSM waits for interrupt request"]
pub type WAITINT_R = crate::BitReader;
#[doc = "Field `WAITINT` writer - CPSM waits for interrupt request"]
pub type WAITINT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAITPEND` reader - CPSM Waits for ends of data transfer (CmdPend internal signal)"]
pub type WAITPEND_R = crate::BitReader;
#[doc = "Field `WAITPEND` writer - CPSM Waits for ends of data transfer (CmdPend internal signal)"]
pub type WAITPEND_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPSMEN` reader - Command path state machine (CPSM) Enable bit"]
pub type CPSMEN_R = crate::BitReader;
#[doc = "Field `CPSMEN` writer - Command path state machine (CPSM) Enable bit"]
pub type CPSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIOSuspend` reader - SD I/O suspend command"]
pub type SDIOSUSPEND_R = crate::BitReader;
#[doc = "Field `SDIOSuspend` writer - SD I/O suspend command"]
pub type SDIOSUSPEND_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Command index"]
    #[inline(always)]
    pub fn cmdindex(&self) -> CMDINDEX_R {
        CMDINDEX_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Wait for response bits"]
    #[inline(always)]
    pub fn waitresp(&self) -> WAITRESP_R {
        WAITRESP_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - CPSM waits for interrupt request"]
    #[inline(always)]
    pub fn waitint(&self) -> WAITINT_R {
        WAITINT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CPSM Waits for ends of data transfer (CmdPend internal signal)"]
    #[inline(always)]
    pub fn waitpend(&self) -> WAITPEND_R {
        WAITPEND_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Command path state machine (CPSM) Enable bit"]
    #[inline(always)]
    pub fn cpsmen(&self) -> CPSMEN_R {
        CPSMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SD I/O suspend command"]
    #[inline(always)]
    pub fn sdiosuspend(&self) -> SDIOSUSPEND_R {
        SDIOSUSPEND_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Command index"]
    #[inline(always)]
    #[must_use]
    pub fn cmdindex(&mut self) -> CMDINDEX_W<CMDrs> {
        CMDINDEX_W::new(self, 0)
    }
    #[doc = "Bits 6:7 - Wait for response bits"]
    #[inline(always)]
    #[must_use]
    pub fn waitresp(&mut self) -> WAITRESP_W<CMDrs> {
        WAITRESP_W::new(self, 6)
    }
    #[doc = "Bit 8 - CPSM waits for interrupt request"]
    #[inline(always)]
    #[must_use]
    pub fn waitint(&mut self) -> WAITINT_W<CMDrs> {
        WAITINT_W::new(self, 8)
    }
    #[doc = "Bit 9 - CPSM Waits for ends of data transfer (CmdPend internal signal)"]
    #[inline(always)]
    #[must_use]
    pub fn waitpend(&mut self) -> WAITPEND_W<CMDrs> {
        WAITPEND_W::new(self, 9)
    }
    #[doc = "Bit 10 - Command path state machine (CPSM) Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn cpsmen(&mut self) -> CPSMEN_W<CMDrs> {
        CPSMEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - SD I/O suspend command"]
    #[inline(always)]
    #[must_use]
    pub fn sdiosuspend(&mut self) -> SDIOSUSPEND_W<CMDrs> {
        SDIOSUSPEND_W::new(self, 11)
    }
}
#[doc = "command register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMDrs;
impl crate::RegisterSpec for CMDrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd::R`](R) reader structure"]
impl crate::Readable for CMDrs {}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CMDrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMDrs {
    const RESET_VALUE: u32 = 0;
}
