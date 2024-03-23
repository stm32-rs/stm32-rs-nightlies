#[doc = "Register `TTOCF` reader"]
pub type R = crate::R<TTOCFrs>;
#[doc = "Register `TTOCF` writer"]
pub type W = crate::W<TTOCFrs>;
#[doc = "Field `OM` reader - Operation Mode"]
pub type OM_R = crate::FieldReader;
#[doc = "Field `OM` writer - Operation Mode"]
pub type OM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GEN` reader - Gap Enable"]
pub type GEN_R = crate::BitReader;
#[doc = "Field `GEN` writer - Gap Enable"]
pub type GEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM` reader - Time Master"]
pub type TM_R = crate::BitReader;
#[doc = "Field `TM` writer - Time Master"]
pub type TM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LDSDL` reader - LD of Synchronization Deviation Limit"]
pub type LDSDL_R = crate::FieldReader;
#[doc = "Field `LDSDL` writer - LD of Synchronization Deviation Limit"]
pub type LDSDL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IRTO` reader - Initial Reference Trigger Offset"]
pub type IRTO_R = crate::FieldReader;
#[doc = "Field `IRTO` writer - Initial Reference Trigger Offset"]
pub type IRTO_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `EECS` reader - Enable External Clock Synchronization"]
pub type EECS_R = crate::BitReader;
#[doc = "Field `EECS` writer - Enable External Clock Synchronization"]
pub type EECS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWL` reader - Application Watchdog Limit"]
pub type AWL_R = crate::FieldReader;
#[doc = "Field `AWL` writer - Application Watchdog Limit"]
pub type AWL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EGTF` reader - Enable Global Time Filtering"]
pub type EGTF_R = crate::BitReader;
#[doc = "Field `EGTF` writer - Enable Global Time Filtering"]
pub type EGTF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC` reader - Enable Clock Calibration"]
pub type ECC_R = crate::BitReader;
#[doc = "Field `ECC` writer - Enable Clock Calibration"]
pub type ECC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVTP` reader - Event Trigger Polarity"]
pub type EVTP_R = crate::BitReader;
#[doc = "Field `EVTP` writer - Event Trigger Polarity"]
pub type EVTP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Operation Mode"]
    #[inline(always)]
    pub fn om(&self) -> OM_R {
        OM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Gap Enable"]
    #[inline(always)]
    pub fn gen(&self) -> GEN_R {
        GEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Time Master"]
    #[inline(always)]
    pub fn tm(&self) -> TM_R {
        TM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - LD of Synchronization Deviation Limit"]
    #[inline(always)]
    pub fn ldsdl(&self) -> LDSDL_R {
        LDSDL_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:14 - Initial Reference Trigger Offset"]
    #[inline(always)]
    pub fn irto(&self) -> IRTO_R {
        IRTO_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Enable External Clock Synchronization"]
    #[inline(always)]
    pub fn eecs(&self) -> EECS_R {
        EECS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Application Watchdog Limit"]
    #[inline(always)]
    pub fn awl(&self) -> AWL_R {
        AWL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Enable Global Time Filtering"]
    #[inline(always)]
    pub fn egtf(&self) -> EGTF_R {
        EGTF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Clock Calibration"]
    #[inline(always)]
    pub fn ecc(&self) -> ECC_R {
        ECC_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Event Trigger Polarity"]
    #[inline(always)]
    pub fn evtp(&self) -> EVTP_R {
        EVTP_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Operation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn om(&mut self) -> OM_W<TTOCFrs> {
        OM_W::new(self, 0)
    }
    #[doc = "Bit 3 - Gap Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gen(&mut self) -> GEN_W<TTOCFrs> {
        GEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Time Master"]
    #[inline(always)]
    #[must_use]
    pub fn tm(&mut self) -> TM_W<TTOCFrs> {
        TM_W::new(self, 4)
    }
    #[doc = "Bits 5:7 - LD of Synchronization Deviation Limit"]
    #[inline(always)]
    #[must_use]
    pub fn ldsdl(&mut self) -> LDSDL_W<TTOCFrs> {
        LDSDL_W::new(self, 5)
    }
    #[doc = "Bits 8:14 - Initial Reference Trigger Offset"]
    #[inline(always)]
    #[must_use]
    pub fn irto(&mut self) -> IRTO_W<TTOCFrs> {
        IRTO_W::new(self, 8)
    }
    #[doc = "Bit 15 - Enable External Clock Synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn eecs(&mut self) -> EECS_W<TTOCFrs> {
        EECS_W::new(self, 15)
    }
    #[doc = "Bits 16:23 - Application Watchdog Limit"]
    #[inline(always)]
    #[must_use]
    pub fn awl(&mut self) -> AWL_W<TTOCFrs> {
        AWL_W::new(self, 16)
    }
    #[doc = "Bit 24 - Enable Global Time Filtering"]
    #[inline(always)]
    #[must_use]
    pub fn egtf(&mut self) -> EGTF_W<TTOCFrs> {
        EGTF_W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Clock Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn ecc(&mut self) -> ECC_W<TTOCFrs> {
        ECC_W::new(self, 25)
    }
    #[doc = "Bit 26 - Event Trigger Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn evtp(&mut self) -> EVTP_W<TTOCFrs> {
        EVTP_W::new(self, 26)
    }
}
#[doc = "FDCAN TT Operation Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ttocf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ttocf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TTOCFrs;
impl crate::RegisterSpec for TTOCFrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ttocf::R`](R) reader structure"]
impl crate::Readable for TTOCFrs {}
#[doc = "`write(|w| ..)` method takes [`ttocf::W`](W) writer structure"]
impl crate::Writable for TTOCFrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TTOCF to value 0"]
impl crate::Resettable for TTOCFrs {
    const RESET_VALUE: u32 = 0;
}
