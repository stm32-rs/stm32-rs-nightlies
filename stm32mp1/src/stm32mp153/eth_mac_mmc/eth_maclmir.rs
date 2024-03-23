#[doc = "Register `ETH_MACLMIR` reader"]
pub type R = crate::R<ETH_MACLMIRrs>;
#[doc = "Register `ETH_MACLMIR` writer"]
pub type W = crate::W<ETH_MACLMIRrs>;
#[doc = "Field `LSI` reader - LSI"]
pub type LSI_R = crate::FieldReader;
#[doc = "Field `LSI` writer - LSI"]
pub type LSI_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DRSYNCR` reader - DRSYNCR"]
pub type DRSYNCR_R = crate::FieldReader;
#[doc = "Field `DRSYNCR` writer - DRSYNCR"]
pub type DRSYNCR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LMPDRI` reader - LMPDRI"]
pub type LMPDRI_R = crate::FieldReader;
#[doc = "Field `LMPDRI` writer - LMPDRI"]
pub type LMPDRI_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - LSI"]
    #[inline(always)]
    pub fn lsi(&self) -> LSI_R {
        LSI_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - DRSYNCR"]
    #[inline(always)]
    pub fn drsyncr(&self) -> DRSYNCR_R {
        DRSYNCR_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 24:31 - LMPDRI"]
    #[inline(always)]
    pub fn lmpdri(&self) -> LMPDRI_R {
        LMPDRI_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - LSI"]
    #[inline(always)]
    #[must_use]
    pub fn lsi(&mut self) -> LSI_W<ETH_MACLMIRrs> {
        LSI_W::new(self, 0)
    }
    #[doc = "Bits 8:10 - DRSYNCR"]
    #[inline(always)]
    #[must_use]
    pub fn drsyncr(&mut self) -> DRSYNCR_W<ETH_MACLMIRrs> {
        DRSYNCR_W::new(self, 8)
    }
    #[doc = "Bits 24:31 - LMPDRI"]
    #[inline(always)]
    #[must_use]
    pub fn lmpdri(&mut self) -> LMPDRI_W<ETH_MACLMIRrs> {
        LMPDRI_W::new(self, 24)
    }
}
#[doc = "This register contains the periodic intervals for automatic PTP packet generation.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_maclmir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_maclmir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MACLMIRrs;
impl crate::RegisterSpec for ETH_MACLMIRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_maclmir::R`](R) reader structure"]
impl crate::Readable for ETH_MACLMIRrs {}
#[doc = "`write(|w| ..)` method takes [`eth_maclmir::W`](W) writer structure"]
impl crate::Writable for ETH_MACLMIRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_MACLMIR to value 0"]
impl crate::Resettable for ETH_MACLMIRrs {
    const RESET_VALUE: u32 = 0;
}
