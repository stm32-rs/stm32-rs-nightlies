#[doc = "Register `RCC_MC_APB4LPENSETR` reader"]
pub type R = crate::R<RCC_MC_APB4LPENSETRrs>;
#[doc = "Register `RCC_MC_APB4LPENSETR` writer"]
pub type W = crate::W<RCC_MC_APB4LPENSETRrs>;
#[doc = "Field `LTDCLPEN` reader - LTDCLPEN"]
pub type LTDCLPEN_R = crate::BitReader;
#[doc = "Field `LTDCLPEN` writer - LTDCLPEN"]
pub type LTDCLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSILPEN` reader - DSILPEN"]
pub type DSILPEN_R = crate::BitReader;
#[doc = "Field `DSILPEN` writer - DSILPEN"]
pub type DSILPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDRPERFMLPEN` reader - DDRPERFMLPEN"]
pub type DDRPERFMLPEN_R = crate::BitReader;
#[doc = "Field `DDRPERFMLPEN` writer - DDRPERFMLPEN"]
pub type DDRPERFMLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBPHYLPEN` reader - USBPHYLPEN"]
pub type USBPHYLPEN_R = crate::BitReader;
#[doc = "Field `USBPHYLPEN` writer - USBPHYLPEN"]
pub type USBPHYLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STGENROLPEN` reader - STGENROLPEN"]
pub type STGENROLPEN_R = crate::BitReader;
#[doc = "Field `STGENROLPEN` writer - STGENROLPEN"]
pub type STGENROLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STGENROSTPEN` reader - STGENROSTPEN"]
pub type STGENROSTPEN_R = crate::BitReader;
#[doc = "Field `STGENROSTPEN` writer - STGENROSTPEN"]
pub type STGENROSTPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LTDCLPEN"]
    #[inline(always)]
    pub fn ltdclpen(&self) -> LTDCLPEN_R {
        LTDCLPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - DSILPEN"]
    #[inline(always)]
    pub fn dsilpen(&self) -> DSILPEN_R {
        DSILPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - DDRPERFMLPEN"]
    #[inline(always)]
    pub fn ddrperfmlpen(&self) -> DDRPERFMLPEN_R {
        DDRPERFMLPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - USBPHYLPEN"]
    #[inline(always)]
    pub fn usbphylpen(&self) -> USBPHYLPEN_R {
        USBPHYLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - STGENROLPEN"]
    #[inline(always)]
    pub fn stgenrolpen(&self) -> STGENROLPEN_R {
        STGENROLPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - STGENROSTPEN"]
    #[inline(always)]
    pub fn stgenrostpen(&self) -> STGENROSTPEN_R {
        STGENROSTPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LTDCLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn ltdclpen(&mut self) -> LTDCLPEN_W<RCC_MC_APB4LPENSETRrs> {
        LTDCLPEN_W::new(self, 0)
    }
    #[doc = "Bit 4 - DSILPEN"]
    #[inline(always)]
    #[must_use]
    pub fn dsilpen(&mut self) -> DSILPEN_W<RCC_MC_APB4LPENSETRrs> {
        DSILPEN_W::new(self, 4)
    }
    #[doc = "Bit 8 - DDRPERFMLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn ddrperfmlpen(&mut self) -> DDRPERFMLPEN_W<RCC_MC_APB4LPENSETRrs> {
        DDRPERFMLPEN_W::new(self, 8)
    }
    #[doc = "Bit 16 - USBPHYLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn usbphylpen(&mut self) -> USBPHYLPEN_W<RCC_MC_APB4LPENSETRrs> {
        USBPHYLPEN_W::new(self, 16)
    }
    #[doc = "Bit 20 - STGENROLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn stgenrolpen(&mut self) -> STGENROLPEN_W<RCC_MC_APB4LPENSETRrs> {
        STGENROLPEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - STGENROSTPEN"]
    #[inline(always)]
    #[must_use]
    pub fn stgenrostpen(&mut self) -> STGENROSTPEN_W<RCC_MC_APB4LPENSETRrs> {
        STGENROSTPEN_W::new(self, 21)
    }
}
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_apb4lpensetr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_apb4lpensetr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_MC_APB4LPENSETRrs;
impl crate::RegisterSpec for RCC_MC_APB4LPENSETRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_mc_apb4lpensetr::R`](R) reader structure"]
impl crate::Readable for RCC_MC_APB4LPENSETRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_mc_apb4lpensetr::W`](W) writer structure"]
impl crate::Writable for RCC_MC_APB4LPENSETRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_MC_APB4LPENSETR to value 0x0011_0111"]
impl crate::Resettable for RCC_MC_APB4LPENSETRrs {
    const RESET_VALUE: u32 = 0x0011_0111;
}
