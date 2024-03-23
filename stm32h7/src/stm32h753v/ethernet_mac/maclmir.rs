#[doc = "Register `MACLMIR` reader"]
pub type R = crate::R<MACLMIRrs>;
#[doc = "Register `MACLMIR` writer"]
pub type W = crate::W<MACLMIRrs>;
#[doc = "Field `LSI` reader - Log Sync Interval"]
pub type LSI_R = crate::FieldReader;
#[doc = "Field `LSI` writer - Log Sync Interval"]
pub type LSI_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DRSYNCR` reader - Delay_Req to SYNC Ratio"]
pub type DRSYNCR_R = crate::FieldReader;
#[doc = "Field `DRSYNCR` writer - Delay_Req to SYNC Ratio"]
pub type DRSYNCR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LMPDRI` reader - Log Min Pdelay_Req Interval"]
pub type LMPDRI_R = crate::FieldReader;
#[doc = "Field `LMPDRI` writer - Log Min Pdelay_Req Interval"]
pub type LMPDRI_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Log Sync Interval"]
    #[inline(always)]
    pub fn lsi(&self) -> LSI_R {
        LSI_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Delay_Req to SYNC Ratio"]
    #[inline(always)]
    pub fn drsyncr(&self) -> DRSYNCR_R {
        DRSYNCR_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 24:31 - Log Min Pdelay_Req Interval"]
    #[inline(always)]
    pub fn lmpdri(&self) -> LMPDRI_R {
        LMPDRI_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Log Sync Interval"]
    #[inline(always)]
    #[must_use]
    pub fn lsi(&mut self) -> LSI_W<MACLMIRrs> {
        LSI_W::new(self, 0)
    }
    #[doc = "Bits 8:10 - Delay_Req to SYNC Ratio"]
    #[inline(always)]
    #[must_use]
    pub fn drsyncr(&mut self) -> DRSYNCR_W<MACLMIRrs> {
        DRSYNCR_W::new(self, 8)
    }
    #[doc = "Bits 24:31 - Log Min Pdelay_Req Interval"]
    #[inline(always)]
    #[must_use]
    pub fn lmpdri(&mut self) -> LMPDRI_W<MACLMIRrs> {
        LMPDRI_W::new(self, 24)
    }
}
#[doc = "Log message interval register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maclmir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maclmir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACLMIRrs;
impl crate::RegisterSpec for MACLMIRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maclmir::R`](R) reader structure"]
impl crate::Readable for MACLMIRrs {}
#[doc = "`write(|w| ..)` method takes [`maclmir::W`](W) writer structure"]
impl crate::Writable for MACLMIRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACLMIR to value 0"]
impl crate::Resettable for MACLMIRrs {
    const RESET_VALUE: u32 = 0;
}
