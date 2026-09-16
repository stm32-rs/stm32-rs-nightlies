///Register `SWIER2` reader
pub type R = crate::R<SWIER2rs>;
///Register `SWIER2` writer
pub type W = crate::W<SWIER2rs>;
/**Software interrupt on line 32

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOFTWARE_INTERRUPT {
    ///1: Generates an interrupt request
    Pend = 1,
}
impl From<SOFTWARE_INTERRUPT> for bool {
    #[inline(always)]
    fn from(variant: SOFTWARE_INTERRUPT) -> Self {
        variant as u8 != 0
    }
}
///Field `SWIER32` reader - Software interrupt on line 32
pub type SWIER32_R = crate::BitReader<SOFTWARE_INTERRUPT>;
impl SWIER32_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SOFTWARE_INTERRUPT> {
        match self.bits {
            true => Some(SOFTWARE_INTERRUPT::Pend),
            _ => None,
        }
    }
    ///Generates an interrupt request
    #[inline(always)]
    pub fn is_pend(&self) -> bool {
        *self == SOFTWARE_INTERRUPT::Pend
    }
}
///Field `SWIER32` writer - Software interrupt on line 32
pub type SWIER32_W<'a, REG> = crate::BitWriter<'a, REG, SOFTWARE_INTERRUPT>;
impl<'a, REG> SWIER32_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut crate::W<REG> {
        self.variant(SOFTWARE_INTERRUPT::Pend)
    }
}
///Field `SWIER33` reader - Software interrupt on line 33
pub use SWIER32_R as SWIER33_R;
///Field `SWIER33` writer - Software interrupt on line 33
pub use SWIER32_W as SWIER33_W;
impl R {
    ///Bit 0 - Software interrupt on line 32
    #[inline(always)]
    pub fn swier32(&self) -> SWIER32_R {
        SWIER32_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Software interrupt on line 33
    #[inline(always)]
    pub fn swier33(&self) -> SWIER33_R {
        SWIER33_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWIER2")
            .field("swier32", &self.swier32())
            .field("swier33", &self.swier33())
            .finish()
    }
}
impl W {
    ///Bit 0 - Software interrupt on line 32
    #[inline(always)]
    pub fn swier32(&mut self) -> SWIER32_W<'_, SWIER2rs> {
        SWIER32_W::new(self, 0)
    }
    ///Bit 1 - Software interrupt on line 33
    #[inline(always)]
    pub fn swier33(&mut self) -> SWIER33_W<'_, SWIER2rs> {
        SWIER33_W::new(self, 1)
    }
}
/**Software interrupt event register

You can [`read`](crate::Reg::read) this register and get [`swier2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F303.html#EXTI:SWIER2)*/
pub struct SWIER2rs;
impl crate::RegisterSpec for SWIER2rs {
    type Ux = u32;
}
///`read()` method returns [`swier2::R`](R) reader structure
impl crate::Readable for SWIER2rs {}
///`write(|w| ..)` method takes [`swier2::W`](W) writer structure
impl crate::Writable for SWIER2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWIER2 to value 0
impl crate::Resettable for SWIER2rs {}
