///Register `IER` reader
pub type R = crate::R<IERrs>;
///Register `IER` writer
pub type W = crate::W<IERrs>;
///Field `FLT1IE` reader - Fault 1 Interrupt Enable
pub type FLT1IE_R = crate::BitReader;
///Field `FLT1IE` writer - Fault 1 Interrupt Enable
pub type FLT1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT2IE` reader - Fault 2 Interrupt Enable
pub type FLT2IE_R = crate::BitReader;
///Field `FLT2IE` writer - Fault 2 Interrupt Enable
pub type FLT2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT3IE` reader - Fault 3 Interrupt Enable
pub type FLT3IE_R = crate::BitReader;
///Field `FLT3IE` writer - Fault 3 Interrupt Enable
pub type FLT3IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT4IE` reader - Fault 4 Interrupt Enable
pub type FLT4IE_R = crate::BitReader;
///Field `FLT4IE` writer - Fault 4 Interrupt Enable
pub type FLT4IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT5IE` reader - Fault 5 Interrupt Enable
pub type FLT5IE_R = crate::BitReader;
///Field `FLT5IE` writer - Fault 5 Interrupt Enable
pub type FLT5IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYSFLTE` reader - System Fault Interrupt Enable
pub type SYSFLTE_R = crate::BitReader;
///Field `SYSFLTE` writer - System Fault Interrupt Enable
pub type SYSFLTE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DLLRDYIE` reader - DLL Ready Interrupt Enable
pub type DLLRDYIE_R = crate::BitReader;
///Field `DLLRDYIE` writer - DLL Ready Interrupt Enable
pub type DLLRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BMPERIE` reader - Burst mode period Interrupt Enable
pub type BMPERIE_R = crate::BitReader;
///Field `BMPERIE` writer - Burst mode period Interrupt Enable
pub type BMPERIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Fault 1 Interrupt Enable
    #[inline(always)]
    pub fn flt1ie(&self) -> FLT1IE_R {
        FLT1IE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Fault 2 Interrupt Enable
    #[inline(always)]
    pub fn flt2ie(&self) -> FLT2IE_R {
        FLT2IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Fault 3 Interrupt Enable
    #[inline(always)]
    pub fn flt3ie(&self) -> FLT3IE_R {
        FLT3IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Fault 4 Interrupt Enable
    #[inline(always)]
    pub fn flt4ie(&self) -> FLT4IE_R {
        FLT4IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Fault 5 Interrupt Enable
    #[inline(always)]
    pub fn flt5ie(&self) -> FLT5IE_R {
        FLT5IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - System Fault Interrupt Enable
    #[inline(always)]
    pub fn sysflte(&self) -> SYSFLTE_R {
        SYSFLTE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 16 - DLL Ready Interrupt Enable
    #[inline(always)]
    pub fn dllrdyie(&self) -> DLLRDYIE_R {
        DLLRDYIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Burst mode period Interrupt Enable
    #[inline(always)]
    pub fn bmperie(&self) -> BMPERIE_R {
        BMPERIE_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER")
            .field("bmperie", &self.bmperie())
            .field("dllrdyie", &self.dllrdyie())
            .field("sysflte", &self.sysflte())
            .field("flt5ie", &self.flt5ie())
            .field("flt4ie", &self.flt4ie())
            .field("flt3ie", &self.flt3ie())
            .field("flt2ie", &self.flt2ie())
            .field("flt1ie", &self.flt1ie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Fault 1 Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn flt1ie(&mut self) -> FLT1IE_W<IERrs> {
        FLT1IE_W::new(self, 0)
    }
    ///Bit 1 - Fault 2 Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn flt2ie(&mut self) -> FLT2IE_W<IERrs> {
        FLT2IE_W::new(self, 1)
    }
    ///Bit 2 - Fault 3 Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn flt3ie(&mut self) -> FLT3IE_W<IERrs> {
        FLT3IE_W::new(self, 2)
    }
    ///Bit 3 - Fault 4 Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn flt4ie(&mut self) -> FLT4IE_W<IERrs> {
        FLT4IE_W::new(self, 3)
    }
    ///Bit 4 - Fault 5 Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn flt5ie(&mut self) -> FLT5IE_W<IERrs> {
        FLT5IE_W::new(self, 4)
    }
    ///Bit 5 - System Fault Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn sysflte(&mut self) -> SYSFLTE_W<IERrs> {
        SYSFLTE_W::new(self, 5)
    }
    ///Bit 16 - DLL Ready Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn dllrdyie(&mut self) -> DLLRDYIE_W<IERrs> {
        DLLRDYIE_W::new(self, 16)
    }
    ///Bit 17 - Burst mode period Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn bmperie(&mut self) -> BMPERIE_W<IERrs> {
        BMPERIE_W::new(self, 17)
    }
}
/**Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM7.html#HRTIM_Common:IER)*/
pub struct IERrs;
impl crate::RegisterSpec for IERrs {
    type Ux = u32;
}
///`read()` method returns [`ier::R`](R) reader structure
impl crate::Readable for IERrs {}
///`write(|w| ..)` method takes [`ier::W`](W) writer structure
impl crate::Writable for IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IER to value 0
impl crate::Resettable for IERrs {
    const RESET_VALUE: u32 = 0;
}
