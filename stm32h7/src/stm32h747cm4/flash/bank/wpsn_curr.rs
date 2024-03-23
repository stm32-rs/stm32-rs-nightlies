#[doc = "Register `WPSN_CURR` reader"]
pub type R = crate::R<WPSN_CURRrs>;
#[doc = "Register `WPSN_CURR` writer"]
pub type W = crate::W<WPSN_CURRrs>;
#[doc = "Field `WRPSn` reader - Bank 1 sector write protection option status byte"]
pub type WRPSN_R = crate::FieldReader;
#[doc = "Field `WRPSn` writer - Bank 1 sector write protection option status byte"]
pub type WRPSN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Bank 1 sector write protection option status byte"]
    #[inline(always)]
    pub fn wrpsn(&self) -> WRPSN_R {
        WRPSN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bank 1 sector write protection option status byte"]
    #[inline(always)]
    #[must_use]
    pub fn wrpsn(&mut self) -> WRPSN_W<WPSN_CURRrs> {
        WRPSN_W::new(self, 0)
    }
}
#[doc = "FLASH write sector protection for bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpsn_curr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpsn_curr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WPSN_CURRrs;
impl crate::RegisterSpec for WPSN_CURRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wpsn_curr::R`](R) reader structure"]
impl crate::Readable for WPSN_CURRrs {}
#[doc = "`write(|w| ..)` method takes [`wpsn_curr::W`](W) writer structure"]
impl crate::Writable for WPSN_CURRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WPSN_CURR to value 0"]
impl crate::Resettable for WPSN_CURRrs {
    const RESET_VALUE: u32 = 0;
}
