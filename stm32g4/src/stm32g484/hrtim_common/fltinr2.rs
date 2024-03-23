#[doc = "Register `FLTINR2` reader"]
pub type R = crate::R<FLTINR2rs>;
#[doc = "Register `FLTINR2` writer"]
pub type W = crate::W<FLTINR2rs>;
#[doc = "Field `FLT5E` reader - FLT5E"]
pub type FLT5E_R = crate::BitReader;
#[doc = "Field `FLT5E` writer - FLT5E"]
pub type FLT5E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT5P` reader - FLT5P"]
pub type FLT5P_R = crate::BitReader;
#[doc = "Field `FLT5P` writer - FLT5P"]
pub type FLT5P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT5SRC` reader - FLT5SRC"]
pub type FLT5SRC_R = crate::BitReader;
#[doc = "Field `FLT5SRC` writer - FLT5SRC"]
pub type FLT5SRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT5F` reader - FLT5F"]
pub type FLT5F_R = crate::FieldReader;
#[doc = "Field `FLT5F` writer - FLT5F"]
pub type FLT5F_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FLT5LCK` reader - FLT5LCK"]
pub type FLT5LCK_R = crate::BitReader;
#[doc = "Field `FLT5LCK` writer - FLT5LCK"]
pub type FLT5LCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT6E` reader - FLT6E"]
pub type FLT6E_R = crate::BitReader;
#[doc = "Field `FLT6E` writer - FLT6E"]
pub type FLT6E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT6P` reader - FLT6P"]
pub type FLT6P_R = crate::BitReader;
#[doc = "Field `FLT6P` writer - FLT6P"]
pub type FLT6P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT6SRC_0` reader - FLT6F"]
pub type FLT6SRC_0_R = crate::BitReader;
#[doc = "Field `FLT6SRC_0` writer - FLT6F"]
pub type FLT6SRC_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT6F` reader - FLT6F"]
pub type FLT6F_R = crate::FieldReader;
#[doc = "Field `FLT6F` writer - FLT6F"]
pub type FLT6F_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FLT6LCK` reader - FLT6LCK"]
pub type FLT6LCK_R = crate::BitReader;
#[doc = "Field `FLT6LCK` writer - FLT6LCK"]
pub type FLT6LCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT1SRC_1` reader - FLT1SRC_1"]
pub type FLT1SRC_1_R = crate::BitReader;
#[doc = "Field `FLT1SRC_1` writer - FLT1SRC_1"]
pub type FLT1SRC_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT2SRC_1` reader - FLT2SRC_1"]
pub type FLT2SRC_1_R = crate::BitReader;
#[doc = "Field `FLT2SRC_1` writer - FLT2SRC_1"]
pub type FLT2SRC_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT3SRC_1` reader - FLT3SRC_1"]
pub type FLT3SRC_1_R = crate::BitReader;
#[doc = "Field `FLT3SRC_1` writer - FLT3SRC_1"]
pub type FLT3SRC_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT4SRC_1` reader - FLT4SRC_1"]
pub type FLT4SRC_1_R = crate::BitReader;
#[doc = "Field `FLT4SRC_1` writer - FLT4SRC_1"]
pub type FLT4SRC_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT5SRC_1` reader - FLT5SRC_1"]
pub type FLT5SRC_1_R = crate::BitReader;
#[doc = "Field `FLT5SRC_1` writer - FLT5SRC_1"]
pub type FLT5SRC_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT6SRC_1` reader - FLT6SRC"]
pub type FLT6SRC_1_R = crate::BitReader;
#[doc = "Field `FLT6SRC_1` writer - FLT6SRC"]
pub type FLT6SRC_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLTSD` reader - FLTSD"]
pub type FLTSD_R = crate::FieldReader;
#[doc = "Field `FLTSD` writer - FLTSD"]
pub type FLTSD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - FLT5E"]
    #[inline(always)]
    pub fn flt5e(&self) -> FLT5E_R {
        FLT5E_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FLT5P"]
    #[inline(always)]
    pub fn flt5p(&self) -> FLT5P_R {
        FLT5P_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FLT5SRC"]
    #[inline(always)]
    pub fn flt5src(&self) -> FLT5SRC_R {
        FLT5SRC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:6 - FLT5F"]
    #[inline(always)]
    pub fn flt5f(&self) -> FLT5F_R {
        FLT5F_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - FLT5LCK"]
    #[inline(always)]
    pub fn flt5lck(&self) -> FLT5LCK_R {
        FLT5LCK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - FLT6E"]
    #[inline(always)]
    pub fn flt6e(&self) -> FLT6E_R {
        FLT6E_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - FLT6P"]
    #[inline(always)]
    pub fn flt6p(&self) -> FLT6P_R {
        FLT6P_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - FLT6F"]
    #[inline(always)]
    pub fn flt6src_0(&self) -> FLT6SRC_0_R {
        FLT6SRC_0_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14 - FLT6F"]
    #[inline(always)]
    pub fn flt6f(&self) -> FLT6F_R {
        FLT6F_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - FLT6LCK"]
    #[inline(always)]
    pub fn flt6lck(&self) -> FLT6LCK_R {
        FLT6LCK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - FLT1SRC_1"]
    #[inline(always)]
    pub fn flt1src_1(&self) -> FLT1SRC_1_R {
        FLT1SRC_1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - FLT2SRC_1"]
    #[inline(always)]
    pub fn flt2src_1(&self) -> FLT2SRC_1_R {
        FLT2SRC_1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - FLT3SRC_1"]
    #[inline(always)]
    pub fn flt3src_1(&self) -> FLT3SRC_1_R {
        FLT3SRC_1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - FLT4SRC_1"]
    #[inline(always)]
    pub fn flt4src_1(&self) -> FLT4SRC_1_R {
        FLT4SRC_1_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - FLT5SRC_1"]
    #[inline(always)]
    pub fn flt5src_1(&self) -> FLT5SRC_1_R {
        FLT5SRC_1_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - FLT6SRC"]
    #[inline(always)]
    pub fn flt6src_1(&self) -> FLT6SRC_1_R {
        FLT6SRC_1_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:25 - FLTSD"]
    #[inline(always)]
    pub fn fltsd(&self) -> FLTSD_R {
        FLTSD_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - FLT5E"]
    #[inline(always)]
    #[must_use]
    pub fn flt5e(&mut self) -> FLT5E_W<FLTINR2rs> {
        FLT5E_W::new(self, 0)
    }
    #[doc = "Bit 1 - FLT5P"]
    #[inline(always)]
    #[must_use]
    pub fn flt5p(&mut self) -> FLT5P_W<FLTINR2rs> {
        FLT5P_W::new(self, 1)
    }
    #[doc = "Bit 2 - FLT5SRC"]
    #[inline(always)]
    #[must_use]
    pub fn flt5src(&mut self) -> FLT5SRC_W<FLTINR2rs> {
        FLT5SRC_W::new(self, 2)
    }
    #[doc = "Bits 3:6 - FLT5F"]
    #[inline(always)]
    #[must_use]
    pub fn flt5f(&mut self) -> FLT5F_W<FLTINR2rs> {
        FLT5F_W::new(self, 3)
    }
    #[doc = "Bit 7 - FLT5LCK"]
    #[inline(always)]
    #[must_use]
    pub fn flt5lck(&mut self) -> FLT5LCK_W<FLTINR2rs> {
        FLT5LCK_W::new(self, 7)
    }
    #[doc = "Bit 8 - FLT6E"]
    #[inline(always)]
    #[must_use]
    pub fn flt6e(&mut self) -> FLT6E_W<FLTINR2rs> {
        FLT6E_W::new(self, 8)
    }
    #[doc = "Bit 9 - FLT6P"]
    #[inline(always)]
    #[must_use]
    pub fn flt6p(&mut self) -> FLT6P_W<FLTINR2rs> {
        FLT6P_W::new(self, 9)
    }
    #[doc = "Bit 10 - FLT6F"]
    #[inline(always)]
    #[must_use]
    pub fn flt6src_0(&mut self) -> FLT6SRC_0_W<FLTINR2rs> {
        FLT6SRC_0_W::new(self, 10)
    }
    #[doc = "Bits 11:14 - FLT6F"]
    #[inline(always)]
    #[must_use]
    pub fn flt6f(&mut self) -> FLT6F_W<FLTINR2rs> {
        FLT6F_W::new(self, 11)
    }
    #[doc = "Bit 15 - FLT6LCK"]
    #[inline(always)]
    #[must_use]
    pub fn flt6lck(&mut self) -> FLT6LCK_W<FLTINR2rs> {
        FLT6LCK_W::new(self, 15)
    }
    #[doc = "Bit 16 - FLT1SRC_1"]
    #[inline(always)]
    #[must_use]
    pub fn flt1src_1(&mut self) -> FLT1SRC_1_W<FLTINR2rs> {
        FLT1SRC_1_W::new(self, 16)
    }
    #[doc = "Bit 17 - FLT2SRC_1"]
    #[inline(always)]
    #[must_use]
    pub fn flt2src_1(&mut self) -> FLT2SRC_1_W<FLTINR2rs> {
        FLT2SRC_1_W::new(self, 17)
    }
    #[doc = "Bit 18 - FLT3SRC_1"]
    #[inline(always)]
    #[must_use]
    pub fn flt3src_1(&mut self) -> FLT3SRC_1_W<FLTINR2rs> {
        FLT3SRC_1_W::new(self, 18)
    }
    #[doc = "Bit 19 - FLT4SRC_1"]
    #[inline(always)]
    #[must_use]
    pub fn flt4src_1(&mut self) -> FLT4SRC_1_W<FLTINR2rs> {
        FLT4SRC_1_W::new(self, 19)
    }
    #[doc = "Bit 20 - FLT5SRC_1"]
    #[inline(always)]
    #[must_use]
    pub fn flt5src_1(&mut self) -> FLT5SRC_1_W<FLTINR2rs> {
        FLT5SRC_1_W::new(self, 20)
    }
    #[doc = "Bit 21 - FLT6SRC"]
    #[inline(always)]
    #[must_use]
    pub fn flt6src_1(&mut self) -> FLT6SRC_1_W<FLTINR2rs> {
        FLT6SRC_1_W::new(self, 21)
    }
    #[doc = "Bits 24:25 - FLTSD"]
    #[inline(always)]
    #[must_use]
    pub fn fltsd(&mut self) -> FLTSD_W<FLTINR2rs> {
        FLTSD_W::new(self, 24)
    }
}
#[doc = "HRTIM Fault Input Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fltinr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fltinr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLTINR2rs;
impl crate::RegisterSpec for FLTINR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fltinr2::R`](R) reader structure"]
impl crate::Readable for FLTINR2rs {}
#[doc = "`write(|w| ..)` method takes [`fltinr2::W`](W) writer structure"]
impl crate::Writable for FLTINR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLTINR2 to value 0"]
impl crate::Resettable for FLTINR2rs {
    const RESET_VALUE: u32 = 0;
}
