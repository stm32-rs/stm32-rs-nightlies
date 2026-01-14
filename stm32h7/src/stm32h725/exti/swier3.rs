///Register `SWIER3` reader
pub type R = crate::R<SWIER3rs>;
///Register `SWIER3` writer
pub type W = crate::W<SWIER3rs>;
/**Software interrupt on line x+64

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
///Field `SWIER82` reader - Software interrupt on line x+64
pub type SWIER82_R = crate::BitReader<SOFTWARE_INTERRUPT>;
impl SWIER82_R {
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
///Field `SWIER82` writer - Software interrupt on line x+64
pub type SWIER82_W<'a, REG> = crate::BitWriter<'a, REG, SOFTWARE_INTERRUPT>;
impl<'a, REG> SWIER82_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut crate::W<REG> {
        self.variant(SOFTWARE_INTERRUPT::Pend)
    }
}
///Field `SWIER84` reader - Software interrupt on line x+64
pub use SWIER82_R as SWIER84_R;
///Field `SWIER85` reader - Software interrupt on line x+64
pub use SWIER82_R as SWIER85_R;
///Field `SWIER86` reader - Software interrupt on line x+64
pub use SWIER82_R as SWIER86_R;
///Field `SWIER84` writer - Software interrupt on line x+64
pub use SWIER82_W as SWIER84_W;
///Field `SWIER85` writer - Software interrupt on line x+64
pub use SWIER82_W as SWIER85_W;
///Field `SWIER86` writer - Software interrupt on line x+64
pub use SWIER82_W as SWIER86_W;
impl R {
    ///Bit 18 - Software interrupt on line x+64
    #[inline(always)]
    pub fn swier82(&self) -> SWIER82_R {
        SWIER82_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - Software interrupt on line x+64
    #[inline(always)]
    pub fn swier84(&self) -> SWIER84_R {
        SWIER84_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Software interrupt on line x+64
    #[inline(always)]
    pub fn swier85(&self) -> SWIER85_R {
        SWIER85_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Software interrupt on line x+64
    #[inline(always)]
    pub fn swier86(&self) -> SWIER86_R {
        SWIER86_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWIER3")
            .field("swier82", &self.swier82())
            .field("swier84", &self.swier84())
            .field("swier85", &self.swier85())
            .field("swier86", &self.swier86())
            .finish()
    }
}
impl W {
    ///Bit 18 - Software interrupt on line x+64
    #[inline(always)]
    pub fn swier82(&mut self) -> SWIER82_W<'_, SWIER3rs> {
        SWIER82_W::new(self, 18)
    }
    ///Bit 20 - Software interrupt on line x+64
    #[inline(always)]
    pub fn swier84(&mut self) -> SWIER84_W<'_, SWIER3rs> {
        SWIER84_W::new(self, 20)
    }
    ///Bit 21 - Software interrupt on line x+64
    #[inline(always)]
    pub fn swier85(&mut self) -> SWIER85_W<'_, SWIER3rs> {
        SWIER85_W::new(self, 21)
    }
    ///Bit 22 - Software interrupt on line x+64
    #[inline(always)]
    pub fn swier86(&mut self) -> SWIER86_W<'_, SWIER3rs> {
        SWIER86_W::new(self, 22)
    }
}
/**EXTI software interrupt event register

You can [`read`](crate::Reg::read) this register and get [`swier3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#EXTI:SWIER3)*/
pub struct SWIER3rs;
impl crate::RegisterSpec for SWIER3rs {
    type Ux = u32;
}
///`read()` method returns [`swier3::R`](R) reader structure
impl crate::Readable for SWIER3rs {}
///`write(|w| ..)` method takes [`swier3::W`](W) writer structure
impl crate::Writable for SWIER3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWIER3 to value 0
impl crate::Resettable for SWIER3rs {}
