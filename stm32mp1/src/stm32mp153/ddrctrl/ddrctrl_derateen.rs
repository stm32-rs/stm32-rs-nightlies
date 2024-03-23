#[doc = "Register `DDRCTRL_DERATEEN` reader"]
pub type R = crate::R<DDRCTRL_DERATEENrs>;
#[doc = "Register `DDRCTRL_DERATEEN` writer"]
pub type W = crate::W<DDRCTRL_DERATEENrs>;
#[doc = "Field `DERATE_ENABLE` reader - DERATE_ENABLE"]
pub type DERATE_ENABLE_R = crate::BitReader;
#[doc = "Field `DERATE_ENABLE` writer - DERATE_ENABLE"]
pub type DERATE_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DERATE_VALUE` reader - DERATE_VALUE"]
pub type DERATE_VALUE_R = crate::FieldReader;
#[doc = "Field `DERATE_VALUE` writer - DERATE_VALUE"]
pub type DERATE_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DERATE_BYTE` reader - DERATE_BYTE"]
pub type DERATE_BYTE_R = crate::FieldReader;
#[doc = "Field `DERATE_BYTE` writer - DERATE_BYTE"]
pub type DERATE_BYTE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - DERATE_ENABLE"]
    #[inline(always)]
    pub fn derate_enable(&self) -> DERATE_ENABLE_R {
        DERATE_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - DERATE_VALUE"]
    #[inline(always)]
    pub fn derate_value(&self) -> DERATE_VALUE_R {
        DERATE_VALUE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 4:7 - DERATE_BYTE"]
    #[inline(always)]
    pub fn derate_byte(&self) -> DERATE_BYTE_R {
        DERATE_BYTE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DERATE_ENABLE"]
    #[inline(always)]
    #[must_use]
    pub fn derate_enable(&mut self) -> DERATE_ENABLE_W<DDRCTRL_DERATEENrs> {
        DERATE_ENABLE_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - DERATE_VALUE"]
    #[inline(always)]
    #[must_use]
    pub fn derate_value(&mut self) -> DERATE_VALUE_W<DDRCTRL_DERATEENrs> {
        DERATE_VALUE_W::new(self, 1)
    }
    #[doc = "Bits 4:7 - DERATE_BYTE"]
    #[inline(always)]
    #[must_use]
    pub fn derate_byte(&mut self) -> DERATE_BYTE_W<DDRCTRL_DERATEENrs> {
        DERATE_BYTE_W::new(self, 4)
    }
}
#[doc = "DDRCTRL temperature derate enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_derateen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_derateen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_DERATEENrs;
impl crate::RegisterSpec for DDRCTRL_DERATEENrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_derateen::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_DERATEENrs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_derateen::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_DERATEENrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_DERATEEN to value 0"]
impl crate::Resettable for DDRCTRL_DERATEENrs {
    const RESET_VALUE: u32 = 0;
}
