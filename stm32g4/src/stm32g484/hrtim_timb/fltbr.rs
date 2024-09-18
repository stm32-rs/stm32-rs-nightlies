///Register `FLTBR` reader
pub type R = crate::R<FLTBRrs>;
///Register `FLTBR` writer
pub type W = crate::W<FLTBRrs>;
///Field `FLT1EN` reader - Fault 1 enable
pub type FLT1EN_R = crate::BitReader;
///Field `FLT1EN` writer - Fault 1 enable
pub type FLT1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT2EN` reader - Fault 2 enable
pub type FLT2EN_R = crate::BitReader;
///Field `FLT2EN` writer - Fault 2 enable
pub type FLT2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT3EN` reader - Fault 3 enable
pub type FLT3EN_R = crate::BitReader;
///Field `FLT3EN` writer - Fault 3 enable
pub type FLT3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT4EN` reader - Fault 4 enable
pub type FLT4EN_R = crate::BitReader;
///Field `FLT4EN` writer - Fault 4 enable
pub type FLT4EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT5EN` reader - Fault 5 enable
pub type FLT5EN_R = crate::BitReader;
///Field `FLT5EN` writer - Fault 5 enable
pub type FLT5EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT6EN` reader - Fault 6 enable
pub type FLT6EN_R = crate::BitReader;
///Field `FLT6EN` writer - Fault 6 enable
pub type FLT6EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLTLCK` reader - Fault sources Lock
pub type FLTLCK_R = crate::BitReader;
///Field `FLTLCK` writer - Fault sources Lock
pub type FLTLCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Fault 1 enable
    #[inline(always)]
    pub fn flt1en(&self) -> FLT1EN_R {
        FLT1EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Fault 2 enable
    #[inline(always)]
    pub fn flt2en(&self) -> FLT2EN_R {
        FLT2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Fault 3 enable
    #[inline(always)]
    pub fn flt3en(&self) -> FLT3EN_R {
        FLT3EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Fault 4 enable
    #[inline(always)]
    pub fn flt4en(&self) -> FLT4EN_R {
        FLT4EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Fault 5 enable
    #[inline(always)]
    pub fn flt5en(&self) -> FLT5EN_R {
        FLT5EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Fault 6 enable
    #[inline(always)]
    pub fn flt6en(&self) -> FLT6EN_R {
        FLT6EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 31 - Fault sources Lock
    #[inline(always)]
    pub fn fltlck(&self) -> FLTLCK_R {
        FLTLCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLTBR")
            .field("fltlck", &self.fltlck())
            .field("flt6en", &self.flt6en())
            .field("flt5en", &self.flt5en())
            .field("flt4en", &self.flt4en())
            .field("flt3en", &self.flt3en())
            .field("flt2en", &self.flt2en())
            .field("flt1en", &self.flt1en())
            .finish()
    }
}
impl W {
    ///Bit 0 - Fault 1 enable
    #[inline(always)]
    #[must_use]
    pub fn flt1en(&mut self) -> FLT1EN_W<FLTBRrs> {
        FLT1EN_W::new(self, 0)
    }
    ///Bit 1 - Fault 2 enable
    #[inline(always)]
    #[must_use]
    pub fn flt2en(&mut self) -> FLT2EN_W<FLTBRrs> {
        FLT2EN_W::new(self, 1)
    }
    ///Bit 2 - Fault 3 enable
    #[inline(always)]
    #[must_use]
    pub fn flt3en(&mut self) -> FLT3EN_W<FLTBRrs> {
        FLT3EN_W::new(self, 2)
    }
    ///Bit 3 - Fault 4 enable
    #[inline(always)]
    #[must_use]
    pub fn flt4en(&mut self) -> FLT4EN_W<FLTBRrs> {
        FLT4EN_W::new(self, 3)
    }
    ///Bit 4 - Fault 5 enable
    #[inline(always)]
    #[must_use]
    pub fn flt5en(&mut self) -> FLT5EN_W<FLTBRrs> {
        FLT5EN_W::new(self, 4)
    }
    ///Bit 5 - Fault 6 enable
    #[inline(always)]
    #[must_use]
    pub fn flt6en(&mut self) -> FLT6EN_W<FLTBRrs> {
        FLT6EN_W::new(self, 5)
    }
    ///Bit 31 - Fault sources Lock
    #[inline(always)]
    #[must_use]
    pub fn fltlck(&mut self) -> FLTLCK_W<FLTBRrs> {
        FLTLCK_W::new(self, 31)
    }
}
/**Timerx Fault Register

You can [`read`](crate::Reg::read) this register and get [`fltbr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltbr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G484xx.html#HRTIM_TIMB:FLTBR)*/
pub struct FLTBRrs;
impl crate::RegisterSpec for FLTBRrs {
    type Ux = u32;
}
///`read()` method returns [`fltbr::R`](R) reader structure
impl crate::Readable for FLTBRrs {}
///`write(|w| ..)` method takes [`fltbr::W`](W) writer structure
impl crate::Writable for FLTBRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FLTBR to value 0
impl crate::Resettable for FLTBRrs {
    const RESET_VALUE: u32 = 0;
}
