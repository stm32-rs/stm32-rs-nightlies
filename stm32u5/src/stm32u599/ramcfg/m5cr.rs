#[doc = "Register `M5CR` reader"]
pub type R = crate::R<M5CRrs>;
#[doc = "Register `M5CR` writer"]
pub type W = crate::W<M5CRrs>;
#[doc = "Field `ECCE` reader - ECCE"]
pub type ECCE_R = crate::BitReader;
#[doc = "Field `ECCE` writer - ECCE"]
pub type ECCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALE` reader - ALE"]
pub type ALE_R = crate::BitReader;
#[doc = "Field `ALE` writer - ALE"]
pub type ALE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAMER` reader - SRAMER"]
pub type SRAMER_R = crate::BitReader;
#[doc = "Field `SRAMER` writer - SRAMER"]
pub type SRAMER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WSC` reader - WSC"]
pub type WSC_R = crate::FieldReader;
#[doc = "Field `WSC` writer - WSC"]
pub type WSC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - ECCE"]
    #[inline(always)]
    pub fn ecce(&self) -> ECCE_R {
        ECCE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - ALE"]
    #[inline(always)]
    pub fn ale(&self) -> ALE_R {
        ALE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - SRAMER"]
    #[inline(always)]
    pub fn sramer(&self) -> SRAMER_R {
        SRAMER_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:18 - WSC"]
    #[inline(always)]
    pub fn wsc(&self) -> WSC_R {
        WSC_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ECCE"]
    #[inline(always)]
    #[must_use]
    pub fn ecce(&mut self) -> ECCE_W<M5CRrs> {
        ECCE_W::new(self, 0)
    }
    #[doc = "Bit 4 - ALE"]
    #[inline(always)]
    #[must_use]
    pub fn ale(&mut self) -> ALE_W<M5CRrs> {
        ALE_W::new(self, 4)
    }
    #[doc = "Bit 8 - SRAMER"]
    #[inline(always)]
    #[must_use]
    pub fn sramer(&mut self) -> SRAMER_W<M5CRrs> {
        SRAMER_W::new(self, 8)
    }
    #[doc = "Bits 16:18 - WSC"]
    #[inline(always)]
    #[must_use]
    pub fn wsc(&mut self) -> WSC_W<M5CRrs> {
        WSC_W::new(self, 16)
    }
}
#[doc = "RAMCFG SRAM x control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m5cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m5cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M5CRrs;
impl crate::RegisterSpec for M5CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m5cr::R`](R) reader structure"]
impl crate::Readable for M5CRrs {}
#[doc = "`write(|w| ..)` method takes [`m5cr::W`](W) writer structure"]
impl crate::Writable for M5CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets M5CR to value 0"]
impl crate::Resettable for M5CRrs {
    const RESET_VALUE: u32 = 0;
}
