///Register `GCR` reader
pub type R = crate::R<GCRrs>;
///Register `GCR` writer
pub type W = crate::W<GCRrs>;
/**WWDG1 reset scope control

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WW1RSC {
    ///0: Clear WWDG1 scope control
    Clear = 0,
    ///1: Set WWDG1 scope control
    Set = 1,
}
impl From<WW1RSC> for bool {
    #[inline(always)]
    fn from(variant: WW1RSC) -> Self {
        variant as u8 != 0
    }
}
///Field `WW1RSC` reader - WWDG1 reset scope control
pub type WW1RSC_R = crate::BitReader<WW1RSC>;
impl WW1RSC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WW1RSC {
        match self.bits {
            false => WW1RSC::Clear,
            true => WW1RSC::Set,
        }
    }
    ///Clear WWDG1 scope control
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == WW1RSC::Clear
    }
    ///Set WWDG1 scope control
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == WW1RSC::Set
    }
}
///Field `WW1RSC` writer - WWDG1 reset scope control
pub type WW1RSC_W<'a, REG> = crate::BitWriter<'a, REG, WW1RSC>;
impl<'a, REG> WW1RSC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear WWDG1 scope control
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(WW1RSC::Clear)
    }
    ///Set WWDG1 scope control
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(WW1RSC::Set)
    }
}
///Field `WW2RSC` reader - WWDG2 reset scope control
pub type WW2RSC_R = crate::BitReader;
///Field `WW2RSC` writer - WWDG2 reset scope control
pub type WW2RSC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BOOT_C1` reader - Force allow CPU1 to boot
pub type BOOT_C1_R = crate::BitReader;
///Field `BOOT_C1` writer - Force allow CPU1 to boot
pub type BOOT_C1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BOOT_C2` reader - Force allow CPU2 to boot
pub type BOOT_C2_R = crate::BitReader;
///Field `BOOT_C2` writer - Force allow CPU2 to boot
pub type BOOT_C2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - WWDG1 reset scope control
    #[inline(always)]
    pub fn ww1rsc(&self) -> WW1RSC_R {
        WW1RSC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - WWDG2 reset scope control
    #[inline(always)]
    pub fn ww2rsc(&self) -> WW2RSC_R {
        WW2RSC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Force allow CPU1 to boot
    #[inline(always)]
    pub fn boot_c1(&self) -> BOOT_C1_R {
        BOOT_C1_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Force allow CPU2 to boot
    #[inline(always)]
    pub fn boot_c2(&self) -> BOOT_C2_R {
        BOOT_C2_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GCR")
            .field("ww1rsc", &self.ww1rsc())
            .field("ww2rsc", &self.ww2rsc())
            .field("boot_c1", &self.boot_c1())
            .field("boot_c2", &self.boot_c2())
            .finish()
    }
}
impl W {
    ///Bit 0 - WWDG1 reset scope control
    #[inline(always)]
    pub fn ww1rsc(&mut self) -> WW1RSC_W<'_, GCRrs> {
        WW1RSC_W::new(self, 0)
    }
    ///Bit 1 - WWDG2 reset scope control
    #[inline(always)]
    pub fn ww2rsc(&mut self) -> WW2RSC_W<'_, GCRrs> {
        WW2RSC_W::new(self, 1)
    }
    ///Bit 2 - Force allow CPU1 to boot
    #[inline(always)]
    pub fn boot_c1(&mut self) -> BOOT_C1_W<'_, GCRrs> {
        BOOT_C1_W::new(self, 2)
    }
    ///Bit 3 - Force allow CPU2 to boot
    #[inline(always)]
    pub fn boot_c2(&mut self) -> BOOT_C2_W<'_, GCRrs> {
        BOOT_C2_W::new(self, 3)
    }
}
/**RCC Global Control Register

You can [`read`](crate::Reg::read) this register and get [`gcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#RCC:GCR)*/
pub struct GCRrs;
impl crate::RegisterSpec for GCRrs {
    type Ux = u32;
}
///`read()` method returns [`gcr::R`](R) reader structure
impl crate::Readable for GCRrs {}
///`write(|w| ..)` method takes [`gcr::W`](W) writer structure
impl crate::Writable for GCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GCR to value 0
impl crate::Resettable for GCRrs {}
