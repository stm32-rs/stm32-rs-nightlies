///Register `CMDR` reader
pub type R = crate::R<CMDRrs>;
///Register `CMDR` writer
pub type W = crate::W<CMDRrs>;
///Field `CMDINDEX` reader - CMDINDEX
pub type CMDINDEX_R = crate::FieldReader;
///Field `CMDINDEX` writer - CMDINDEX
pub type CMDINDEX_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `CMDTRANS` reader - CMDTRANS
pub type CMDTRANS_R = crate::BitReader;
///Field `CMDTRANS` writer - CMDTRANS
pub type CMDTRANS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMDSTOP` reader - CMDSTOP
pub type CMDSTOP_R = crate::BitReader;
///Field `CMDSTOP` writer - CMDSTOP
pub type CMDSTOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WAITRESP` reader - WAITRESP
pub type WAITRESP_R = crate::FieldReader;
///Field `WAITRESP` writer - WAITRESP
pub type WAITRESP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `WAITINT` reader - WAITINT
pub type WAITINT_R = crate::BitReader;
///Field `WAITINT` writer - WAITINT
pub type WAITINT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WAITPEND` reader - WAITPEND
pub type WAITPEND_R = crate::BitReader;
///Field `WAITPEND` writer - WAITPEND
pub type WAITPEND_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CPSMEN` reader - CPSMEN
pub type CPSMEN_R = crate::BitReader;
///Field `CPSMEN` writer - CPSMEN
pub type CPSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTHOLD` reader - DTHOLD
pub type DTHOLD_R = crate::BitReader;
///Field `DTHOLD` writer - DTHOLD
pub type DTHOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BOOTMODE` reader - BOOTMODE
pub type BOOTMODE_R = crate::BitReader;
///Field `BOOTMODE` writer - BOOTMODE
pub type BOOTMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BOOTEN` reader - BOOTEN
pub type BOOTEN_R = crate::BitReader;
///Field `BOOTEN` writer - BOOTEN
pub type BOOTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMDSUSPEND` reader - CMDSUSPEND
pub type CMDSUSPEND_R = crate::BitReader;
///Field `CMDSUSPEND` writer - CMDSUSPEND
pub type CMDSUSPEND_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:5 - CMDINDEX
    #[inline(always)]
    pub fn cmdindex(&self) -> CMDINDEX_R {
        CMDINDEX_R::new((self.bits & 0x3f) as u8)
    }
    ///Bit 6 - CMDTRANS
    #[inline(always)]
    pub fn cmdtrans(&self) -> CMDTRANS_R {
        CMDTRANS_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CMDSTOP
    #[inline(always)]
    pub fn cmdstop(&self) -> CMDSTOP_R {
        CMDSTOP_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - WAITRESP
    #[inline(always)]
    pub fn waitresp(&self) -> WAITRESP_R {
        WAITRESP_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 10 - WAITINT
    #[inline(always)]
    pub fn waitint(&self) -> WAITINT_R {
        WAITINT_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - WAITPEND
    #[inline(always)]
    pub fn waitpend(&self) -> WAITPEND_R {
        WAITPEND_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - CPSMEN
    #[inline(always)]
    pub fn cpsmen(&self) -> CPSMEN_R {
        CPSMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - DTHOLD
    #[inline(always)]
    pub fn dthold(&self) -> DTHOLD_R {
        DTHOLD_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - BOOTMODE
    #[inline(always)]
    pub fn bootmode(&self) -> BOOTMODE_R {
        BOOTMODE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - BOOTEN
    #[inline(always)]
    pub fn booten(&self) -> BOOTEN_R {
        BOOTEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - CMDSUSPEND
    #[inline(always)]
    pub fn cmdsuspend(&self) -> CMDSUSPEND_R {
        CMDSUSPEND_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDR")
            .field("cmdindex", &self.cmdindex())
            .field("cmdtrans", &self.cmdtrans())
            .field("cmdstop", &self.cmdstop())
            .field("waitresp", &self.waitresp())
            .field("waitint", &self.waitint())
            .field("waitpend", &self.waitpend())
            .field("cpsmen", &self.cpsmen())
            .field("dthold", &self.dthold())
            .field("bootmode", &self.bootmode())
            .field("booten", &self.booten())
            .field("cmdsuspend", &self.cmdsuspend())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - CMDINDEX
    #[inline(always)]
    pub fn cmdindex(&mut self) -> CMDINDEX_W<'_, CMDRrs> {
        CMDINDEX_W::new(self, 0)
    }
    ///Bit 6 - CMDTRANS
    #[inline(always)]
    pub fn cmdtrans(&mut self) -> CMDTRANS_W<'_, CMDRrs> {
        CMDTRANS_W::new(self, 6)
    }
    ///Bit 7 - CMDSTOP
    #[inline(always)]
    pub fn cmdstop(&mut self) -> CMDSTOP_W<'_, CMDRrs> {
        CMDSTOP_W::new(self, 7)
    }
    ///Bits 8:9 - WAITRESP
    #[inline(always)]
    pub fn waitresp(&mut self) -> WAITRESP_W<'_, CMDRrs> {
        WAITRESP_W::new(self, 8)
    }
    ///Bit 10 - WAITINT
    #[inline(always)]
    pub fn waitint(&mut self) -> WAITINT_W<'_, CMDRrs> {
        WAITINT_W::new(self, 10)
    }
    ///Bit 11 - WAITPEND
    #[inline(always)]
    pub fn waitpend(&mut self) -> WAITPEND_W<'_, CMDRrs> {
        WAITPEND_W::new(self, 11)
    }
    ///Bit 12 - CPSMEN
    #[inline(always)]
    pub fn cpsmen(&mut self) -> CPSMEN_W<'_, CMDRrs> {
        CPSMEN_W::new(self, 12)
    }
    ///Bit 13 - DTHOLD
    #[inline(always)]
    pub fn dthold(&mut self) -> DTHOLD_W<'_, CMDRrs> {
        DTHOLD_W::new(self, 13)
    }
    ///Bit 14 - BOOTMODE
    #[inline(always)]
    pub fn bootmode(&mut self) -> BOOTMODE_W<'_, CMDRrs> {
        BOOTMODE_W::new(self, 14)
    }
    ///Bit 15 - BOOTEN
    #[inline(always)]
    pub fn booten(&mut self) -> BOOTEN_W<'_, CMDRrs> {
        BOOTEN_W::new(self, 15)
    }
    ///Bit 16 - CMDSUSPEND
    #[inline(always)]
    pub fn cmdsuspend(&mut self) -> CMDSUSPEND_W<'_, CMDRrs> {
        CMDSUSPEND_W::new(self, 16)
    }
}
/**The SDMMC_CMDR register contains the command index and command type bits. The command index is sent to a card as part of a command message. The command type bits control the command path state machine (CPSM).

You can [`read`](crate::Reg::read) this register and get [`cmdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:CMDR)*/
pub struct CMDRrs;
impl crate::RegisterSpec for CMDRrs {
    type Ux = u32;
}
///`read()` method returns [`cmdr::R`](R) reader structure
impl crate::Readable for CMDRrs {}
///`write(|w| ..)` method takes [`cmdr::W`](W) writer structure
impl crate::Writable for CMDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CMDR to value 0
impl crate::Resettable for CMDRrs {}
