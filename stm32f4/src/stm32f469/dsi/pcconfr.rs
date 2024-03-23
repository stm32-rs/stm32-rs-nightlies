#[doc = "Register `PCCONFR` reader"]
pub type R = crate::R<PCCONFRrs>;
#[doc = "Register `PCCONFR` writer"]
pub type W = crate::W<PCCONFRrs>;
#[doc = "Field `NL` reader - NL"]
pub type NL_R = crate::FieldReader;
#[doc = "Field `NL` writer - NL"]
pub type NL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SW_TIME` reader - SW_TIME"]
pub type SW_TIME_R = crate::FieldReader;
#[doc = "Field `SW_TIME` writer - SW_TIME"]
pub type SW_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:1 - NL"]
    #[inline(always)]
    pub fn nl(&self) -> NL_R {
        NL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:15 - SW_TIME"]
    #[inline(always)]
    pub fn sw_time(&self) -> SW_TIME_R {
        SW_TIME_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - NL"]
    #[inline(always)]
    #[must_use]
    pub fn nl(&mut self) -> NL_W<PCCONFRrs> {
        NL_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - SW_TIME"]
    #[inline(always)]
    #[must_use]
    pub fn sw_time(&mut self) -> SW_TIME_W<PCCONFRrs> {
        SW_TIME_W::new(self, 8)
    }
}
#[doc = "DSI Host PHY Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcconfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcconfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCCONFRrs;
impl crate::RegisterSpec for PCCONFRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcconfr::R`](R) reader structure"]
impl crate::Readable for PCCONFRrs {}
#[doc = "`write(|w| ..)` method takes [`pcconfr::W`](W) writer structure"]
impl crate::Writable for PCCONFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCCONFR to value 0x3133_302a"]
impl crate::Resettable for PCCONFRrs {
    const RESET_VALUE: u32 = 0x3133_302a;
}
