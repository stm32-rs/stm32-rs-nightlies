#[doc = "Register `FDCAN_TTOCF` reader"]
pub type R = crate::R<FDCAN_TTOCFrs>;
#[doc = "Register `FDCAN_TTOCF` writer"]
pub type W = crate::W<FDCAN_TTOCFrs>;
#[doc = "Field `OM` reader - OM"]
pub type OM_R = crate::FieldReader;
#[doc = "Field `OM` writer - OM"]
pub type OM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GEN` reader - GEN"]
pub type GEN_R = crate::BitReader;
#[doc = "Field `GEN` writer - GEN"]
pub type GEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM` reader - TM"]
pub type TM_R = crate::BitReader;
#[doc = "Field `TM` writer - TM"]
pub type TM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LDSDL` reader - LDSDL"]
pub type LDSDL_R = crate::FieldReader;
#[doc = "Field `LDSDL` writer - LDSDL"]
pub type LDSDL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IRTO` reader - IRTO"]
pub type IRTO_R = crate::FieldReader;
#[doc = "Field `IRTO` writer - IRTO"]
pub type IRTO_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `EECS` reader - EECS"]
pub type EECS_R = crate::BitReader;
#[doc = "Field `EECS` writer - EECS"]
pub type EECS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWL` reader - AWL"]
pub type AWL_R = crate::FieldReader;
#[doc = "Field `AWL` writer - AWL"]
pub type AWL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EGTF` reader - EGTF"]
pub type EGTF_R = crate::BitReader;
#[doc = "Field `EGTF` writer - EGTF"]
pub type EGTF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC` reader - ECC"]
pub type ECC_R = crate::BitReader;
#[doc = "Field `ECC` writer - ECC"]
pub type ECC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVTP` reader - EVTP"]
pub type EVTP_R = crate::BitReader;
#[doc = "Field `EVTP` writer - EVTP"]
pub type EVTP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - OM"]
    #[inline(always)]
    pub fn om(&self) -> OM_R {
        OM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - GEN"]
    #[inline(always)]
    pub fn gen(&self) -> GEN_R {
        GEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TM"]
    #[inline(always)]
    pub fn tm(&self) -> TM_R {
        TM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - LDSDL"]
    #[inline(always)]
    pub fn ldsdl(&self) -> LDSDL_R {
        LDSDL_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:14 - IRTO"]
    #[inline(always)]
    pub fn irto(&self) -> IRTO_R {
        IRTO_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - EECS"]
    #[inline(always)]
    pub fn eecs(&self) -> EECS_R {
        EECS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - AWL"]
    #[inline(always)]
    pub fn awl(&self) -> AWL_R {
        AWL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - EGTF"]
    #[inline(always)]
    pub fn egtf(&self) -> EGTF_R {
        EGTF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - ECC"]
    #[inline(always)]
    pub fn ecc(&self) -> ECC_R {
        ECC_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - EVTP"]
    #[inline(always)]
    pub fn evtp(&self) -> EVTP_R {
        EVTP_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - OM"]
    #[inline(always)]
    #[must_use]
    pub fn om(&mut self) -> OM_W<FDCAN_TTOCFrs> {
        OM_W::new(self, 0)
    }
    #[doc = "Bit 3 - GEN"]
    #[inline(always)]
    #[must_use]
    pub fn gen(&mut self) -> GEN_W<FDCAN_TTOCFrs> {
        GEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - TM"]
    #[inline(always)]
    #[must_use]
    pub fn tm(&mut self) -> TM_W<FDCAN_TTOCFrs> {
        TM_W::new(self, 4)
    }
    #[doc = "Bits 5:7 - LDSDL"]
    #[inline(always)]
    #[must_use]
    pub fn ldsdl(&mut self) -> LDSDL_W<FDCAN_TTOCFrs> {
        LDSDL_W::new(self, 5)
    }
    #[doc = "Bits 8:14 - IRTO"]
    #[inline(always)]
    #[must_use]
    pub fn irto(&mut self) -> IRTO_W<FDCAN_TTOCFrs> {
        IRTO_W::new(self, 8)
    }
    #[doc = "Bit 15 - EECS"]
    #[inline(always)]
    #[must_use]
    pub fn eecs(&mut self) -> EECS_W<FDCAN_TTOCFrs> {
        EECS_W::new(self, 15)
    }
    #[doc = "Bits 16:23 - AWL"]
    #[inline(always)]
    #[must_use]
    pub fn awl(&mut self) -> AWL_W<FDCAN_TTOCFrs> {
        AWL_W::new(self, 16)
    }
    #[doc = "Bit 24 - EGTF"]
    #[inline(always)]
    #[must_use]
    pub fn egtf(&mut self) -> EGTF_W<FDCAN_TTOCFrs> {
        EGTF_W::new(self, 24)
    }
    #[doc = "Bit 25 - ECC"]
    #[inline(always)]
    #[must_use]
    pub fn ecc(&mut self) -> ECC_W<FDCAN_TTOCFrs> {
        ECC_W::new(self, 25)
    }
    #[doc = "Bit 26 - EVTP"]
    #[inline(always)]
    #[must_use]
    pub fn evtp(&mut self) -> EVTP_W<FDCAN_TTOCFrs> {
        EVTP_W::new(self, 26)
    }
}
#[doc = "FDCAN TT operation configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ttocf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ttocf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_TTOCFrs;
impl crate::RegisterSpec for FDCAN_TTOCFrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ttocf::R`](R) reader structure"]
impl crate::Readable for FDCAN_TTOCFrs {}
#[doc = "`write(|w| ..)` method takes [`fdcan_ttocf::W`](W) writer structure"]
impl crate::Writable for FDCAN_TTOCFrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_TTOCF to value 0x0001_0000"]
impl crate::Resettable for FDCAN_TTOCFrs {
    const RESET_VALUE: u32 = 0x0001_0000;
}
