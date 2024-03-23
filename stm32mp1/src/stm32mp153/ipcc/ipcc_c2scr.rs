#[doc = "Register `IPCC_C2SCR` reader"]
pub type R = crate::R<IPCC_C2SCRrs>;
#[doc = "Register `IPCC_C2SCR` writer"]
pub type W = crate::W<IPCC_C2SCRrs>;
#[doc = "Field `CHxC` reader - CHxC"]
pub type CHX_C_R = crate::FieldReader;
#[doc = "Field `CHxC` writer - CHxC"]
pub type CHX_C_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CHxS` reader - CHxS"]
pub type CHX_S_R = crate::FieldReader;
#[doc = "Field `CHxS` writer - CHxS"]
pub type CHX_S_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - CHxC"]
    #[inline(always)]
    pub fn chx_c(&self) -> CHX_C_R {
        CHX_C_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - CHxS"]
    #[inline(always)]
    pub fn chx_s(&self) -> CHX_S_R {
        CHX_S_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - CHxC"]
    #[inline(always)]
    #[must_use]
    pub fn chx_c(&mut self) -> CHX_C_W<IPCC_C2SCRrs> {
        CHX_C_W::new(self, 0)
    }
    #[doc = "Bits 16:21 - CHxS"]
    #[inline(always)]
    #[must_use]
    pub fn chx_s(&mut self) -> CHX_S_W<IPCC_C2SCRrs> {
        CHX_S_W::new(self, 16)
    }
}
#[doc = "Reading this register will always return 0x0000 0000.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipcc_c2scr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipcc_c2scr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IPCC_C2SCRrs;
impl crate::RegisterSpec for IPCC_C2SCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipcc_c2scr::R`](R) reader structure"]
impl crate::Readable for IPCC_C2SCRrs {}
#[doc = "`write(|w| ..)` method takes [`ipcc_c2scr::W`](W) writer structure"]
impl crate::Writable for IPCC_C2SCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IPCC_C2SCR to value 0"]
impl crate::Resettable for IPCC_C2SCRrs {
    const RESET_VALUE: u32 = 0;
}
