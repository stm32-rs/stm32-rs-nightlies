///Register `D3PMR3` reader
pub type R = crate::R<D3PMR3rs>;
///Register `D3PMR3` writer
pub type W = crate::W<D3PMR3rs>;
/**D3 Pending Mask on Event input x+64

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTERRUPT_MASK {
    ///0: Interrupt request line is masked
    Masked = 0,
    ///1: Interrupt request line is unmasked
    Unmasked = 1,
}
impl From<INTERRUPT_MASK> for bool {
    #[inline(always)]
    fn from(variant: INTERRUPT_MASK) -> Self {
        variant as u8 != 0
    }
}
///Field `MR88` reader - D3 Pending Mask on Event input x+64
pub type MR88_R = crate::BitReader<INTERRUPT_MASK>;
impl MR88_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> INTERRUPT_MASK {
        match self.bits {
            false => INTERRUPT_MASK::Masked,
            true => INTERRUPT_MASK::Unmasked,
        }
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == INTERRUPT_MASK::Masked
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == INTERRUPT_MASK::Unmasked
    }
}
///Field `MR88` writer - D3 Pending Mask on Event input x+64
pub type MR88_W<'a, REG> = crate::BitWriter<'a, REG, INTERRUPT_MASK>;
impl<'a, REG> MR88_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(INTERRUPT_MASK::Masked)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(INTERRUPT_MASK::Unmasked)
    }
}
impl R {
    ///Bit 24 - D3 Pending Mask on Event input x+64
    #[inline(always)]
    pub fn mr88(&self) -> MR88_R {
        MR88_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("D3PMR3")
            .field("mr88", &self.mr88())
            .finish()
    }
}
impl W {
    ///Bit 24 - D3 Pending Mask on Event input x+64
    #[inline(always)]
    pub fn mr88(&mut self) -> MR88_W<'_, D3PMR3rs> {
        MR88_W::new(self, 24)
    }
}
/**EXTI D3 pending mask register

You can [`read`](crate::Reg::read) this register and get [`d3pmr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d3pmr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#EXTI:D3PMR3)*/
pub struct D3PMR3rs;
impl crate::RegisterSpec for D3PMR3rs {
    type Ux = u32;
}
///`read()` method returns [`d3pmr3::R`](R) reader structure
impl crate::Readable for D3PMR3rs {}
///`write(|w| ..)` method takes [`d3pmr3::W`](W) writer structure
impl crate::Writable for D3PMR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets D3PMR3 to value 0
impl crate::Resettable for D3PMR3rs {}
