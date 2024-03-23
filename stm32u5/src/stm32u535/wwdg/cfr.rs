#[doc = "Register `CFR` reader"]
pub type R = crate::R<CFRrs>;
#[doc = "Register `CFR` writer"]
pub type W = crate::W<CFRrs>;
#[doc = "Field `W` reader - 7-bit window value"]
pub type W_R = crate::FieldReader;
#[doc = "Field `W` writer - 7-bit window value"]
pub type W_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `EWI` reader - Early wakeup interrupt"]
pub type EWI_R = crate::BitReader;
#[doc = "Field `EWI` writer - Early wakeup interrupt"]
pub type EWI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDGTB` reader - Timer base"]
pub type WDGTB_R = crate::FieldReader;
#[doc = "Field `WDGTB` writer - Timer base"]
pub type WDGTB_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:6 - 7-bit window value"]
    #[inline(always)]
    pub fn w(&self) -> W_R {
        W_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 9 - Early wakeup interrupt"]
    #[inline(always)]
    pub fn ewi(&self) -> EWI_R {
        EWI_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 11:13 - Timer base"]
    #[inline(always)]
    pub fn wdgtb(&self) -> WDGTB_R {
        WDGTB_R::new(((self.bits >> 11) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 7-bit window value"]
    #[inline(always)]
    #[must_use]
    pub fn w(&mut self) -> W_W<CFRrs> {
        W_W::new(self, 0)
    }
    #[doc = "Bit 9 - Early wakeup interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ewi(&mut self) -> EWI_W<CFRrs> {
        EWI_W::new(self, 9)
    }
    #[doc = "Bits 11:13 - Timer base"]
    #[inline(always)]
    #[must_use]
    pub fn wdgtb(&mut self) -> WDGTB_W<CFRrs> {
        WDGTB_W::new(self, 11)
    }
}
#[doc = "Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFRrs;
impl crate::RegisterSpec for CFRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfr::R`](R) reader structure"]
impl crate::Readable for CFRrs {}
#[doc = "`write(|w| ..)` method takes [`cfr::W`](W) writer structure"]
impl crate::Writable for CFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFR to value 0x7f"]
impl crate::Resettable for CFRrs {
    const RESET_VALUE: u32 = 0x7f;
}
