#[doc = "Register `APB3RSTR` reader"]
pub type R = crate::R<APB3RSTRrs>;
#[doc = "Register `APB3RSTR` writer"]
pub type W = crate::W<APB3RSTRrs>;
#[doc = "Field `RFRST` reader - Radio system BLE reset"]
pub type RFRST_R = crate::BitReader;
#[doc = "Field `RFRST` writer - Radio system BLE reset"]
pub type RFRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Radio system BLE reset"]
    #[inline(always)]
    pub fn rfrst(&self) -> RFRST_R {
        RFRST_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Radio system BLE reset"]
    #[inline(always)]
    #[must_use]
    pub fn rfrst(&mut self) -> RFRST_W<APB3RSTRrs> {
        RFRST_W::new(self, 0)
    }
}
#[doc = "APB3 peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb3rstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb3rstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB3RSTRrs;
impl crate::RegisterSpec for APB3RSTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb3rstr::R`](R) reader structure"]
impl crate::Readable for APB3RSTRrs {}
#[doc = "`write(|w| ..)` method takes [`apb3rstr::W`](W) writer structure"]
impl crate::Writable for APB3RSTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB3RSTR to value 0"]
impl crate::Resettable for APB3RSTRrs {
    const RESET_VALUE: u32 = 0;
}
