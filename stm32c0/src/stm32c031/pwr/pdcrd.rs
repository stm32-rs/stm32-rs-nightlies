#[doc = "Register `PDCRD` reader"]
pub type R = crate::R<PDCRDrs>;
#[doc = "Register `PDCRD` writer"]
pub type W = crate::W<PDCRDrs>;
#[doc = "Field `PD0` reader - Port D pull-down bit i (i = 3 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PD\\[i\\]
I/O."]
pub type PD0_R = crate::BitReader;
#[doc = "Field `PD0` writer - Port D pull-down bit i (i = 3 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PD\\[i\\]
I/O."]
pub type PD0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD1` reader - Port D pull-down bit i (i = 3 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PD\\[i\\]
I/O."]
pub type PD1_R = crate::BitReader;
#[doc = "Field `PD1` writer - Port D pull-down bit i (i = 3 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PD\\[i\\]
I/O."]
pub type PD1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD2` reader - Port D pull-down bit i (i = 3 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PD\\[i\\]
I/O."]
pub type PD2_R = crate::BitReader;
#[doc = "Field `PD2` writer - Port D pull-down bit i (i = 3 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PD\\[i\\]
I/O."]
pub type PD2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD3` reader - Port D pull-down bit i (i = 3 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PD\\[i\\]
I/O."]
pub type PD3_R = crate::BitReader;
#[doc = "Field `PD3` writer - Port D pull-down bit i (i = 3 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PD\\[i\\]
I/O."]
pub type PD3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port D pull-down bit i (i = 3 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PD\\[i\\]
I/O."]
    #[inline(always)]
    pub fn pd0(&self) -> PD0_R {
        PD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port D pull-down bit i (i = 3 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PD\\[i\\]
I/O."]
    #[inline(always)]
    pub fn pd1(&self) -> PD1_R {
        PD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port D pull-down bit i (i = 3 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PD\\[i\\]
I/O."]
    #[inline(always)]
    pub fn pd2(&self) -> PD2_R {
        PD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port D pull-down bit i (i = 3 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PD\\[i\\]
I/O."]
    #[inline(always)]
    pub fn pd3(&self) -> PD3_R {
        PD3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port D pull-down bit i (i = 3 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PD\\[i\\]
I/O."]
    #[inline(always)]
    #[must_use]
    pub fn pd0(&mut self) -> PD0_W<PDCRDrs> {
        PD0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Port D pull-down bit i (i = 3 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PD\\[i\\]
I/O."]
    #[inline(always)]
    #[must_use]
    pub fn pd1(&mut self) -> PD1_W<PDCRDrs> {
        PD1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Port D pull-down bit i (i = 3 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PD\\[i\\]
I/O."]
    #[inline(always)]
    #[must_use]
    pub fn pd2(&mut self) -> PD2_W<PDCRDrs> {
        PD2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Port D pull-down bit i (i = 3 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PD\\[i\\]
I/O."]
    #[inline(always)]
    #[must_use]
    pub fn pd3(&mut self) -> PD3_W<PDCRDrs> {
        PD3_W::new(self, 3)
    }
}
#[doc = "PWR Port D pull-down control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdcrd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdcrd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDCRDrs;
impl crate::RegisterSpec for PDCRDrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdcrd::R`](R) reader structure"]
impl crate::Readable for PDCRDrs {}
#[doc = "`write(|w| ..)` method takes [`pdcrd::W`](W) writer structure"]
impl crate::Writable for PDCRDrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDCRD to value 0"]
impl crate::Resettable for PDCRDrs {
    const RESET_VALUE: u32 = 0;
}
