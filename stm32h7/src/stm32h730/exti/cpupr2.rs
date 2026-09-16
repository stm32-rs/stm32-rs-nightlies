///Register `CPUPR2` reader
pub type R = crate::R<CPUPR2rs>;
///Register `CPUPR2` writer
pub type W = crate::W<CPUPR2rs>;
/**Configurable event inputs x+32 Pending bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PR49R {
    ///0: No trigger request occurred
    NotPending = 0,
    ///1: Selected trigger request occurred
    Pending = 1,
}
impl From<PR49R> for bool {
    #[inline(always)]
    fn from(variant: PR49R) -> Self {
        variant as u8 != 0
    }
}
///Field `PR49` reader - Configurable event inputs x+32 Pending bit
pub type PR49_R = crate::BitReader<PR49R>;
impl PR49_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PR49R {
        match self.bits {
            false => PR49R::NotPending,
            true => PR49R::Pending,
        }
    }
    ///No trigger request occurred
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == PR49R::NotPending
    }
    ///Selected trigger request occurred
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == PR49R::Pending
    }
}
/**Configurable event inputs x+32 Pending bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PR49W {
    ///1: Clears pending bit
    Clear = 1,
}
impl From<PR49W> for bool {
    #[inline(always)]
    fn from(variant: PR49W) -> Self {
        variant as u8 != 0
    }
}
///Field `PR49` writer - Configurable event inputs x+32 Pending bit
pub type PR49_W<'a, REG> = crate::BitWriter1C<'a, REG, PR49W>;
impl<'a, REG> PR49_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PR49W::Clear)
    }
}
///Field `PR51` reader - Configurable event inputs x+32 Pending bit
pub use PR49_R as PR51_R;
///Field `PR51` writer - Configurable event inputs x+32 Pending bit
pub use PR49_W as PR51_W;
impl R {
    ///Bit 17 - Configurable event inputs x+32 Pending bit
    #[inline(always)]
    pub fn pr49(&self) -> PR49_R {
        PR49_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 19 - Configurable event inputs x+32 Pending bit
    #[inline(always)]
    pub fn pr51(&self) -> PR51_R {
        PR51_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUPR2")
            .field("pr49", &self.pr49())
            .field("pr51", &self.pr51())
            .finish()
    }
}
impl W {
    ///Bit 17 - Configurable event inputs x+32 Pending bit
    #[inline(always)]
    pub fn pr49(&mut self) -> PR49_W<'_, CPUPR2rs> {
        PR49_W::new(self, 17)
    }
    ///Bit 19 - Configurable event inputs x+32 Pending bit
    #[inline(always)]
    pub fn pr51(&mut self) -> PR51_W<'_, CPUPR2rs> {
        PR51_W::new(self, 19)
    }
}
/**EXTI pending register

You can [`read`](crate::Reg::read) this register and get [`cpupr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpupr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#EXTI:CPUPR2)*/
pub struct CPUPR2rs;
impl crate::RegisterSpec for CPUPR2rs {
    type Ux = u32;
}
///`read()` method returns [`cpupr2::R`](R) reader structure
impl crate::Readable for CPUPR2rs {}
///`write(|w| ..)` method takes [`cpupr2::W`](W) writer structure
impl crate::Writable for CPUPR2rs {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x000a_0000;
}
///`reset()` method sets CPUPR2 to value 0
impl crate::Resettable for CPUPR2rs {}
