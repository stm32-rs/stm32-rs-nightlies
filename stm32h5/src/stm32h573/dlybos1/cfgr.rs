#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CFGRrs>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CFGRrs>;
#[doc = "Field `SEL` reader - SEL"]
pub type SEL_R = crate::FieldReader;
#[doc = "Field `SEL` writer - SEL"]
pub type SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `UNIT` reader - UNIT"]
pub type UNIT_R = crate::FieldReader;
#[doc = "Field `UNIT` writer - UNIT"]
pub type UNIT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `LNG` reader - LNG"]
pub type LNG_R = crate::FieldReader<u16>;
#[doc = "Field `LNGF` reader - LNGF"]
pub type LNGF_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - SEL"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:14 - UNIT"]
    #[inline(always)]
    pub fn unit(&self) -> UNIT_R {
        UNIT_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:27 - LNG"]
    #[inline(always)]
    pub fn lng(&self) -> LNG_R {
        LNG_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - LNGF"]
    #[inline(always)]
    pub fn lngf(&self) -> LNGF_R {
        LNGF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - SEL"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SEL_W<CFGRrs> {
        SEL_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - UNIT"]
    #[inline(always)]
    #[must_use]
    pub fn unit(&mut self) -> UNIT_W<CFGRrs> {
        UNIT_W::new(self, 8)
    }
}
#[doc = "configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGRrs;
impl crate::RegisterSpec for CFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CFGRrs {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CFGRrs {
    const RESET_VALUE: u32 = 0;
}
