///Register `TTTMK` reader
pub type R = crate::R<TTTMKrs>;
///Register `TTTMK` writer
pub type W = crate::W<TTTMKrs>;
///Field `TM` reader - TM
pub type TM_R = crate::FieldReader<u16>;
///Field `TM` writer - TM
pub type TM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `TICC` reader - TICC
pub type TICC_R = crate::FieldReader;
///Field `TICC` writer - TICC
pub type TICC_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `LCKM` reader - LCKM
pub type LCKM_R = crate::BitReader;
impl R {
    ///Bits 0:15 - TM
    #[inline(always)]
    pub fn tm(&self) -> TM_R {
        TM_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:22 - TICC
    #[inline(always)]
    pub fn ticc(&self) -> TICC_R {
        TICC_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    ///Bit 31 - LCKM
    #[inline(always)]
    pub fn lckm(&self) -> LCKM_R {
        LCKM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TTTMK")
            .field("tm", &self.tm())
            .field("ticc", &self.ticc())
            .field("lckm", &self.lckm())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - TM
    #[inline(always)]
    pub fn tm(&mut self) -> TM_W<TTTMKrs> {
        TM_W::new(self, 0)
    }
    ///Bits 16:22 - TICC
    #[inline(always)]
    pub fn ticc(&mut self) -> TICC_W<TTTMKrs> {
        TICC_W::new(self, 16)
    }
}
/**A time mark interrupt (FDCAN_TTIR.TMI = 1) is generated when the time base indicated by FDCAN_TTOCN.TMC (cycle time, local time, or global time) has the same value as TM.

You can [`read`](crate::Reg::read) this register and get [`tttmk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tttmk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:TTTMK)*/
pub struct TTTMKrs;
impl crate::RegisterSpec for TTTMKrs {
    type Ux = u32;
}
///`read()` method returns [`tttmk::R`](R) reader structure
impl crate::Readable for TTTMKrs {}
///`write(|w| ..)` method takes [`tttmk::W`](W) writer structure
impl crate::Writable for TTTMKrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TTTMK to value 0
impl crate::Resettable for TTTMKrs {}
