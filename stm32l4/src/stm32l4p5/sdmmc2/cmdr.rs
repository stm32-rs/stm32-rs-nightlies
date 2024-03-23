#[doc = "Register `CMDR` reader"]
pub type R = crate::R<CMDRrs>;
#[doc = "Register `CMDR` writer"]
pub type W = crate::W<CMDRrs>;
#[doc = "Field `CMDINDEX` reader - Command index"]
pub type CMDINDEX_R = crate::FieldReader;
#[doc = "Field `CMDINDEX` writer - Command index"]
pub type CMDINDEX_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CMDTRANS` reader - The CPSM treats the command as a data transfer command, stops the interrupt period, and signals DataEnable to the DPSM"]
pub type CMDTRANS_R = crate::BitReader;
#[doc = "Field `CMDTRANS` writer - The CPSM treats the command as a data transfer command, stops the interrupt period, and signals DataEnable to the DPSM"]
pub type CMDTRANS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDSTOP` reader - The CPSM treats the command as a Stop Transmission command and signals Abort to the DPSM"]
pub type CMDSTOP_R = crate::BitReader;
#[doc = "Field `CMDSTOP` writer - The CPSM treats the command as a Stop Transmission command and signals Abort to the DPSM"]
pub type CMDSTOP_W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `DTHOLD` reader - Hold new data block transmission and reception in the DPSM"]
pub type DTHOLD_R = crate::BitReader;
#[doc = "Field `DTHOLD` writer - Hold new data block transmission and reception in the DPSM"]
pub type DTHOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOTMODE` reader - Select the boot mode procedure to be used"]
pub type BOOTMODE_R = crate::BitReader;
#[doc = "Field `BOOTMODE` writer - Select the boot mode procedure to be used"]
pub type BOOTMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOTEN` reader - Enable boot mode procedure"]
pub type BOOTEN_R = crate::BitReader;
#[doc = "Field `BOOTEN` writer - Enable boot mode procedure"]
pub type BOOTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDSUSPEND` reader - The CPSM treats the command as a Suspend or Resume command and signals interrupt period start/end"]
pub type CMDSUSPEND_R = crate::BitReader;
#[doc = "Field `CMDSUSPEND` writer - The CPSM treats the command as a Suspend or Resume command and signals interrupt period start/end"]
pub type CMDSUSPEND_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Command index"]
    #[inline(always)]
    pub fn cmdindex(&self) -> CMDINDEX_R {
        CMDINDEX_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - The CPSM treats the command as a data transfer command, stops the interrupt period, and signals DataEnable to the DPSM"]
    #[inline(always)]
    pub fn cmdtrans(&self) -> CMDTRANS_R {
        CMDTRANS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The CPSM treats the command as a Stop Transmission command and signals Abort to the DPSM"]
    #[inline(always)]
    pub fn cmdstop(&self) -> CMDSTOP_R {
        CMDSTOP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Wait for response bits"]
    #[inline(always)]
    pub fn waitresp(&self) -> WAITRESP_R {
        WAITRESP_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - CPSM waits for interrupt request"]
    #[inline(always)]
    pub fn waitint(&self) -> WAITINT_R {
        WAITINT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CPSM Waits for ends of data transfer (CmdPend internal signal)"]
    #[inline(always)]
    pub fn waitpend(&self) -> WAITPEND_R {
        WAITPEND_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Command path state machine (CPSM) Enable bit"]
    #[inline(always)]
    pub fn cpsmen(&self) -> CPSMEN_R {
        CPSMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Hold new data block transmission and reception in the DPSM"]
    #[inline(always)]
    pub fn dthold(&self) -> DTHOLD_R {
        DTHOLD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Select the boot mode procedure to be used"]
    #[inline(always)]
    pub fn bootmode(&self) -> BOOTMODE_R {
        BOOTMODE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable boot mode procedure"]
    #[inline(always)]
    pub fn booten(&self) -> BOOTEN_R {
        BOOTEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The CPSM treats the command as a Suspend or Resume command and signals interrupt period start/end"]
    #[inline(always)]
    pub fn cmdsuspend(&self) -> CMDSUSPEND_R {
        CMDSUSPEND_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Command index"]
    #[inline(always)]
    #[must_use]
    pub fn cmdindex(&mut self) -> CMDINDEX_W<CMDRrs> {
        CMDINDEX_W::new(self, 0)
    }
    #[doc = "Bit 6 - The CPSM treats the command as a data transfer command, stops the interrupt period, and signals DataEnable to the DPSM"]
    #[inline(always)]
    #[must_use]
    pub fn cmdtrans(&mut self) -> CMDTRANS_W<CMDRrs> {
        CMDTRANS_W::new(self, 6)
    }
    #[doc = "Bit 7 - The CPSM treats the command as a Stop Transmission command and signals Abort to the DPSM"]
    #[inline(always)]
    #[must_use]
    pub fn cmdstop(&mut self) -> CMDSTOP_W<CMDRrs> {
        CMDSTOP_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - Wait for response bits"]
    #[inline(always)]
    #[must_use]
    pub fn waitresp(&mut self) -> WAITRESP_W<CMDRrs> {
        WAITRESP_W::new(self, 8)
    }
    #[doc = "Bit 10 - CPSM waits for interrupt request"]
    #[inline(always)]
    #[must_use]
    pub fn waitint(&mut self) -> WAITINT_W<CMDRrs> {
        WAITINT_W::new(self, 10)
    }
    #[doc = "Bit 11 - CPSM Waits for ends of data transfer (CmdPend internal signal)"]
    #[inline(always)]
    #[must_use]
    pub fn waitpend(&mut self) -> WAITPEND_W<CMDRrs> {
        WAITPEND_W::new(self, 11)
    }
    #[doc = "Bit 12 - Command path state machine (CPSM) Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn cpsmen(&mut self) -> CPSMEN_W<CMDRrs> {
        CPSMEN_W::new(self, 12)
    }
    #[doc = "Bit 13 - Hold new data block transmission and reception in the DPSM"]
    #[inline(always)]
    #[must_use]
    pub fn dthold(&mut self) -> DTHOLD_W<CMDRrs> {
        DTHOLD_W::new(self, 13)
    }
    #[doc = "Bit 14 - Select the boot mode procedure to be used"]
    #[inline(always)]
    #[must_use]
    pub fn bootmode(&mut self) -> BOOTMODE_W<CMDRrs> {
        BOOTMODE_W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable boot mode procedure"]
    #[inline(always)]
    #[must_use]
    pub fn booten(&mut self) -> BOOTEN_W<CMDRrs> {
        BOOTEN_W::new(self, 15)
    }
    #[doc = "Bit 16 - The CPSM treats the command as a Suspend or Resume command and signals interrupt period start/end"]
    #[inline(always)]
    #[must_use]
    pub fn cmdsuspend(&mut self) -> CMDSUSPEND_W<CMDRrs> {
        CMDSUSPEND_W::new(self, 16)
    }
}
#[doc = "command register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMDRrs;
impl crate::RegisterSpec for CMDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmdr::R`](R) reader structure"]
impl crate::Readable for CMDRrs {}
#[doc = "`write(|w| ..)` method takes [`cmdr::W`](W) writer structure"]
impl crate::Writable for CMDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMDR to value 0"]
impl crate::Resettable for CMDRrs {
    const RESET_VALUE: u32 = 0;
}
