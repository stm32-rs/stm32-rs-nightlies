#[doc = "Register `C2MR` reader"]
pub type R = crate::R<C2MRrs>;
#[doc = "Register `C2MR` writer"]
pub type W = crate::W<C2MRrs>;
#[doc = "CH1OM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH1OM {
    #[doc = "0: Receive channel n occupied interrupt not masked"]
    Unmasked = 0,
    #[doc = "1: Receive channel n occupied interrupt masked"]
    Masked = 1,
}
impl From<CH1OM> for bool {
    #[inline(always)]
    fn from(variant: CH1OM) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1OM` reader - CH1OM"]
pub type CH1OM_R = crate::BitReader<CH1OM>;
impl CH1OM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CH1OM {
        match self.bits {
            false => CH1OM::Unmasked,
            true => CH1OM::Masked,
        }
    }
    #[doc = "Receive channel n occupied interrupt not masked"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == CH1OM::Unmasked
    }
    #[doc = "Receive channel n occupied interrupt masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == CH1OM::Masked
    }
}
#[doc = "Field `CH1OM` writer - CH1OM"]
pub type CH1OM_W<'a, REG> = crate::BitWriter<'a, REG, CH1OM>;
impl<'a, REG> CH1OM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receive channel n occupied interrupt not masked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(CH1OM::Unmasked)
    }
    #[doc = "Receive channel n occupied interrupt masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(CH1OM::Masked)
    }
}
#[doc = "Field `CH2OM` reader - CH2OM"]
pub use CH1OM_R as CH2OM_R;
#[doc = "Field `CH3OM` reader - CH3OM"]
pub use CH1OM_R as CH3OM_R;
#[doc = "Field `CH4OM` reader - CH4OM"]
pub use CH1OM_R as CH4OM_R;
#[doc = "Field `CH5OM` reader - CH5OM"]
pub use CH1OM_R as CH5OM_R;
#[doc = "Field `CH6OM` reader - CH6OM"]
pub use CH1OM_R as CH6OM_R;
#[doc = "Field `CH2OM` writer - CH2OM"]
pub use CH1OM_W as CH2OM_W;
#[doc = "Field `CH3OM` writer - CH3OM"]
pub use CH1OM_W as CH3OM_W;
#[doc = "Field `CH4OM` writer - CH4OM"]
pub use CH1OM_W as CH4OM_W;
#[doc = "Field `CH5OM` writer - CH5OM"]
pub use CH1OM_W as CH5OM_W;
#[doc = "Field `CH6OM` writer - CH6OM"]
pub use CH1OM_W as CH6OM_W;
#[doc = "CH1FM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH1FM {
    #[doc = "0: Transmit channel n free interrupt not masked"]
    Unmasked = 0,
    #[doc = "1: Transmit channel n free interrupt masked"]
    Masked = 1,
}
impl From<CH1FM> for bool {
    #[inline(always)]
    fn from(variant: CH1FM) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1FM` reader - CH1FM"]
pub type CH1FM_R = crate::BitReader<CH1FM>;
impl CH1FM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CH1FM {
        match self.bits {
            false => CH1FM::Unmasked,
            true => CH1FM::Masked,
        }
    }
    #[doc = "Transmit channel n free interrupt not masked"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == CH1FM::Unmasked
    }
    #[doc = "Transmit channel n free interrupt masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == CH1FM::Masked
    }
}
#[doc = "Field `CH1FM` writer - CH1FM"]
pub type CH1FM_W<'a, REG> = crate::BitWriter<'a, REG, CH1FM>;
impl<'a, REG> CH1FM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit channel n free interrupt not masked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(CH1FM::Unmasked)
    }
    #[doc = "Transmit channel n free interrupt masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(CH1FM::Masked)
    }
}
#[doc = "Field `CH2FM` reader - CH2FM"]
pub use CH1FM_R as CH2FM_R;
#[doc = "Field `CH3FM` reader - CH3FM"]
pub use CH1FM_R as CH3FM_R;
#[doc = "Field `CH4FM` reader - CH4FM"]
pub use CH1FM_R as CH4FM_R;
#[doc = "Field `CH5FM` reader - CH5FM"]
pub use CH1FM_R as CH5FM_R;
#[doc = "Field `CH6FM` reader - CH6FM"]
pub use CH1FM_R as CH6FM_R;
#[doc = "Field `CH2FM` writer - CH2FM"]
pub use CH1FM_W as CH2FM_W;
#[doc = "Field `CH3FM` writer - CH3FM"]
pub use CH1FM_W as CH3FM_W;
#[doc = "Field `CH4FM` writer - CH4FM"]
pub use CH1FM_W as CH4FM_W;
#[doc = "Field `CH5FM` writer - CH5FM"]
pub use CH1FM_W as CH5FM_W;
#[doc = "Field `CH6FM` writer - CH6FM"]
pub use CH1FM_W as CH6FM_W;
impl R {
    #[doc = "Bit 0 - CH1OM"]
    #[inline(always)]
    pub fn ch1om(&self) -> CH1OM_R {
        CH1OM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CH2OM"]
    #[inline(always)]
    pub fn ch2om(&self) -> CH2OM_R {
        CH2OM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CH3OM"]
    #[inline(always)]
    pub fn ch3om(&self) -> CH3OM_R {
        CH3OM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CH4OM"]
    #[inline(always)]
    pub fn ch4om(&self) -> CH4OM_R {
        CH4OM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CH5OM"]
    #[inline(always)]
    pub fn ch5om(&self) -> CH5OM_R {
        CH5OM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CH6OM"]
    #[inline(always)]
    pub fn ch6om(&self) -> CH6OM_R {
        CH6OM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - CH1FM"]
    #[inline(always)]
    pub fn ch1fm(&self) -> CH1FM_R {
        CH1FM_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CH2FM"]
    #[inline(always)]
    pub fn ch2fm(&self) -> CH2FM_R {
        CH2FM_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CH3FM"]
    #[inline(always)]
    pub fn ch3fm(&self) -> CH3FM_R {
        CH3FM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CH4FM"]
    #[inline(always)]
    pub fn ch4fm(&self) -> CH4FM_R {
        CH4FM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - CH5FM"]
    #[inline(always)]
    pub fn ch5fm(&self) -> CH5FM_R {
        CH5FM_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CH6FM"]
    #[inline(always)]
    pub fn ch6fm(&self) -> CH6FM_R {
        CH6FM_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH1OM"]
    #[inline(always)]
    #[must_use]
    pub fn ch1om(&mut self) -> CH1OM_W<C2MRrs> {
        CH1OM_W::new(self, 0)
    }
    #[doc = "Bit 1 - CH2OM"]
    #[inline(always)]
    #[must_use]
    pub fn ch2om(&mut self) -> CH2OM_W<C2MRrs> {
        CH2OM_W::new(self, 1)
    }
    #[doc = "Bit 2 - CH3OM"]
    #[inline(always)]
    #[must_use]
    pub fn ch3om(&mut self) -> CH3OM_W<C2MRrs> {
        CH3OM_W::new(self, 2)
    }
    #[doc = "Bit 3 - CH4OM"]
    #[inline(always)]
    #[must_use]
    pub fn ch4om(&mut self) -> CH4OM_W<C2MRrs> {
        CH4OM_W::new(self, 3)
    }
    #[doc = "Bit 4 - CH5OM"]
    #[inline(always)]
    #[must_use]
    pub fn ch5om(&mut self) -> CH5OM_W<C2MRrs> {
        CH5OM_W::new(self, 4)
    }
    #[doc = "Bit 5 - CH6OM"]
    #[inline(always)]
    #[must_use]
    pub fn ch6om(&mut self) -> CH6OM_W<C2MRrs> {
        CH6OM_W::new(self, 5)
    }
    #[doc = "Bit 16 - CH1FM"]
    #[inline(always)]
    #[must_use]
    pub fn ch1fm(&mut self) -> CH1FM_W<C2MRrs> {
        CH1FM_W::new(self, 16)
    }
    #[doc = "Bit 17 - CH2FM"]
    #[inline(always)]
    #[must_use]
    pub fn ch2fm(&mut self) -> CH2FM_W<C2MRrs> {
        CH2FM_W::new(self, 17)
    }
    #[doc = "Bit 18 - CH3FM"]
    #[inline(always)]
    #[must_use]
    pub fn ch3fm(&mut self) -> CH3FM_W<C2MRrs> {
        CH3FM_W::new(self, 18)
    }
    #[doc = "Bit 19 - CH4FM"]
    #[inline(always)]
    #[must_use]
    pub fn ch4fm(&mut self) -> CH4FM_W<C2MRrs> {
        CH4FM_W::new(self, 19)
    }
    #[doc = "Bit 20 - CH5FM"]
    #[inline(always)]
    #[must_use]
    pub fn ch5fm(&mut self) -> CH5FM_W<C2MRrs> {
        CH5FM_W::new(self, 20)
    }
    #[doc = "Bit 21 - CH6FM"]
    #[inline(always)]
    #[must_use]
    pub fn ch6fm(&mut self) -> CH6FM_W<C2MRrs> {
        CH6FM_W::new(self, 21)
    }
}
#[doc = "IPCC Processor 2 mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2mr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2mr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2MRrs;
impl crate::RegisterSpec for C2MRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c2mr::R`](R) reader structure"]
impl crate::Readable for C2MRrs {}
#[doc = "`write(|w| ..)` method takes [`c2mr::W`](W) writer structure"]
impl crate::Writable for C2MRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C2MR to value 0xffff_ffff"]
impl crate::Resettable for C2MRrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
