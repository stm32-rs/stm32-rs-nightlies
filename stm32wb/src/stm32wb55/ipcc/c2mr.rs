///Register `C2MR` reader
pub type R = crate::R<C2MRrs>;
///Register `C2MR` writer
pub type W = crate::W<C2MRrs>;
///Field `CH1OM` reader - processor 2 Receive channel 1 occupied interrupt enable
pub type CH1OM_R = crate::BitReader;
///Field `CH1OM` writer - processor 2 Receive channel 1 occupied interrupt enable
pub type CH1OM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH2OM` reader - processor 2 Receive channel 2 occupied interrupt enable
pub type CH2OM_R = crate::BitReader;
///Field `CH2OM` writer - processor 2 Receive channel 2 occupied interrupt enable
pub type CH2OM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH3OM` reader - processor 2 Receive channel 3 occupied interrupt enable
pub type CH3OM_R = crate::BitReader;
///Field `CH3OM` writer - processor 2 Receive channel 3 occupied interrupt enable
pub type CH3OM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH4OM` reader - processor 2 Receive channel 4 occupied interrupt enable
pub type CH4OM_R = crate::BitReader;
///Field `CH4OM` writer - processor 2 Receive channel 4 occupied interrupt enable
pub type CH4OM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH5OM` reader - processor 2 Receive channel 5 occupied interrupt enable
pub type CH5OM_R = crate::BitReader;
///Field `CH5OM` writer - processor 2 Receive channel 5 occupied interrupt enable
pub type CH5OM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH6OM` reader - processor 2 Receive channel 6 occupied interrupt enable
pub type CH6OM_R = crate::BitReader;
///Field `CH6OM` writer - processor 2 Receive channel 6 occupied interrupt enable
pub type CH6OM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH1FM` reader - processor 2 Transmit channel 1 free interrupt mask
pub type CH1FM_R = crate::BitReader;
///Field `CH1FM` writer - processor 2 Transmit channel 1 free interrupt mask
pub type CH1FM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH2FM` reader - processor 2 Transmit channel 2 free interrupt mask
pub type CH2FM_R = crate::BitReader;
///Field `CH2FM` writer - processor 2 Transmit channel 2 free interrupt mask
pub type CH2FM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH3FM` reader - processor 2 Transmit channel 3 free interrupt mask
pub type CH3FM_R = crate::BitReader;
///Field `CH3FM` writer - processor 2 Transmit channel 3 free interrupt mask
pub type CH3FM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH4FM` reader - processor 2 Transmit channel 4 free interrupt mask
pub type CH4FM_R = crate::BitReader;
///Field `CH4FM` writer - processor 2 Transmit channel 4 free interrupt mask
pub type CH4FM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH5FM` reader - processor 2 Transmit channel 5 free interrupt mask
pub type CH5FM_R = crate::BitReader;
///Field `CH5FM` writer - processor 2 Transmit channel 5 free interrupt mask
pub type CH5FM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH6FM` reader - processor 2 Transmit channel 6 free interrupt mask
pub type CH6FM_R = crate::BitReader;
///Field `CH6FM` writer - processor 2 Transmit channel 6 free interrupt mask
pub type CH6FM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - processor 2 Receive channel 1 occupied interrupt enable
    #[inline(always)]
    pub fn ch1om(&self) -> CH1OM_R {
        CH1OM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - processor 2 Receive channel 2 occupied interrupt enable
    #[inline(always)]
    pub fn ch2om(&self) -> CH2OM_R {
        CH2OM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - processor 2 Receive channel 3 occupied interrupt enable
    #[inline(always)]
    pub fn ch3om(&self) -> CH3OM_R {
        CH3OM_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - processor 2 Receive channel 4 occupied interrupt enable
    #[inline(always)]
    pub fn ch4om(&self) -> CH4OM_R {
        CH4OM_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - processor 2 Receive channel 5 occupied interrupt enable
    #[inline(always)]
    pub fn ch5om(&self) -> CH5OM_R {
        CH5OM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - processor 2 Receive channel 6 occupied interrupt enable
    #[inline(always)]
    pub fn ch6om(&self) -> CH6OM_R {
        CH6OM_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 16 - processor 2 Transmit channel 1 free interrupt mask
    #[inline(always)]
    pub fn ch1fm(&self) -> CH1FM_R {
        CH1FM_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - processor 2 Transmit channel 2 free interrupt mask
    #[inline(always)]
    pub fn ch2fm(&self) -> CH2FM_R {
        CH2FM_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - processor 2 Transmit channel 3 free interrupt mask
    #[inline(always)]
    pub fn ch3fm(&self) -> CH3FM_R {
        CH3FM_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - processor 2 Transmit channel 4 free interrupt mask
    #[inline(always)]
    pub fn ch4fm(&self) -> CH4FM_R {
        CH4FM_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - processor 2 Transmit channel 5 free interrupt mask
    #[inline(always)]
    pub fn ch5fm(&self) -> CH5FM_R {
        CH5FM_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - processor 2 Transmit channel 6 free interrupt mask
    #[inline(always)]
    pub fn ch6fm(&self) -> CH6FM_R {
        CH6FM_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C2MR")
            .field("ch6fm", &self.ch6fm())
            .field("ch5fm", &self.ch5fm())
            .field("ch4fm", &self.ch4fm())
            .field("ch3fm", &self.ch3fm())
            .field("ch2fm", &self.ch2fm())
            .field("ch1fm", &self.ch1fm())
            .field("ch6om", &self.ch6om())
            .field("ch5om", &self.ch5om())
            .field("ch4om", &self.ch4om())
            .field("ch3om", &self.ch3om())
            .field("ch2om", &self.ch2om())
            .field("ch1om", &self.ch1om())
            .finish()
    }
}
impl W {
    ///Bit 0 - processor 2 Receive channel 1 occupied interrupt enable
    #[inline(always)]
    pub fn ch1om(&mut self) -> CH1OM_W<'_, C2MRrs> {
        CH1OM_W::new(self, 0)
    }
    ///Bit 1 - processor 2 Receive channel 2 occupied interrupt enable
    #[inline(always)]
    pub fn ch2om(&mut self) -> CH2OM_W<'_, C2MRrs> {
        CH2OM_W::new(self, 1)
    }
    ///Bit 2 - processor 2 Receive channel 3 occupied interrupt enable
    #[inline(always)]
    pub fn ch3om(&mut self) -> CH3OM_W<'_, C2MRrs> {
        CH3OM_W::new(self, 2)
    }
    ///Bit 3 - processor 2 Receive channel 4 occupied interrupt enable
    #[inline(always)]
    pub fn ch4om(&mut self) -> CH4OM_W<'_, C2MRrs> {
        CH4OM_W::new(self, 3)
    }
    ///Bit 4 - processor 2 Receive channel 5 occupied interrupt enable
    #[inline(always)]
    pub fn ch5om(&mut self) -> CH5OM_W<'_, C2MRrs> {
        CH5OM_W::new(self, 4)
    }
    ///Bit 5 - processor 2 Receive channel 6 occupied interrupt enable
    #[inline(always)]
    pub fn ch6om(&mut self) -> CH6OM_W<'_, C2MRrs> {
        CH6OM_W::new(self, 5)
    }
    ///Bit 16 - processor 2 Transmit channel 1 free interrupt mask
    #[inline(always)]
    pub fn ch1fm(&mut self) -> CH1FM_W<'_, C2MRrs> {
        CH1FM_W::new(self, 16)
    }
    ///Bit 17 - processor 2 Transmit channel 2 free interrupt mask
    #[inline(always)]
    pub fn ch2fm(&mut self) -> CH2FM_W<'_, C2MRrs> {
        CH2FM_W::new(self, 17)
    }
    ///Bit 18 - processor 2 Transmit channel 3 free interrupt mask
    #[inline(always)]
    pub fn ch3fm(&mut self) -> CH3FM_W<'_, C2MRrs> {
        CH3FM_W::new(self, 18)
    }
    ///Bit 19 - processor 2 Transmit channel 4 free interrupt mask
    #[inline(always)]
    pub fn ch4fm(&mut self) -> CH4FM_W<'_, C2MRrs> {
        CH4FM_W::new(self, 19)
    }
    ///Bit 20 - processor 2 Transmit channel 5 free interrupt mask
    #[inline(always)]
    pub fn ch5fm(&mut self) -> CH5FM_W<'_, C2MRrs> {
        CH5FM_W::new(self, 20)
    }
    ///Bit 21 - processor 2 Transmit channel 6 free interrupt mask
    #[inline(always)]
    pub fn ch6fm(&mut self) -> CH6FM_W<'_, C2MRrs> {
        CH6FM_W::new(self, 21)
    }
}
/**Mask register CPU2

You can [`read`](crate::Reg::read) this register and get [`c2mr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2mr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#IPCC:C2MR)*/
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
