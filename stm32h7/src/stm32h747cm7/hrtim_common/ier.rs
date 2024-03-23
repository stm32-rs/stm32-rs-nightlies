#[doc = "Register `IER` reader"]
pub type R = crate::R<IERrs>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IERrs>;
#[doc = "Field `FLT1IE` reader - Fault 1 Interrupt Enable"]
pub type FLT1IE_R = crate::BitReader;
#[doc = "Field `FLT1IE` writer - Fault 1 Interrupt Enable"]
pub type FLT1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT2IE` reader - Fault 2 Interrupt Enable"]
pub type FLT2IE_R = crate::BitReader;
#[doc = "Field `FLT2IE` writer - Fault 2 Interrupt Enable"]
pub type FLT2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT3IE` reader - Fault 3 Interrupt Enable"]
pub type FLT3IE_R = crate::BitReader;
#[doc = "Field `FLT3IE` writer - Fault 3 Interrupt Enable"]
pub type FLT3IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT4IE` reader - Fault 4 Interrupt Enable"]
pub type FLT4IE_R = crate::BitReader;
#[doc = "Field `FLT4IE` writer - Fault 4 Interrupt Enable"]
pub type FLT4IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT5IE` reader - Fault 5 Interrupt Enable"]
pub type FLT5IE_R = crate::BitReader;
#[doc = "Field `FLT5IE` writer - Fault 5 Interrupt Enable"]
pub type FLT5IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSFLTE` reader - System Fault Interrupt Enable"]
pub type SYSFLTE_R = crate::BitReader;
#[doc = "Field `SYSFLTE` writer - System Fault Interrupt Enable"]
pub type SYSFLTE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLLRDYIE` reader - DLL Ready Interrupt Enable"]
pub type DLLRDYIE_R = crate::BitReader;
#[doc = "Field `DLLRDYIE` writer - DLL Ready Interrupt Enable"]
pub type DLLRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BMPERIE` reader - Burst mode period Interrupt Enable"]
pub type BMPERIE_R = crate::BitReader;
#[doc = "Field `BMPERIE` writer - Burst mode period Interrupt Enable"]
pub type BMPERIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Fault 1 Interrupt Enable"]
    #[inline(always)]
    pub fn flt1ie(&self) -> FLT1IE_R {
        FLT1IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fault 2 Interrupt Enable"]
    #[inline(always)]
    pub fn flt2ie(&self) -> FLT2IE_R {
        FLT2IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fault 3 Interrupt Enable"]
    #[inline(always)]
    pub fn flt3ie(&self) -> FLT3IE_R {
        FLT3IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Fault 4 Interrupt Enable"]
    #[inline(always)]
    pub fn flt4ie(&self) -> FLT4IE_R {
        FLT4IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Fault 5 Interrupt Enable"]
    #[inline(always)]
    pub fn flt5ie(&self) -> FLT5IE_R {
        FLT5IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - System Fault Interrupt Enable"]
    #[inline(always)]
    pub fn sysflte(&self) -> SYSFLTE_R {
        SYSFLTE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - DLL Ready Interrupt Enable"]
    #[inline(always)]
    pub fn dllrdyie(&self) -> DLLRDYIE_R {
        DLLRDYIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Burst mode period Interrupt Enable"]
    #[inline(always)]
    pub fn bmperie(&self) -> BMPERIE_R {
        BMPERIE_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fault 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt1ie(&mut self) -> FLT1IE_W<IERrs> {
        FLT1IE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Fault 2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt2ie(&mut self) -> FLT2IE_W<IERrs> {
        FLT2IE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Fault 3 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt3ie(&mut self) -> FLT3IE_W<IERrs> {
        FLT3IE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Fault 4 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt4ie(&mut self) -> FLT4IE_W<IERrs> {
        FLT4IE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Fault 5 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt5ie(&mut self) -> FLT5IE_W<IERrs> {
        FLT5IE_W::new(self, 4)
    }
    #[doc = "Bit 5 - System Fault Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sysflte(&mut self) -> SYSFLTE_W<IERrs> {
        SYSFLTE_W::new(self, 5)
    }
    #[doc = "Bit 16 - DLL Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dllrdyie(&mut self) -> DLLRDYIE_W<IERrs> {
        DLLRDYIE_W::new(self, 16)
    }
    #[doc = "Bit 17 - Burst mode period Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bmperie(&mut self) -> BMPERIE_W<IERrs> {
        BMPERIE_W::new(self, 17)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IERrs;
impl crate::RegisterSpec for IERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IERrs {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IERrs {
    const RESET_VALUE: u32 = 0;
}
