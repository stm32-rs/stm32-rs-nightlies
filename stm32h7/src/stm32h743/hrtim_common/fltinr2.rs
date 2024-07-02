///Register `FLTINR2` reader
pub type R = crate::R<FLTINR2rs>;
///Register `FLTINR2` writer
pub type W = crate::W<FLTINR2rs>;
///Field `FLT5E` reader - FLT5E
pub type FLT5E_R = crate::BitReader;
///Field `FLT5E` writer - FLT5E
pub type FLT5E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT5P` reader - FLT5P
pub type FLT5P_R = crate::BitReader;
///Field `FLT5P` writer - FLT5P
pub type FLT5P_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT5SRC` reader - FLT5SRC
pub type FLT5SRC_R = crate::BitReader;
///Field `FLT5SRC` writer - FLT5SRC
pub type FLT5SRC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLT5F` reader - FLT5F
pub type FLT5F_R = crate::FieldReader;
///Field `FLT5F` writer - FLT5F
pub type FLT5F_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `FLT5LCK` reader - FLT5LCK
pub type FLT5LCK_R = crate::BitReader;
///Field `FLT5LCK` writer - FLT5LCK
pub type FLT5LCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLTSD` reader - FLTSD
pub type FLTSD_R = crate::FieldReader;
///Field `FLTSD` writer - FLTSD
pub type FLTSD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - FLT5E
    #[inline(always)]
    pub fn flt5e(&self) -> FLT5E_R {
        FLT5E_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - FLT5P
    #[inline(always)]
    pub fn flt5p(&self) -> FLT5P_R {
        FLT5P_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - FLT5SRC
    #[inline(always)]
    pub fn flt5src(&self) -> FLT5SRC_R {
        FLT5SRC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:6 - FLT5F
    #[inline(always)]
    pub fn flt5f(&self) -> FLT5F_R {
        FLT5F_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    ///Bit 7 - FLT5LCK
    #[inline(always)]
    pub fn flt5lck(&self) -> FLT5LCK_R {
        FLT5LCK_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 24:25 - FLTSD
    #[inline(always)]
    pub fn fltsd(&self) -> FLTSD_R {
        FLTSD_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLTINR2")
            .field("fltsd", &self.fltsd())
            .field("flt5lck", &self.flt5lck())
            .field("flt5f", &self.flt5f())
            .field("flt5src", &self.flt5src())
            .field("flt5p", &self.flt5p())
            .field("flt5e", &self.flt5e())
            .finish()
    }
}
impl W {
    ///Bit 0 - FLT5E
    #[inline(always)]
    #[must_use]
    pub fn flt5e(&mut self) -> FLT5E_W<FLTINR2rs> {
        FLT5E_W::new(self, 0)
    }
    ///Bit 1 - FLT5P
    #[inline(always)]
    #[must_use]
    pub fn flt5p(&mut self) -> FLT5P_W<FLTINR2rs> {
        FLT5P_W::new(self, 1)
    }
    ///Bit 2 - FLT5SRC
    #[inline(always)]
    #[must_use]
    pub fn flt5src(&mut self) -> FLT5SRC_W<FLTINR2rs> {
        FLT5SRC_W::new(self, 2)
    }
    ///Bits 3:6 - FLT5F
    #[inline(always)]
    #[must_use]
    pub fn flt5f(&mut self) -> FLT5F_W<FLTINR2rs> {
        FLT5F_W::new(self, 3)
    }
    ///Bit 7 - FLT5LCK
    #[inline(always)]
    #[must_use]
    pub fn flt5lck(&mut self) -> FLT5LCK_W<FLTINR2rs> {
        FLT5LCK_W::new(self, 7)
    }
    ///Bits 24:25 - FLTSD
    #[inline(always)]
    #[must_use]
    pub fn fltsd(&mut self) -> FLTSD_W<FLTINR2rs> {
        FLTSD_W::new(self, 24)
    }
}
/**HRTIM Fault Input Register 2

You can [`read`](crate::Reg::read) this register and get [`fltinr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltinr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#HRTIM_Common:FLTINR2)*/
pub struct FLTINR2rs;
impl crate::RegisterSpec for FLTINR2rs {
    type Ux = u32;
}
///`read()` method returns [`fltinr2::R`](R) reader structure
impl crate::Readable for FLTINR2rs {}
///`write(|w| ..)` method takes [`fltinr2::W`](W) writer structure
impl crate::Writable for FLTINR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FLTINR2 to value 0
impl crate::Resettable for FLTINR2rs {
    const RESET_VALUE: u32 = 0;
}
