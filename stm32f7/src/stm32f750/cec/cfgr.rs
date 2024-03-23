#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CFGRrs>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CFGRrs>;
#[doc = "Field `SFT` reader - Signal Free Time"]
pub type SFT_R = crate::FieldReader;
#[doc = "Field `SFT` writer - Signal Free Time"]
pub type SFT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3>;
#[doc = "Field `RXTOL` reader - Rx-Tolerance"]
pub type RXTOL_R = crate::BitReader;
#[doc = "Field `RXTOL` writer - Rx-Tolerance"]
pub type RXTOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRESTP` reader - Rx-stop on bit rising error"]
pub type BRESTP_R = crate::BitReader;
#[doc = "Field `BRESTP` writer - Rx-stop on bit rising error"]
pub type BRESTP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BREGEN` reader - Generate error-bit on bit rising error"]
pub type BREGEN_R = crate::BitReader;
#[doc = "Field `BREGEN` writer - Generate error-bit on bit rising error"]
pub type BREGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBPEGEN` reader - Generate Error-Bit on Long Bit Period Error"]
pub type LBPEGEN_R = crate::BitReader;
#[doc = "Field `LBPEGEN` writer - Generate Error-Bit on Long Bit Period Error"]
pub type LBPEGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRDNOGEN` reader - Avoid Error-Bit Generation in Broadcast"]
pub type BRDNOGEN_R = crate::BitReader;
#[doc = "Field `BRDNOGEN` writer - Avoid Error-Bit Generation in Broadcast"]
pub type BRDNOGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SFTOP` reader - SFT Option Bit"]
pub type SFTOP_R = crate::BitReader;
#[doc = "Field `SFTOP` writer - SFT Option Bit"]
pub type SFTOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OAR` reader - Own addresses configuration"]
pub type OAR_R = crate::FieldReader<u16>;
#[doc = "Field `OAR` writer - Own addresses configuration"]
pub type OAR_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 15, u16>;
#[doc = "Field `LSTN` reader - Listen mode"]
pub type LSTN_R = crate::BitReader;
#[doc = "Field `LSTN` writer - Listen mode"]
pub type LSTN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Signal Free Time"]
    #[inline(always)]
    pub fn sft(&self) -> SFT_R {
        SFT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Rx-Tolerance"]
    #[inline(always)]
    pub fn rxtol(&self) -> RXTOL_R {
        RXTOL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rx-stop on bit rising error"]
    #[inline(always)]
    pub fn brestp(&self) -> BRESTP_R {
        BRESTP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Generate error-bit on bit rising error"]
    #[inline(always)]
    pub fn bregen(&self) -> BREGEN_R {
        BREGEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Generate Error-Bit on Long Bit Period Error"]
    #[inline(always)]
    pub fn lbpegen(&self) -> LBPEGEN_R {
        LBPEGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Avoid Error-Bit Generation in Broadcast"]
    #[inline(always)]
    pub fn brdnogen(&self) -> BRDNOGEN_R {
        BRDNOGEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SFT Option Bit"]
    #[inline(always)]
    pub fn sftop(&self) -> SFTOP_R {
        SFTOP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:30 - Own addresses configuration"]
    #[inline(always)]
    pub fn oar(&self) -> OAR_R {
        OAR_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - Listen mode"]
    #[inline(always)]
    pub fn lstn(&self) -> LSTN_R {
        LSTN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Signal Free Time"]
    #[inline(always)]
    #[must_use]
    pub fn sft(&mut self) -> SFT_W<CFGRrs> {
        SFT_W::new(self, 0)
    }
    #[doc = "Bit 3 - Rx-Tolerance"]
    #[inline(always)]
    #[must_use]
    pub fn rxtol(&mut self) -> RXTOL_W<CFGRrs> {
        RXTOL_W::new(self, 3)
    }
    #[doc = "Bit 4 - Rx-stop on bit rising error"]
    #[inline(always)]
    #[must_use]
    pub fn brestp(&mut self) -> BRESTP_W<CFGRrs> {
        BRESTP_W::new(self, 4)
    }
    #[doc = "Bit 5 - Generate error-bit on bit rising error"]
    #[inline(always)]
    #[must_use]
    pub fn bregen(&mut self) -> BREGEN_W<CFGRrs> {
        BREGEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Generate Error-Bit on Long Bit Period Error"]
    #[inline(always)]
    #[must_use]
    pub fn lbpegen(&mut self) -> LBPEGEN_W<CFGRrs> {
        LBPEGEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Avoid Error-Bit Generation in Broadcast"]
    #[inline(always)]
    #[must_use]
    pub fn brdnogen(&mut self) -> BRDNOGEN_W<CFGRrs> {
        BRDNOGEN_W::new(self, 7)
    }
    #[doc = "Bit 8 - SFT Option Bit"]
    #[inline(always)]
    #[must_use]
    pub fn sftop(&mut self) -> SFTOP_W<CFGRrs> {
        SFTOP_W::new(self, 8)
    }
    #[doc = "Bits 16:30 - Own addresses configuration"]
    #[inline(always)]
    #[must_use]
    pub fn oar(&mut self) -> OAR_W<CFGRrs> {
        OAR_W::new(self, 16)
    }
    #[doc = "Bit 31 - Listen mode"]
    #[inline(always)]
    #[must_use]
    pub fn lstn(&mut self) -> LSTN_W<CFGRrs> {
        LSTN_W::new(self, 31)
    }
}
#[doc = "configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGRrs;
impl crate::RegisterSpec for CFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CFGRrs {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CFGRrs {
    const RESET_VALUE: u32 = 0;
}
