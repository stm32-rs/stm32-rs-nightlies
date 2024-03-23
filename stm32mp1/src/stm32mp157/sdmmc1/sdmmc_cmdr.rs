#[doc = "Register `SDMMC_CMDR` reader"]
pub type R = crate::R<SDMMC_CMDRrs>;
#[doc = "Register `SDMMC_CMDR` writer"]
pub type W = crate::W<SDMMC_CMDRrs>;
#[doc = "Field `CMDINDEX` reader - CMDINDEX"]
pub type CMDINDEX_R = crate::FieldReader;
#[doc = "Field `CMDINDEX` writer - CMDINDEX"]
pub type CMDINDEX_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CMDTRANS` reader - CMDTRANS"]
pub type CMDTRANS_R = crate::BitReader;
#[doc = "Field `CMDTRANS` writer - CMDTRANS"]
pub type CMDTRANS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDSTOP` reader - CMDSTOP"]
pub type CMDSTOP_R = crate::BitReader;
#[doc = "Field `CMDSTOP` writer - CMDSTOP"]
pub type CMDSTOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAITRESP` reader - WAITRESP"]
pub type WAITRESP_R = crate::FieldReader;
#[doc = "Field `WAITRESP` writer - WAITRESP"]
pub type WAITRESP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WAITINT` reader - WAITINT"]
pub type WAITINT_R = crate::BitReader;
#[doc = "Field `WAITINT` writer - WAITINT"]
pub type WAITINT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAITPEND` reader - WAITPEND"]
pub type WAITPEND_R = crate::BitReader;
#[doc = "Field `WAITPEND` writer - WAITPEND"]
pub type WAITPEND_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPSMEN` reader - CPSMEN"]
pub type CPSMEN_R = crate::BitReader;
#[doc = "Field `CPSMEN` writer - CPSMEN"]
pub type CPSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTHOLD` reader - DTHOLD"]
pub type DTHOLD_R = crate::BitReader;
#[doc = "Field `DTHOLD` writer - DTHOLD"]
pub type DTHOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOTMODE` reader - BOOTMODE"]
pub type BOOTMODE_R = crate::BitReader;
#[doc = "Field `BOOTMODE` writer - BOOTMODE"]
pub type BOOTMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOTEN` reader - BOOTEN"]
pub type BOOTEN_R = crate::BitReader;
#[doc = "Field `BOOTEN` writer - BOOTEN"]
pub type BOOTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDSUSPEND` reader - CMDSUSPEND"]
pub type CMDSUSPEND_R = crate::BitReader;
#[doc = "Field `CMDSUSPEND` writer - CMDSUSPEND"]
pub type CMDSUSPEND_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - CMDINDEX"]
    #[inline(always)]
    pub fn cmdindex(&self) -> CMDINDEX_R {
        CMDINDEX_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - CMDTRANS"]
    #[inline(always)]
    pub fn cmdtrans(&self) -> CMDTRANS_R {
        CMDTRANS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CMDSTOP"]
    #[inline(always)]
    pub fn cmdstop(&self) -> CMDSTOP_R {
        CMDSTOP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - WAITRESP"]
    #[inline(always)]
    pub fn waitresp(&self) -> WAITRESP_R {
        WAITRESP_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - WAITINT"]
    #[inline(always)]
    pub fn waitint(&self) -> WAITINT_R {
        WAITINT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - WAITPEND"]
    #[inline(always)]
    pub fn waitpend(&self) -> WAITPEND_R {
        WAITPEND_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CPSMEN"]
    #[inline(always)]
    pub fn cpsmen(&self) -> CPSMEN_R {
        CPSMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DTHOLD"]
    #[inline(always)]
    pub fn dthold(&self) -> DTHOLD_R {
        DTHOLD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - BOOTMODE"]
    #[inline(always)]
    pub fn bootmode(&self) -> BOOTMODE_R {
        BOOTMODE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - BOOTEN"]
    #[inline(always)]
    pub fn booten(&self) -> BOOTEN_R {
        BOOTEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - CMDSUSPEND"]
    #[inline(always)]
    pub fn cmdsuspend(&self) -> CMDSUSPEND_R {
        CMDSUSPEND_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - CMDINDEX"]
    #[inline(always)]
    #[must_use]
    pub fn cmdindex(&mut self) -> CMDINDEX_W<SDMMC_CMDRrs> {
        CMDINDEX_W::new(self, 0)
    }
    #[doc = "Bit 6 - CMDTRANS"]
    #[inline(always)]
    #[must_use]
    pub fn cmdtrans(&mut self) -> CMDTRANS_W<SDMMC_CMDRrs> {
        CMDTRANS_W::new(self, 6)
    }
    #[doc = "Bit 7 - CMDSTOP"]
    #[inline(always)]
    #[must_use]
    pub fn cmdstop(&mut self) -> CMDSTOP_W<SDMMC_CMDRrs> {
        CMDSTOP_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - WAITRESP"]
    #[inline(always)]
    #[must_use]
    pub fn waitresp(&mut self) -> WAITRESP_W<SDMMC_CMDRrs> {
        WAITRESP_W::new(self, 8)
    }
    #[doc = "Bit 10 - WAITINT"]
    #[inline(always)]
    #[must_use]
    pub fn waitint(&mut self) -> WAITINT_W<SDMMC_CMDRrs> {
        WAITINT_W::new(self, 10)
    }
    #[doc = "Bit 11 - WAITPEND"]
    #[inline(always)]
    #[must_use]
    pub fn waitpend(&mut self) -> WAITPEND_W<SDMMC_CMDRrs> {
        WAITPEND_W::new(self, 11)
    }
    #[doc = "Bit 12 - CPSMEN"]
    #[inline(always)]
    #[must_use]
    pub fn cpsmen(&mut self) -> CPSMEN_W<SDMMC_CMDRrs> {
        CPSMEN_W::new(self, 12)
    }
    #[doc = "Bit 13 - DTHOLD"]
    #[inline(always)]
    #[must_use]
    pub fn dthold(&mut self) -> DTHOLD_W<SDMMC_CMDRrs> {
        DTHOLD_W::new(self, 13)
    }
    #[doc = "Bit 14 - BOOTMODE"]
    #[inline(always)]
    #[must_use]
    pub fn bootmode(&mut self) -> BOOTMODE_W<SDMMC_CMDRrs> {
        BOOTMODE_W::new(self, 14)
    }
    #[doc = "Bit 15 - BOOTEN"]
    #[inline(always)]
    #[must_use]
    pub fn booten(&mut self) -> BOOTEN_W<SDMMC_CMDRrs> {
        BOOTEN_W::new(self, 15)
    }
    #[doc = "Bit 16 - CMDSUSPEND"]
    #[inline(always)]
    #[must_use]
    pub fn cmdsuspend(&mut self) -> CMDSUSPEND_W<SDMMC_CMDRrs> {
        CMDSUSPEND_W::new(self, 16)
    }
}
#[doc = "The SDMMC_CMDR register contains the command index and command type bits. The command index is sent to a card as part of a command message. The command type bits control the command path state machine (CPSM).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_cmdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_cmdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDMMC_CMDRrs;
impl crate::RegisterSpec for SDMMC_CMDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_cmdr::R`](R) reader structure"]
impl crate::Readable for SDMMC_CMDRrs {}
#[doc = "`write(|w| ..)` method takes [`sdmmc_cmdr::W`](W) writer structure"]
impl crate::Writable for SDMMC_CMDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDMMC_CMDR to value 0"]
impl crate::Resettable for SDMMC_CMDRrs {
    const RESET_VALUE: u32 = 0;
}
