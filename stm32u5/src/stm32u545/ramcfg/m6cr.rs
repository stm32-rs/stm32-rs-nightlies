#[doc = "Register `M6CR` reader"]
pub type R = crate::R<M6CRrs>;
#[doc = "Register `M6CR` writer"]
pub type W = crate::W<M6CRrs>;
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
    pub fn ecce(&mut self) -> ECCE_W<M6CRrs> {
        ECCE_W::new(self, 0)
    }
    #[doc = "Bit 4 - ALE"]
    #[inline(always)]
    #[must_use]
    pub fn ale(&mut self) -> ALE_W<M6CRrs> {
        ALE_W::new(self, 4)
    }
    #[doc = "Bit 8 - SRAMER"]
    #[inline(always)]
    #[must_use]
    pub fn sramer(&mut self) -> SRAMER_W<M6CRrs> {
        SRAMER_W::new(self, 8)
    }
    #[doc = "Bits 16:18 - WSC"]
    #[inline(always)]
    #[must_use]
    pub fn wsc(&mut self) -> WSC_W<M6CRrs> {
        WSC_W::new(self, 16)
    }
}
#[doc = "memory x control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m6cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m6cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M6CRrs;
impl crate::RegisterSpec for M6CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m6cr::R`](R) reader structure"]
impl crate::Readable for M6CRrs {}
#[doc = "`write(|w| ..)` method takes [`m6cr::W`](W) writer structure"]
impl crate::Writable for M6CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets M6CR to value 0"]
impl crate::Resettable for M6CRrs {
    const RESET_VALUE: u32 = 0;
}
