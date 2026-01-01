///Register `FPUIMR` reader
pub type R = crate::R<FPUIMRrs>;
///Register `FPUIMR` writer
pub type W = crate::W<FPUIMRrs>;
/**FPU interrupt enable

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPU_IE0 {
    ///0: Interrupt disabled
    Disabled = 0,
    ///1: Interrupt enabled
    Enabled = 1,
}
impl From<FPU_IE0> for bool {
    #[inline(always)]
    fn from(variant: FPU_IE0) -> Self {
        variant as u8 != 0
    }
}
///Field `FPU_IE0` reader - FPU interrupt enable
pub type FPU_IE0_R = crate::BitReader<FPU_IE0>;
impl FPU_IE0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FPU_IE0 {
        match self.bits {
            false => FPU_IE0::Disabled,
            true => FPU_IE0::Enabled,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FPU_IE0::Disabled
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FPU_IE0::Enabled
    }
}
///Field `FPU_IE0` writer - FPU interrupt enable
pub type FPU_IE0_W<'a, REG> = crate::BitWriter<'a, REG, FPU_IE0>;
impl<'a, REG> FPU_IE0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FPU_IE0::Disabled)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FPU_IE0::Enabled)
    }
}
///Field `FPU_IE1` reader - FPU interrupt enable
pub use FPU_IE0_R as FPU_IE1_R;
///Field `FPU_IE2` reader - FPU interrupt enable
pub use FPU_IE0_R as FPU_IE2_R;
///Field `FPU_IE3` reader - FPU interrupt enable
pub use FPU_IE0_R as FPU_IE3_R;
///Field `FPU_IE4` reader - FPU interrupt enable
pub use FPU_IE0_R as FPU_IE4_R;
///Field `FPU_IE5` reader - FPU interrupt enable
pub use FPU_IE0_R as FPU_IE5_R;
///Field `FPU_IE1` writer - FPU interrupt enable
pub use FPU_IE0_W as FPU_IE1_W;
///Field `FPU_IE2` writer - FPU interrupt enable
pub use FPU_IE0_W as FPU_IE2_W;
///Field `FPU_IE3` writer - FPU interrupt enable
pub use FPU_IE0_W as FPU_IE3_W;
///Field `FPU_IE4` writer - FPU interrupt enable
pub use FPU_IE0_W as FPU_IE4_W;
///Field `FPU_IE5` writer - FPU interrupt enable
pub use FPU_IE0_W as FPU_IE5_W;
impl R {
    ///Bit 0 - FPU interrupt enable
    #[inline(always)]
    pub fn fpu_ie0(&self) -> FPU_IE0_R {
        FPU_IE0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - FPU interrupt enable
    #[inline(always)]
    pub fn fpu_ie1(&self) -> FPU_IE1_R {
        FPU_IE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - FPU interrupt enable
    #[inline(always)]
    pub fn fpu_ie2(&self) -> FPU_IE2_R {
        FPU_IE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - FPU interrupt enable
    #[inline(always)]
    pub fn fpu_ie3(&self) -> FPU_IE3_R {
        FPU_IE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - FPU interrupt enable
    #[inline(always)]
    pub fn fpu_ie4(&self) -> FPU_IE4_R {
        FPU_IE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - FPU interrupt enable
    #[inline(always)]
    pub fn fpu_ie5(&self) -> FPU_IE5_R {
        FPU_IE5_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FPUIMR")
            .field("fpu_ie0", &self.fpu_ie0())
            .field("fpu_ie1", &self.fpu_ie1())
            .field("fpu_ie2", &self.fpu_ie2())
            .field("fpu_ie3", &self.fpu_ie3())
            .field("fpu_ie4", &self.fpu_ie4())
            .field("fpu_ie5", &self.fpu_ie5())
            .finish()
    }
}
impl W {
    ///Bit 0 - FPU interrupt enable
    #[inline(always)]
    pub fn fpu_ie0(&mut self) -> FPU_IE0_W<'_, FPUIMRrs> {
        FPU_IE0_W::new(self, 0)
    }
    ///Bit 1 - FPU interrupt enable
    #[inline(always)]
    pub fn fpu_ie1(&mut self) -> FPU_IE1_W<'_, FPUIMRrs> {
        FPU_IE1_W::new(self, 1)
    }
    ///Bit 2 - FPU interrupt enable
    #[inline(always)]
    pub fn fpu_ie2(&mut self) -> FPU_IE2_W<'_, FPUIMRrs> {
        FPU_IE2_W::new(self, 2)
    }
    ///Bit 3 - FPU interrupt enable
    #[inline(always)]
    pub fn fpu_ie3(&mut self) -> FPU_IE3_W<'_, FPUIMRrs> {
        FPU_IE3_W::new(self, 3)
    }
    ///Bit 4 - FPU interrupt enable
    #[inline(always)]
    pub fn fpu_ie4(&mut self) -> FPU_IE4_W<'_, FPUIMRrs> {
        FPU_IE4_W::new(self, 4)
    }
    ///Bit 5 - FPU interrupt enable
    #[inline(always)]
    pub fn fpu_ie5(&mut self) -> FPU_IE5_W<'_, FPUIMRrs> {
        FPU_IE5_W::new(self, 5)
    }
}
/**SBS FPU interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`fpuimr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpuimr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#SBS:FPUIMR)*/
pub struct FPUIMRrs;
impl crate::RegisterSpec for FPUIMRrs {
    type Ux = u32;
}
///`read()` method returns [`fpuimr::R`](R) reader structure
impl crate::Readable for FPUIMRrs {}
///`write(|w| ..)` method takes [`fpuimr::W`](W) writer structure
impl crate::Writable for FPUIMRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FPUIMR to value 0x1f
impl crate::Resettable for FPUIMRrs {
    const RESET_VALUE: u32 = 0x1f;
}
