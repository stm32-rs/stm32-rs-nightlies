///Register `CPUPR3` reader
pub type R = crate::R<CPUPR3rs>;
///Register `CPUPR3` writer
pub type W = crate::W<CPUPR3rs>;
/**Configurable event inputs x+64 Pending bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PR82R {
    ///0: No trigger request occurred
    NotPending = 0,
    ///1: Selected trigger request occurred
    Pending = 1,
}
impl From<PR82R> for bool {
    #[inline(always)]
    fn from(variant: PR82R) -> Self {
        variant as u8 != 0
    }
}
///Field `PR82` reader - Configurable event inputs x+64 Pending bit
pub type PR82_R = crate::BitReader<PR82R>;
impl PR82_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PR82R {
        match self.bits {
            false => PR82R::NotPending,
            true => PR82R::Pending,
        }
    }
    ///No trigger request occurred
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == PR82R::NotPending
    }
    ///Selected trigger request occurred
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == PR82R::Pending
    }
}
/**Configurable event inputs x+64 Pending bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PR82W {
    ///1: Clears pending bit
    Clear = 1,
}
impl From<PR82W> for bool {
    #[inline(always)]
    fn from(variant: PR82W) -> Self {
        variant as u8 != 0
    }
}
///Field `PR82` writer - Configurable event inputs x+64 Pending bit
pub type PR82_W<'a, REG> = crate::BitWriter1C<'a, REG, PR82W>;
impl<'a, REG> PR82_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PR82W::Clear)
    }
}
///Field `PR84` reader - Configurable event inputs x+64 Pending bit
pub use PR82_R as PR84_R;
///Field `PR85` reader - Configurable event inputs x+64 Pending bit
pub use PR82_R as PR85_R;
///Field `PR86` reader - Configurable event inputs x+64 Pending bit
pub use PR82_R as PR86_R;
///Field `PR84` writer - Configurable event inputs x+64 Pending bit
pub use PR82_W as PR84_W;
///Field `PR85` writer - Configurable event inputs x+64 Pending bit
pub use PR82_W as PR85_W;
///Field `PR86` writer - Configurable event inputs x+64 Pending bit
pub use PR82_W as PR86_W;
impl R {
    ///Bit 18 - Configurable event inputs x+64 Pending bit
    #[inline(always)]
    pub fn pr82(&self) -> PR82_R {
        PR82_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - Configurable event inputs x+64 Pending bit
    #[inline(always)]
    pub fn pr84(&self) -> PR84_R {
        PR84_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Configurable event inputs x+64 Pending bit
    #[inline(always)]
    pub fn pr85(&self) -> PR85_R {
        PR85_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Configurable event inputs x+64 Pending bit
    #[inline(always)]
    pub fn pr86(&self) -> PR86_R {
        PR86_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUPR3")
            .field("pr82", &self.pr82())
            .field("pr84", &self.pr84())
            .field("pr85", &self.pr85())
            .field("pr86", &self.pr86())
            .finish()
    }
}
impl W {
    ///Bit 18 - Configurable event inputs x+64 Pending bit
    #[inline(always)]
    pub fn pr82(&mut self) -> PR82_W<CPUPR3rs> {
        PR82_W::new(self, 18)
    }
    ///Bit 20 - Configurable event inputs x+64 Pending bit
    #[inline(always)]
    pub fn pr84(&mut self) -> PR84_W<CPUPR3rs> {
        PR84_W::new(self, 20)
    }
    ///Bit 21 - Configurable event inputs x+64 Pending bit
    #[inline(always)]
    pub fn pr85(&mut self) -> PR85_W<CPUPR3rs> {
        PR85_W::new(self, 21)
    }
    ///Bit 22 - Configurable event inputs x+64 Pending bit
    #[inline(always)]
    pub fn pr86(&mut self) -> PR86_W<CPUPR3rs> {
        PR86_W::new(self, 22)
    }
}
/**EXTI pending register

You can [`read`](crate::Reg::read) this register and get [`cpupr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpupr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#EXTI:CPUPR3)*/
pub struct CPUPR3rs;
impl crate::RegisterSpec for CPUPR3rs {
    type Ux = u32;
}
///`read()` method returns [`cpupr3::R`](R) reader structure
impl crate::Readable for CPUPR3rs {}
///`write(|w| ..)` method takes [`cpupr3::W`](W) writer structure
impl crate::Writable for CPUPR3rs {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0074_0000;
}
///`reset()` method sets CPUPR3 to value 0
impl crate::Resettable for CPUPR3rs {}
