#[doc = "Register `RCC_MP_APB3LPENCLRR` reader"]
pub type R = crate::R<RCC_MP_APB3LPENCLRRrs>;
#[doc = "Register `RCC_MP_APB3LPENCLRR` writer"]
pub type W = crate::W<RCC_MP_APB3LPENCLRRrs>;
#[doc = "Field `LPTIM2LPEN` reader - LPTIM2LPEN"]
pub type LPTIM2LPEN_R = crate::BitReader;
#[doc = "Field `LPTIM2LPEN` writer - LPTIM2LPEN"]
pub type LPTIM2LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM3LPEN` reader - LPTIM3LPEN"]
pub type LPTIM3LPEN_R = crate::BitReader;
#[doc = "Field `LPTIM3LPEN` writer - LPTIM3LPEN"]
pub type LPTIM3LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM4LPEN` reader - LPTIM4LPEN"]
pub type LPTIM4LPEN_R = crate::BitReader;
#[doc = "Field `LPTIM4LPEN` writer - LPTIM4LPEN"]
pub type LPTIM4LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM5LPEN` reader - LPTIM5LPEN"]
pub type LPTIM5LPEN_R = crate::BitReader;
#[doc = "Field `LPTIM5LPEN` writer - LPTIM5LPEN"]
pub type LPTIM5LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAI4LPEN` reader - SAI4LPEN"]
pub type SAI4LPEN_R = crate::BitReader;
#[doc = "Field `SAI4LPEN` writer - SAI4LPEN"]
pub type SAI4LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSCFGLPEN` reader - SYSCFGLPEN"]
pub type SYSCFGLPEN_R = crate::BitReader;
#[doc = "Field `SYSCFGLPEN` writer - SYSCFGLPEN"]
pub type SYSCFGLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREFLPEN` reader - VREFLPEN"]
pub type VREFLPEN_R = crate::BitReader;
#[doc = "Field `VREFLPEN` writer - VREFLPEN"]
pub type VREFLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTSLPEN` reader - DTSLPEN"]
pub type DTSLPEN_R = crate::BitReader;
#[doc = "Field `DTSLPEN` writer - DTSLPEN"]
pub type DTSLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LPTIM2LPEN"]
    #[inline(always)]
    pub fn lptim2lpen(&self) -> LPTIM2LPEN_R {
        LPTIM2LPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LPTIM3LPEN"]
    #[inline(always)]
    pub fn lptim3lpen(&self) -> LPTIM3LPEN_R {
        LPTIM3LPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LPTIM4LPEN"]
    #[inline(always)]
    pub fn lptim4lpen(&self) -> LPTIM4LPEN_R {
        LPTIM4LPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LPTIM5LPEN"]
    #[inline(always)]
    pub fn lptim5lpen(&self) -> LPTIM5LPEN_R {
        LPTIM5LPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - SAI4LPEN"]
    #[inline(always)]
    pub fn sai4lpen(&self) -> SAI4LPEN_R {
        SAI4LPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - SYSCFGLPEN"]
    #[inline(always)]
    pub fn syscfglpen(&self) -> SYSCFGLPEN_R {
        SYSCFGLPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - VREFLPEN"]
    #[inline(always)]
    pub fn vreflpen(&self) -> VREFLPEN_R {
        VREFLPEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - DTSLPEN"]
    #[inline(always)]
    pub fn dtslpen(&self) -> DTSLPEN_R {
        DTSLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPTIM2LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2lpen(&mut self) -> LPTIM2LPEN_W<RCC_MP_APB3LPENCLRRrs> {
        LPTIM2LPEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - LPTIM3LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn lptim3lpen(&mut self) -> LPTIM3LPEN_W<RCC_MP_APB3LPENCLRRrs> {
        LPTIM3LPEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - LPTIM4LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn lptim4lpen(&mut self) -> LPTIM4LPEN_W<RCC_MP_APB3LPENCLRRrs> {
        LPTIM4LPEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - LPTIM5LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn lptim5lpen(&mut self) -> LPTIM5LPEN_W<RCC_MP_APB3LPENCLRRrs> {
        LPTIM5LPEN_W::new(self, 3)
    }
    #[doc = "Bit 8 - SAI4LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn sai4lpen(&mut self) -> SAI4LPEN_W<RCC_MP_APB3LPENCLRRrs> {
        SAI4LPEN_W::new(self, 8)
    }
    #[doc = "Bit 11 - SYSCFGLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn syscfglpen(&mut self) -> SYSCFGLPEN_W<RCC_MP_APB3LPENCLRRrs> {
        SYSCFGLPEN_W::new(self, 11)
    }
    #[doc = "Bit 13 - VREFLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn vreflpen(&mut self) -> VREFLPEN_W<RCC_MP_APB3LPENCLRRrs> {
        VREFLPEN_W::new(self, 13)
    }
    #[doc = "Bit 16 - DTSLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn dtslpen(&mut self) -> DTSLPEN_W<RCC_MP_APB3LPENCLRRrs> {
        DTSLPEN_W::new(self, 16)
    }
}
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_apb3lpenclrr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_apb3lpenclrr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_MP_APB3LPENCLRRrs;
impl crate::RegisterSpec for RCC_MP_APB3LPENCLRRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_mp_apb3lpenclrr::R`](R) reader structure"]
impl crate::Readable for RCC_MP_APB3LPENCLRRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_mp_apb3lpenclrr::W`](W) writer structure"]
impl crate::Writable for RCC_MP_APB3LPENCLRRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_MP_APB3LPENCLRR to value 0x0003_290f"]
impl crate::Resettable for RCC_MP_APB3LPENCLRRrs {
    const RESET_VALUE: u32 = 0x0003_290f;
}
