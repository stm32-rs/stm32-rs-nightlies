///Register `MTLOMR` reader
pub type R = crate::R<MTLOMRrs>;
///Register `MTLOMR` writer
pub type W = crate::W<MTLOMRrs>;
///Field `DTXSTS` reader - Drop Transmit Status
pub type DTXSTS_R = crate::BitReader;
///Field `DTXSTS` writer - Drop Transmit Status
pub type DTXSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CNTPRST` reader - Counters Preset
pub type CNTPRST_R = crate::BitReader;
///Field `CNTPRST` writer - Counters Preset
pub type CNTPRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CNTCLR` reader - Counters Reset
pub type CNTCLR_R = crate::BitReader;
///Field `CNTCLR` writer - Counters Reset
pub type CNTCLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - Drop Transmit Status
    #[inline(always)]
    pub fn dtxsts(&self) -> DTXSTS_R {
        DTXSTS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - Counters Preset
    #[inline(always)]
    pub fn cntprst(&self) -> CNTPRST_R {
        CNTPRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Counters Reset
    #[inline(always)]
    pub fn cntclr(&self) -> CNTCLR_R {
        CNTCLR_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLOMR")
            .field("cntclr", &self.cntclr())
            .field("cntprst", &self.cntprst())
            .field("dtxsts", &self.dtxsts())
            .finish()
    }
}
impl W {
    ///Bit 1 - Drop Transmit Status
    #[inline(always)]
    pub fn dtxsts(&mut self) -> DTXSTS_W<'_, MTLOMRrs> {
        DTXSTS_W::new(self, 1)
    }
    ///Bit 8 - Counters Preset
    #[inline(always)]
    pub fn cntprst(&mut self) -> CNTPRST_W<'_, MTLOMRrs> {
        CNTPRST_W::new(self, 8)
    }
    ///Bit 9 - Counters Reset
    #[inline(always)]
    pub fn cntclr(&mut self) -> CNTCLR_W<'_, MTLOMRrs> {
        CNTCLR_W::new(self, 9)
    }
}
/**Operating mode Register

You can [`read`](crate::Reg::read) this register and get [`mtlomr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlomr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#Ethernet_MTL:MTLOMR)*/
pub struct MTLOMRrs;
impl crate::RegisterSpec for MTLOMRrs {
    type Ux = u32;
}
///`read()` method returns [`mtlomr::R`](R) reader structure
impl crate::Readable for MTLOMRrs {}
///`write(|w| ..)` method takes [`mtlomr::W`](W) writer structure
impl crate::Writable for MTLOMRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MTLOMR to value 0
impl crate::Resettable for MTLOMRrs {}
