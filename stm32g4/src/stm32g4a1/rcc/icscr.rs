#[doc = "Register `ICSCR` reader"]
pub type R = crate::R<ICSCRrs>;
#[doc = "Register `ICSCR` writer"]
pub type W = crate::W<ICSCRrs>;
#[doc = "Field `HSICAL0` reader - Internal High Speed clock Calibration"]
pub type HSICAL0_R = crate::FieldReader;
#[doc = "Field `HSITRIM` reader - Internal High Speed clock trimming"]
pub type HSITRIM_R = crate::FieldReader;
#[doc = "Field `HSITRIM` writer - Internal High Speed clock trimming"]
pub type HSITRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 16:23 - Internal High Speed clock Calibration"]
    #[inline(always)]
    pub fn hsical0(&self) -> HSICAL0_R {
        HSICAL0_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:30 - Internal High Speed clock trimming"]
    #[inline(always)]
    pub fn hsitrim(&self) -> HSITRIM_R {
        HSITRIM_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:30 - Internal High Speed clock trimming"]
    #[inline(always)]
    #[must_use]
    pub fn hsitrim(&mut self) -> HSITRIM_W<ICSCRrs> {
        HSITRIM_W::new(self, 24)
    }
}
#[doc = "Internal clock sources calibration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICSCRrs;
impl crate::RegisterSpec for ICSCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icscr::R`](R) reader structure"]
impl crate::Readable for ICSCRrs {}
#[doc = "`write(|w| ..)` method takes [`icscr::W`](W) writer structure"]
impl crate::Writable for ICSCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICSCR to value 0x4000_0000"]
impl crate::Resettable for ICSCRrs {
    const RESET_VALUE: u32 = 0x4000_0000;
}
