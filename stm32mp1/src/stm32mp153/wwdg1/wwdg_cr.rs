#[doc = "Register `WWDG_CR` reader"]
pub type R = crate::R<WWDG_CRrs>;
#[doc = "Register `WWDG_CR` writer"]
pub type W = crate::W<WWDG_CRrs>;
#[doc = "Field `T` reader - T"]
pub type T_R = crate::FieldReader;
#[doc = "Field `T` writer - T"]
pub type T_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `WDGA` reader - WDGA"]
pub type WDGA_R = crate::BitReader;
#[doc = "Field `WDGA` writer - WDGA"]
pub type WDGA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - T"]
    #[inline(always)]
    pub fn t(&self) -> T_R {
        T_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - WDGA"]
    #[inline(always)]
    pub fn wdga(&self) -> WDGA_R {
        WDGA_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - T"]
    #[inline(always)]
    #[must_use]
    pub fn t(&mut self) -> T_W<WWDG_CRrs> {
        T_W::new(self, 0)
    }
    #[doc = "Bit 7 - WDGA"]
    #[inline(always)]
    #[must_use]
    pub fn wdga(&mut self) -> WDGA_W<WWDG_CRrs> {
        WDGA_W::new(self, 7)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wwdg_cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wwdg_cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WWDG_CRrs;
impl crate::RegisterSpec for WWDG_CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wwdg_cr::R`](R) reader structure"]
impl crate::Readable for WWDG_CRrs {}
#[doc = "`write(|w| ..)` method takes [`wwdg_cr::W`](W) writer structure"]
impl crate::Writable for WWDG_CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WWDG_CR to value 0x7f"]
impl crate::Resettable for WWDG_CRrs {
    const RESET_VALUE: u32 = 0x7f;
}
