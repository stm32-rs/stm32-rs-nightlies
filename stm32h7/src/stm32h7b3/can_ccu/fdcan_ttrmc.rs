#[doc = "Register `FDCAN_TTRMC` reader"]
pub type R = crate::R<FDCAN_TTRMCrs>;
#[doc = "Register `FDCAN_TTRMC` writer"]
pub type W = crate::W<FDCAN_TTRMCrs>;
#[doc = "Field `RID` reader - Reference Identifier."]
pub type RID_R = crate::FieldReader<u32>;
#[doc = "Field `RID` writer - Reference Identifier."]
pub type RID_W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
#[doc = "Field `XTD` reader - Extended Identifier"]
pub type XTD_R = crate::BitReader;
#[doc = "Field `XTD` writer - Extended Identifier"]
pub type XTD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RMPS` reader - Reference Message Payload Select"]
pub type RMPS_R = crate::BitReader;
#[doc = "Field `RMPS` writer - Reference Message Payload Select"]
pub type RMPS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:28 - Reference Identifier."]
    #[inline(always)]
    pub fn rid(&self) -> RID_R {
        RID_R::new(self.bits & 0x1fff_ffff)
    }
    #[doc = "Bit 30 - Extended Identifier"]
    #[inline(always)]
    pub fn xtd(&self) -> XTD_R {
        XTD_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Reference Message Payload Select"]
    #[inline(always)]
    pub fn rmps(&self) -> RMPS_R {
        RMPS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:28 - Reference Identifier."]
    #[inline(always)]
    #[must_use]
    pub fn rid(&mut self) -> RID_W<FDCAN_TTRMCrs> {
        RID_W::new(self, 0)
    }
    #[doc = "Bit 30 - Extended Identifier"]
    #[inline(always)]
    #[must_use]
    pub fn xtd(&mut self) -> XTD_W<FDCAN_TTRMCrs> {
        XTD_W::new(self, 30)
    }
    #[doc = "Bit 31 - Reference Message Payload Select"]
    #[inline(always)]
    #[must_use]
    pub fn rmps(&mut self) -> RMPS_W<FDCAN_TTRMCrs> {
        RMPS_W::new(self, 31)
    }
}
#[doc = "FDCAN TT Reference Message Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ttrmc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ttrmc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_TTRMCrs;
impl crate::RegisterSpec for FDCAN_TTRMCrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ttrmc::R`](R) reader structure"]
impl crate::Readable for FDCAN_TTRMCrs {}
#[doc = "`write(|w| ..)` method takes [`fdcan_ttrmc::W`](W) writer structure"]
impl crate::Writable for FDCAN_TTRMCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_TTRMC to value 0"]
impl crate::Resettable for FDCAN_TTRMCrs {
    const RESET_VALUE: u32 = 0;
}
