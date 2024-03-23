#[doc = "Register `PDCRB` reader"]
pub type R = crate::R<PDCRBrs>;
#[doc = "Register `PDCRB` writer"]
pub type W = crate::W<PDCRBrs>;
#[doc = "Field `PD6` reader - Port B pull-down bit i (i = 15 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PB\\[i\\]
I/O. On STM32C011xx, only PD7 and PD6 are available"]
pub type PD6_R = crate::BitReader;
#[doc = "Field `PD6` writer - Port B pull-down bit i (i = 15 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PB\\[i\\]
I/O. On STM32C011xx, only PD7 and PD6 are available"]
pub type PD6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD7` reader - Port B pull-down bit i (i = 15 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PB\\[i\\]
I/O. On STM32C011xx, only PD7 and PD6 are available"]
pub type PD7_R = crate::BitReader;
#[doc = "Field `PD7` writer - Port B pull-down bit i (i = 15 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PB\\[i\\]
I/O. On STM32C011xx, only PD7 and PD6 are available"]
pub type PD7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 6 - Port B pull-down bit i (i = 15 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PB\\[i\\]
I/O. On STM32C011xx, only PD7 and PD6 are available"]
    #[inline(always)]
    pub fn pd6(&self) -> PD6_R {
        PD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port B pull-down bit i (i = 15 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PB\\[i\\]
I/O. On STM32C011xx, only PD7 and PD6 are available"]
    #[inline(always)]
    pub fn pd7(&self) -> PD7_R {
        PD7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Port B pull-down bit i (i = 15 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PB\\[i\\]
I/O. On STM32C011xx, only PD7 and PD6 are available"]
    #[inline(always)]
    #[must_use]
    pub fn pd6(&mut self) -> PD6_W<PDCRBrs> {
        PD6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Port B pull-down bit i (i = 15 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PB\\[i\\]
I/O. On STM32C011xx, only PD7 and PD6 are available"]
    #[inline(always)]
    #[must_use]
    pub fn pd7(&mut self) -> PD7_W<PDCRBrs> {
        PD7_W::new(self, 7)
    }
}
#[doc = "PWR Port B pull-down control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdcrb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdcrb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDCRBrs;
impl crate::RegisterSpec for PDCRBrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdcrb::R`](R) reader structure"]
impl crate::Readable for PDCRBrs {}
#[doc = "`write(|w| ..)` method takes [`pdcrb::W`](W) writer structure"]
impl crate::Writable for PDCRBrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDCRB to value 0"]
impl crate::Resettable for PDCRBrs {
    const RESET_VALUE: u32 = 0;
}
