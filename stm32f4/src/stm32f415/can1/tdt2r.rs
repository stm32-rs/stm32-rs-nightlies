///Register `TDT2R` reader
pub type R = crate::R<TDT2Rrs>;
///Register `TDT2R` writer
pub type W = crate::W<TDT2Rrs>;
///Field `DLC` reader - DLC
pub type DLC_R = crate::FieldReader;
///Field `DLC` writer - DLC
pub type DLC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TGT` reader - TGT
pub type TGT_R = crate::BitReader;
///Field `TGT` writer - TGT
pub type TGT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIME` reader - TIME
pub type TIME_R = crate::FieldReader<u16>;
///Field `TIME` writer - TIME
pub type TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:3 - DLC
    #[inline(always)]
    pub fn dlc(&self) -> DLC_R {
        DLC_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 8 - TGT
    #[inline(always)]
    pub fn tgt(&self) -> TGT_R {
        TGT_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 16:31 - TIME
    #[inline(always)]
    pub fn time(&self) -> TIME_R {
        TIME_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TDT2R")
            .field("time", &self.time())
            .field("tgt", &self.tgt())
            .field("dlc", &self.dlc())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - DLC
    #[inline(always)]
    pub fn dlc(&mut self) -> DLC_W<'_, TDT2Rrs> {
        DLC_W::new(self, 0)
    }
    ///Bit 8 - TGT
    #[inline(always)]
    pub fn tgt(&mut self) -> TGT_W<'_, TDT2Rrs> {
        TGT_W::new(self, 8)
    }
    ///Bits 16:31 - TIME
    #[inline(always)]
    pub fn time(&mut self) -> TIME_W<'_, TDT2Rrs> {
        TIME_W::new(self, 16)
    }
}
/**mailbox data length control and time stamp register

You can [`read`](crate::Reg::read) this register and get [`tdt2r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdt2r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#CAN1:TDT2R)*/
pub struct TDT2Rrs;
impl crate::RegisterSpec for TDT2Rrs {
    type Ux = u32;
}
///`read()` method returns [`tdt2r::R`](R) reader structure
impl crate::Readable for TDT2Rrs {}
///`write(|w| ..)` method takes [`tdt2r::W`](W) writer structure
impl crate::Writable for TDT2Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TDT2R to value 0
impl crate::Resettable for TDT2Rrs {}
