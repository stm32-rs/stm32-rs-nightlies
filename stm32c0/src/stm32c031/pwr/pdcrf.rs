#[doc = "Register `PDCRF` reader"]
pub type R = crate::R<PDCRFrs>;
#[doc = "Register `PDCRF` writer"]
pub type W = crate::W<PDCRFrs>;
#[doc = "Field `PD0` reader - Port F pull-down bit i (i = 2 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PF\\[i\\]
I/O. On STM32C011xx, only PD2 is available."]
pub type PD0_R = crate::BitReader;
#[doc = "Field `PD0` writer - Port F pull-down bit i (i = 2 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PF\\[i\\]
I/O. On STM32C011xx, only PD2 is available."]
pub type PD0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD1` reader - Port F pull-down bit i (i = 2 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PF\\[i\\]
I/O. On STM32C011xx, only PD2 is available."]
pub type PD1_R = crate::BitReader;
#[doc = "Field `PD1` writer - Port F pull-down bit i (i = 2 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PF\\[i\\]
I/O. On STM32C011xx, only PD2 is available."]
pub type PD1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD2` reader - Port F pull-down bit i (i = 2 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PF\\[i\\]
I/O. On STM32C011xx, only PD2 is available."]
pub type PD2_R = crate::BitReader;
#[doc = "Field `PD2` writer - Port F pull-down bit i (i = 2 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PF\\[i\\]
I/O. On STM32C011xx, only PD2 is available."]
pub type PD2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port F pull-down bit i (i = 2 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PF\\[i\\]
I/O. On STM32C011xx, only PD2 is available."]
    #[inline(always)]
    pub fn pd0(&self) -> PD0_R {
        PD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port F pull-down bit i (i = 2 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PF\\[i\\]
I/O. On STM32C011xx, only PD2 is available."]
    #[inline(always)]
    pub fn pd1(&self) -> PD1_R {
        PD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port F pull-down bit i (i = 2 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PF\\[i\\]
I/O. On STM32C011xx, only PD2 is available."]
    #[inline(always)]
    pub fn pd2(&self) -> PD2_R {
        PD2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port F pull-down bit i (i = 2 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PF\\[i\\]
I/O. On STM32C011xx, only PD2 is available."]
    #[inline(always)]
    #[must_use]
    pub fn pd0(&mut self) -> PD0_W<PDCRFrs> {
        PD0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Port F pull-down bit i (i = 2 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PF\\[i\\]
I/O. On STM32C011xx, only PD2 is available."]
    #[inline(always)]
    #[must_use]
    pub fn pd1(&mut self) -> PD1_W<PDCRFrs> {
        PD1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Port F pull-down bit i (i = 2 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PF\\[i\\]
I/O. On STM32C011xx, only PD2 is available."]
    #[inline(always)]
    #[must_use]
    pub fn pd2(&mut self) -> PD2_W<PDCRFrs> {
        PD2_W::new(self, 2)
    }
}
#[doc = "PWR Port F pull-down control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdcrf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdcrf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDCRFrs;
impl crate::RegisterSpec for PDCRFrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdcrf::R`](R) reader structure"]
impl crate::Readable for PDCRFrs {}
#[doc = "`write(|w| ..)` method takes [`pdcrf::W`](W) writer structure"]
impl crate::Writable for PDCRFrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDCRF to value 0"]
impl crate::Resettable for PDCRFrs {
    const RESET_VALUE: u32 = 0;
}
