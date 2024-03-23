#[doc = "Register `FPUIMR` reader"]
pub type R = crate::R<FPUIMRrs>;
#[doc = "Register `FPUIMR` writer"]
pub type W = crate::W<FPUIMRrs>;
#[doc = "Field `FPU_IE0` reader - FPU interrupt enable"]
pub type FPU_IE0_R = crate::BitReader;
#[doc = "Field `FPU_IE0` writer - FPU interrupt enable"]
pub type FPU_IE0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPU_IE1` reader - FPU interrupt enable"]
pub type FPU_IE1_R = crate::BitReader;
#[doc = "Field `FPU_IE1` writer - FPU interrupt enable"]
pub type FPU_IE1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPU_IE2` reader - FPU interrupt enable"]
pub type FPU_IE2_R = crate::BitReader;
#[doc = "Field `FPU_IE2` writer - FPU interrupt enable"]
pub type FPU_IE2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPU_IE3` reader - FPU interrupt enable"]
pub type FPU_IE3_R = crate::BitReader;
#[doc = "Field `FPU_IE3` writer - FPU interrupt enable"]
pub type FPU_IE3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPU_IE4` reader - FPU interrupt enable"]
pub type FPU_IE4_R = crate::BitReader;
#[doc = "Field `FPU_IE4` writer - FPU interrupt enable"]
pub type FPU_IE4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPU_IE5` reader - FPU interrupt enable"]
pub type FPU_IE5_R = crate::BitReader;
#[doc = "Field `FPU_IE5` writer - FPU interrupt enable"]
pub type FPU_IE5_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - FPU interrupt enable"]
    #[inline(always)]
    pub fn fpu_ie0(&self) -> FPU_IE0_R {
        FPU_IE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FPU interrupt enable"]
    #[inline(always)]
    pub fn fpu_ie1(&self) -> FPU_IE1_R {
        FPU_IE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FPU interrupt enable"]
    #[inline(always)]
    pub fn fpu_ie2(&self) -> FPU_IE2_R {
        FPU_IE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FPU interrupt enable"]
    #[inline(always)]
    pub fn fpu_ie3(&self) -> FPU_IE3_R {
        FPU_IE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FPU interrupt enable"]
    #[inline(always)]
    pub fn fpu_ie4(&self) -> FPU_IE4_R {
        FPU_IE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FPU interrupt enable"]
    #[inline(always)]
    pub fn fpu_ie5(&self) -> FPU_IE5_R {
        FPU_IE5_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FPU interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn fpu_ie0(&mut self) -> FPU_IE0_W<FPUIMRrs> {
        FPU_IE0_W::new(self, 0)
    }
    #[doc = "Bit 1 - FPU interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn fpu_ie1(&mut self) -> FPU_IE1_W<FPUIMRrs> {
        FPU_IE1_W::new(self, 1)
    }
    #[doc = "Bit 2 - FPU interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn fpu_ie2(&mut self) -> FPU_IE2_W<FPUIMRrs> {
        FPU_IE2_W::new(self, 2)
    }
    #[doc = "Bit 3 - FPU interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn fpu_ie3(&mut self) -> FPU_IE3_W<FPUIMRrs> {
        FPU_IE3_W::new(self, 3)
    }
    #[doc = "Bit 4 - FPU interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn fpu_ie4(&mut self) -> FPU_IE4_W<FPUIMRrs> {
        FPU_IE4_W::new(self, 4)
    }
    #[doc = "Bit 5 - FPU interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn fpu_ie5(&mut self) -> FPU_IE5_W<FPUIMRrs> {
        FPU_IE5_W::new(self, 5)
    }
}
#[doc = "SBS FPU interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpuimr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpuimr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FPUIMRrs;
impl crate::RegisterSpec for FPUIMRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpuimr::R`](R) reader structure"]
impl crate::Readable for FPUIMRrs {}
#[doc = "`write(|w| ..)` method takes [`fpuimr::W`](W) writer structure"]
impl crate::Writable for FPUIMRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FPUIMR to value 0x1f"]
impl crate::Resettable for FPUIMRrs {
    const RESET_VALUE: u32 = 0x1f;
}
