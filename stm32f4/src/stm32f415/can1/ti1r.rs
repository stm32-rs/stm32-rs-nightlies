///Register `TI1R` reader
pub type R = crate::R<TI1Rrs>;
///Register `TI1R` writer
pub type W = crate::W<TI1Rrs>;
///Field `TXRQ` reader - TXRQ
pub type TXRQ_R = crate::BitReader;
///Field `TXRQ` writer - TXRQ
pub type TXRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTR` reader - RTR
pub type RTR_R = crate::BitReader;
///Field `RTR` writer - RTR
pub type RTR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IDE` reader - IDE
pub type IDE_R = crate::BitReader;
///Field `IDE` writer - IDE
pub type IDE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXID` reader - EXID
pub type EXID_R = crate::FieldReader<u32>;
///Field `EXID` writer - EXID
pub type EXID_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
///Field `STID` reader - STID
pub type STID_R = crate::FieldReader<u16>;
///Field `STID` writer - STID
pub type STID_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    ///Bit 0 - TXRQ
    #[inline(always)]
    pub fn txrq(&self) -> TXRQ_R {
        TXRQ_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - RTR
    #[inline(always)]
    pub fn rtr(&self) -> RTR_R {
        RTR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IDE
    #[inline(always)]
    pub fn ide(&self) -> IDE_R {
        IDE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:20 - EXID
    #[inline(always)]
    pub fn exid(&self) -> EXID_R {
        EXID_R::new((self.bits >> 3) & 0x0003_ffff)
    }
    ///Bits 21:31 - STID
    #[inline(always)]
    pub fn stid(&self) -> STID_R {
        STID_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TI1R")
            .field("stid", &self.stid())
            .field("exid", &self.exid())
            .field("ide", &self.ide())
            .field("rtr", &self.rtr())
            .field("txrq", &self.txrq())
            .finish()
    }
}
impl W {
    ///Bit 0 - TXRQ
    #[inline(always)]
    pub fn txrq(&mut self) -> TXRQ_W<'_, TI1Rrs> {
        TXRQ_W::new(self, 0)
    }
    ///Bit 1 - RTR
    #[inline(always)]
    pub fn rtr(&mut self) -> RTR_W<'_, TI1Rrs> {
        RTR_W::new(self, 1)
    }
    ///Bit 2 - IDE
    #[inline(always)]
    pub fn ide(&mut self) -> IDE_W<'_, TI1Rrs> {
        IDE_W::new(self, 2)
    }
    ///Bits 3:20 - EXID
    #[inline(always)]
    pub fn exid(&mut self) -> EXID_W<'_, TI1Rrs> {
        EXID_W::new(self, 3)
    }
    ///Bits 21:31 - STID
    #[inline(always)]
    pub fn stid(&mut self) -> STID_W<'_, TI1Rrs> {
        STID_W::new(self, 21)
    }
}
/**mailbox identifier register

You can [`read`](crate::Reg::read) this register and get [`ti1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ti1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#CAN1:TI1R)*/
pub struct TI1Rrs;
impl crate::RegisterSpec for TI1Rrs {
    type Ux = u32;
}
///`read()` method returns [`ti1r::R`](R) reader structure
impl crate::Readable for TI1Rrs {}
///`write(|w| ..)` method takes [`ti1r::W`](W) writer structure
impl crate::Writable for TI1Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TI1R to value 0
impl crate::Resettable for TI1Rrs {}
