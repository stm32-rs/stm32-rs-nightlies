///Register `SWIER2` reader
pub type R = crate::R<SWIER2rs>;
///Register `SWIER2` writer
pub type W = crate::W<SWIER2rs>;
/**Software interrupt on line x+32

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
///Field `SWIER49` reader - Software interrupt on line x+32
pub type SWIER49_R = crate::BitReader<SOFTWARE_INTERRUPT>;
impl SWIER49_R {
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
///Field `SWIER49` writer - Software interrupt on line x+32
pub type SWIER49_W<'a, REG> = crate::BitWriter<'a, REG, SOFTWARE_INTERRUPT>;
impl<'a, REG> SWIER49_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut crate::W<REG> {
        self.variant(SOFTWARE_INTERRUPT::Pend)
    }
}
///Field `SWIER51` reader - Software interrupt on line x+32
pub use SWIER49_R as SWIER51_R;
///Field `SWIER51` writer - Software interrupt on line x+32
pub use SWIER49_W as SWIER51_W;
impl R {
    ///Bit 17 - Software interrupt on line x+32
    #[inline(always)]
    pub fn swier49(&self) -> SWIER49_R {
        SWIER49_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 19 - Software interrupt on line x+32
    #[inline(always)]
    pub fn swier51(&self) -> SWIER51_R {
        SWIER51_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWIER2")
            .field("swier49", &self.swier49())
            .field("swier51", &self.swier51())
            .finish()
    }
}
impl W {
    ///Bit 17 - Software interrupt on line x+32
    #[inline(always)]
    pub fn swier49(&mut self) -> SWIER49_W<'_, SWIER2rs> {
        SWIER49_W::new(self, 17)
    }
    ///Bit 19 - Software interrupt on line x+32
    #[inline(always)]
    pub fn swier51(&mut self) -> SWIER51_W<'_, SWIER2rs> {
        SWIER51_W::new(self, 19)
    }
}
/**EXTI software interrupt event register

You can [`read`](crate::Reg::read) this register and get [`swier2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#EXTI:SWIER2)*/
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
