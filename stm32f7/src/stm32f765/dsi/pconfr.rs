#[doc = "Register `PCONFR` reader"]
pub type R = crate::R<PCONFRrs>;
#[doc = "Register `PCONFR` writer"]
pub type W = crate::W<PCONFRrs>;
#[doc = "Field `NL` reader - Number of Lanes"]
pub type NL_R = crate::FieldReader;
#[doc = "Field `NL` writer - Number of Lanes"]
pub type NL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SW_TIME` reader - Stop Wait Time"]
pub type SW_TIME_R = crate::FieldReader;
#[doc = "Field `SW_TIME` writer - Stop Wait Time"]
pub type SW_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:1 - Number of Lanes"]
    #[inline(always)]
    pub fn nl(&self) -> NL_R {
        NL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:15 - Stop Wait Time"]
    #[inline(always)]
    pub fn sw_time(&self) -> SW_TIME_R {
        SW_TIME_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Number of Lanes"]
    #[inline(always)]
    #[must_use]
    pub fn nl(&mut self) -> NL_W<PCONFRrs> {
        NL_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Stop Wait Time"]
    #[inline(always)]
    #[must_use]
    pub fn sw_time(&mut self) -> SW_TIME_W<PCONFRrs> {
        SW_TIME_W::new(self, 8)
    }
}
#[doc = "DSI Host PHY Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pconfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pconfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCONFRrs;
impl crate::RegisterSpec for PCONFRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pconfr::R`](R) reader structure"]
impl crate::Readable for PCONFRrs {}
#[doc = "`write(|w| ..)` method takes [`pconfr::W`](W) writer structure"]
impl crate::Writable for PCONFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCONFR to value 0"]
impl crate::Resettable for PCONFRrs {
    const RESET_VALUE: u32 = 0;
}
