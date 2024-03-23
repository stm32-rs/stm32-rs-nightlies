#[doc = "Register `PDCRC` reader"]
pub type R = crate::R<PDCRCrs>;
#[doc = "Register `PDCRC` writer"]
pub type W = crate::W<PDCRCrs>;
#[doc = "Field `PD14` reader - Port C pull-down bit i (i = 15, 14, 13, 7, 6) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PC\\[i\\]
I/O. On STM32C011xx, only PD15 and PD14 are available."]
pub type PD14_R = crate::BitReader;
#[doc = "Field `PD14` writer - Port C pull-down bit i (i = 15, 14, 13, 7, 6) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PC\\[i\\]
I/O. On STM32C011xx, only PD15 and PD14 are available."]
pub type PD14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD15` reader - Port C pull-down bit i (i = 15, 14, 13, 7, 6) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PC\\[i\\]
I/O. On STM32C011xx, only PD15 and PD14 are available."]
pub type PD15_R = crate::BitReader;
#[doc = "Field `PD15` writer - Port C pull-down bit i (i = 15, 14, 13, 7, 6) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PC\\[i\\]
I/O. On STM32C011xx, only PD15 and PD14 are available."]
pub type PD15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 14 - Port C pull-down bit i (i = 15, 14, 13, 7, 6) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PC\\[i\\]
I/O. On STM32C011xx, only PD15 and PD14 are available."]
    #[inline(always)]
    pub fn pd14(&self) -> PD14_R {
        PD14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port C pull-down bit i (i = 15, 14, 13, 7, 6) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PC\\[i\\]
I/O. On STM32C011xx, only PD15 and PD14 are available."]
    #[inline(always)]
    pub fn pd15(&self) -> PD15_R {
        PD15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - Port C pull-down bit i (i = 15, 14, 13, 7, 6) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PC\\[i\\]
I/O. On STM32C011xx, only PD15 and PD14 are available."]
    #[inline(always)]
    #[must_use]
    pub fn pd14(&mut self) -> PD14_W<PDCRCrs> {
        PD14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Port C pull-down bit i (i = 15, 14, 13, 7, 6) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PC\\[i\\]
I/O. On STM32C011xx, only PD15 and PD14 are available."]
    #[inline(always)]
    #[must_use]
    pub fn pd15(&mut self) -> PD15_W<PDCRCrs> {
        PD15_W::new(self, 15)
    }
}
#[doc = "PWR Port C pull-down control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdcrc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdcrc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDCRCrs;
impl crate::RegisterSpec for PDCRCrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdcrc::R`](R) reader structure"]
impl crate::Readable for PDCRCrs {}
#[doc = "`write(|w| ..)` method takes [`pdcrc::W`](W) writer structure"]
impl crate::Writable for PDCRCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDCRC to value 0"]
impl crate::Resettable for PDCRCrs {
    const RESET_VALUE: u32 = 0;
}
