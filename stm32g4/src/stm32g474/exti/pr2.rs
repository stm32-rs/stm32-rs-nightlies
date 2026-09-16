///Register `PR2` reader
pub type R = crate::R<PR2rs>;
///Register `PR2` writer
pub type W = crate::W<PR2rs>;
/**Pending bit 32

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIF32R {
    ///0: No trigger request occurred
    NotPending = 0,
    ///1: Selected trigger request occurred
    Pending = 1,
}
impl From<PIF32R> for bool {
    #[inline(always)]
    fn from(variant: PIF32R) -> Self {
        variant as u8 != 0
    }
}
///Field `PIF32` reader - Pending bit 32
pub type PIF32_R = crate::BitReader<PIF32R>;
impl PIF32_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIF32R {
        match self.bits {
            false => PIF32R::NotPending,
            true => PIF32R::Pending,
        }
    }
    ///No trigger request occurred
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == PIF32R::NotPending
    }
    ///Selected trigger request occurred
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == PIF32R::Pending
    }
}
/**Pending bit 32

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIF32W {
    ///1: Clears pending bit
    Clear = 1,
}
impl From<PIF32W> for bool {
    #[inline(always)]
    fn from(variant: PIF32W) -> Self {
        variant as u8 != 0
    }
}
///Field `PIF32` writer - Pending bit 32
pub type PIF32_W<'a, REG> = crate::BitWriter1C<'a, REG, PIF32W>;
impl<'a, REG> PIF32_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PIF32W::Clear)
    }
}
///Field `PIF33` reader - Pending bit 33
pub use PIF32_R as PIF33_R;
///Field `PIF40` reader - Pending bit 40
pub use PIF32_R as PIF40_R;
///Field `PIF41` reader - Pending bit 41
pub use PIF32_R as PIF41_R;
///Field `PIF33` writer - Pending bit 33
pub use PIF32_W as PIF33_W;
///Field `PIF40` writer - Pending bit 40
pub use PIF32_W as PIF40_W;
///Field `PIF41` writer - Pending bit 41
pub use PIF32_W as PIF41_W;
impl R {
    ///Bit 0 - Pending bit 32
    #[inline(always)]
    pub fn pif32(&self) -> PIF32_R {
        PIF32_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Pending bit 33
    #[inline(always)]
    pub fn pif33(&self) -> PIF33_R {
        PIF33_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - Pending bit 40
    #[inline(always)]
    pub fn pif40(&self) -> PIF40_R {
        PIF40_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Pending bit 41
    #[inline(always)]
    pub fn pif41(&self) -> PIF41_R {
        PIF41_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PR2")
            .field("pif32", &self.pif32())
            .field("pif33", &self.pif33())
            .field("pif40", &self.pif40())
            .field("pif41", &self.pif41())
            .finish()
    }
}
impl W {
    ///Bit 0 - Pending bit 32
    #[inline(always)]
    pub fn pif32(&mut self) -> PIF32_W<'_, PR2rs> {
        PIF32_W::new(self, 0)
    }
    ///Bit 1 - Pending bit 33
    #[inline(always)]
    pub fn pif33(&mut self) -> PIF33_W<'_, PR2rs> {
        PIF33_W::new(self, 1)
    }
    ///Bit 8 - Pending bit 40
    #[inline(always)]
    pub fn pif40(&mut self) -> PIF40_W<'_, PR2rs> {
        PIF40_W::new(self, 8)
    }
    ///Bit 9 - Pending bit 41
    #[inline(always)]
    pub fn pif41(&mut self) -> PIF41_W<'_, PR2rs> {
        PIF41_W::new(self, 9)
    }
}
/**Pending register

You can [`read`](crate::Reg::read) this register and get [`pr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#EXTI:PR2)*/
pub struct PR2rs;
impl crate::RegisterSpec for PR2rs {
    type Ux = u32;
}
///`read()` method returns [`pr2::R`](R) reader structure
impl crate::Readable for PR2rs {}
///`write(|w| ..)` method takes [`pr2::W`](W) writer structure
impl crate::Writable for PR2rs {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0303;
}
///`reset()` method sets PR2 to value 0
impl crate::Resettable for PR2rs {}
