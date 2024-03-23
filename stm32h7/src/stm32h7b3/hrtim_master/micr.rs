#[doc = "Register `MICR` writer"]
pub type W = crate::W<MICRrs>;
#[doc = "Field `MCMP1C` writer - Master Compare 1 Interrupt flag clear"]
pub type MCMP1C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCMP2C` writer - Master Compare 2 Interrupt flag clear"]
pub type MCMP2C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCMP3C` writer - Master Compare 3 Interrupt flag clear"]
pub type MCMP3C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCMP4C` writer - Master Compare 4 Interrupt flag clear"]
pub type MCMP4C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MREPC` writer - Repetition Interrupt flag clear"]
pub type MREPC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCC` writer - Sync Input Interrupt flag clear"]
pub type SYNCC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUPDC` writer - Master update Interrupt flag clear"]
pub type MUPDC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Master Compare 1 Interrupt flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn mcmp1c(&mut self) -> MCMP1C_W<MICRrs> {
        MCMP1C_W::new(self, 0)
    }
    #[doc = "Bit 1 - Master Compare 2 Interrupt flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn mcmp2c(&mut self) -> MCMP2C_W<MICRrs> {
        MCMP2C_W::new(self, 1)
    }
    #[doc = "Bit 2 - Master Compare 3 Interrupt flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn mcmp3c(&mut self) -> MCMP3C_W<MICRrs> {
        MCMP3C_W::new(self, 2)
    }
    #[doc = "Bit 3 - Master Compare 4 Interrupt flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn mcmp4c(&mut self) -> MCMP4C_W<MICRrs> {
        MCMP4C_W::new(self, 3)
    }
    #[doc = "Bit 4 - Repetition Interrupt flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn mrepc(&mut self) -> MREPC_W<MICRrs> {
        MREPC_W::new(self, 4)
    }
    #[doc = "Bit 5 - Sync Input Interrupt flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn syncc(&mut self) -> SYNCC_W<MICRrs> {
        SYNCC_W::new(self, 5)
    }
    #[doc = "Bit 6 - Master update Interrupt flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn mupdc(&mut self) -> MUPDC_W<MICRrs> {
        MUPDC_W::new(self, 6)
    }
}
#[doc = "Master Timer Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`micr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MICRrs;
impl crate::RegisterSpec for MICRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`micr::W`](W) writer structure"]
impl crate::Writable for MICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MICR to value 0"]
impl crate::Resettable for MICRrs {
    const RESET_VALUE: u32 = 0;
}
