#[doc = "Register `ICR` reader"]
pub type R = crate::R<ICRrs>;
#[doc = "Register `ICR` writer"]
pub type W = crate::W<ICRrs>;
#[doc = "Field `FLT1C` writer - Fault 1 Interrupt Flag Clear"]
pub type FLT1C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT2C` writer - Fault 2 Interrupt Flag Clear"]
pub type FLT2C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT3C` writer - Fault 3 Interrupt Flag Clear"]
pub type FLT3C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT4C` writer - Fault 4 Interrupt Flag Clear"]
pub type FLT4C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT5C` writer - Fault 5 Interrupt Flag Clear"]
pub type FLT5C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSFLTC` reader - System Fault Interrupt Flag Clear"]
pub type SYSFLTC_R = crate::BitReader;
#[doc = "Field `SYSFLTC` writer - System Fault Interrupt Flag Clear"]
pub type SYSFLTC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLLRDYC` writer - DLL Ready Interrupt flag Clear"]
pub type DLLRDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BMPERC` writer - Burst mode period flag Clear"]
pub type BMPERC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - System Fault Interrupt Flag Clear"]
    #[inline(always)]
    pub fn sysfltc(&self) -> SYSFLTC_R {
        SYSFLTC_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fault 1 Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn flt1c(&mut self) -> FLT1C_W<ICRrs> {
        FLT1C_W::new(self, 0)
    }
    #[doc = "Bit 1 - Fault 2 Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn flt2c(&mut self) -> FLT2C_W<ICRrs> {
        FLT2C_W::new(self, 1)
    }
    #[doc = "Bit 2 - Fault 3 Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn flt3c(&mut self) -> FLT3C_W<ICRrs> {
        FLT3C_W::new(self, 2)
    }
    #[doc = "Bit 3 - Fault 4 Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn flt4c(&mut self) -> FLT4C_W<ICRrs> {
        FLT4C_W::new(self, 3)
    }
    #[doc = "Bit 4 - Fault 5 Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn flt5c(&mut self) -> FLT5C_W<ICRrs> {
        FLT5C_W::new(self, 4)
    }
    #[doc = "Bit 5 - System Fault Interrupt Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn sysfltc(&mut self) -> SYSFLTC_W<ICRrs> {
        SYSFLTC_W::new(self, 5)
    }
    #[doc = "Bit 16 - DLL Ready Interrupt flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dllrdyc(&mut self) -> DLLRDYC_W<ICRrs> {
        DLLRDYC_W::new(self, 16)
    }
    #[doc = "Bit 17 - Burst mode period flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn bmperc(&mut self) -> BMPERC_W<ICRrs> {
        BMPERC_W::new(self, 17)
    }
}
#[doc = "Interrupt Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icr::R`](R) reader structure"]
impl crate::Readable for ICRrs {}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICRrs {
    const RESET_VALUE: u32 = 0;
}
