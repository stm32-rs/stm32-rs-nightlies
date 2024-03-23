#[doc = "Register `RCC_MP_SREQCLRR` reader"]
pub type R = crate::R<RCC_MP_SREQCLRRrs>;
#[doc = "Register `RCC_MP_SREQCLRR` writer"]
pub type W = crate::W<RCC_MP_SREQCLRRrs>;
#[doc = "Field `STPREQ_P0` reader - STPREQ_P0"]
pub type STPREQ_P0_R = crate::BitReader;
#[doc = "Field `STPREQ_P0` writer - STPREQ_P0"]
pub type STPREQ_P0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STPREQ_P1` reader - STPREQ_P1"]
pub type STPREQ_P1_R = crate::BitReader;
#[doc = "Field `STPREQ_P1` writer - STPREQ_P1"]
pub type STPREQ_P1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - STPREQ_P0"]
    #[inline(always)]
    pub fn stpreq_p0(&self) -> STPREQ_P0_R {
        STPREQ_P0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - STPREQ_P1"]
    #[inline(always)]
    pub fn stpreq_p1(&self) -> STPREQ_P1_R {
        STPREQ_P1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - STPREQ_P0"]
    #[inline(always)]
    #[must_use]
    pub fn stpreq_p0(&mut self) -> STPREQ_P0_W<RCC_MP_SREQCLRRrs> {
        STPREQ_P0_W::new(self, 0)
    }
    #[doc = "Bit 1 - STPREQ_P1"]
    #[inline(always)]
    #[must_use]
    pub fn stpreq_p1(&mut self) -> STPREQ_P1_W<RCC_MP_SREQCLRRrs> {
        STPREQ_P1_W::new(self, 1)
    }
}
#[doc = "Writing has no effect, reading will return the effective values of the bits. Writing a sets the corresponding bit to . The MCU cannot access to this register. If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_sreqclrr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_sreqclrr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_MP_SREQCLRRrs;
impl crate::RegisterSpec for RCC_MP_SREQCLRRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_mp_sreqclrr::R`](R) reader structure"]
impl crate::Readable for RCC_MP_SREQCLRRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_mp_sreqclrr::W`](W) writer structure"]
impl crate::Writable for RCC_MP_SREQCLRRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_MP_SREQCLRR to value 0"]
impl crate::Resettable for RCC_MP_SREQCLRRrs {
    const RESET_VALUE: u32 = 0;
}
