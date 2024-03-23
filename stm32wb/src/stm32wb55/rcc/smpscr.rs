#[doc = "Register `SMPSCR` reader"]
pub type R = crate::R<SMPSCRrs>;
#[doc = "Register `SMPSCR` writer"]
pub type W = crate::W<SMPSCRrs>;
#[doc = "Field `SMPSSEL` reader - Step Down converter clock selection"]
pub type SMPSSEL_R = crate::FieldReader;
#[doc = "Field `SMPSSEL` writer - Step Down converter clock selection"]
pub type SMPSSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SMPSDIV` reader - Step Down converter clock prescaler"]
pub type SMPSDIV_R = crate::FieldReader;
#[doc = "Field `SMPSDIV` writer - Step Down converter clock prescaler"]
pub type SMPSDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SMPSSWS` reader - Step Down converter clock switch status"]
pub type SMPSSWS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Step Down converter clock selection"]
    #[inline(always)]
    pub fn smpssel(&self) -> SMPSSEL_R {
        SMPSSEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Step Down converter clock prescaler"]
    #[inline(always)]
    pub fn smpsdiv(&self) -> SMPSDIV_R {
        SMPSDIV_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Step Down converter clock switch status"]
    #[inline(always)]
    pub fn smpssws(&self) -> SMPSSWS_R {
        SMPSSWS_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Step Down converter clock selection"]
    #[inline(always)]
    #[must_use]
    pub fn smpssel(&mut self) -> SMPSSEL_W<SMPSCRrs> {
        SMPSSEL_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - Step Down converter clock prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn smpsdiv(&mut self) -> SMPSDIV_W<SMPSCRrs> {
        SMPSDIV_W::new(self, 4)
    }
}
#[doc = "Step Down converter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smpscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smpscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMPSCRrs;
impl crate::RegisterSpec for SMPSCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smpscr::R`](R) reader structure"]
impl crate::Readable for SMPSCRrs {}
#[doc = "`write(|w| ..)` method takes [`smpscr::W`](W) writer structure"]
impl crate::Writable for SMPSCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SMPSCR to value 0x0301"]
impl crate::Resettable for SMPSCRrs {
    const RESET_VALUE: u32 = 0x0301;
}
