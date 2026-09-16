///Register `D3PMR2` reader
pub type R = crate::R<D3PMR2rs>;
///Register `D3PMR2` writer
pub type W = crate::W<D3PMR2rs>;
/**D3 Pending Mask on Event input x+32

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
///Field `MR34` reader - D3 Pending Mask on Event input x+32
pub type MR34_R = crate::BitReader<INTERRUPT_MASK>;
impl MR34_R {
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
///Field `MR34` writer - D3 Pending Mask on Event input x+32
pub type MR34_W<'a, REG> = crate::BitWriter<'a, REG, INTERRUPT_MASK>;
impl<'a, REG> MR34_W<'a, REG>
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
///Field `MR35` reader - D3 Pending Mask on Event input x+32
pub use MR34_R as MR35_R;
///Field `MR41` reader - D3 Pending Mask on Event input x+32
pub use MR34_R as MR41_R;
///Field `MR48` reader - D3 Pending Mask on Event input x+32
pub use MR34_R as MR48_R;
///Field `MR49` reader - D3 Pending Mask on Event input x+32
pub use MR34_R as MR49_R;
///Field `MR50` reader - D3 Pending Mask on Event input x+32
pub use MR34_R as MR50_R;
///Field `MR51` reader - D3 Pending Mask on Event input x+32
pub use MR34_R as MR51_R;
///Field `MR52` reader - D3 Pending Mask on Event input x+32
pub use MR34_R as MR52_R;
///Field `MR53` reader - D3 Pending Mask on Event input x+32
pub use MR34_R as MR53_R;
///Field `MR35` writer - D3 Pending Mask on Event input x+32
pub use MR34_W as MR35_W;
///Field `MR41` writer - D3 Pending Mask on Event input x+32
pub use MR34_W as MR41_W;
///Field `MR48` writer - D3 Pending Mask on Event input x+32
pub use MR34_W as MR48_W;
///Field `MR49` writer - D3 Pending Mask on Event input x+32
pub use MR34_W as MR49_W;
///Field `MR50` writer - D3 Pending Mask on Event input x+32
pub use MR34_W as MR50_W;
///Field `MR51` writer - D3 Pending Mask on Event input x+32
pub use MR34_W as MR51_W;
///Field `MR52` writer - D3 Pending Mask on Event input x+32
pub use MR34_W as MR52_W;
///Field `MR53` writer - D3 Pending Mask on Event input x+32
pub use MR34_W as MR53_W;
impl R {
    ///Bit 2 - D3 Pending Mask on Event input x+32
    #[inline(always)]
    pub fn mr34(&self) -> MR34_R {
        MR34_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - D3 Pending Mask on Event input x+32
    #[inline(always)]
    pub fn mr35(&self) -> MR35_R {
        MR35_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 9 - D3 Pending Mask on Event input x+32
    #[inline(always)]
    pub fn mr41(&self) -> MR41_R {
        MR41_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 16 - D3 Pending Mask on Event input x+32
    #[inline(always)]
    pub fn mr48(&self) -> MR48_R {
        MR48_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - D3 Pending Mask on Event input x+32
    #[inline(always)]
    pub fn mr49(&self) -> MR49_R {
        MR49_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - D3 Pending Mask on Event input x+32
    #[inline(always)]
    pub fn mr50(&self) -> MR50_R {
        MR50_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - D3 Pending Mask on Event input x+32
    #[inline(always)]
    pub fn mr51(&self) -> MR51_R {
        MR51_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - D3 Pending Mask on Event input x+32
    #[inline(always)]
    pub fn mr52(&self) -> MR52_R {
        MR52_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - D3 Pending Mask on Event input x+32
    #[inline(always)]
    pub fn mr53(&self) -> MR53_R {
        MR53_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("D3PMR2")
            .field("mr34", &self.mr34())
            .field("mr35", &self.mr35())
            .field("mr41", &self.mr41())
            .field("mr48", &self.mr48())
            .field("mr49", &self.mr49())
            .field("mr50", &self.mr50())
            .field("mr51", &self.mr51())
            .field("mr52", &self.mr52())
            .field("mr53", &self.mr53())
            .finish()
    }
}
impl W {
    ///Bit 2 - D3 Pending Mask on Event input x+32
    #[inline(always)]
    pub fn mr34(&mut self) -> MR34_W<'_, D3PMR2rs> {
        MR34_W::new(self, 2)
    }
    ///Bit 3 - D3 Pending Mask on Event input x+32
    #[inline(always)]
    pub fn mr35(&mut self) -> MR35_W<'_, D3PMR2rs> {
        MR35_W::new(self, 3)
    }
    ///Bit 9 - D3 Pending Mask on Event input x+32
    #[inline(always)]
    pub fn mr41(&mut self) -> MR41_W<'_, D3PMR2rs> {
        MR41_W::new(self, 9)
    }
    ///Bit 16 - D3 Pending Mask on Event input x+32
    #[inline(always)]
    pub fn mr48(&mut self) -> MR48_W<'_, D3PMR2rs> {
        MR48_W::new(self, 16)
    }
    ///Bit 17 - D3 Pending Mask on Event input x+32
    #[inline(always)]
    pub fn mr49(&mut self) -> MR49_W<'_, D3PMR2rs> {
        MR49_W::new(self, 17)
    }
    ///Bit 18 - D3 Pending Mask on Event input x+32
    #[inline(always)]
    pub fn mr50(&mut self) -> MR50_W<'_, D3PMR2rs> {
        MR50_W::new(self, 18)
    }
    ///Bit 19 - D3 Pending Mask on Event input x+32
    #[inline(always)]
    pub fn mr51(&mut self) -> MR51_W<'_, D3PMR2rs> {
        MR51_W::new(self, 19)
    }
    ///Bit 20 - D3 Pending Mask on Event input x+32
    #[inline(always)]
    pub fn mr52(&mut self) -> MR52_W<'_, D3PMR2rs> {
        MR52_W::new(self, 20)
    }
    ///Bit 21 - D3 Pending Mask on Event input x+32
    #[inline(always)]
    pub fn mr53(&mut self) -> MR53_W<'_, D3PMR2rs> {
        MR53_W::new(self, 21)
    }
}
/**EXTI D3 pending mask register

You can [`read`](crate::Reg::read) this register and get [`d3pmr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d3pmr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#EXTI:D3PMR2)*/
pub struct D3PMR2rs;
impl crate::RegisterSpec for D3PMR2rs {
    type Ux = u32;
}
///`read()` method returns [`d3pmr2::R`](R) reader structure
impl crate::Readable for D3PMR2rs {}
///`write(|w| ..)` method takes [`d3pmr2::W`](W) writer structure
impl crate::Writable for D3PMR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets D3PMR2 to value 0
impl crate::Resettable for D3PMR2rs {}
