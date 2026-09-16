///Register `PR2` reader
pub type R = crate::R<PR2rs>;
///Register `PR2` writer
pub type W = crate::W<PR2rs>;
/**Pending bit on line 32

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PR32R {
    ///0: No trigger request occurred
    NotPending = 0,
    ///1: Selected trigger request occurred
    Pending = 1,
}
impl From<PR32R> for bool {
    #[inline(always)]
    fn from(variant: PR32R) -> Self {
        variant as u8 != 0
    }
}
///Field `PR32` reader - Pending bit on line 32
pub type PR32_R = crate::BitReader<PR32R>;
impl PR32_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PR32R {
        match self.bits {
            false => PR32R::NotPending,
            true => PR32R::Pending,
        }
    }
    ///No trigger request occurred
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == PR32R::NotPending
    }
    ///Selected trigger request occurred
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == PR32R::Pending
    }
}
/**Pending bit on line 32

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PR32W {
    ///1: Clears pending bit
    Clear = 1,
}
impl From<PR32W> for bool {
    #[inline(always)]
    fn from(variant: PR32W) -> Self {
        variant as u8 != 0
    }
}
///Field `PR32` writer - Pending bit on line 32
pub type PR32_W<'a, REG> = crate::BitWriter1C<'a, REG, PR32W>;
impl<'a, REG> PR32_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PR32W::Clear)
    }
}
///Field `PR33` reader - Pending bit on line 33
pub use PR32_R as PR33_R;
///Field `PR33` writer - Pending bit on line 33
pub use PR32_W as PR33_W;
impl R {
    ///Bit 0 - Pending bit on line 32
    #[inline(always)]
    pub fn pr32(&self) -> PR32_R {
        PR32_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Pending bit on line 33
    #[inline(always)]
    pub fn pr33(&self) -> PR33_R {
        PR33_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PR2")
            .field("pr32", &self.pr32())
            .field("pr33", &self.pr33())
            .finish()
    }
}
impl W {
    ///Bit 0 - Pending bit on line 32
    #[inline(always)]
    pub fn pr32(&mut self) -> PR32_W<'_, PR2rs> {
        PR32_W::new(self, 0)
    }
    ///Bit 1 - Pending bit on line 33
    #[inline(always)]
    pub fn pr33(&mut self) -> PR33_W<'_, PR2rs> {
        PR33_W::new(self, 1)
    }
}
/**Pending register

You can [`read`](crate::Reg::read) this register and get [`pr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F302.html#EXTI:PR2)*/
pub struct PR2rs;
impl crate::RegisterSpec for PR2rs {
    type Ux = u32;
}
///`read()` method returns [`pr2::R`](R) reader structure
impl crate::Readable for PR2rs {}
///`write(|w| ..)` method takes [`pr2::W`](W) writer structure
impl crate::Writable for PR2rs {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x03;
}
///`reset()` method sets PR2 to value 0
impl crate::Resettable for PR2rs {}
