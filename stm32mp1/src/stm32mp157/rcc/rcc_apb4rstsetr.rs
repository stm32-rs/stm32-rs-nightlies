#[doc = "Register `RCC_APB4RSTSETR` reader"]
pub type R = crate::R<RCC_APB4RSTSETRrs>;
#[doc = "Register `RCC_APB4RSTSETR` writer"]
pub type W = crate::W<RCC_APB4RSTSETRrs>;
#[doc = "Field `LTDCRST` reader - LTDCRST"]
pub type LTDCRST_R = crate::BitReader;
#[doc = "Field `LTDCRST` writer - LTDCRST"]
pub type LTDCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSIRST` reader - DSIRST"]
pub type DSIRST_R = crate::BitReader;
#[doc = "Field `DSIRST` writer - DSIRST"]
pub type DSIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDRPERFMRST` reader - DDRPERFMRST"]
pub type DDRPERFMRST_R = crate::BitReader;
#[doc = "Field `DDRPERFMRST` writer - DDRPERFMRST"]
pub type DDRPERFMRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBPHYRST` reader - USBPHYRST"]
pub type USBPHYRST_R = crate::BitReader;
#[doc = "Field `USBPHYRST` writer - USBPHYRST"]
pub type USBPHYRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LTDCRST"]
    #[inline(always)]
    pub fn ltdcrst(&self) -> LTDCRST_R {
        LTDCRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - DSIRST"]
    #[inline(always)]
    pub fn dsirst(&self) -> DSIRST_R {
        DSIRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - DDRPERFMRST"]
    #[inline(always)]
    pub fn ddrperfmrst(&self) -> DDRPERFMRST_R {
        DDRPERFMRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - USBPHYRST"]
    #[inline(always)]
    pub fn usbphyrst(&self) -> USBPHYRST_R {
        USBPHYRST_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LTDCRST"]
    #[inline(always)]
    #[must_use]
    pub fn ltdcrst(&mut self) -> LTDCRST_W<RCC_APB4RSTSETRrs> {
        LTDCRST_W::new(self, 0)
    }
    #[doc = "Bit 4 - DSIRST"]
    #[inline(always)]
    #[must_use]
    pub fn dsirst(&mut self) -> DSIRST_W<RCC_APB4RSTSETRrs> {
        DSIRST_W::new(self, 4)
    }
    #[doc = "Bit 8 - DDRPERFMRST"]
    #[inline(always)]
    #[must_use]
    pub fn ddrperfmrst(&mut self) -> DDRPERFMRST_W<RCC_APB4RSTSETRrs> {
        DDRPERFMRST_W::new(self, 8)
    }
    #[doc = "Bit 16 - USBPHYRST"]
    #[inline(always)]
    #[must_use]
    pub fn usbphyrst(&mut self) -> USBPHYRST_W<RCC_APB4RSTSETRrs> {
        USBPHYRST_W::new(self, 16)
    }
}
#[doc = "This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apb4rstsetr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apb4rstsetr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_APB4RSTSETRrs;
impl crate::RegisterSpec for RCC_APB4RSTSETRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_apb4rstsetr::R`](R) reader structure"]
impl crate::Readable for RCC_APB4RSTSETRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_apb4rstsetr::W`](W) writer structure"]
impl crate::Writable for RCC_APB4RSTSETRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_APB4RSTSETR to value 0"]
impl crate::Resettable for RCC_APB4RSTSETRrs {
    const RESET_VALUE: u32 = 0;
}
