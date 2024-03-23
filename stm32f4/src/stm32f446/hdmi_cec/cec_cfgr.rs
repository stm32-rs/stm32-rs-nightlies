#[doc = "Register `CEC_CFGR` reader"]
pub type R = crate::R<CEC_CFGRrs>;
#[doc = "Register `CEC_CFGR` writer"]
pub type W = crate::W<CEC_CFGRrs>;
#[doc = "Field `SFT` reader - Signal Free Time"]
pub type SFT_R = crate::FieldReader;
#[doc = "Field `SFT` writer - Signal Free Time"]
pub type SFT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RXTOL` reader - Rx-Tolerance"]
pub type RXTOL_R = crate::BitReader;
#[doc = "Field `RXTOL` writer - Rx-Tolerance"]
pub type RXTOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRESTP` reader - Rx-Stop on Bit Rising Error"]
pub type BRESTP_R = crate::BitReader;
#[doc = "Field `BRESTP` writer - Rx-Stop on Bit Rising Error"]
pub type BRESTP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BREGEN` reader - Generate Error-Bit on Bit Rising Error"]
pub type BREGEN_R = crate::BitReader;
#[doc = "Field `BREGEN` writer - Generate Error-Bit on Bit Rising Error"]
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
pub type OAR_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
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
    #[doc = "Bit 4 - Rx-Stop on Bit Rising Error"]
    #[inline(always)]
    pub fn brestp(&self) -> BRESTP_R {
        BRESTP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Generate Error-Bit on Bit Rising Error"]
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
    pub fn sft(&mut self) -> SFT_W<CEC_CFGRrs> {
        SFT_W::new(self, 0)
    }
    #[doc = "Bit 3 - Rx-Tolerance"]
    #[inline(always)]
    #[must_use]
    pub fn rxtol(&mut self) -> RXTOL_W<CEC_CFGRrs> {
        RXTOL_W::new(self, 3)
    }
    #[doc = "Bit 4 - Rx-Stop on Bit Rising Error"]
    #[inline(always)]
    #[must_use]
    pub fn brestp(&mut self) -> BRESTP_W<CEC_CFGRrs> {
        BRESTP_W::new(self, 4)
    }
    #[doc = "Bit 5 - Generate Error-Bit on Bit Rising Error"]
    #[inline(always)]
    #[must_use]
    pub fn bregen(&mut self) -> BREGEN_W<CEC_CFGRrs> {
        BREGEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Generate Error-Bit on Long Bit Period Error"]
    #[inline(always)]
    #[must_use]
    pub fn lbpegen(&mut self) -> LBPEGEN_W<CEC_CFGRrs> {
        LBPEGEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Avoid Error-Bit Generation in Broadcast"]
    #[inline(always)]
    #[must_use]
    pub fn brdnogen(&mut self) -> BRDNOGEN_W<CEC_CFGRrs> {
        BRDNOGEN_W::new(self, 7)
    }
    #[doc = "Bit 8 - SFT Option Bit"]
    #[inline(always)]
    #[must_use]
    pub fn sftop(&mut self) -> SFTOP_W<CEC_CFGRrs> {
        SFTOP_W::new(self, 8)
    }
    #[doc = "Bits 16:30 - Own addresses configuration"]
    #[inline(always)]
    #[must_use]
    pub fn oar(&mut self) -> OAR_W<CEC_CFGRrs> {
        OAR_W::new(self, 16)
    }
    #[doc = "Bit 31 - Listen mode"]
    #[inline(always)]
    #[must_use]
    pub fn lstn(&mut self) -> LSTN_W<CEC_CFGRrs> {
        LSTN_W::new(self, 31)
    }
}
#[doc = "CEC configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cec_cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cec_cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CEC_CFGRrs;
impl crate::RegisterSpec for CEC_CFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cec_cfgr::R`](R) reader structure"]
impl crate::Readable for CEC_CFGRrs {}
#[doc = "`write(|w| ..)` method takes [`cec_cfgr::W`](W) writer structure"]
impl crate::Writable for CEC_CFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CEC_CFGR to value 0"]
impl crate::Resettable for CEC_CFGRrs {
    const RESET_VALUE: u32 = 0;
}
