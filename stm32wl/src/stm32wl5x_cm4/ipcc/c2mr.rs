///Register `C2MR` reader
pub type R = crate::R<C2MRrs>;
///Register `C2MR` writer
pub type W = crate::W<C2MRrs>;
/**CH1OM

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH1OM {
    ///0: Receive channel n occupied interrupt not masked
    Unmasked = 0,
    ///1: Receive channel n occupied interrupt masked
    Masked = 1,
}
impl From<CH1OM> for bool {
    #[inline(always)]
    fn from(variant: CH1OM) -> Self {
        variant as u8 != 0
    }
}
///Field `CH1OM` reader - CH1OM
pub type CH1OM_R = crate::BitReader<CH1OM>;
impl CH1OM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CH1OM {
        match self.bits {
            false => CH1OM::Unmasked,
            true => CH1OM::Masked,
        }
    }
    ///Receive channel n occupied interrupt not masked
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == CH1OM::Unmasked
    }
    ///Receive channel n occupied interrupt masked
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == CH1OM::Masked
    }
}
///Field `CH1OM` writer - CH1OM
pub type CH1OM_W<'a, REG> = crate::BitWriter<'a, REG, CH1OM>;
impl<'a, REG> CH1OM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Receive channel n occupied interrupt not masked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(CH1OM::Unmasked)
    }
    ///Receive channel n occupied interrupt masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(CH1OM::Masked)
    }
}
///Field `CH2OM` reader - CH2OM
pub use CH1OM_R as CH2OM_R;
///Field `CH3OM` reader - CH3OM
pub use CH1OM_R as CH3OM_R;
///Field `CH4OM` reader - CH4OM
pub use CH1OM_R as CH4OM_R;
///Field `CH5OM` reader - CH5OM
pub use CH1OM_R as CH5OM_R;
///Field `CH6OM` reader - CH6OM
pub use CH1OM_R as CH6OM_R;
///Field `CH2OM` writer - CH2OM
pub use CH1OM_W as CH2OM_W;
///Field `CH3OM` writer - CH3OM
pub use CH1OM_W as CH3OM_W;
///Field `CH4OM` writer - CH4OM
pub use CH1OM_W as CH4OM_W;
///Field `CH5OM` writer - CH5OM
pub use CH1OM_W as CH5OM_W;
///Field `CH6OM` writer - CH6OM
pub use CH1OM_W as CH6OM_W;
/**CH1FM

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH1FM {
    ///0: Transmit channel n free interrupt not masked
    Unmasked = 0,
    ///1: Transmit channel n free interrupt masked
    Masked = 1,
}
impl From<CH1FM> for bool {
    #[inline(always)]
    fn from(variant: CH1FM) -> Self {
        variant as u8 != 0
    }
}
///Field `CH1FM` reader - CH1FM
pub type CH1FM_R = crate::BitReader<CH1FM>;
impl CH1FM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CH1FM {
        match self.bits {
            false => CH1FM::Unmasked,
            true => CH1FM::Masked,
        }
    }
    ///Transmit channel n free interrupt not masked
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == CH1FM::Unmasked
    }
    ///Transmit channel n free interrupt masked
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == CH1FM::Masked
    }
}
///Field `CH1FM` writer - CH1FM
pub type CH1FM_W<'a, REG> = crate::BitWriter<'a, REG, CH1FM>;
impl<'a, REG> CH1FM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transmit channel n free interrupt not masked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(CH1FM::Unmasked)
    }
    ///Transmit channel n free interrupt masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(CH1FM::Masked)
    }
}
///Field `CH2FM` reader - CH2FM
pub use CH1FM_R as CH2FM_R;
///Field `CH3FM` reader - CH3FM
pub use CH1FM_R as CH3FM_R;
///Field `CH4FM` reader - CH4FM
pub use CH1FM_R as CH4FM_R;
///Field `CH5FM` reader - CH5FM
pub use CH1FM_R as CH5FM_R;
///Field `CH6FM` reader - CH6FM
pub use CH1FM_R as CH6FM_R;
///Field `CH2FM` writer - CH2FM
pub use CH1FM_W as CH2FM_W;
///Field `CH3FM` writer - CH3FM
pub use CH1FM_W as CH3FM_W;
///Field `CH4FM` writer - CH4FM
pub use CH1FM_W as CH4FM_W;
///Field `CH5FM` writer - CH5FM
pub use CH1FM_W as CH5FM_W;
///Field `CH6FM` writer - CH6FM
pub use CH1FM_W as CH6FM_W;
impl R {
    ///Bit 0 - CH1OM
    #[inline(always)]
    pub fn ch1om(&self) -> CH1OM_R {
        CH1OM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CH2OM
    #[inline(always)]
    pub fn ch2om(&self) -> CH2OM_R {
        CH2OM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CH3OM
    #[inline(always)]
    pub fn ch3om(&self) -> CH3OM_R {
        CH3OM_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CH4OM
    #[inline(always)]
    pub fn ch4om(&self) -> CH4OM_R {
        CH4OM_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CH5OM
    #[inline(always)]
    pub fn ch5om(&self) -> CH5OM_R {
        CH5OM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - CH6OM
    #[inline(always)]
    pub fn ch6om(&self) -> CH6OM_R {
        CH6OM_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 16 - CH1FM
    #[inline(always)]
    pub fn ch1fm(&self) -> CH1FM_R {
        CH1FM_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - CH2FM
    #[inline(always)]
    pub fn ch2fm(&self) -> CH2FM_R {
        CH2FM_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - CH3FM
    #[inline(always)]
    pub fn ch3fm(&self) -> CH3FM_R {
        CH3FM_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - CH4FM
    #[inline(always)]
    pub fn ch4fm(&self) -> CH4FM_R {
        CH4FM_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - CH5FM
    #[inline(always)]
    pub fn ch5fm(&self) -> CH5FM_R {
        CH5FM_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - CH6FM
    #[inline(always)]
    pub fn ch6fm(&self) -> CH6FM_R {
        CH6FM_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C2MR")
            .field("ch1om", &self.ch1om())
            .field("ch2om", &self.ch2om())
            .field("ch3om", &self.ch3om())
            .field("ch4om", &self.ch4om())
            .field("ch5om", &self.ch5om())
            .field("ch6om", &self.ch6om())
            .field("ch1fm", &self.ch1fm())
            .field("ch2fm", &self.ch2fm())
            .field("ch3fm", &self.ch3fm())
            .field("ch4fm", &self.ch4fm())
            .field("ch5fm", &self.ch5fm())
            .field("ch6fm", &self.ch6fm())
            .finish()
    }
}
impl W {
    ///Bit 0 - CH1OM
    #[inline(always)]
    pub fn ch1om(&mut self) -> CH1OM_W<'_, C2MRrs> {
        CH1OM_W::new(self, 0)
    }
    ///Bit 1 - CH2OM
    #[inline(always)]
    pub fn ch2om(&mut self) -> CH2OM_W<'_, C2MRrs> {
        CH2OM_W::new(self, 1)
    }
    ///Bit 2 - CH3OM
    #[inline(always)]
    pub fn ch3om(&mut self) -> CH3OM_W<'_, C2MRrs> {
        CH3OM_W::new(self, 2)
    }
    ///Bit 3 - CH4OM
    #[inline(always)]
    pub fn ch4om(&mut self) -> CH4OM_W<'_, C2MRrs> {
        CH4OM_W::new(self, 3)
    }
    ///Bit 4 - CH5OM
    #[inline(always)]
    pub fn ch5om(&mut self) -> CH5OM_W<'_, C2MRrs> {
        CH5OM_W::new(self, 4)
    }
    ///Bit 5 - CH6OM
    #[inline(always)]
    pub fn ch6om(&mut self) -> CH6OM_W<'_, C2MRrs> {
        CH6OM_W::new(self, 5)
    }
    ///Bit 16 - CH1FM
    #[inline(always)]
    pub fn ch1fm(&mut self) -> CH1FM_W<'_, C2MRrs> {
        CH1FM_W::new(self, 16)
    }
    ///Bit 17 - CH2FM
    #[inline(always)]
    pub fn ch2fm(&mut self) -> CH2FM_W<'_, C2MRrs> {
        CH2FM_W::new(self, 17)
    }
    ///Bit 18 - CH3FM
    #[inline(always)]
    pub fn ch3fm(&mut self) -> CH3FM_W<'_, C2MRrs> {
        CH3FM_W::new(self, 18)
    }
    ///Bit 19 - CH4FM
    #[inline(always)]
    pub fn ch4fm(&mut self) -> CH4FM_W<'_, C2MRrs> {
        CH4FM_W::new(self, 19)
    }
    ///Bit 20 - CH5FM
    #[inline(always)]
    pub fn ch5fm(&mut self) -> CH5FM_W<'_, C2MRrs> {
        CH5FM_W::new(self, 20)
    }
    ///Bit 21 - CH6FM
    #[inline(always)]
    pub fn ch6fm(&mut self) -> CH6FM_W<'_, C2MRrs> {
        CH6FM_W::new(self, 21)
    }
}
/**IPCC Processor 2 mask register

You can [`read`](crate::Reg::read) this register and get [`c2mr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2mr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#IPCC:C2MR)*/
pub struct C2MRrs;
impl crate::RegisterSpec for C2MRrs {
    type Ux = u32;
}
///`read()` method returns [`c2mr::R`](R) reader structure
impl crate::Readable for C2MRrs {}
///`write(|w| ..)` method takes [`c2mr::W`](W) writer structure
impl crate::Writable for C2MRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C2MR to value 0xffff_ffff
impl crate::Resettable for C2MRrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
