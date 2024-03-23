#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `RESET` reader - RESET bit"]
pub type RESET_R = crate::BitReader;
#[doc = "Field `RESET` writer - RESET bit"]
pub type RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POLYSIZE` reader - Polynomial size"]
pub type POLYSIZE_R = crate::FieldReader;
#[doc = "Field `POLYSIZE` writer - Polynomial size"]
pub type POLYSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REV_IN` reader - Reverse input data"]
pub type REV_IN_R = crate::FieldReader;
#[doc = "Field `REV_IN` writer - Reverse input data"]
pub type REV_IN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REV_OUT` reader - Reverse output data"]
pub type REV_OUT_R = crate::BitReader;
#[doc = "Field `REV_OUT` writer - Reverse output data"]
pub type REV_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RESET bit"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 3:4 - Polynomial size"]
    #[inline(always)]
    pub fn polysize(&self) -> POLYSIZE_R {
        POLYSIZE_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - Reverse input data"]
    #[inline(always)]
    pub fn rev_in(&self) -> REV_IN_R {
        REV_IN_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Reverse output data"]
    #[inline(always)]
    pub fn rev_out(&self) -> REV_OUT_R {
        REV_OUT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RESET bit"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<CRrs> {
        RESET_W::new(self, 0)
    }
    #[doc = "Bits 3:4 - Polynomial size"]
    #[inline(always)]
    #[must_use]
    pub fn polysize(&mut self) -> POLYSIZE_W<CRrs> {
        POLYSIZE_W::new(self, 3)
    }
    #[doc = "Bits 5:6 - Reverse input data"]
    #[inline(always)]
    #[must_use]
    pub fn rev_in(&mut self) -> REV_IN_W<CRrs> {
        REV_IN_W::new(self, 5)
    }
    #[doc = "Bit 7 - Reverse output data"]
    #[inline(always)]
    #[must_use]
    pub fn rev_out(&mut self) -> REV_OUT_W<CRrs> {
        REV_OUT_W::new(self, 7)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CRrs {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
