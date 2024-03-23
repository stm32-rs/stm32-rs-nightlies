#[doc = "Register `C1MR` reader"]
pub type R = crate::R<C1MRrs>;
#[doc = "Register `C1MR` writer"]
pub type W = crate::W<C1MRrs>;
#[doc = "Field `CH1OM` reader - processor 1 Receive channel 1 occupied interrupt enable"]
pub type CH1OM_R = crate::BitReader;
#[doc = "Field `CH1OM` writer - processor 1 Receive channel 1 occupied interrupt enable"]
pub type CH1OM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2OM` reader - processor 1 Receive channel 2 occupied interrupt enable"]
pub type CH2OM_R = crate::BitReader;
#[doc = "Field `CH2OM` writer - processor 1 Receive channel 2 occupied interrupt enable"]
pub type CH2OM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3OM` reader - processor 1 Receive channel 3 occupied interrupt enable"]
pub type CH3OM_R = crate::BitReader;
#[doc = "Field `CH3OM` writer - processor 1 Receive channel 3 occupied interrupt enable"]
pub type CH3OM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4OM` reader - processor 1 Receive channel 4 occupied interrupt enable"]
pub type CH4OM_R = crate::BitReader;
#[doc = "Field `CH4OM` writer - processor 1 Receive channel 4 occupied interrupt enable"]
pub type CH4OM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5OM` reader - processor 1 Receive channel 5 occupied interrupt enable"]
pub type CH5OM_R = crate::BitReader;
#[doc = "Field `CH5OM` writer - processor 1 Receive channel 5 occupied interrupt enable"]
pub type CH5OM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH6OM` reader - processor 1 Receive channel 6 occupied interrupt enable"]
pub type CH6OM_R = crate::BitReader;
#[doc = "Field `CH6OM` writer - processor 1 Receive channel 6 occupied interrupt enable"]
pub type CH6OM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1FM` reader - processor 1 Transmit channel 1 free interrupt mask"]
pub type CH1FM_R = crate::BitReader;
#[doc = "Field `CH1FM` writer - processor 1 Transmit channel 1 free interrupt mask"]
pub type CH1FM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2FM` reader - processor 1 Transmit channel 2 free interrupt mask"]
pub type CH2FM_R = crate::BitReader;
#[doc = "Field `CH2FM` writer - processor 1 Transmit channel 2 free interrupt mask"]
pub type CH2FM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3FM` reader - processor 1 Transmit channel 3 free interrupt mask"]
pub type CH3FM_R = crate::BitReader;
#[doc = "Field `CH3FM` writer - processor 1 Transmit channel 3 free interrupt mask"]
pub type CH3FM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4FM` reader - processor 1 Transmit channel 4 free interrupt mask"]
pub type CH4FM_R = crate::BitReader;
#[doc = "Field `CH4FM` writer - processor 1 Transmit channel 4 free interrupt mask"]
pub type CH4FM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5FM` reader - processor 1 Transmit channel 5 free interrupt mask"]
pub type CH5FM_R = crate::BitReader;
#[doc = "Field `CH5FM` writer - processor 1 Transmit channel 5 free interrupt mask"]
pub type CH5FM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH6FM` reader - processor 1 Transmit channel 6 free interrupt mask"]
pub type CH6FM_R = crate::BitReader;
#[doc = "Field `CH6FM` writer - processor 1 Transmit channel 6 free interrupt mask"]
pub type CH6FM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - processor 1 Receive channel 1 occupied interrupt enable"]
    #[inline(always)]
    pub fn ch1om(&self) -> CH1OM_R {
        CH1OM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - processor 1 Receive channel 2 occupied interrupt enable"]
    #[inline(always)]
    pub fn ch2om(&self) -> CH2OM_R {
        CH2OM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - processor 1 Receive channel 3 occupied interrupt enable"]
    #[inline(always)]
    pub fn ch3om(&self) -> CH3OM_R {
        CH3OM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - processor 1 Receive channel 4 occupied interrupt enable"]
    #[inline(always)]
    pub fn ch4om(&self) -> CH4OM_R {
        CH4OM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - processor 1 Receive channel 5 occupied interrupt enable"]
    #[inline(always)]
    pub fn ch5om(&self) -> CH5OM_R {
        CH5OM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - processor 1 Receive channel 6 occupied interrupt enable"]
    #[inline(always)]
    pub fn ch6om(&self) -> CH6OM_R {
        CH6OM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - processor 1 Transmit channel 1 free interrupt mask"]
    #[inline(always)]
    pub fn ch1fm(&self) -> CH1FM_R {
        CH1FM_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - processor 1 Transmit channel 2 free interrupt mask"]
    #[inline(always)]
    pub fn ch2fm(&self) -> CH2FM_R {
        CH2FM_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - processor 1 Transmit channel 3 free interrupt mask"]
    #[inline(always)]
    pub fn ch3fm(&self) -> CH3FM_R {
        CH3FM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - processor 1 Transmit channel 4 free interrupt mask"]
    #[inline(always)]
    pub fn ch4fm(&self) -> CH4FM_R {
        CH4FM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - processor 1 Transmit channel 5 free interrupt mask"]
    #[inline(always)]
    pub fn ch5fm(&self) -> CH5FM_R {
        CH5FM_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - processor 1 Transmit channel 6 free interrupt mask"]
    #[inline(always)]
    pub fn ch6fm(&self) -> CH6FM_R {
        CH6FM_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - processor 1 Receive channel 1 occupied interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1om(&mut self) -> CH1OM_W<C1MRrs> {
        CH1OM_W::new(self, 0)
    }
    #[doc = "Bit 1 - processor 1 Receive channel 2 occupied interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2om(&mut self) -> CH2OM_W<C1MRrs> {
        CH2OM_W::new(self, 1)
    }
    #[doc = "Bit 2 - processor 1 Receive channel 3 occupied interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch3om(&mut self) -> CH3OM_W<C1MRrs> {
        CH3OM_W::new(self, 2)
    }
    #[doc = "Bit 3 - processor 1 Receive channel 4 occupied interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch4om(&mut self) -> CH4OM_W<C1MRrs> {
        CH4OM_W::new(self, 3)
    }
    #[doc = "Bit 4 - processor 1 Receive channel 5 occupied interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch5om(&mut self) -> CH5OM_W<C1MRrs> {
        CH5OM_W::new(self, 4)
    }
    #[doc = "Bit 5 - processor 1 Receive channel 6 occupied interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch6om(&mut self) -> CH6OM_W<C1MRrs> {
        CH6OM_W::new(self, 5)
    }
    #[doc = "Bit 16 - processor 1 Transmit channel 1 free interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn ch1fm(&mut self) -> CH1FM_W<C1MRrs> {
        CH1FM_W::new(self, 16)
    }
    #[doc = "Bit 17 - processor 1 Transmit channel 2 free interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn ch2fm(&mut self) -> CH2FM_W<C1MRrs> {
        CH2FM_W::new(self, 17)
    }
    #[doc = "Bit 18 - processor 1 Transmit channel 3 free interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn ch3fm(&mut self) -> CH3FM_W<C1MRrs> {
        CH3FM_W::new(self, 18)
    }
    #[doc = "Bit 19 - processor 1 Transmit channel 4 free interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn ch4fm(&mut self) -> CH4FM_W<C1MRrs> {
        CH4FM_W::new(self, 19)
    }
    #[doc = "Bit 20 - processor 1 Transmit channel 5 free interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn ch5fm(&mut self) -> CH5FM_W<C1MRrs> {
        CH5FM_W::new(self, 20)
    }
    #[doc = "Bit 21 - processor 1 Transmit channel 6 free interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn ch6fm(&mut self) -> CH6FM_W<C1MRrs> {
        CH6FM_W::new(self, 21)
    }
}
#[doc = "Mask register CPU1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1mr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1mr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1MRrs;
impl crate::RegisterSpec for C1MRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c1mr::R`](R) reader structure"]
impl crate::Readable for C1MRrs {}
#[doc = "`write(|w| ..)` method takes [`c1mr::W`](W) writer structure"]
impl crate::Writable for C1MRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C1MR to value 0xffff_ffff"]
impl crate::Resettable for C1MRrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
