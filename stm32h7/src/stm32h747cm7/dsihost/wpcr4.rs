#[doc = "Register `WPCR4` reader"]
pub type R = crate::R<WPCR4rs>;
#[doc = "Register `WPCR4` writer"]
pub type W = crate::W<WPCR4rs>;
#[doc = "Field `TCLKPOST` reader - tCLK-POST"]
pub type TCLKPOST_R = crate::FieldReader;
#[doc = "Field `TCLKPOST` writer - tCLK-POST"]
pub type TCLKPOST_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - tCLK-POST"]
    #[inline(always)]
    pub fn tclkpost(&self) -> TCLKPOST_R {
        TCLKPOST_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - tCLK-POST"]
    #[inline(always)]
    #[must_use]
    pub fn tclkpost(&mut self) -> TCLKPOST_W<WPCR4rs> {
        TCLKPOST_W::new(self, 0)
    }
}
#[doc = "DSI wrapper PHY configuration register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpcr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpcr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WPCR4rs;
impl crate::RegisterSpec for WPCR4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wpcr4::R`](R) reader structure"]
impl crate::Readable for WPCR4rs {}
#[doc = "`write(|w| ..)` method takes [`wpcr4::W`](W) writer structure"]
impl crate::Writable for WPCR4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WPCR4 to value 0"]
impl crate::Resettable for WPCR4rs {
    const RESET_VALUE: u32 = 0;
}
